import { MessageResponse } from "@/api/messages/messages";
import { UserResponse } from "@/api/user";
import { useState, useEffect, useRef } from "react";
import { useRealtime } from "@/app/realtime";
import { useRouter } from "next/navigation";
import { getMessages } from "@/api/messages/messages";
import { PostAvatar } from "../post/header/avatar";
import style from "./scss/message-content.module.scss";
import EPicker from "../createPost/emojipicker";
import { MdEmojiEmotions } from "react-icons/md";
import { FaPaperPlane } from "react-icons/fa6";
import { formatSocialMediaDate } from "../post/helpers/dateFormater";
import { SlOptions } from "react-icons/sl";

function MessageSkeleton() {
    return <div className="message-skeleton">Loading message...</div>;
}

// getting target profile through MessageList
// Currently only supports one-on-one messages
// no group
// * Need pagination

// check user_id (sender) before rendering message content and profile information
interface MessageContentProps {
    profile: UserResponse;
    currentUser: UserResponse;
}

function MessageContent({ profile, currentUser }: MessageContentProps) {
    const [loading, setLoading] = useState(false);

    const { messages: realtimeMessage, connected, sendMessage } = useRealtime();

    useEffect(() => {
        // append new realtime messages to the existing messages
        // setMessages((current) => [...current, ...realtimeMessage]);
        console.log("New realtime messages:", realtimeMessage);
    }, [realtimeMessage]);

    const [messages, setMessages] = useState<MessageResponse[]>([]);
    const [loadingMore, setLoadingMore] = useState(false);
    const [hasMore, setHasMore] = useState(true);
    const [beforeDate, setBeforeDate] = useState(new Date());

    const messageContentRef = useRef<HTMLElement | null>(null);
    const topSentinelRef = useRef<HTMLDivElement | null>(null);

    const [messageInput, setMessageInput] = useState("");

    const router = useRouter();
    const handlePathToProfile = () => {
        const URL = `/u/${profile.username}`;
        router.push(URL);
    };

    const handleEmojiClick = (emojiObject: any) => {
        setMessageInput((prev) => prev + emojiObject.emoji);
    };
    useEffect(() => {
        const loadMessages = async () => {
            setLoading(true);
            setMessages([]);
            setMessageInput("");
            setHasMore(true);

            const initialBefore = new Date();
            setBeforeDate(initialBefore);

            try {
                const res = await getMessages(
                    profile.id,
                    initialBefore.toISOString(),
                    11,
                );

                setMessages(res ?? []);

                if (!res || res.length < 11) {
                    setHasMore(false);
                }

                if (res && res.length > 0) {
                    setBeforeDate(new Date(res[0].created_at));
                }
            } catch (error) {
                console.error(error);
            } finally {
                setLoading(false);
            }
        };

        loadMessages();
    }, [profile.id]);

    // State and logic for handling message loading and pagination
    const loadMoreMessages = async () => {
        if (!hasMore || loadingMore || !messages || messages.length === 0) {
            return;
        }

        setLoadingMore(true);

        try {
            const res = await getMessages(
                profile.id,
                beforeDate.toISOString(),
                11,
            );

            if (!res || res.length === 0) {
                setHasMore(false);
                return;
            }

            setMessages((prevMessages) => {
                const existingIds = new Set(
                    prevMessages.map((message) => message.id),
                );

                const newMessages = res.filter(
                    (message) => !existingIds.has(message.id),
                );

                return [...newMessages, ...prevMessages];
            });

            const oldestMessage = res[0];
            setBeforeDate(new Date(oldestMessage.created_at));

            if (res.length < 11) {
                setHasMore(false);
            }
        } catch (error) {
            console.error(error);
        } finally {
            setLoadingMore(false);
        }
    };

    // observer
    useEffect(() => {
        const messageContent = messageContentRef.current;
        const topSentinel = topSentinelRef.current;

        if (!messageContent || !topSentinel) {
            return;
        }

        const observer = new IntersectionObserver(
            ([entry]) => {
                if (entry.isIntersecting) {
                    loadMoreMessages();
                }
            },
            {
                // * Use the message container as the viewport
                root: messageContent,
                threshold: 1.0,
            },
        );

        observer.observe(topSentinel);

        return () => {
            observer.disconnect();
        };
    }, [loadMoreMessages]);

    const handleSubmitMessage = () => {
        if (!messageInput.trim()) {
            return;
        }

        sendMessage(profile.id, messageInput);

        setMessageInput("");
    };

    useEffect(() => {
        const container = messageContentRef.current;

        if (!container || messages.length === 0) {
            return;
        }

        container.scrollTop = container.scrollHeight;
    }, [messages]);

    return (
        <section className={style["message-container"]}>
            <div className={style["message-header"]}>
                <PostAvatar
                    userId={profile.id}
                    username={profile.username}
                    displayName={profile.display_name ?? profile.username}
                    avatar={profile.avatar ?? null}
                    thumbhash={profile.avatar_thumbhash ?? null}
                />
                <span
                    onClick={handlePathToProfile}
                    className={style["message-header-username"]}
                >
                    {profile.display_name ?? profile.username}
                </span>
            </div>
            <section
                className={style["message-content"]}
                ref={messageContentRef}
            >
                {/* Top sentinel for loadmore */}
                <div ref={topSentinelRef} />

                {messages.slice().reverse().map((message) => (
                    <Message
                        key={message.id}
                        message={message}
                        target={profile} 
                        currentUser={currentUser}
                />
                ))}
            </section>
            <div className={style["message-input"]}>
                <EPicker onEmojiClick={handleEmojiClick}>
                    <MdEmojiEmotions />
                </EPicker>
                <textarea
                    className={style["message-textarea"]}
                    placeholder="Type a message..."
                    value={messageInput}
                    onChange={(e) => setMessageInput(e.target.value)}
                    maxLength={MAX_LENGTH_MESSAGE}
                />
                <button
                    type="button"
                    className={style["message-send-button"]}
                    onClick={handleSubmitMessage}
                >
                    <FaPaperPlane />
                </button>
            </div>
        </section>
    );
}

