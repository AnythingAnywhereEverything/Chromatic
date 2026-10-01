use crate::application::{
    repository::guild::{
        check, create, delete, get, join,
        row::{GuildPage, GuildRow},
    },
    service::errors::GuildServiceError,
    state::AppState,
};

/// Guild create, join, browse and delete.
///
/// Stateless, like `AdminService`: every method is an associated function
/// taking `&AppState`, which is where the pool and the snowflake generator
/// live. The requester's id is always passed in explicitly rather than read
/// from ambient state, so a caller is whoever the extractor vouched for.
pub struct GuildService;

impl GuildService {
    /// Create a guild, seed it with the `#general` channel, and seat the owner.
    ///
    /// All three writes run in one transaction, so a guild can never exist
    /// without its `#general` channel or its owner's membership. The chore is
    /// three repository primitives composed here, not one big INSERT:
    ///
    /// 1. `create::guild` inserts the row with both counters at 0.
    /// 2. `create::channel` adds `general` and bumps `total_channels`.
    /// 3. `join::insert(owner)` — the same primitive an ordinary join uses —
    ///    seats the owner and bumps `total_members`.
    ///
    /// Reusing the join primitive for the owner is deliberate: the owner is a
    /// member like any other, and the counter logic cannot drift behind a
    /// bespoke ownership insert. The guild is then re-read so the returned row
    /// carries the accurate `total_members = 1` / `total_channels = 1` rather
    /// than the 0s the row held when it was inserted.
    ///
    /// If the join primitive reports the owner already a member, the only way
    /// that happens is an invariant violation (a brand-new guild has no
    /// members), so it is surfaced as `Database` rather than `AlreadyMember`.
    ///
    /// `description` is trimmed, and an all-whitespace description becomes
    /// `NULL` rather than an empty string — there is no "I set a description"
    /// that is actually blank.
    ///
    /// The returned row is read back against `owner_id`, so it always carries
    /// `is_owner = true`: a creator who is somehow not the owner would be a
    /// schema violation, not a state this flow can produce.
    pub async fn create(
        state: &AppState,
        owner_id: i64,
        name: &str,
        description: Option<&str>,
    ) -> Result<GuildRow, GuildServiceError> {

        let name = name.trim();
        if name.is_empty() || name.chars().count() > 128 {
            return Err(GuildServiceError::InvalidName);
        }
        let description = description.map(str::trim).filter(|d| !d.is_empty());

        let mut tx = state.db_pool.begin().await?;

        let guild_id = state.snowflake_generator.generate_id()?;
        let _ = create::guild(&mut tx, guild_id, owner_id, name, description).await?;

        let channel_id = state.snowflake_generator.generate_id()?;
        let _ = create::channel(&mut tx, channel_id, guild_id, "general", "text").await?;

        let member = join::insert(&mut tx, guild_id, owner_id).await?;
        if member.is_none() {
            return Err(GuildServiceError::Database);
        }

        let guild = get::by_id(&mut tx, guild_id, Some(owner_id))
            .await?
            .ok_or(GuildServiceError::Database)?;

        tx.commit().await?;

        Ok(guild)
    }

    /// Add `user_id` to `guild_id`.
    ///
    /// Gate and write are separate repository calls for a reason the
    /// repository documents: `check::guild_is_joinable` distinguishes "guild
    /// is gone" from "already joined", which `join::insert`'s `None` alone
    /// cannot. The membership insert itself stays race-free via
    /// `ON CONFLICT DO NOTHING`, so a duplicate join cannot slip past this
    /// check-then-insert pair and double-count.
    ///
    /// The returned row is the guild re-read after the join, so
    /// `total_members` reflects the new member. The re-read is done against
    /// `user_id`, which is what makes `is_owner` answer correctly: somebody
    /// joining a guild they do not own gets `false`, not a guess. In practice
    /// the owner cannot reach this line — they are seated by the create flow,
    /// so their own join is a 409 — and every successful join here is therefore
    /// `is_owner = false`.
    pub async fn join(
        state: &AppState,
        user_id: i64,
        guild_id: i64,
    ) -> Result<GuildRow, GuildServiceError> {
        let mut tx = state.db_pool.begin().await?;

        if !check::guild_is_joinable(&mut tx, guild_id).await? {
            return Err(GuildServiceError::GuildNotFound);
        }

        let member = join::insert(&mut tx, guild_id, user_id).await?;
        if member.is_none() {
            return Err(GuildServiceError::AlreadyMember);
        }

        let guild = get::by_id(&mut tx, guild_id, Some(user_id))
            .await?
            .ok_or(GuildServiceError::GuildNotFound)?;

        tx.commit().await?;

        Ok(guild)
    }

