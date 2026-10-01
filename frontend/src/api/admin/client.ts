import { fetchWithAuth } from "@/handler/token_handler";
import { AdminApiError } from "./types";

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
            const entry = body?.errors?.[0];
            if (entry?.message) message = entry.message;
            if (entry?.code) code = entry.code;
        } catch {
        }
        throw new AdminApiError(message, res.status, code);
    }
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
