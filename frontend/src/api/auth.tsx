//authenticcation api call

import { fetchWithAuth, getToken } from "@/handler/token_handler";
import { useRouter } from "next/router";

interface RegisterResponse {
    message?: string;
}

interface RegisterData {
    username: string;
    email: string;
    password: string;
}

interface LoginData {
    username_or_email: string;
    password: string;
}

interface LoginResponse {
    user_id: string;
    token: string;
}

export async function login(data: LoginData): Promise<LoginResponse> {
    const res = await fetch(`${process.env.NEXT_PUBLIC_API_URL}v2/auth/login`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(data),
    });
    if (!res.ok) throw new Error("Username or password is incorrect");
    const response = await res.json();
    return response;
}

export async function register(data: RegisterData): Promise<RegisterResponse> {
    const res = await fetch(`${process.env.NEXT_PUBLIC_API_URL}v2/auth/register`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(data),
    });
    if (!res.ok) throw new Error("Registration failed");
    const response = await res.json();
    return response;
}

export async function logout(router: ReturnType<typeof useRouter>) {
    const res = await fetchWithAuth("v2/auth/logout");

    if (!res.ok) throw new Error("Logout failed");

    router.push("/auth/signin");
}