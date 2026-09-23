"use client";
import { useUser } from "@/hooks/useUser";
import { useEffect } from "react";
import MessageContent from "@/app/_components/ui/chromatic/message/messageContent";
import { MessageList } from "@/app/_components/ui/chromatic/message/messageList";
import style from "./message.module.scss";
import { useState } from "react";
import { UserResponse } from "@/api/user";
import { getFollowedUsers } from "@/api/messages/messages";

function NonSelectedMessage() {
    return (
        <div className={style["no-selected-message"]}>
            <h3>No chat selected</h3>
            <span>
                Select a conversation or click the <strong>"+"</strong> button
                to start messaging.
            </span>
        </div>
    );
}

// just in case
interface MessageContainerProps {
    userId?: string;
}

function MessageContainer({ userId }: MessageContainerProps) {
    const currentUser = useUser();
    const [chatUsers, setChatUsers] = useState<UserResponse[]>([]);
    const [selectedChatUser, setSelectedChatUser] =
        useState<UserResponse | null>(null);
    const [user, setUser] = useState<UserResponse | null>(null);

    useEffect(() => {
        if (!currentUser.data?.id) {
            return;
        }
        setUser(currentUser.data);

        const fetchFollowedUsers = async () => {
            try {
                const res = await getFollowedUsers();
                setChatUsers(res ?? []);
            } catch (error) {
                console.error(error);
            }
        };

        fetchFollowedUsers();
    }, [currentUser.data?.id]);

    useEffect(() => {
        if (!userId || chatUsers.length === 0) {
            return;
        }

        const user = chatUsers.find((chatUser) => chatUser.id === userId);

        if (user) {
            setSelectedChatUser(user);
        }
    }, [userId, chatUsers]);

    // select and getting messages
    const handleSelectChatUser = (user: UserResponse) => {
        setSelectedChatUser(user);

        window.history.pushState(
            { chatId: user.id },
            "",
            `/messages/c/${user.id}`,
        );
    };

    useEffect(() => {
        const handlePopState = () => {
            const match = window.location.pathname.match(
                /^\/messages\/c\/([^/]+)$/,
            );

            if (!match) {
                setSelectedChatUser(null);
                return;
            }

            const user = chatUsers.find((chatUser) => chatUser.id === match[1]);

            setSelectedChatUser(user ?? null);
        };

        window.addEventListener("popstate", handlePopState);

        return () => {
            window.removeEventListener("popstate", handlePopState);
        };
    }, [chatUsers]);

    return (
        <div className={style["message-layout"]}>
            <MessageList
                chatUsers={chatUsers}
                selectedChatUser={selectedChatUser}
                onSelectChatUser={handleSelectChatUser}
            />

            <div className={style["message-body"]}>
                {selectedChatUser ? (
                    <MessageContent
                        profile={selectedChatUser}
                        currentUser={user!}
                    />
                ) : (
                    <NonSelectedMessage />
                )}
            </div>
        </div>
    );
}

export default MessageContainer;
