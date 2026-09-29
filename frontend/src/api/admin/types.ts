/**
 * Types for the admin API (`v2/admin/*`).
 *
 * These mirror the Rust row structs in
 * `backend/rust-service/src/application/repository/admin/row.rs` exactly.
 * There is no DTO layer on the backend, so these are the response shape.
 *
 * Two things that are easy to get wrong, and why:
 *
 * 1. **Nullable fields are optional, not `| null`.** `AdminUserRow` carries
 *    `#[skip_serializing_none]`, which drops the key entirely when the value
 *    is `None` rather than emitting `"display_name": null`. So an absent key
 *    is normal, and `display_name: null` will never appear.
 *
 * 2. **Ids are strings and must stay strings.** Snowflake ids are 17 digits;
 *    `Number.MAX_SAFE_INTEGER` is 16, so `Number(id)` silently corrupts them
 *    (96638107465551872 -> 96638107465551870). This is why the backend also
 *    carries an unserialized `id_raw: i64` for ordering. Never parse an id.
 */

/** A user as seen by the admin panel. */
export interface AdminUser {
    /** Snowflake id, as a string. Never convert with `Number()`. */
    id: string;
    username: string;
    email: string;
    display_name?: string;
    /** Avatar file name; builds `avatars/{id}/{avatar}`. Absent = no picture. */
    avatar?: string;
    avatar_thumbhash?: string;
    is_active: boolean;
    is_superuser: boolean;
    email_verified_at?: string;
    /** All three are counts, and all three are nullable in the database. */
    followers_count?: number;
    following_count?: number;
    posts_count?: number;
    created_at: string;
    updated_at: string;
    /** Present only when the account is soft-deleted. */
    deleted_at?: string;
}

/**
 * One page of users.
 *
 * Keyset pagination, so there is no total count — `has_more` is derived by
 * fetching one row past the limit. If a page count is ever needed, it has to
 * be added to the backend deliberately.
 */
export interface AdminUserPage {
    rows: AdminUser[];
    has_more: boolean;
}

/** One user plus their moderation history. */
export interface AdminUserDetail {
    user: AdminUser;
    audit: AdminAuditEntry[];
}

/** Dashboard counters, one round trip. */
export interface AdminStats {
    total_users: number;
    active_users: number;
    suspended_users: number;
    superusers: number;
    /**
     * Overlaps the other three rather than being a fourth state: a soft
     * delete leaves `is_active` untouched, so a deleted account counts as
     * active *and* deleted. The real invariant is
     * `active_users + suspended_users = total_users`.
     */
    deleted_users: number;
}

/** The only actions the admin API writes to `audit_logs`. */
export type AdminAction =
    | "suspend"
    | "activate"
    | "grant_role"
    | "revoke_role";

/** The `action_data` payload written for a suspend/activate. */
export interface FlagChange {
    previous: boolean;
    new: boolean;
}

/**
 * One `audit_logs` row.
 *
 * `action_data` is a JSONB blob the service wrote at mutation time, so its
 * shape depends on which action ran: a suspend/activate writes `is_active`,
 * a role change writes `is_superuser`. Both are optional here because the
 * field is whichever one applies, not both.
 */
export interface AdminAuditEntry {
    id: string;
    target_id: string;
    /** Always `"user"` for every entry the admin API produces. */
    target_type: string;
    action: AdminAction;
    /** The superuser who performed the action, as a string id. */
    performed_by: string;
    /** Handle of the target user. Absent only if the user row is gone. */
    target_username?: string;
    /** Handle of the admin who performed the action. */
    performed_by_username?: string;
    action_data: {
        is_active?: FlagChange;
        is_superuser?: FlagChange;
    };
    created_at: string;
}

/**
 * Query filters for the user list.
 *
 * Every field is optional and an **absent** filter is not the same as
 * `false`: `?is_active=false` is a real request for suspended users. So the
 * request builder skips undefined values rather than sending them.
 *
 * `before_id` stays a string for the reason above — it is a snowflake id.
 */
export interface AdminUserListParams {
    /** Matches username, email, or a numeric id. */
    q?: string;
    is_active?: boolean;
    is_superuser?: boolean;
    /** Keyset cursor. RFC3339 timestamp. Only meaningful with `before_id`. */
    before?: string;
    /** Keyset cursor, as a raw string. Only meaningful with `before`. */
    before_id?: string;
    /** Server clamps to 1..=200 and defaults to 50. */
    limit?: number;
}

/**
 * Error thrown by every call in this folder.
 *
 * Carries `status` and `code` so callers can branch on the admin-specific
 * refusals instead of matching on message text:
 *
 * - 409 `admin_last_superuser_protected` — would remove the final admin
 * - 403 `admin_self_demotion_forbidden` / `admin_self_suspension_forbidden`
 * - 403 `authentication_forbidden` — authenticated, but not a superuser
 * - 401 — no session at all
 * - 404 `user_not_found`
 */
export class AdminApiError extends Error {
    status: number;
    code?: string;

    constructor(message: string, status: number, code?: string) {
        super(message);
        this.name = "AdminApiError";
        this.status = status;
        this.code = code;
    }
}
