import { adminFetch, buildQuery } from "./client";
import {
    AdminUser,
    AdminUserDetail,
    AdminUserListParams,
    AdminUserPage,
} from "./types";

// todo: Sei.. about remove the before_id parameter from the API calls if it's no longer needed.
/**
 * Admin user management (`v2/admin/users`).
 *
 * All five calls require a superuser session. The backend re-reads
 * `is_superuser` from postgres on every request, so a revoked admin loses
 * access immediately rather than when their token expires.
 */

/** One page of users, exact matches first. */
export const getAdminUsers = async (
    params: AdminUserListParams = {},
): Promise<AdminUserPage> => {
    const query = buildQuery({
        q: params.q,
        is_active: params.is_active,
        is_superuser: params.is_superuser,
        before: params.before,
        before_id: params.before_id,
        limit: params.limit,
    });

    return adminFetch<AdminUserPage>(
        `v2/admin/users${query}`,
        { method: "GET" },
        "Failed to load users",
    );
};

/**
 * One user plus their moderation history.
 *
 * Returns 200 for a soft-deleted account (a moderator still needs to inspect
 * one); only an id that was never a user is a 404.
 */
export const getAdminUser = async (
    userId: string,
    auditLimit?: number,
): Promise<AdminUserDetail> => {
    const query = buildQuery({ audit_limit: auditLimit });

    return adminFetch<AdminUserDetail>(
        `v2/admin/users/${userId}${query}`,
        { method: "GET" },
        "Failed to load user",
    );
};

/**
 * Suspend a user.
 *
 * POST with no body — the backend route is bodyless, so there is no flag that
 * a caller could get backwards. Suspending yourself is refused with 403
 * `admin_self_suspension_forbidden`.
 */
export const suspendUser = async (userId: string): Promise<AdminUser> => {
    return adminFetch<AdminUser>(
        `v2/admin/users/${userId}/suspend`,
        { method: "POST" },
        "Failed to suspend user",
    );
};

/**
 * Reactivate a user. The mirror of `suspendUser`; no session restoration,
 * since a suspended user's session rows were already deleted.
 */
export const activateUser = async (userId: string): Promise<AdminUser> => {
    return adminFetch<AdminUser>(
        `v2/admin/users/${userId}/activate`,
        { method: "POST" },
        "Failed to activate user",
    );
};

/**
 * Grant or revoke superuser.
 *
 * Revoking your own role is 403 `admin_self_demotion_forbidden`; revoking the
 * last remaining superuser is 409 `admin_last_superuser_protected`. Under a
 * race between two admins demoting each other the loser can also get 403 from
 * the auth gate instead, depending on which side of the winner's commit its
 * gate re-read lands — both refusals are correct.
 */
export const updateUserRole = async (
    userId: string,
    isSuperuser: boolean,
): Promise<AdminUser> => {
    return adminFetch<AdminUser>(
        `v2/admin/users/${userId}/role`,
        {
            method: "PATCH",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ is_superuser: isSuperuser }),
        },
        "Failed to update role",
    );
};
