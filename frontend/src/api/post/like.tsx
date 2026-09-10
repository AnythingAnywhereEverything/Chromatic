import { fetchWithAuth } from "@/handler/token_handler";

export interface LikeResponse {
    id: string;
    total_liked: number;
}

export const TogglePostLike = async (
    id: string,
    is_like: boolean
): Promise<LikeResponse> => {

    const res = await fetchWithAuth(`v2/posts/${id}/like`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify({
            is_like
        })
    });

    if (!res.ok) {
        throw new Error(`Failed to toggle post like: ${res.status}`);
    }

    const data: LikeResponse = await res.json();

    return data;
};