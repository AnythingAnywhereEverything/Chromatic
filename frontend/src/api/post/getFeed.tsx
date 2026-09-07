import {
    fetchWithAuth,
    fetchWithOptionAuth,
    getCacheUserId,
    getToken,
} from "@/handler/token_handler";
import { getUser } from "../user";
import { Media } from "../types/media";

// * from community? post

export interface Author {
    id: string;
    username: string;
    display_name: string;
    avatar: string;
    avatar_thumbhash: string;
    created_at: string;
}
export interface RepostedPost {
    post_id: string;
    author: Author;
    content: string;
    is_reposted: boolean;
    visibility: string;
    has_attachment: boolean;
    media: Media[];
    created_at: string;
    updated_at: string;
}
export interface PostProps {
    post_id: string;

    author: Author;

    content: string;
    visibility: string;
    total_comments: number;
    total_likes: number;

    bookmark: boolean;

    is_reposted: boolean;
    reposted_post?: RepostedPost;

    is_liked: boolean;
    tag: PostTag[];

    has_attachment: boolean;
    attachments: Media[];

    created_at: string;
    updated_at: string;
}

export interface commentProps {
    comment_id: string;
    // post_id: string // If still need to used
    author: Author; // commenter

    content: string;
    total_likes: number;

    has_attachment: boolean;
    attachments: Media[];

    created_at: string;
    updated_at: string;
}

interface PostTag {
    tag_id: string;
    tag_name: string;
    tag_color: string;
    target_id: string;
}

const LIMIT = 8;

export const getUserFeed = async (): Promise<PostProps[]> => {
    const res = await fetchWithAuth(`v2/posts/feed?${LIMIT}`);
    if (!res.ok) throw new Error("Failed to get post data");
    const data = await res.json();
    console.log(data);

    return data;
};

export const getFocusedPost = async (postId: string): Promise<PostProps> => {
    const res = await fetchWithOptionAuth(`v2/posts/${postId}`);
    if (!res.ok) throw new Error("Failed to get post data");
    const data = await res.json();
    return data;
};

export const getCommentsOnPost = async (
    postId: string,
): Promise<commentProps[]> => {
    const res = await fetchWithOptionAuth(`v2/posts/${postId}/comments`);
    if (!res.ok) throw new Error("Failed to get comments data");
    const data = await res.json();
    return data;
};

export const deletePost = async (postId: string): Promise<void> => {
    const res = await fetchWithAuth(`v2/posts/${postId}`, {
        method: "DELETE",
    });

    if (!res.ok) {
        throw new Error(`Failed to delete post: ${res.status}`);
    }
};
