use sqlx::Transaction;

use crate::application::repository::RepositoryResult;

/// The difference between the caller's desired role set and what is stored.
///
/// Computed inside the caller's transaction by comparing the two lists, so it
/// reflects committed state rather than whatever the caller assumed. The service
/// turns each side into its own `audit_logs` row.
#[derive(Debug, Clone, Default)]
pub struct RoleSetDiff {
    pub added: Vec<i64>,
    pub removed: Vec<i64>,
}

/// Read one user's currently assigned role ids, ascending.
///
/// Sorted here rather than relying on the caller so the diff is deterministic
/// and the audit trail reads in a stable order.
pub async fn current_role_ids(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    user_id: i64,
) -> RepositoryResult<Vec<i64>> {
    let ids: Vec<i64> = sqlx::query_scalar(
        r#"
        SELECT role_id FROM user_staff_roles
        WHERE user_id = $1
        ORDER BY role_id ASC
        "#,
    )
    .bind(user_id)
    .fetch_all(tx.as_mut())
    .await?;

    Ok(ids)
}

/// Compute added/removed against the stored set. `desired` is sorted and deduped
/// first, so a caller sending `[3, 3, 1]` produces the same diff as `[1, 3]`.
pub async fn diff(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    user_id: i64,
    desired: &[i64],
) -> RepositoryResult<RoleSetDiff> {
    let mut target: Vec<i64> = desired.to_vec();
    target.sort_unstable();
    target.dedup();

    let current = current_role_ids(tx, user_id).await?;

    let added: Vec<i64> = target
        .iter()
        .copied()
        .filter(|id| !current.contains(id))
        .collect();
    let removed: Vec<i64> = current
        .iter()
        .copied()
        .filter(|id| !target.contains(id))
        .collect();

    Ok(RoleSetDiff { added, removed })
}

/// Replace one user's role set atomically: delete what is gone, insert what is new.
///
/// Two statements inside the caller's transaction, so there is no window where
/// the user holds neither their old nor their new role. A caller that wants a
/// single-statement `INSERT ... ON CONFLICT` per role would need one, but a
/// replace is inherently two-sided — an assignment cannot be added without
/// removing the one it replaces — so the transaction is the boundary, not a
/// narrower query.
///
/// The insert uses `ON CONFLICT DO NOTHING` because the primary key is
/// `(user_id, role_id)`. That makes a re-send idempotent: submitting the same set
/// twice inserts nothing the second time and produces an empty diff, rather than
/// a 409 from the uniqueness the caller cannot see.
pub async fn set_user_roles(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    user_id: i64,
    role_ids: &[i64],
    assigned_by: i64,
) -> RepositoryResult<()> {
    sqlx::query(
        r#"
        DELETE FROM user_staff_roles
        WHERE user_id = $1
        "#,
    )
    .bind(user_id)
    .execute(tx.as_mut())
    .await?;

    for &role_id in role_ids {
        sqlx::query(
            r#"
            INSERT INTO user_staff_roles (user_id, role_id, assigned_by)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id, role_id) DO NOTHING
            "#,
        )
        .bind(user_id)
        .bind(role_id)
        .bind(assigned_by)
        .execute(tx.as_mut())
        .await?;
    }

    Ok(())
}

/// The user ids currently holding a role, ascending.
///
/// Read before a delete, so the service can record in the audit trail exactly
/// whose assignments the cascade is about to remove.
pub async fn role_member_ids(
    tx: &mut Transaction<'_, sqlx::Postgres>,
    role_id: i64,
) -> RepositoryResult<Vec<i64>> {
    let ids: Vec<i64> = sqlx::query_scalar(
        r#"
        SELECT user_id FROM user_staff_roles
        WHERE role_id = $1
        ORDER BY user_id ASC
        "#,
    )
    .bind(role_id)
    .fetch_all(tx.as_mut())
    .await?;

    Ok(ids)
}