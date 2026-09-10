import { Media } from "../types/media";
import { Author } from "./getFeed";
import type { LikeResponse } from "./like";
import { fetchWithOptionAuth } from "@/handler/token_handler";

export interface commentProps {
    id: string;
    post_id: string // If still need to used
    author: Author; // commenter

    content: string;
    total_likes: number;

    has_attachment: boolean;
    attachments: Media[];

    created_at: string;
    updated_at: string;
    is_like: boolean;
}

export const getCommentsOnPost = async (
    postId: string,
): Promise<commentProps[]> => {
    const res = await fetchWithOptionAuth(`v2/posts/${postId}/comments`);
    if (!res.ok) throw new Error("Failed to get comments data");
    const data = await res.json();
    return data;
};

export const createComment = async (
    postId: string,
    payload: FormData,
): Promise<commentProps | null> => {
    const res = await fetchWithOptionAuth(`v2/posts/${postId}/comments`, {
        method: "POST",
        body: payload,
    });
    if (!res.ok) throw new Error("Failed to create comment");
    const data = await res.json();
    return data;
};

export const deleteComment = async (
    postId: string,
    commentId: string,
): Promise<void> => {
    const res = await fetchWithOptionAuth(`v2/posts/${postId}/comments/${commentId}`, {
        method: "DELETE",
    });
    if (!res.ok) throw new Error("Failed to delete comment");
};

export const likeComment = async (
    postId: string,
    commentId: string,
    is_like: boolean,
): Promise<LikeResponse> => {
    const res = await fetchWithOptionAuth(`v2/posts/${postId}/comments/${commentId}/like`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({is_like}),
    });
    if (!res.ok) throw new Error("Failed to like comment");

    const data: LikeResponse = await res.json();

    return data;
};