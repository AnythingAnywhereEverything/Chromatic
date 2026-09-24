"use client";

import style from "./style.module.scss";
import React, { useState } from "react";
import { GetAllTagAttachments } from "@/api/tags/tags";

import { CreatePost } from "@/api/post/post";
import { PostAvatar } from "../post/header/avatar";
import { PostStatus, Visibility } from "./status";
import { UserResponse } from "@/api/user";
import { MdClose, MdEmojiEmotions, MdImage } from "react-icons/md";
import { FaPaperPlane } from "react-icons/fa6";
import { PostProps } from "@/api/post/getFeed";
import EPicker from "./emojipicker";
import { TagRow } from "@/api/tags/tags";
import PostTags from "./tags";

// TODO: Zartex, Refactor this.

interface CreatePostProps {
    author: UserResponse;
    onPostCreated?: (res: PostProps) => void;
}

let tagAttachmentsCache: TagRow[] = [];

function CreatePostComponent({ author, onPostCreated }: CreatePostProps) {
    const MAX_TEXT_LENGTH = 2500;

    const [text, setText] = useState("");
    const [hasEdited, setHasEdited] = useState(false);

    const [media, setMedia] = useState<MediaFileProps[]>([]);
    const [activeIndex, setActiveIndex] = useState(0);

    const [visibility, setVisibility] = useState<Visibility>(
        Visibility.Everyone,
    );
    const [isSubmittable, setIsSubmittable] = useState(false);
    const [isPending, setIsPending] = useState(false);
    const [tagAttachments, setTagAttachments] = useState<TagRow[]>([]);
    // Taking 1 tag to cause less complexity and easier management in demo

    const [selectedTag, setSelectedTag] = useState<TagRow | null>(null);
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
            .filter(
                (file) =>
                    file.type.startsWith("image/") ||
                    file.type.startsWith("video/"),
            )
            .map((file) => ({
                file,
                url: URL.createObjectURL(file),
                type: file.type.startsWith("video/")
                    ? ("video" as const)
                    : ("image" as const),
            }));

        setMedia((current) => [...current, ...newMedia]);
    };

    const handleTextChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
        setText(e.target.value);
    };

    const handleEmojiClick = (emojiObject: any, event: MouseEvent) => {
        setText((current) => current + emojiObject.emoji);
    };

    React.useEffect(() => {
        setHasEdited(text.length > 0 || media.length > 0);
    }, [text, media]);

    const handleRemoveMedia = (index: number) => {
        setMedia((current) => current.filter((_, i) => i !== index));
    };

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
        if (visibility != undefined) {
            formData.append("visibility", visibility);
        }
        if (selectedTag != undefined) {
            formData.append("media_tags", selectedTag.id);
        }

        try {
            setIsSubmittable(false);
            setIsPending(true);

            const res = await CreatePost(formData);

            if (res) {
                setText("");
                setMedia([]);
                setHasEdited(false);
                if (onPostCreated) {
                    onPostCreated(res);
                }
            }
            console.log("Post created successfully:", res);
        } catch (err) {
            console.error(err);
        } finally {
            setIsSubmittable(true);
            setIsPending(false);
        }
    };

    React.useEffect(() => {
        setIsSubmittable(hasEdited || media.length > 0);
    }, [hasEdited, media]);

    const textareaRef = React.useRef<HTMLTextAreaElement | null>(null);
    React.useLayoutEffect(() => {
        if (textareaRef.current) {
            textareaRef.current.style.height = "auto";
            textareaRef.current.style.height = `${textareaRef.current.scrollHeight}px`;
        }
    }, [text]);

    const fileInputRef = React.useRef<HTMLInputElement | null>(null);
    const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        const selectedFiles = Array.from(event.target.files ?? []);
        const remaining = MAX_MEDIA_FILES - media.length;

        if (remaining <= 0) {
            return;
        }

        const newMedia = selectedFiles
            .slice(0, remaining)
            .filter(
                (file) =>
                    file.type.startsWith("image/") ||
                    file.type.startsWith("video/"),
            )
            .map((file) => ({
                file,
                url: URL.createObjectURL(file),
                type: file.type.startsWith("video/")
                    ? ("video" as const)
                    : ("image" as const),
            }));

        setMedia((current) => [...current, ...newMedia]);
        event.target.value = "";
    };

    const handleTagClick = (tag: TagRow) => {
        console.log("clicked:", tag);
        setSelectedTag(tag);
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

    React.useEffect(() => {
        const fetchTags = async () => {
            if (tagAttachmentsCache.length === 0) {
                const tags = await GetAllTagAttachments();
                if (tags) {
                    tagAttachmentsCache = tags;
                    setTagAttachments(tags);
                    console.log("Fetched and cached tags:", tags);
                }
            } else {
                setTagAttachments(tagAttachmentsCache);
            }
        };

        fetchTags();
    }, []);

    return (
        <section className={style["create-container"]}>
            <div className={style["editable-contents"]}>
                <div className={style["header"]}>
                    <PostAvatar
                        className={style["profile"]}
                        userId={author.id}
                        username={author.username}
                        displayName={author.display_name || author.username}
                        thumbhash={author.avatar_thumbhash ?? null}
                        avatar={author.avatar ?? null}
                    />
                    <div className={style["text-container"]}>
                        <textarea
                            placeholder="Which topic do you want to discuss?"
                            value={text}
                            maxLength={MAX_TEXT_LENGTH}
                            onChange={handleTextChange}
                            onPaste={handlePaste}
                            ref={textareaRef}
                        />
                    </div>
                </div>
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
                                {file.type === "image" ? (
                                    <img
                                        src={file.url}
                                        draggable="false"
                                        alt=""
                                    />
                                ) : (
                                    <video
                                        src={file.url}
                                        controls
                                        draggable="false"
                                        disablePictureInPicture
                                    />
                                )}
                            </div>
                        ))}
                    </div>
                </div>

                {hasEdited && (
                    <div className={style["visibility"]}>
                        <PostStatus
                            visibility={visibility}
                            onChange={(value) => setVisibility(value)}
                        />

                        <PostTags
                            tags={tagAttachments}
                            selectedTag={selectedTag}
                            onChangeTag={handleTagClick}
                        />

                    </div>
                )}
            </div>

            <div className={style["separator"]} />

            <div className={style["extended"]}>
                <div className={style["button-group"]}>
                    <button
                        type="button"
                        onClick={() => fileInputRef.current?.click()}
                    >
                        <input
                            ref={fileInputRef}
                            type="file"
                            accept="image/*,video/*"
                            multiple
                            onChange={handleFileChange}
                            style={{ display: "none" }}
                        />
                        <MdImage />
                    </button>
                    <EPicker onEmojiClick={handleEmojiClick}>
                        <MdEmojiEmotions />
                    </EPicker>
                </div>
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
                            <img src="/asset/svgs/dot_loading.svg" alt="Post" />
                            <span>Post</span>
                        </>
                    )}
                </button>
            </div>
        </section>
    );
}

export type CreatePostPayload = {
    content?: string;
    media_src?: File[];
    visability: Visibility;
    tags?: string[];
};

interface MediaFileProps {
    file: File;
    url: string;
    type: "image" | "video";
}
const MAX_MEDIA_FILES = 5;

export { CreatePostComponent };
