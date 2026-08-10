import { fetchWithAuth, getCacheUserId, getToken } from "@/handler/token_handler"
import { getUser } from "./user";

export interface MediaReponse{
    currentPage: number;
    media: mediaPostProps[];
}

interface mediaPostProps  {
    id: string
    user_id: string
    content: string
    total_comment:number
    total_likes:number
    visibility:string
    repost_from:string
    is_repost:boolean
    has_attachment:boolean
    created_at:string
    updated_at:string
    media_tags:string[]

    media_attachment?: mediaPostAttechment[]
}

interface mediaPostAttechment  {
    id:string // * media from media_data
    user_id:string
    media_url:string
    media_preview_url:string
}

const LIMIT = 15;

export const getUserFeed = async(
    pageParam: number,
):Promise<MediaReponse> => {
    const token = getToken();
    if (!token) throw new Error("No token found");

    const user = getUser();
    const res = await fetchWithAuth(`/api/v2/getfeed?page=${pageParam}&limit=${LIMIT}`);
    if (!res.ok) throw new Error("Failed to get post data")
    const data = await res.json();

    return data;
}