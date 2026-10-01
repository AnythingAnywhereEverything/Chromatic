
export interface Guild {

    id: string;
    owner_id: string;

    is_owner: boolean;
    name: string;

    description?: string;
    total_members: number;
    total_channels: number;
    created_at: string;
    updated_at: string;
    deleted_at?: string;
}

/**
 * One page of guilds.
 *
 * Keyset pagination, so there is no total count: `has_more` comes from fetching
 * one row past the limit. The browse query orders by `(created_at, id) DESC`, so
 * a cursor is both of those.
 */
export interface GuildPage {
    rows: Guild[];
    has_more: boolean;
}

/** Query params for the browse list. All are optional. */
export interface GuildListParams {
    before?: string;
    before_id?: string;
    limit?: number;
}

/** Body of `POST v2/guilds`. */
export interface CreateGuildRequest {
    name: string;
    /** Absent, or sent trimmed. The backend collapses blank to null. */
    description?: string;
}

/**
 * A failed guild request, carrying the backend's error code.
 *
 * The code is what makes the create dialog able to put "Guild name must be
 * 1-128 characters." under the name field rather than in a generic banner.
 * Backend codes: `guild_invalid_name`, `guild_not_found`, `already_member`,
 * `not_guild_owner`.
 */
export class GuildApiError extends Error {
    status: number;
    code?: string;

    constructor(message: string, status: number, code?: string) {
        super(message);
        this.name = "GuildApiError";
        this.status = status;
        this.code = code;
    }
}