import { fetchWithAuth, getCacheUserId, getToken } from "@/handler/token_handler"
import { getUser } from "../user";

export interface MediaReponse {
    currentPage: number;
    media: mediaPostProps[];
}

// * from community? post
export interface mediaPostProps {
    id: string
    user_id: string
    username: string
    content: string
    total_comment: number
    total_likes: number
    visibility: string
    repost_from: string
    is_repost: boolean
    has_attachment: boolean
    created_at: string
    updated_at: string
    bookmark: boolean

    tag: PostTag[]
    media: mediaPostAttechment[]
    is_liked: boolean
    current_user_id: string
}

interface mediaPostAttechment {
    id: string // * media from media_data
    uploader_id: string
    name: string
    path: string
    status: string
    thumbhash: string
    lock_has: string
    create_at: string
    updated_at: string
    lock_expiration: string
    width: number
    height: number
    duration: number
}

interface PostTag {
    tag_id : string
    tag_name: string
    target_id :string
}

const LIMIT = 15;

export const getUserFeed = async(
):Promise<mediaPostProps[]> => {
    const res = await fetchWithAuth(`v2/posts/feed`);
    if (!res.ok) throw new Error("Failed to get post data")
    const data = await res.json();
    console.log(data);

    return data;
}

export const deletePost = async (postId: string): Promise<void> => {
    const res = await fetchWithAuth(`v2/posts/${postId}`, {
        method: "DELETE",
    });

    if (!res.ok) {
        throw new Error(`Failed to delete post: ${res.status}`);
    }
}