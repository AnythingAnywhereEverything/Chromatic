import { fetchWithAuth } from "@/handler/token_handler";

interface FollowResponse {
    user_id: string;
    follower_id: string;
    status: string;
    created_at: string;
}

export const followUser = async (targetId: string):Promise<FollowResponse> => {
    const response = await fetchWithAuth(`v2/users/follow/${targetId}`, {
        method: "POST",
    });
    if (!response.ok) {
        throw new Error("Failed to follow user");
    }

    return response.json();
};

export const unfollowUser = async (targetId: string) => {
    const response = await fetchWithAuth(`v2/users/unfollow/${targetId}`, {
        method: "DELETE",
    });
    if (!response.ok) {
        throw new Error("Failed to unfollow user");
    }
    return;
};