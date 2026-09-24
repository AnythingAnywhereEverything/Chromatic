import { fetchWithAuth } from "@/handler/token_handler";

interface FollowResponse {
    user_id: string;
    follower_id: string;
    status: string;
    created_at: string;
}

export interface PendingFollow {
    follower_id: string;
    username: string;
    display_name: string | null;
    avatar: string | null;
    avatar_thumbhash: string | null;
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

export const getPendingFollowRequests = async (): Promise<PendingFollow[]> => {
    const response = await fetchWithAuth(`v2/users/follow/requests`, {
        method: "GET",
        headers: {
            "Content-Type": "application/json",
        },
    });
    if (!response.ok) {
        throw new Error("Failed to get pending follow requests");
    }

    return response.json();
};

export const acceptFollowRequest = async (followerId: string): Promise<FollowResponse> => {
    const response = await fetchWithAuth(`v2/users/follow/${followerId}/accept`, {
        method: "POST",
    });
    if (!response.ok) {
        throw new Error("Failed to accept follow request");
    }

    return response.json();
};

export const rejectFollowRequest = async (followerId: string) => {
    const response = await fetchWithAuth(`v2/users/follow/${followerId}/reject`, {
        method: "DELETE",
    });
    if (!response.ok) {
        throw new Error("Failed to reject follow request");
    }
    return;
};