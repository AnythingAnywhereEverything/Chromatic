import { fetchWithAuth } from "@/handler/token_handler";
import { AdminApiError } from "./types";

/**
 * Shared request helper for the admin API.
 *
 * Every call goes through here so error handling is uniform and does not
 * repeat the one mistake the older api files make: they call `res.json()`
 * *before* checking `res.ok`, which throws a bare `SyntaxError` on a
 * non-JSON body and hides the real status. Here the status is checked first,
 * and the body is only parsed when there is one to parse.
 *
 * The other thing worth guarding: `fetchWithAuth` rejects with
 * `Error("No token found")` when there is no token, so a missing session
 * never even produces a response. That error is passed through untouched
 * rather than being turned into an `AdminApiError` with a fake status.
 */
export async function adminFetch<T>(
    path: string,
    options: RequestInit = {},
    fallbackMessage = "Admin request failed",
): Promise<T> {
    const res = await fetchWithAuth(path, options);

    if (!res.ok) {
        let message = fallbackMessage;
        let code: string | undefined;

        try {
            const body = await res.json();
            // Backend shape: { status, errors: [{ code, message }] }
            const entry = body?.errors?.[0];
            if (entry?.message) message = entry.message;
            if (entry?.code) code = entry.code;
        } catch {
            // No JSON body (gateway error, empty response). The status alone
            // is the useful signal, so keep the fallback message.
        }

        throw new AdminApiError(message, res.status, code);
    }

    // 204 and friends have no body; anything the admin API returns does.
    return (await res.json()) as T;
}

/**
 * Builds a query string, skipping undefined values.
 *
 * Skipping is the whole point: the backend reads a *missing* `is_active` as
 * "no filter" but `?is_active=false` as "suspended users only". Sending
 * `is_active=undefined` would be a 400 from serde, and sending `false` for an
 * unset filter would silently change the result set.
 */
export function buildQuery(
    params: Record<string, string | number | boolean | undefined>,
): string {
    const query = new URLSearchParams();
    for (const [key, value] of Object.entries(params)) {
        if (value === undefined) continue;
        query.append(key, String(value));
    }
    const serialized = query.toString();
    return serialized ? `?${serialized}` : "";
}
