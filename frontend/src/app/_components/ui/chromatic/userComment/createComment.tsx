"use client";
import { UserResponse } from "@/api/user";
import React, { useState } from "react";
import { useFloating, useDismiss } from "@floating-ui/react";
import { PostAvatar } from "../post/header/avatar";
import style from "./create-comment.module.scss";
import { MdClose, MdEmojiEmotions, MdImage } from "react-icons/md";
import EPicker from "../createPost/emojipicker";
import { FaPaperPlane } from "react-icons/fa";
import { createComment } from "@/api/post/comments";

interface CreateCommentProps {
    postId: string;
    author: UserResponse;
    onCommentCreated?: () => void;
}

const CreateComment: React.FC<CreateCommentProps> = ({ postId, author, onCommentCreated }) => {
    const [text, setText] = React.useState("");
    const [media, setMedia] = React.useState<MediaFileProps[]>([]);

    const [isSubmittable, setIsSubmittable] = React.useState(false);
    const [isPending, setIsPending] = React.useState(false);
    const [activeIndex, setActiveIndex] = useState(0);

    const [isOpenEmoji, setIsOpenEmoji] = React.useState(false);

    const { refs, context } = useFloating({
        open: isOpenEmoji,
        onOpenChange: setIsOpenEmoji,
    });
    const dismiss = useDismiss(context);

    const textareaRef = React.useRef<HTMLTextAreaElement>(null);
    const warpperRef = React.useRef<HTMLDivElement>(null);
    const fileInputRef = React.useRef<HTMLInputElement | null>(null);

    React.useEffect(() => {
        if (textareaRef.current) {
            textareaRef.current.style.height = "auto";
            textareaRef.current.style.height = `${textareaRef.current.scrollHeight}px`;
        }
    }, [text]);
    const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        const selectedFiles = Array.from(event.target.files ?? []);
        const remaining = MAX_MEDIA_FILES - media.length;

        if (remaining <= 0) {
            return;
        }

        const newMedia = selectedFiles
            .slice(0, remaining)
            .filter((file) => file.type.startsWith("image/"))
            .map((file) => ({
                file,
                url: URL.createObjectURL(file),
                type: "image" as const,
            }));

        setMedia((current) => [...current, ...newMedia]);
        event.target.value = "";
    };

    const handleRemoveMedia = (index: number) => {
        setMedia((current) => current.filter((_, i) => i !== index));
    };

    const handleTextChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
        setText(e.target.value);
    };

    const handleEmojiClick = (emojiObject: any, event: MouseEvent) => {
        setText((current) => current + emojiObject.emoji);
    };

    const MediaContainerRef = React.useRef<HTMLDivElement | null>(null);
    const groupRef = React.useRef<HTMLDivElement | null>(null);
    const [translateX, setTranslateX] = useState(0);
    const [showRightController, setShowRightController] = useState(false);
    const [showLeftController, setShowLeftController] = useState(false);

    React.useEffect(() => {
        const container = MediaContainerRef.current;
        const group = groupRef.current;

        if (!container || !group) {
            return;
        }
        const maxTranslate = Math.max(
            group.scrollWidth - container.clientWidth,
            0,
        );

        const targetTranslate = Math.min(
            activeIndex * group.scrollWidth,
            maxTranslate,
        );

        targetTranslate >= maxTranslate
            ? setShowRightController(false)
            : setShowRightController(true);
        targetTranslate <= 0
            ? setShowLeftController(false)
            : setShowLeftController(true);

        setTranslateX(targetTranslate);
    }, [activeIndex, media, groupRef]);

    const slideMedia = (direction: "left" | "right") => {
        setActiveIndex((currentIndex) => {
            if (direction === "right") {
                return Math.min(currentIndex + 1, media.length - 1);
            }

            return Math.max(currentIndex - 1, 0);
        });
    };

    const handlePaste = (e: React.ClipboardEvent<HTMLTextAreaElement>) => {
        const files = Array.from(e.clipboardData.items)
            .filter((item) => item.kind === "file")
            .map((item) => item.getAsFile())
            .filter((file): file is File => file !== null);

        if (files.length === 0) {
            return;
        }

        const remaining = MAX_MEDIA_FILES - media.length;

        const newMedia = files
            .slice(0, remaining)
            .filter((file) => file.type.startsWith("image/"))
            .map((file) => ({
                file,
                url: URL.createObjectURL(file),
                type: "image" as const,
            }));

        setMedia((current) => [...current, ...newMedia]);
    };

    React.useEffect(() => {
        setIsSubmittable(text.trim().length > 0 || media.length > 0);
    }, [text, media]);

    if (!author) {
        return null;
    }

    const handleSubmit = async () => {
        if (!isSubmittable) {
            return;
        }
        const allMedia: File[] = media.map((item) => item.file);

        const formData = new FormData();
        if (text != undefined) {
            formData.append("content", text);
        }
        if (allMedia != undefined) {
            allMedia.forEach((file) => {
                formData.append("files", file);
            });
        }

        try {
            setIsPending(true);
            setIsSubmittable(false);
            const res = await createComment(postId, formData);
            if (res) {
                setText("");
                setMedia([]);
                if (onCommentCreated) {
                    onCommentCreated();
                }
            }
        } catch (err) {
            console.error(err);
        } finally {
            setIsSubmittable(true);
            setIsPending(false);
        }
    };

    return (
        <section className={style["create-comment"]}>
            <form action="#" className={style["container"]}>
                <section className={style["profile-section"]}>
                    <div className={style["profile"]} key={author.id}>
                        <PostAvatar
                            userId={author.id}
                            username={author.username}
                            displayName={author.display_name}
                            avatar={author.avatar}
                            thumbhash={author.avatar_thumbhash}
                        />
                    </div>

                    <div className={style["text-container"]} ref={warpperRef}>
                        <textarea
                            ref={textareaRef}
                            className={style["content"]}
                            placeholder="Enter your comment here."
                            value={text}
                            maxLength={MAX_TEXT_LENGTH}
                            onChange={handleTextChange}
                            onPaste={handlePaste}
                        />
                    </div>
                </section>

                <div
                    style={{ display: media.length > 0 ? "block" : "none" }}
                    className={style["media-container"]}
                    ref={MediaContainerRef}
                >
                    {showLeftController && (
                        <button
                            type="button"
                            onClick={() => slideMedia("left")}
                            className={style["show-left"]}
                        >
                            &lt;
                        </button>
                    )}
                    {showRightController && (
                        <button
                            type="button"
                            onClick={() => slideMedia("right")}
                            className={style["show-right"]}
                        >
                            &gt;
                        </button>
                    )}
                    <div
                        className={style["media-group"]}
                        ref={groupRef}
                        style={{
                            transform: `translate3d(-${translateX}px, 0, 0)`,
                        }}
                    >
                        {media.map((file, index) => (
                            <div key={index} className={style["media-item"]}>
                                <button
                                    className={style["remove-media"]}
                                    type="button"
                                    onClick={() => handleRemoveMedia(index)}
                                >
                                    <MdClose />
                                </button>
                                {file.type === "image" && (
                                    <img
                                        src={file.url}
                                        draggable="false"
                                        alt=""
                                    />
                                )}
                            </div>
                        ))}
                    </div>
                </div>

                <div className={style["separator"]} />

                <div className={style["extended"]}>
                    <section className={style["button-group"]}>
                        <button
                            type="button"
                            onClick={() => {
                                fileInputRef.current?.click();
                            }}
                        >
                            <input
                                ref={fileInputRef}
                                type="file"
                                accept="image/*"
                                multiple
                                onChange={handleFileChange}
                                style={{ display: "none" }}
                            />
                            <MdImage />
                        </button>
                        <EPicker onEmojiClick={handleEmojiClick}>
                            <MdEmojiEmotions />
                        </EPicker>
                    </section>

                    <button
                        className={style["send-button"]}
                        disabled={!isSubmittable}
                        onClick={handleSubmit}
                    >
                        {!isPending ? (
                            <>
                                <FaPaperPlane />
                                <span>Post</span>
                            </>
                        ) : (
                            <>
                                <img
                                    src="/asset/svgs/dot_loading.svg"
                                    alt="Post"
                                />
                                <span>Post</span>
                            </>
                        )}
                    </button>
                </div>
            </form>
        </section>
    );
};

interface MediaFileProps {
    file: File;
    url: string;
    type: "image";
}
const MAX_TEXT_LENGTH = 1000;
const MAX_MEDIA_FILES = 5;

export default CreateComment;
