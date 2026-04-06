import { queryClient } from "@/hooks/clientQuery";

const TOKEN_KEY = "token";
//we're not using Jwts, so we dont need to decode them or check expiration, we just check if they exist and are valid on the server

export function setToken(token: string) {
    localStorage.setItem(TOKEN_KEY, token);
}

export function setCacheUserId(userId: string) {
    localStorage.setItem("userId", userId);
}
export function clearCacheUserId() {
    localStorage.removeItem("userId");
}

export function getCacheUserId(): string | null {
    return localStorage.getItem("userId");
}

export function getToken(): string | null {
    return localStorage.getItem(TOKEN_KEY)?.toString() || null;
}

export function clearToken() {
    localStorage.removeItem(TOKEN_KEY);
    clearCacheUserId();

    //also clear react query cache for user data
    queryClient.invalidateQueries();
}

export function isAuthenticated(): boolean {
    //fetch the token from api
    const token = getToken();
    fetch("/api/v2/auth/validate", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
            "token": `${token}`,
        },
    }).then(res => {
        if (!res.ok) {
            clearToken();
            return false;
        }
        return true;
    }).catch(() => {
        clearToken();
        return false;
    });

    return !!token;
}

export function fetchWithAuth(url: string, options: RequestInit = {}): Promise<Response> {
    const token = getToken();
    if (!token) {
        return Promise.reject(new Error("No token found"));
    }

    const headers = new Headers(options.headers || {});
    headers.set("token", `${token}`);

    return fetch(url, { ...options, headers });
}

export function fetchWithOptionAuth(url: string, options: RequestInit = {}): Promise<Response> {
    const token = getToken();

    const headers = new Headers(options.headers || {});
    headers.set("token", `${token}`);

    return fetch(url, { ...options, headers });
}