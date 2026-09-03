import { CreatePostPayload } from "@/app/_components/ui/chromatic/createPost";
import { fetchWithAuth, fetchWithOptionAuth } from "@/handler/token_handler";
import { PostProps } from "./getFeed";

export async function CreatePost (payload: FormData): Promise<PostProps | null> {
    if (payload === undefined) {
        return null;
    }
    console.log(payload)
    const res = await fetchWithAuth(`v2/posts/new`, {
        method: "POST",
        body: payload,
    });

    if(!res.ok) return null;
    const data = await res.json();
    return data; 
}