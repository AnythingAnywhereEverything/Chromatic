import { adminFetch } from "./client";
import { AdminStats } from "./types";

/**
 * Admin dashboard counters (`v2/admin/stats`).
 *
 * One round trip rather than four separate count queries.
 */
export const getAdminStats = async (): Promise<AdminStats> => {
    return adminFetch<AdminStats>(
        "v2/admin/stats",
        { method: "GET" },
        "Failed to load stats",
    );
};
