import { buildQuery, guildFetch } from "./client";
import {
    CreateGuildRequest,
    Guild,
    GuildListParams,
    GuildPage,
} from "./types";

/**
 * One page of guilds, newest first.
 *
 * Public by design: seeing what exists is not gated on being a member. That is
 * why this sends the token as optional — an anonymous visitor gets the list
 * rather than a rejected fetch. Send one anyway if you have it: every row comes
 * back with `is_owner`, which is `false` for an anonymous caller and the only
 * thing that distinguishes "my community" from "someone else's" in the list.
 */
export const getGuilds = async (
    params: GuildListParams = {},
): Promise<GuildPage> => {
    const query = buildQuery({
        before: params.before,
        before_id: params.before_id,
        limit: params.limit,
    });

    return guildFetch<GuildPage>(
        `v2/guilds${query}`,
        { method: "GET" },
        "Failed to load communities",
        true,
    );
};

/**
 * Create a guild. The caller becomes its owner and first member.
 *
 * The response already carries the accurate counters — `total_members` is 1 and
 * `total_channels` is 1 for the auto-created `#general` — because the service
 * re-reads the row after the whole transaction commits. So the returned guild
 * can go straight into the list without a refetch. `is_owner` is `true`, since
 * the caller is who the guild was created for.
 */
export const createGuild = async (
    body: CreateGuildRequest,
): Promise<Guild> => {
    return guildFetch<Guild>(
        "v2/guilds",
        {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({
                name: body.name.trim(),
                // Absent rather than null when there is nothing to say: the
                // backend already collapses a blank description to NULL.
                ...(body.description?.trim()
                    ? { description: body.description.trim() }
                    : {}),
            }),
        },
        "Failed to create community",
    );
};

/**
 * Join a guild.
 *
 * Returns the guild re-read after the join, so `total_members` already
 * reflects the new member and the list can be patched without a refetch.
 */
export const joinGuild = async (guildId: string): Promise<Guild> => {
    return guildFetch<Guild>(
        `v2/guilds/${guildId}/join`,
        { method: "POST" },
        "Failed to join community",
    );
};

/**
 * Delete a guild. Owner only.
 *
 * A **soft** delete: the backend stamps `deleted_at` and keeps the row, so the
 * guild disappears from the browse list and can no longer be joined, but nothing
 * is destroyed. There is no restore endpoint yet, so from the client's point of
 * view this is one-way — a confirmation should not offer an undo.
 */
export const deleteGuild = async (
    guildId: string,
): Promise<{ deleted: boolean }> => {
    return guildFetch<{ deleted: boolean }>(
        `v2/guilds/${guildId}`,
        { method: "DELETE" },
        "Failed to delete community",
    );
};