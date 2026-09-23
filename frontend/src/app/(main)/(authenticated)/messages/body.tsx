"use client";

import MessageContent from "@/app/_components/ui/chromatic/message/messageContent";
import { MessageList } from "@/app/_components/ui/chromatic/message/messageList";
import style from "./message.module.scss";
import { useState } from "react";
import { UserResponse } from "@/api/user";

function MessageSkeleton() {
    return <div className="message-skeleton">Loading message...</div>;
}

function MessageContainer() {
    const [chatUsers, setChatUsers] = useState<UserResponse[]>([]);
    const [selectedChatUser, setSelectedChatUser] =
        useState<UserResponse | null>(null);
    const [messages, setMessages] = useState([]);

    return (
        <div className={style["message-layout"]}>
            <MessageList
                chatUsers={[]}
                selectedChatUser={null}
                onSelectChatUser={function (user: UserResponse): void {
                    throw new Error("Function not implemented.");
                }}
            />
            <div className={style["message-body"]}>
                {/* Import and use the MessageBody component here */}
                <MessageContent />
            </div>
        </div>
    );
}

export default MessageContainer;
