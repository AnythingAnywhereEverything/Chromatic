"use client";
import { useState, useEffect } from "react";
import style from "./scss/message-list.module.scss";
import { useUser } from "@/hooks/useUser";
import { UserResponse } from "@/api/user";
import { PostAvatar } from "../post/header/avatar";
import { BsThreeDots } from "react-icons/bs";
import { Tooltip, TooltipContent, TooltipTrigger } from "../tooltip";

import { CreateDirectMessage } from "./createNewMessage";
function MessageListSkeleton() {
    return (
        <div className={style["message-list-skeleton"]}>
            Loading message list...
        </div>
    );
}

// fetch in messages table if has messages get last message for each followed user
// for showing as a preview in the message list
interface MessageListProps {
    chatUsers: UserResponse[];
    selectedChatUser: UserResponse | null;
    onSelectChatUser: (user: UserResponse) => void;
}

function MessageList({
    chatUsers,
    selectedChatUser,
    onSelectChatUser,
}: MessageListProps) {
    const [user, setUser] = useState<UserResponse | null>(null);
    const currentUser = useUser();

    useEffect(() => {
        if (currentUser?.data) {
            setUser(currentUser.data);
        }
    }, [currentUser]);

    // todo: pagination for follow in message

    return (
        <>
            <div className={style["message-list"]}>
                <h2>Messages</h2>
                <input
                    type="search"
                    className={style["search-input"]}
                    placeholder="Search for friends"
                />
                <div className={style["people-list"]}>
                    <h3>Direct Messages</h3>
                    {/* Friend profile goes here */}
                    <div className={style["friend-profile"]}>
                        <div className={style["your-profile-header"]}>
                            <span>Your Profile</span>
                            {user && (
                                <div className={style["your-profile-tabs"]}>
                                    <PostAvatar
                                        userId={user.id}
                                        username={user.username}
                                        displayName={user.display_name}
                                        avatar={user.avatar}
                                        thumbhash={user.avatar_thumbhash}
                                        width={36}
                                        height={36}
                                    />
                                    <div
                                        className={
                                            style["your-profile-tab-info"]
                                        }
                                    >
                                        <span>{user.username}</span>
                                        {/* Idk, which one */}
                                        <span
                                            className={
                                                style["your-profile-tab-status"]
                                            }
                                        >
                                            Status/ last message /last active
                                        </span>
                                    </div>
                                    <BsThreeDots />
                                </div>
                            )}
                        </div>
                        <div className={style["direct-messages-seperator"]}>
                            <div className={style["create-direct-message"]}>
                                <span>Direct Message</span>
                                <Tooltip>
                                    <TooltipTrigger
                                        className={
                                            style[
                                                "create-direct-message-button"
                                            ]
                                        }
                                    >
                                        <CreateDirectMessage />
                                    </TooltipTrigger>
                                    <TooltipContent>
                                        Add a new direct message
                                    </TooltipContent>
                                </Tooltip>
                            </div>
                        </div>

                        <ul>
                            {/* // ! MOCK UP user profile tab for demonstration purposes */}
                            {user && (
                                <div className={style["your-profile-tabs"]}>
                                    <PostAvatar
                                        userId={user.id}
                                        username={user.username}
                                        displayName={user.display_name}
                                        avatar={user.avatar}
                                        thumbhash={user.avatar_thumbhash}
                                        width={36}
                                        height={36}
                                    />
                                    <div
                                        className={
                                            style["your-profile-tab-info"]
                                        }
                                    >
                                        <span>{user.username}</span>
                                        {/* Idk, which one */}
                                        <span
                                            className={
                                                style["your-profile-tab-status"]
                                            }
                                        >
                                            Status/ last message /last active
                                        </span>
                                    </div>
                                    <BsThreeDots />
                                </div>
                            )}
                        </ul>
                    </div>
                </div>
            </div>
        </>
    );
}

const TempUser = [
    {
        id: 1,
        name: "Friend 1",
    },
    {
        id: 2,
        name: "Friend 2",
    },
];

export { MessageList };
