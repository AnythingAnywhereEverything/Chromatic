import { fetchWithAuth } from "@/handler/token_handler";
import { PostProps } from "./post/getFeed";

export const searchExplorePosts = async (query: {
    limit?: number;
    before?: string;
    before_id?: number;
    query?: string;
}): Promise<PostProps[]> => {
    const params = new URLSearchParams();
    if (query.limit !== undefined) params.append("limit", query.limit.toString());
    if (query.before !== undefined) params.append("before", query.before);
    if (query.before_id !== undefined) params.append("before_id", query.before_id.toString());
    if (query.query !== undefined) params.append("query", query.query);

    const response = await fetchWithAuth(`v2/posts/search?${params.toString()}`);
    if (!response.ok) {
        throw new Error("Failed to fetch search explore posts");
    }
    return response.json();
};