interface MessageProps {
    target: UserResponse;
    currentUser: UserResponse;
    message: MessageResponse;
}

const Message: React.FC<MessageProps> = ({ target, currentUser, message }) => {
    const router = useRouter();
    const handlePathToProfile = () => {
        const URL = `/u/${target.username}`;
        router.push(URL);
    };
    const messageUser =
        message.user_id === currentUser.id ? currentUser : target;
    return (
        <section className={style["message-item"]}>
            <div className={style["message-item-avatar"]}>
                <PostAvatar
                    userId={messageUser.id}
                    username={messageUser.username}
                    displayName={messageUser.display_name ?? messageUser.username}
                    avatar={messageUser.avatar ?? null}
                    thumbhash={messageUser.avatar_thumbhash ?? null}
                    width={36}
                    height={36}
                />
            </div>
            <div className={style["message"]}>
                <div className={style["message-item-header"]}>
                    <span
                        className={style["message-item-username"]}
                        onClick={handlePathToProfile}
                    >
                        {messageUser.display_name ?? messageUser.username}
                    </span>
                    <span>•</span>

                    <p className={style["message-item-date"]}>
                        {formatSocialMediaDate(message.created_at)}
                    </p>
                </div>

                <span className={style["message-item-content"]}>
                    {message.content}
                </span>
            </div>

            <div className={style["message-item-options"]}>
                <SlOptions />
            </div>
        </section>
    );
};

const MAX_LENGTH_MESSAGE = 500;

