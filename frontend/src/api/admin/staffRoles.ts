import { adminFetch, buildQuery } from "./client";
import {
    AdminStaffRole,
    AdminStaffRoleDetail,
    AdminStaffRoleListParams,
    AdminStaffRoleMember,
    AdminStaffRoleMembersPage,
    AdminStaffRoleMembersParams,
    AdminStaffRolePage,
    UserStaffRoles,
} from "./types";

/**
 * Staff role management (`v2/admin/roles`).
 *
 * All eight calls require a superuser session, the same as every other admin
 * endpoint. Nothing here reads or enforces `permission_bitmask` — the flags are
 * descriptive today. See `STAFF_PERMISSIONS` in `./types`.
 */

/** One page of roles, ordered by `position` ascending. */
export const getStaffRoles = async (
    params: AdminStaffRoleListParams = {},
): Promise<AdminStaffRolePage> => {
    const query = buildQuery({
        q: params.q,
        before: params.before,
        // A snowflake cursor, so it goes over the wire as a string. `before_id`
        // is only meaningful alongside `before`; sending one alone degrades to
        // the first page rather than erroring.
        before_id: params.before_id,
        limit: params.limit,
    });

    return adminFetch<AdminStaffRolePage>(
        `v2/admin/roles${query}`,
        { method: "GET" },
        "Failed to load roles",
    );
};

/**
 * One role plus its member count.
 *
 * Deliberately does not include the members themselves — a role can hold
 * thousands, so they are paginated separately. The count is in the same response
 * because it is a scalar the database can produce without shipping rows.
 */
export const getStaffRole = async (
    roleId: string,
): Promise<AdminStaffRoleDetail> => {
    return adminFetch<AdminStaffRoleDetail>(
        `v2/admin/roles/${roleId}`,
        { method: "GET" },
        "Failed to load role",
    );
};

/** One page of a role's members, ordered by user id. */
export const getStaffRoleMembers = async (
    roleId: string,
    params: AdminStaffRoleMembersParams = {},
): Promise<AdminStaffRoleMembersPage> => {
    const query = buildQuery({
        before_user_id: params.before_user_id,
        limit: params.limit,
    });

    return adminFetch<AdminStaffRoleMembersPage>(
        `v2/admin/roles/${roleId}/members${query}`,
        { method: "GET" },
        "Failed to load role members",
    );
};

/**
 * Create a role.
 *
 * `permission_bitmask` is required by the backend rather than defaulted, and
 * that is worth preserving here: an omitted mask would mean "no permissions"
 * while a defaulted one would mean "all six", and silently granting everything
 * is the wrong failure direction for a field a human is meant to be choosing.
 * Pass `[]` deliberately to create a role with no permissions.
 *
 * A duplicate name is 409 `staff_role_name_taken`; the backend gets there with
 * `ON CONFLICT DO NOTHING` rather than a SELECT, so two concurrent creates of
 * the same name cannot both win.
 */
export const createStaffRole = async (payload: {
    name: string;
    description?: string;
    /** Omitted becomes 0, the least-privileged end of the ordering. */
    position?: number;
    permission_bitmask: number[];
}): Promise<AdminStaffRole> => {
    return adminFetch<AdminStaffRole>(
        "v2/admin/roles",
        {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(payload),
        },
        "Failed to create role",
    );
};

/**
 * Partially update a role.
 *
 * Every field is optional and applied through `COALESCE`, so an **absent** field
 * keeps its stored value. The practical consequence: `{"description": null}`
 * cannot clear a description — null and absent mean the same thing to this
 * endpoint. Clearing one would need an explicit empty string.
 *
 * Only send what changed. Sending the full role back is harmless but makes the
 * audit row's `previous` harder to read.
 */
export const updateStaffRole = async (
    roleId: string,
    payload: {
        name?: string;
        description?: string;
        position?: number;
        permission_bitmask?: number[];
    },
): Promise<AdminStaffRole> => {
    return adminFetch<AdminStaffRole>(
        `v2/admin/roles/${roleId}`,
        {
            method: "PATCH",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(payload),
        },
        "Failed to update role",
    );
};

/**
 * Delete a role, permanently.
 *
 * There is no `deleted_at` on `staff_roles`, so this is a **hard delete** and the
 * `user_staff_roles` join cascades with it: every assignment goes with it. The
 * backend reads the affected member ids before deleting and records them in the
 * audit row, because after this returns there is nothing left to read them from.
 *
 * Undo is not possible. The confirmation should say so.
 *
 * Returns `{ deleted: true }` rather than an empty body, for uniformity with the
 * rest of the admin API.
 */
export const deleteStaffRole = async (roleId: string): Promise<unknown> => {
    return adminFetch<{ deleted: boolean }>(
        `v2/admin/roles/${roleId}`,
        { method: "DELETE" },
        "Failed to delete role",
    );
};

/** One user's staff roles, as stored. */
export const getUserStaffRoles = async (
    userId: string,
): Promise<UserStaffRoles> => {
    return adminFetch<UserStaffRoles>(
        `v2/admin/users/${userId}/staff-roles`,
        { method: "GET" },
        "Failed to load user roles",
    );
};

/**
 * Replace a user's staff roles.
 *
 * A **full replace**, not an add or a remove — that is the point of `PUT`. The
 * body is the complete desired set, so the server diffs it against what is
 * stored and writes one audit row per grant and one per revoke. An
 * add-one/remove-one pair of endpoints could not produce that trail without two
 * round trips, and would need the client to guess the current state.
 *
 * The response is re-read from the database rather than echoed back from the
 * request, so what comes back is the set that actually persisted — including if
 * that turned out to differ from what was sent.
 *
 * Duplicates and unknown ids are not an error here; the stored set is keyed by
 * `(user_id, role_id)` and `ON CONFLICT DO NOTHING` collapses them.
 */
export const setUserStaffRoles = async (
    userId: string,
    roleIds: string[],
): Promise<UserStaffRoles> => {
    return adminFetch<UserStaffRoles>(
        `v2/admin/users/${userId}/staff-roles`,
        {
            method: "PUT",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ role_ids: roleIds }),
        },
        "Failed to update user roles",
    );
};


export type { AdminStaffRole, AdminStaffRoleMember };