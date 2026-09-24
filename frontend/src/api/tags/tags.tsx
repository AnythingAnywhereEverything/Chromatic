import { fetchWithOptionAuth } from "@/handler/token_handler";

export interface TagRow {
    id: string;
    tag_name: string;
}
export async function GetAllTagAttachments(): Promise<TagRow[] | null> {
    const res = await fetchWithOptionAuth(`v2/tags/all`, {
        method: "GET",
    });

    if(!res.ok) return null;
    const data = await res.json();
    return data;
}

