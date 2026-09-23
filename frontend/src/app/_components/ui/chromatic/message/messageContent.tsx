import { MessageResponse } from "@/api/messages/messages";
import { UserResponse } from "@/api/user";
import { useState } from "react";

function MessageSkeleton() {
    return <div className="message-skeleton">Loading message...</div>;
}

// getting target profile through MessageList
// Currently only supports one-on-one messages
// no group 
interface MessageContentProps {
    targetName: string;
    profile: UserResponse;
    messages: MessageResponse[];
}

function MessageContent() {
    const [loading, setLoading] = useState(true);

    return (
        <>
            <div className="message-header">Target Name</div>
            <section className="message-content"></section>
        </>
    );
}

export default MessageContent;
