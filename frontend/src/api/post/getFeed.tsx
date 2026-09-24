import {
    fetchWithAuth,
    fetchWithOptionAuth,
} from "@/handler/token_handler";
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
    tags: PostTag[];

    has_attachment: boolean;
    attachments: Media[];

    created_at: string;
    updated_at: string;
    is_followed: boolean;
}

interface PostTag {
    tag_id: string;
    tag_name: string;
    target_id: string;
}

const LIMIT = 8;

export const getUserFeed = async (
    before?: Date,
    beforeId?: string,
    limit: number = LIMIT,
): Promise<PostProps[]> => {
    const query = new URLSearchParams();
    if (before) query.append("before", before.toISOString());
    if (beforeId) query.append("before_id", beforeId);
    query.append("limit", limit.toString());

    const res = await fetchWithAuth(`v2/posts/feed?${query.toString()}`);
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
