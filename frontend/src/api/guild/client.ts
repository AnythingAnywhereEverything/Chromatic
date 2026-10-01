import {
    fetchWithAuth,
    fetchWithOptionAuth,
} from "@/handler/token_handler";
import { GuildApiError } from "./types";

// * I love ai, This is disaster...
async function guildFetch<T>(
    path: string,
    options: RequestInit = {},
    fallbackMessage = "Guild request failed",
    optionalAuth = false,
): Promise<T> {
    const res = optionalAuth
        ? await fetchWithOptionAuth(path, options)
        : await fetchWithAuth(path, options);

    if (!res.ok) {
        let message = fallbackMessage;
        let code: string | undefined;

        try {
            const body = await res.json();
            const entry = body?.errors?.[0];
            if (entry?.message) message = entry.message;
            if (entry?.code) code = entry.code;
        } catch {
            // No JSON body. The status alone is the useful signal.
        }

        throw new GuildApiError(message, res.status, code);
    }

    return (await res.json()) as T;
}

/**
 * Build a query string, skipping undefined values.
 *
 * Skipping matters for `before_id`: serde rejects a bare `before_id` without a
 * `before`, and sending the string `"undefined"` would be a 400 rather than the
 * harmless "first page" the backend's own doc comment describes.
 */
function buildQuery(
    params: Record<string, string | number | undefined>,
): string {
    const query = new URLSearchParams();
    for (const [key, value] of Object.entries(params)) {
        if (value === undefined) continue;
        query.append(key, String(value));
    }
    const serialized = query.toString();
    return serialized ? `?${serialized}` : "";
}

export { buildQuery, guildFetch };