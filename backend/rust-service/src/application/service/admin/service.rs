use serde_json::json;

use crate::application::{
    repository::admin::{
        check,
        create,
        delete,
        find,
        get,
        row::{AdminAuditRow, AdminStatsRow, AdminUserPage, AdminUserRow, ListUsersOpts},
        update,
    },
    service::{admin::types::AdminUserDetail, errors::AdminServiceError},
    state::AppState,
};

/// Moderation actions, and the reads that feed the admin panel.
///
/// Stateless, so every method is an associated function. That matches
/// `auth::AuthService` and `SessionService`, the other services with no fields;
/// the `&self` style in `PostService` exists only because those are called off a
/// value on `AppState`.
///
/// The authorization gate is deliberately *not* here. `is_superuser` is checked
/// by the `AdminAuth` extractor on every request, against the database, which
/// keeps role checks out of the path of any handler that forgets to call one.
pub struct AdminService;

impl AdminService {
    /// One page of users, exact search matches first.
    ///
    /// Read-only, so no commit: the transaction is dropped at the end of the
    /// scope and rolled back, exactly as `post_service::get_user_posts` does.
    ///
    /// Takes the cursor primitives rather than a `ListUsersOpts` so that
    /// building the options — in particular parsing `query_id` out of the
    /// free-text query — stays in one place instead of being repeated per
    /// handler. `limit` is not clamped here; the repository clamps to `1..=200`
    /// as a backstop, and the handler applies the intended default.
    pub async fn list_users(
        state: &AppState,
        query: Option<&str>,
        is_active: Option<bool>,
        is_superuser: Option<bool>,
        before: Option<chrono::DateTime<chrono::Utc>>,
        before_id: Option<i64>,
        limit: i64,
    ) -> Result<AdminUserPage, AdminServiceError> {
        let opts = ListUsersOpts {
            query: query.map(str::to_owned),
            query_id: query.and_then(Self::parse_id),
            is_active,
            is_superuser,
            before,
            before_id,
            limit,
        };

        let mut tx = state.db_pool.begin().await?;

        let page = get::list_users(&mut tx, &opts).await?;

        Ok(page)
    }

    /// One user plus their moderation history, read in one transaction so the
    /// two halves cannot disagree about whether the user exists.
    ///
    /// Unlike the list, this includes soft-deleted users: a moderator needs to
    /// be able to inspect a deleted account, and `AdminUserRow::deleted_at` says
    /// which it was.
    pub async fn get_user_detail(
        state: &AppState,
        target_id: i64,
        audit_limit: i64,
    ) -> Result<AdminUserDetail, AdminServiceError> {
        // Clamped for the same reason `list_users` is: the caller is expected to
        // pick a sane default, and an unbounded audit history is not one thing
        // this endpoint should be able to request by accident.
        let audit_limit = audit_limit.clamp(1, 200);

        let mut tx = state.db_pool.begin().await?;

        let Some(user) = find::find_by_id(&mut tx, target_id).await? else {
            return Err(AdminServiceError::UserNotFound);
        };

        let audit = get::list_for_target(&mut tx, target_id, audit_limit).await?;

        Ok(AdminUserDetail { user, audit })
    }

    /// Suspend (`is_active = false`) or reinstate (`is_active = true`) a user.
    ///
    /// Serves both `POST /suspend` and `POST /activate`; only the direction
    /// differs, and the audit `action` is derived from it.
    ///
    /// A suspension that is already suspended is a no-op in the database but
    /// still writes an audit row. Postgres reports a matched row as affected even
    /// when `SET` writes the value it already held, so there is no way to tell
    /// the two apart here — and recording that an admin tried is more useful
    /// than recording nothing.
    pub async fn set_user_active(
        state: &AppState,
        performed_by: i64,
        target_id: i64,
        is_active: bool,
    ) -> Result<AdminUserRow, AdminServiceError> {
        // Checked before the transaction opens: this is a property of the
        // request, not of the database state, so there is nothing to read.
        if target_id == performed_by && !is_active {
            return Err(AdminServiceError::SelfSuspensionForbidden);
        }

        let mut tx = state.db_pool.begin().await?;

        let Some(previous) = find::find_by_id(&mut tx, target_id).await? else {
            return Err(AdminServiceError::UserNotFound);
        };

        let updated = update::set_is_active(&mut tx, target_id, is_active).await?;
        if updated == 0 {
            return Err(AdminServiceError::UserNotFound);
        }

        if !is_active {
            delete::delete_sessions(&mut tx, target_id).await?;
        }

        // Same transaction as the `users` update above, deliberately. A
        // committed suspension with no audit row, or an audit row for a
        // suspension that rolled back, are both worse than no trail at all.
        //
        // Note what this does *not* do: the redis `session_active:*` keys are
        // left alone. `validate_session` returns on a redis hit without
        // re-reading postgres, so a suspended user's live token keeps working
        // until that key's TTL expires on its own. The rows are gone from the
        // authoritative store, which is the part that is correct; bounding the
        // delay is a session-layer fix and out of scope here.
        let action = if is_active { "activate" } else { "suspend" };

        let audit_id = state.snowflake_generator.generate_id()?;
        create::write(
            &mut tx,
            audit_id,
            target_id,
            "user",
            action,
            performed_by,
            json!({
                "is_active": {
                    "previous": previous.is_active,
                    "new": is_active,
                }
            }),
        )
        .await?;

        // Re-read rather than patching the pre-update row, so the `updated_at`
        // in the response is the one the database actually stored.
        let user = find::find_by_id(&mut tx, target_id)
            .await?
            .ok_or(AdminServiceError::UserNotFound)?;

        tx.commit().await?;

        Ok(user)
    }

