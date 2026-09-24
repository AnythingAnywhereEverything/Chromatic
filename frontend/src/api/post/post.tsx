import { CreatePostPayload } from "@/app/_components/ui/chromatic/createPost";
import { fetchWithAuth, fetchWithOptionAuth } from "@/handler/token_handler";
import { PostProps } from "./getFeed";

export async function CreatePost(payload: FormData): Promise<PostProps | null> {
    if (payload === undefined) {
        return null;
    }
    console.log(payload);
    const res = await fetchWithAuth(`v2/posts/new`, {
        method: "POST",
        body: payload,
    });

    if (!res.ok) return null;
    const data = await res.json();
    return data;
}

export async function GetUserPosts(
    target_id: string,
    before?: Date,
    beforeId?: string,
    limit?: number,
): Promise<PostProps[] | null> {
    const query = new URLSearchParams();
    if (before) {
        query.append("before", before.toISOString());
    }
    if (beforeId) {
        query.append("before_id", beforeId);
    }
    if (limit !== undefined) {
        query.append("limit", limit.toString());
    }

    const res = await fetchWithOptionAuth(
        `v2/posts/user/${target_id}?${query.toString()}`,
        {
            method: "GET",
        },
    );

    if (!res.ok) return null;
    const data = await res.json();
    return data;
}

export const deletePost = async (postId: string): Promise<void> => {
    const res = await fetchWithAuth(`v2/posts/${postId}`, {
        method: "DELETE",
    });

    if (!res.ok) {
        throw new Error(`Failed to delete post: ${res.status}`);
    }
};

export const GetExplorePosts = async (
    before?: Date,
    beforeId?: string,
    tagId?: string,
    limit?: number,
): Promise<PostProps[] | null> => {
    const query = new URLSearchParams();
    if (before) {
        query.append("before", before.toISOString());
    }
    if (beforeId) {
        query.append("before_id", beforeId);
    }
    if (tagId) {
        query.append("tag_id", tagId);
    }
    if (limit !== undefined) {
        query.append("limit", limit.toString());
    }

    const res = await fetchWithOptionAuth(
        `v2/posts/explore?${query.toString()}`,
        {
            method: "GET",
        },
    );

    if (!res.ok) return null;
    const data = await res.json();
    return data;
};