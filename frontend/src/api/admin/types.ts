export interface AdminUser {
    id: string;
    username: string;
    email: string;
    display_name?: string;
    avatar?: string;
    avatar_thumbhash?: string;
    is_active: boolean;
    is_superuser: boolean;
    email_verified_at?: string;
    followers_count?: number;
    following_count?: number;
    posts_count?: number;
    created_at: string;
    updated_at: string;
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

    deleted_users: number;
}

/** The only actions the admin API writes to `audit_logs`. */
export type AdminAction =
    | "suspend"
    | "activate"
    | "grant_role"
    | "revoke_role"
    | "create_role"
    | "update_role"
    | "delete_role"
    | "assign_staff_role"
    | "revoke_staff_role";

export const STAFF_PERMISSIONS = [
    {
        bit: 1,
        key: "access_admin_panel",
        label: "Access admin panel",
        description: "Can reach the admin panel at all.",
    },
    {
        bit: 2,
        key: "manage_users",
        label: "Manage users",
        description: "Suspend, activate, and change superuser status.",
    },
    {
        bit: 3,
        key: "manage_posts",
        label: "Manage posts",
        description: "Edit or delete posts on other people's behalf.",
    },
    {
        bit: 4,
        key: "manage_staff_roles",
        label: "Manage staff roles",
        description: "Create, edit, and delete roles, and assign them.",
    },
    {
        bit: 5,
        key: "view_audit",
        label: "View audit log",
        description: "Read the moderation audit trail.",
    },
    {
        bit: 6,
        key: "view_stats",
        label: "View statistics",
        description: "Read platform-wide counts.",
    },
] as const;

/** The codepoints of `STAFF_PERMISSIONS`, as a plain number array. */
export const STAFF_PERMISSION_BITS: readonly number[] =
    STAFF_PERMISSIONS.map((permission) => permission.bit);

/** One staff role, as `staff_roles` stores it. */
export interface AdminStaffRole {
    id: string;
    name: string;
    description?: string;
    position: number;
    permission_bitmask: number[];
    created_at: string;
    updated_at: string;
}

/**
 * One page of roles.
 *
 * Keyset paginated on `(position, id)`, so there is no total count — same as
 * `AdminUserPage`. The role table is short enough that paging is rarely needed,
 * but the shape is kept honest rather than pretending a total exists.
 */
export interface AdminStaffRolePage {
    rows: AdminStaffRole[];
    has_more: boolean;
}

/**
 * One role plus how many people hold it.
 *
 * `member_count` is a separate scalar, not `members.length`, so the dialog can
 * show "assigned to 1,240 people" without loading the members. It comes from the
 * same round trip as the role via `#[sqlx(flatten)]`.
 */
export interface AdminStaffRoleDetail {
    role: AdminStaffRole;
    member_count: number;
}

/** One holder of a staff role. */
export interface AdminStaffRoleMember {
    user_id: string;
    username: string;
    display_name?: string;
    avatar?: string;
    assigned_at: string;
    /** Snowflake id of the admin who granted it. Carries no FK. */
    assigned_by: string;
}

/** One page of a role's members. */
export interface AdminStaffRoleMembersPage {
    rows: AdminStaffRoleMember[];
    has_more: boolean;
}

/**
 * One user's staff roles.
 *
 * `role_ids` is exactly what the `PUT` body carries, so a form seeds from this
 * and submits it back with changes applied. The client never diffs against its
 * own guess of the current state — the server's set is the only truth.
 */
export interface UserStaffRoles {
    user_id: string;
    roles: AdminStaffRole[];
    role_ids: string[];
}

/**
 * Query filters for the role list.
 *
 * As with users, an **absent** filter is not the same as a falsy one, so the
 * request builder skips `undefined` instead of sending it.
 */
export interface AdminStaffRoleListParams {
    q?: string;
    before?: number;
    before_id?: string;
    limit?: number;
}

/**
 * Query filters for a single role's member list.
 *
 * Keyset cursor is a *user* id here, not a role id: the member query orders by
 * user id, and that string is why it must never be parsed as a number.
 */
export interface AdminStaffRoleMembersParams {
    before_user_id?: string;
    limit?: number;
}

/** The `action_data` payload written for a suspend/activate. */
export interface FlagChange {
    previous: boolean;
    new: boolean;
}

/**
 * One `audit_logs` row.
 *
 * `action_data` is a JSONB blob the service wrote at mutation time, so its
 * shape depends entirely on which action ran. Only the keys for that action are
 * populated; the others are absent, not null.
 *
 * `target_type` is `"user"` for the user moderation and assignment actions,
 * `"staff_role"` for create/update/delete of a role.
 */
export interface AdminAuditEntry {
    id: string;
    target_id: string;
    target_type: string;
    action: AdminAction;
    performed_by: string;
    target_username?: string;
    performed_by_username?: string;
    action_data: {
        is_active?: FlagChange;
        is_superuser?: FlagChange;
        name?: string;
        position?: number;
        permission_bitmask?: number[];
        previous?: {
            name?: string;
            description?: string;
            position?: number;
            permission_bitmask?: number[];
        };
        current?: {
            name?: string;
            description?: string;
            position?: number;
            permission_bitmask?: number[];
        };
        revoked_from_user_ids?: string[];
        role_id?: string;
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
    q?: string;
    is_active?: boolean;
    is_superuser?: boolean;
    before?: string;
    before_id?: string;
    limit?: number;
}

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