const TEST_MESSAGE: MessageResponse[] = [
    {
        id: "1",
        content: "This is a test message",
        user_id: "93443151091470336",
        target_id: "91533710922354688",
        has_attachment: false,
        has_reactions: false,
        created_at: new Date().toISOString(),
        updated_at: "",
    },
    {
        id: "2",
        content:
            "I wanted to share a more detailed update on where we are with the project so everyone has the same context. Over the past few days, I’ve been working through the main user flows and refining the overall layout based on the feedback from the previous review. The core functionality is now in place, but there are still several smaller areas that need attention, particularly around loading states, error handling, spacing, and making sure the interface behaves consistently across different screen sizes. I’ve also gone through some of the existing components and cleaned up a few parts that were becoming difficult to maintain. For the next step, I’m planning to focus on polishing the remaining screens, checking the edge cases, and making sure everything feels consistent before we consider this version ready for a wider review. Nothing major is blocking the work at the moment, so the current priority is mostly improving the details and making the overall experience feel more complete and reliable.",
        user_id: "91533710922354688",
        target_id: "93443151091470336",
        has_attachment: false,
        has_reactions: false,
        created_at: new Date().toISOString(),
        updated_at: "",
    },
    {
        id: "3",
        content:
            "I checked the latest changes this morning and everything looks pretty good so far. There are a couple of small things that could be adjusted, but nothing that should take too much time. The navigation feels much easier to understand now, and the new layout makes the important information easier to find.",
        user_id: "93443151091470336",
        target_id: "91533710922354688",
        has_attachment: false,
        has_reactions: true,
        created_at: new Date().toISOString(),
        updated_at: "",
    },
    {
        id: "4",
        content:
            "Thanks for the update. I agree that the overall flow feels much better now. I especially like the changes to the spacing and the way the different sections are grouped together. I’ll go through the remaining screens later today and check whether there are any inconsistencies that we should clean up before the next review.",
        user_id: "91533710922354688",
        target_id: "93443151091470336",
        has_attachment: false,
        has_reactions: false,
        created_at: new Date().toISOString(),
        updated_at: "",
    },
    {
        id: "5",
        content: "Sounds good. I’ll take care of the remaining small adjustments.",
        user_id: "93443151091470336",
        target_id: "91533710922354688",
        has_attachment: false,
        has_reactions: false,
        created_at: new Date().toISOString(),
        updated_at: "",
    },
    {
        id: "6",
        content:
            "One thing I noticed while testing the interface is that longer messages can make the conversation area grow quite a bit, especially when there are several messages sent close together. It might be worth checking how the container behaves when the content becomes much longer than expected, particularly on smaller screens where there is less horizontal space available.",
        user_id: "91533710922354688",
        target_id: "93443151091470336",
        has_attachment: false,
        has_reactions: true,
        created_at: new Date().toISOString(),
        updated_at: "",
    },
    {
        id: "7",
        content:
            "I’ve also tested the layout with a few different window sizes. The desktop version looks fine, but the smaller viewport exposes a few areas where the text gets quite close to the edge of the message bubble. I think adding a little more padding should make the messages easier to read without changing the overall design.",
        user_id: "93443151091470336",
        target_id: "91533710922354688",
        has_attachment: false,
        has_reactions: false,
        created_at: new Date().toISOString(),
        updated_at: "",
    },
    {
        id: "8",
        content:
            "That makes sense. I’ll keep the current structure and adjust the spacing rather than changing the entire component. It should also make the UI more consistent with the other parts of the application.",
        user_id: "91533710922354688",
        target_id: "93443151091470336",
        has_attachment: false,
        has_reactions: false,
        created_at: new Date().toISOString(),
        updated_at: "",
    },
    {
        id: "9",
        content:
            "After that, I think we should be in a good place for another round of testing. I’ll also check the empty state, loading state, and a conversation with a large number of messages so we can make sure the component behaves correctly in each situation.",
        user_id: "93443151091470336",
        target_id: "91533710922354688",
        has_attachment: false,
        has_reactions: true,
        created_at: new Date().toISOString(),
        updated_at: "",
    },
    {
        id: "10",
        content:
            "Perfect. Let’s keep the current implementation for now and focus on getting the details polished. Once those checks are done, we can review everything together and make any final adjustments that are actually necessary.",
        user_id: "91533710922354688",
        target_id: "93443151091470336",
        has_attachment: false,
        has_reactions: false,
        created_at: new Date().toISOString(),
        updated_at: "",
    },
];

export default MessageContent;
