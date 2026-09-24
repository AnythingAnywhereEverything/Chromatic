import {
    MessageResponse,
    MESSAGE_PAGE_LIMIT,
    getMessages,
} from "@/api/messages/messages";
import { UserResponse } from "@/api/user";
import { useState, useEffect, useRef } from "react";
import { useRealtime } from "@/app/realtime";
import { useRouter } from "next/navigation";
import { PostAvatar } from "../post/header/avatar";
import style from "./scss/message-content.module.scss";
import EPicker from "../createPost/emojipicker";
import { MdEmojiEmotions } from "react-icons/md";
import { FaPaperPlane } from "react-icons/fa6";
import { formatSocialMediaDate } from "../post/helpers/dateFormater";
import { SlOptions } from "react-icons/sl";
import {
    Dropdown,
    DropdownContent,
    DropdownItem,
    DropdownTrigger,
} from "../dropdown";

function MessageSkeleton() {
    return <div className="message-skeleton">Loading message...</div>;
}

// getting target profile through MessageList
// Currently only supports one-on-one messages
// no group
// * Need pagination

// check user_id (sender) before rendering message content and profile information
interface MessageContentProps {
    target: UserResponse;
    currentUser: UserResponse;
}

function MessageContent({ target, currentUser }: MessageContentProps) {
    const [loading, setLoading] = useState(false);

    const {
        message: realtimeMessage,
        connected,
        sendMessage,
        deleteMessage,
        deletedMessage,
    } = useRealtime();

    useEffect(() => {
        // check if the new realtime message belongs to the current chat

        if (realtimeMessage) {
            if (
                realtimeMessage.recipient_id !== target.id &&
                realtimeMessage.sender_id !== target.id
            ) {
                return;
            }

            // check if message id was already in the current messages list
            if (messages.some((msg) => msg.id === realtimeMessage.id)) {
                return;
            }

            const wrappedMessage: MessageResponse = {
                id: realtimeMessage.id,
                user_id: realtimeMessage.sender_id,
                target_id: realtimeMessage.recipient_id,
                content: realtimeMessage.content,
                has_attachment: false,
                has_reactions: false,
                created_at: new Date().toISOString(), // assume as now
                updated_at: new Date().toISOString(), // assume as now
            };
            setMessages((current) => [wrappedMessage, ...current]);
        }
    }, [realtimeMessage]);

    useEffect(() => {
        if (!deletedMessage) {
            return;
        }

        if (
            deletedMessage.sender_id !== target.id &&
            deletedMessage.recipient_id !== target.id
        ) {
            return;
        }

        setMessages((current) =>
            current.filter((msg) => msg.id !== deletedMessage.id),
        );
    }, [deletedMessage, target.id]);

    const [messages, setMessages] = useState<MessageResponse[]>([]);
    const [loadingMore, setLoadingMore] = useState(false);
    const [hasMore, setHasMore] = useState(true);
    const [beforeDate, setBeforeDate] = useState(new Date());
    const [beforeId, setBeforeId] = useState<string | undefined>(undefined);

    const messageContentRef = useRef<HTMLElement | null>(null);
    const topSentinelRef = useRef<HTMLDivElement | null>(null);
    const initialLoadRef = useRef(true);

    const [messageInput, setMessageInput] = useState("");

    const router = useRouter();
    const handlePathToProfile = () => {
        const URL = `/u/${target.username}`;
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
            initialLoadRef.current = true;

            const initialBefore = new Date();
            setBeforeDate(initialBefore);

            try {
                const res = await getMessages(
                    target.id,
                    initialBefore.toISOString(),
                    MESSAGE_PAGE_LIMIT,
                );

                setMessages(res ?? []);

                if (!res || res.length < MESSAGE_PAGE_LIMIT) {
                    setHasMore(false);
                }

                if (res && res.length > 0) {
                    setBeforeDate(new Date(res[res.length - 1].created_at));
                    setBeforeId(res[res.length - 1].id);
                    setBeforeId(res[res.length - 1].id);
                }
            } catch (error) {
                console.error(error);
            } finally {
                setLoading(false);
            }
        };

        loadMessages();
    }, [target.id]);

    // State and logic for handling message loading and pagination
    const loadMoreMessages = async () => {
        if (!hasMore || loadingMore || !messages || messages.length === 0) {
            return;
        }

        setLoadingMore(true);

        try {
            const res = await getMessages(
                target.id,
                beforeDate.toISOString(),
                MESSAGE_PAGE_LIMIT,
                beforeId,
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

                return [...prevMessages, ...newMessages];
            });

            setBeforeDate(new Date(res[res.length - 1].created_at));

            if (res.length < MESSAGE_PAGE_LIMIT) {
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

        sendMessage(target.id, messageInput);

        setMessageInput("");
    };


    useEffect(() => {
        const container = messageContentRef.current;

        if (!container || messages.length === 0) {
            return;
        }

        const nearBottom =
            container.scrollHeight -
                container.scrollTop -
                container.clientHeight <=
            SCROLL_THRESHOLD;

        if (initialLoadRef.current || nearBottom) {
            container.scrollTop = container.scrollHeight;
            initialLoadRef.current = false;
        }
    }, [messages]);

    return (
        <section className={style["message-container"]}>
            <div className={style["message-header"]}>
                <PostAvatar
                    userId={target.id}
                    username={target.username}
                    displayName={target.display_name ?? target.username}
                    avatar={target.avatar ?? null}
                    thumbhash={target.avatar_thumbhash ?? null}
                />
                <span
                    onClick={handlePathToProfile}
                    className={style["message-header-username"]}
                >
                    {target.display_name ?? target.username}
                </span>
            </div>
            <section
                className={style["message-content"]}
                ref={messageContentRef}
            >
                {/* Top sentinel for loadmore */}
                <div ref={topSentinelRef} />

                {messages
                    .slice()
                    .reverse()
                    .map((message) => (
                        <Message
                            key={message.id}
                            message={message}
                            target={target}
                            currentUser={currentUser}
                            deleteMessage={deleteMessage}
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
    deleteMessage: (sender_id: string, messageId: string) => void;
}

const Message: React.FC<MessageProps> = ({ target, currentUser, message, deleteMessage }) => {
    const router = useRouter();

    const messageUser =
        message.user_id === currentUser.id ? currentUser : target;

    const handlePathToProfile = () => {
        const URL = `/u/${messageUser.username}`;
        router.push(URL);
    };

    const handleCopy = () => {
        navigator.clipboard.writeText(message.content);
    };

    return (
        <section className={style["message-item"]}>
            <div className={style["message-item-avatar"]}>
                <PostAvatar
                    userId={messageUser.id}
                    username={messageUser.username}
                    displayName={
                        messageUser.display_name ?? messageUser.username
                    }
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
                <Dropdown>
                    <DropdownTrigger>
                        <SlOptions />
                    </DropdownTrigger>
                    <DropdownContent>
                        <DropdownItem>
                            <button type="button" onClick={handleCopy}>
                                Copy
                            </button>
                        </DropdownItem>
                        {message.user_id === currentUser.id && (
                            <DropdownItem>
                                <button
                                    type="button"
                                    onClick={() =>
                                        deleteMessage(currentUser.id, message.id)
                                    }
                                >
                                    Delete
                                </button>
                            </DropdownItem>
                        )}
                    </DropdownContent>
                </Dropdown>
            </div>
        </section>
    );
};

const MAX_LENGTH_MESSAGE = 500;
const SCROLL_THRESHOLD = 150;
export default MessageContent;