    /// Grant or revoke superuser.
    ///
    /// Serves `PATCH /admin/users/{id}/role` with `{ "is_superuser": bool }`.
    pub async fn update_user_role(
        state: &AppState,
        performed_by: i64,
        target_id: i64,
        is_superuser: bool,
    ) -> Result<AdminUserRow, AdminServiceError> {
        // Self-*promotion* is allowed and harmless; only revocation is refused.
        if target_id == performed_by && !is_superuser {
            return Err(AdminServiceError::SelfDemotionForbidden);
        }

        let mut tx = state.db_pool.begin().await?;

        let Some(previous) = find::find_by_id(&mut tx, target_id).await? else {
            return Err(AdminServiceError::UserNotFound);
        };

        // The `previous.is_superuser` term is load-bearing. Without it, asking
        // for a regular user to stay regular would be refused whenever exactly
        // one superuser exists — that user is not in the count, so demoting them
        // removes nothing and there is nothing to protect. The count is only
        // taken when this call would actually remove a superuser.
        if !is_superuser && previous.is_superuser {
            // Takes the transaction-scoped advisory lock, then counts, so two
            // concurrent demotions serialize: the second observes the first's
            // committed write instead of both reading a stale count of 2.
            let superusers = check::count_superusers_locked(&mut tx).await?;
            if superusers <= 1 {
                return Err(AdminServiceError::LastSuperuserProtected);
            }
        }

        let updated = update::set_is_superuser(&mut tx, target_id, is_superuser).await?;
        if updated == 0 {
            return Err(AdminServiceError::UserNotFound);
        }

        let action = if is_superuser { "grant_role" } else { "revoke_role" };

        let audit_id = state.snowflake_generator.generate_id()?;
        create::write(
            &mut tx,
            audit_id,
            target_id,
            "user",
            action,
            performed_by,
            json!({
                "is_superuser": {
                    "previous": previous.is_superuser,
                    "new": is_superuser,
                }
            }),
        )
        .await?;

        let user = find::find_by_id(&mut tx, target_id)
            .await?
            .ok_or(AdminServiceError::UserNotFound)?;

        tx.commit().await?;

        Ok(user)
    }

    /// Dashboard counts, one round trip.
    pub async fn get_stats(state: &AppState) -> Result<AdminStatsRow, AdminServiceError> {
        let mut tx = state.db_pool.begin().await?;

        let stats = get::stats(&mut tx).await?;

        Ok(stats)
    }

    /// Most recent admin actions across every target, newest first.
    pub async fn list_audit(
        state: &AppState,
        limit: i64,
    ) -> Result<Vec<AdminAuditRow>, AdminServiceError> {
        let mut tx = state.db_pool.begin().await?;

        let entries = get::list_recent(&mut tx, limit.clamp(1, 200)).await?;

        Ok(entries)
    }

    /// Pre-parse a purely numeric search term as a user id.
    ///
    /// The repository keeps `query_id` separate from `query` because `u.id = $n`
    /// with a real `i64` bind stays index-eligible, where `u.id::TEXT = $n` or a
    /// regex-guarded cast does not.
    ///
    /// This is an *addition* to the substring search, not a replacement for it:
    /// a username is allowed to be all digits, so the substring branch still has
    /// to run or that user would go missing.
    ///
    /// The emptiness guard is load-bearing. `"".bytes().all(..)` is `true`, so
    /// without it an empty `?q=` would bind `Some(0)` and the exact-match step
    /// would look for a user with id 0.
    fn parse_id(query: &str) -> Option<i64> {
        if !query.is_empty() && query.bytes().all(|b| b.is_ascii_digit()) {
            query.parse().ok()
        } else {
            None
        }
    }
}