    /// One page of joinable guilds for the open-join browse list.
    ///
    /// Read-only, so no commit: the transaction is dropped at the end of the
    /// scope and rolled back, exactly as `AdminService::list_users` does.
    /// `limit` is not clamped here; the repository clamps to `1..=200` as a
    /// backstop and the handler applies the intended default.
    ///
    /// `requester_id` is `None` for the anonymous visitor the list is public to.
    /// It selects who each row's `is_owner` is measured against and nothing
    /// else: an anonymous and a logged-in caller get the same guilds in the same
    /// order.
    pub async fn browse(
        state: &AppState,
        requester_id: Option<i64>,
        before: Option<chrono::DateTime<chrono::Utc>>,
        before_id: Option<i64>,
        limit: i64,
    ) -> Result<GuildPage, GuildServiceError> {
        let mut tx = state.db_pool.begin().await?;

        let page = get::for_join(&mut tx, before, before_id, limit, requester_id).await?;

        Ok(page)
    }

    /// Soft-delete a guild. Owner only.
    ///
    /// The read comes first, before the write, and that ordering is the whole
    /// design. A single `UPDATE ... WHERE owner_id = $2` cannot tell a caller
    /// apart from a miss, so a non-owner and a nonexistent guild would both be
    /// one indistinguishable "no rows changed". Reading first is what lets those
    /// be a 403 and a 404.
    ///
    /// The `deleted_at` check runs *before* the ownership check, and that order
    /// is load-bearing rather than incidental. Checking ownership first would
    /// tell a non-owner that a soft-deleted guild exists, turning this endpoint
    /// into an oracle for removed guilds — which the public browse list, since it
    /// filters `deleted_at IS NULL`, cannot be. Answering 404 for an already-
    /// deleted guild keeps a removed guild as unlearnable as it was. `AdminUser`
    /// takes the same care in the other direction: it collapses every reason for
    /// refusing into one 403 so it cannot be used to probe which ids are real.
    ///
    /// Guild-only on purpose. A soft delete does not fire the `ON DELETE CASCADE`
    /// the member and channel foreign keys carry, so `guild_members` and
    /// `guild_channels` survive with their rows intact and `total_members` /
    /// `total_channels` keep describing the guild as it was. Nothing observable
    /// is wrong with that today — there is no member or channel read query yet —
    /// and it is what makes the delete reversible for free if a restore ever
    /// lands.
    ///
    /// There is no audit row here, unlike the admin and staff-role deletes. Those
    /// write to `audit_logs` because a moderator acting outside their own scope
    /// is accountable to someone; an owner deleting their own guild is the only
    /// party involved, and the `deleted_at` column is the record.
    pub async fn delete(
        state: &AppState,
        requester_id: i64,
        guild_id: i64,
    ) -> Result<(), GuildServiceError> {
        let mut tx = state.db_pool.begin().await?;

        let Some(guild) = get::by_id(&mut tx, guild_id, Some(requester_id)).await? else {
            return Err(GuildServiceError::GuildNotFound);
        };

        if guild.deleted_at.is_some() {
            return Err(GuildServiceError::GuildNotFound);
        }

        if !guild.is_owner {
            return Err(GuildServiceError::NotOwner);
        }

        let deleted = delete::soft_delete_guild(&mut tx, guild_id, requester_id).await?;
        if deleted == 0 {

            return Err(GuildServiceError::GuildNotFound);
        }

        tx.commit().await?;

        Ok(())
    }
}