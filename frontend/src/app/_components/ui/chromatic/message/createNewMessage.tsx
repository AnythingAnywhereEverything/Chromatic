import {
    Dialog,
    DialogTrigger,
    DialogContent,
    DialogHeading,
    DialogClose,
} from "../dialogue";
import { FiPlus } from "react-icons/fi";
import { useState, useEffect } from "react";
import { getFollowedUsers } from "@/api/messages/messages";
import style from "./scss/create-new-message.module.scss";
import { PostAvatar } from "../post/header/avatar";
import { UserResponse } from "@/api/user";
import { RxCross1 } from "react-icons/rx";

// Component for creating a new direct message, allowing the user to select followed users and initiate a conversation.
// now create only one on one direct message at a time
function CreateDirectMessage({
    isOpen,
    onOpenChange,
}: {
    isOpen: boolean;
    onOpenChange: (open: boolean) => void;
}) {
    const [followedUsers, setFollowedUsers] = useState<UserResponse[]>([]);
    const [selectedUsers, setSelectedUsers] = useState<string[]>([]);
    const [isSubmitting, setIsSubmitting] = useState(false);
    const [enableSubmit, setEnableSubmit] = useState(false);

    useEffect(() => {
        async function fetchFollowedUsers() {
            try {
                const users = await getFollowedUsers();
                setFollowedUsers(users ?? []);
            } catch (error) {
                console.error("Failed to fetch followed users:", error);
            }
        }
        fetchFollowedUsers();
    }, []);

    useEffect(() => {
        setEnableSubmit(selectedUsers.length > 0);
    }, [selectedUsers]);

    const toggleUser = (userId: string) => {
        setSelectedUsers((prev) =>
            prev.includes(userId)
                ? prev.filter((id) => id !== userId)
                : [...prev, userId],
        );
    };

    const handleSingleMessage = async (userId: string) => {
        // * Demo: handle creating/opening a one-on-one conversation here.
        console.log("Create single message with user:", userId);

        // TODO: check if conversation exists
        // TODO: create conversation if it does not exist
        // TODO: add conversation to history
        // TODO: router.replace(...)
    };

    // handle submit check if (is single user OR group)
    // check if user is exist then add history and replace router and loading the messages
    const handleSubmit = async () => {
        if (selectedUsers.length === 0) {
            return;
        }

        setIsSubmitting(true);
        setEnableSubmit(false);

        try {
            if (selectedUsers.length === 1) {
                await handleSingleMessage(selectedUsers[0]);
            } else {
                // TODO: handle group message later
                console.log("Group message:", selectedUsers);
            }
        } finally {
            setIsSubmitting(false);
            setEnableSubmit(selectedUsers.length > 0);
            onOpenChange(false);
        }
    };

    return (
        <Dialog open={isOpen} onOpenChange={onOpenChange}>
            <DialogContent className={style["new-message-container"]}>
                <DialogHeading className={style["new-message-heading"]}>
                    <span>New Message</span>

                    <DialogClose style={{ cursor: "pointer" }}>
                        <RxCross1 />
                    </DialogClose>
                </DialogHeading>

                {/* Following user or follower can be selected as a demo */}
                <ul className={style["followed-user-list"]}>
                    {followedUsers.map((user) => (
                        <li
                            key={user.id}
                            className={style["followed-user-item"]}
                            onClick={() => toggleUser(user.id)}
                        >
                            <div className={style["followed-user-info"]}>
                                <PostAvatar
                                    userId={user.id}
                                    username={user.username}
                                    displayName={user.display_name}
                                    avatar={user.avatar}
                                    thumbhash={user.avatar_thumbhash}
                                    width={32}
                                    height={32}
                                />

                                <div
                                    className={style["followed-user-username"]}
                                >
                                    {user.display_name ? (
                                        <>
                                            <p>{user.display_name}</p>
                                            <p
                                                className={
                                                    style["username-small"]
                                                }
                                            >
                                                {user.username}
                                            </p>
                                        </>
                                    ) : (
                                        <p>{user.username}</p>
                                    )}
                                </div>
                            </div>

                            {/* stop bubbling click event to parent li */}
                            <input
                                type="checkbox"
                                className={style["followed-user-checkbox"]}
                                checked={selectedUsers.includes(user.id)}
                                onChange={() => toggleUser(user.id)}
                                onClick={() => toggleUser(user.id)}
                            />
                        </li>
                    ))}
                </ul>

                <div className={style["bottom-container"]}>
                    <DialogClose
                        className={`${style["bottom-button"]} ${style["cancel-button"]}`}
                    >
                        Cancel
                    </DialogClose>

                    <button
                        type="button"
                        className={`${style["bottom-button"]} ${style["send-button"]}`}
                        disabled={!enableSubmit || isSubmitting}
                        onClick={handleSubmit}
                    >
                        {isSubmitting ? "Creating..." : "Create Messages"}
                    </button>
                </div>
            </DialogContent>
        </Dialog>
    );
}

export { CreateDirectMessage };
