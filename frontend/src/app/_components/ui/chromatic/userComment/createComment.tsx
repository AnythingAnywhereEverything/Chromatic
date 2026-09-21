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

const CreateComment: React.FC<CreateCommentProps> = ({
    postId,
    author,
    onCommentCreated,
}) => {
    const [text, setText] = React.useState("");
    const [media, setMedia] = React.useState<MediaFileProps[]>([]);

    const [isSubmittable, setIsSubmittable] = React.useState(false);
    const [isPending, setIsPending] = React.useState(false);

    const textareaRef = React.useRef<HTMLTextAreaElement>(null);
    const warpperRef = React.useRef<HTMLDivElement>(null);

    React.useEffect(() => {
        if (textareaRef.current) {
            textareaRef.current.style.height = "auto";
            textareaRef.current.style.height = `${textareaRef.current.scrollHeight}px`;
        }
    }, [text]);

    const handleTextChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
        setText(e.target.value);
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

                    <div className={style["content-section"]}>
                        <p className={style["author-info"]}>
                            {author.display_name ? author.display_name : author.username}
                        </p>
                        <div
                            className={style["text-container"]}
                            ref={warpperRef}
                        >
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
                    </div>
                    <button
                        className={style["send-button"]}
                        disabled={!isSubmittable}
                        onClick={handleSubmit}
                    >
                        {!isPending ? (
                            <>
                                <FaPaperPlane />
                                <span>Send</span>
                            </>
                        ) : (
                            <>
                                <img
                                    src="/asset/svgs/dot_loading.svg"
                                    alt="Post"
                                />
                                <span>Send</span>
                            </>
                        )}
                    </button>
                    <span className={style["text-counter"]}>
                        {text.length}/{MAX_TEXT_LENGTH}
                    </span>
                </section>
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
