import { adminFetch, buildQuery } from "./client";
import { AdminAuditEntry } from "./types";

/**
 * Global moderation history (`v2/admin/audit`).
 *
 * Newest first, across every target. This endpoint is deliberately pageless
 * — it takes a `limit` and nothing else, because the per-user subset already
 * rides along on the user detail response. So there is no cursor and no
 * `has_more`; if you need to page further back, that has to be added to the
 * backend.
 */
export const getAdminAudit = async (
    limit?: number,
): Promise<AdminAuditEntry[]> => {
    const query = buildQuery({ limit });

    return adminFetch<AdminAuditEntry[]>(
        `v2/admin/audit${query}`,
        { method: "GET" },
        "Failed to load audit log",
    );
};
