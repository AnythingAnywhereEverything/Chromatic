import { fetchWithAuth } from "@/handler/token_handler";
import { UserResponse } from "../user";
// Currently not accept ANY attachments
export interface MessageResponse {
    id: string;
    user_id: string; // sender
    target_id: string;
    content: string;
    has_attachment: boolean;
    has_reactions: boolean;
    created_at: string;
    updated_at: string;
}

// must stay in sync with the backend default in message_handler.rs
export const MESSAGE_PAGE_LIMIT = 11;

// It should be panigation, but now I'm Rushing
export async function getFollowedUsers(): Promise<UserResponse[] | null> {
    const response = await fetchWithAuth(`v2/messages/followed`);

    if (!response.ok) {
        throw new Error(`Failed to fetch followed users: ${response.statusText}`);
    }
    const data = await response.json();
    console.log("Fetched followed users:", data);
    return data ?? null;
}

// pagination for messages
export async function getMessages(
    targetId: string,
    before: string,
    limit: number = MESSAGE_PAGE_LIMIT,
    beforeId?: string,
): Promise<MessageResponse[] | null> {
    const query = new URLSearchParams();
    query.append("before", before);
    if (beforeId) {
        query.append("before_id", beforeId);
    }
    query.append("limit", limit.toString());
    const response = await fetchWithAuth(`v2/messages/channel/${targetId}?${query.toString()}`);

    if (!response.ok) {
        throw new Error(`Failed to fetch messages: ${response.statusText}`);
    }
    const data = await response.json();
    console.log("Fetched messages:", data);
    return data ?? null;
}
