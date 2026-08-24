"use client";

import { useEffect, useRef, useState } from "react";
import style from "./style.module.scss";
import { LuThumbsUp } from "react-icons/lu";
import { GoComment, GoDotFill } from "react-icons/go";
import { IoMdShare } from "react-icons/io";
import {
    IoBookmarkOutline,
    IoClipboardOutline,
    IoClose,
} from "react-icons/io5";
import { BsThreeDots } from "react-icons/bs";
import { deletePost, mediaPostProps } from "@/api/post/getFeed";
import {
    Dialog,
    DialogClose,
    DialogContent,
    DialogHeading,
    DialogTrigger,
} from "../dialogue";
import { Tooltip, TooltipContent, TooltipTrigger } from "../tooltip";
import { TogglePostLike } from "@/api/post/like";
import { formatSocialMediaDate } from "./dataformat";
import {
    Dropdown,
    DropdownContent,
    DropdownItem,
    DropdownTrigger,
} from "../dropdown";
import { MediaGroup } from "./mediagroup";
import { PostAvatar } from "./profile";
import { useRouter } from "next/navigation";

// todo: community will be add soon
const Post: React.FC<mediaPostProps> = ({
    id,
    user_id, // owner
    username,
    content,
    total_comment,
    total_likes,
    visibility,
    is_repost,
    repost_from,
    has_attachment,
    created_at,
    updated_at,
    media = [],
    tag = [],
    is_liked,
    current_user_id,
    display_name,
    avatar_path,
    avatar_mime,
    avatar_thumbhash,
    banner_path,
    banner_mime,
    banner_thumbhash,
    followers_count,
    following_count,
}) => {
    const [open, setOpen] = useState(false);
    const [showReadMoreButton, setShowReadMoreButton] = useState(false);
    const ref = useRef<HTMLSpanElement | null>(null);
    const [openOption, setOpenOption] = useState(false);
    const [likeState, setLikeState] = useState(is_liked);
    const [likeCount, setLikeCount] = useState(total_likes);
    const [hoveredMediaId, setHoveredMediaId] = useState<string | null>(null);
    const [mediaSrc, setMediaSrc] = useState<string | null>(null);
    const rootRef = useRef<HTMLDivElement>(null);

    const hasDisplayName = display_name || null;
    const handleDeletePost = async () => {
        try {
            console.log("Deleting post with ID:", id);
            await deletePost(id);
            // Optionally, you can add a callback to remove the post from the UI after deletion
        } catch (error) {
            console.error("Failed to delete post:", error);
        }
    };
    useEffect(() => {
        setLikeState(is_liked);
    }, [is_liked]);

    useEffect(() => {
        if (ref.current) {
            setShowReadMoreButton(
                ref.current.scrollHeight !== ref.current.clientHeight,
            );
        }
    }, []);
    // mediaSrc is set on hover per-item; no global effect needed
    return (
        <section className={style["container"]} 
        key={id}
        >
            <div className={style["header"]}>
                <section className={style["profile"]} >
                    <div className={style["avatar"]} key={user_id}>
                        <PostAvatar 
                        userId={user_id} 
                        username={username} 
                        displayName={display_name} 
                        avatar={avatar_path} 
                        thumbhash={avatar_thumbhash}
                        containerRef={rootRef}
                        />
                    </div>
                    <div className={style["user-info"]}>
                        <div className={style["username"]}>
                            <p>
                                {display_name || username}
                            </p>
                        </div>
                        <div className={style["post-date"]}>
                            <p>
                                {hasDisplayName && (
                                    <>
                                         {username} <GoDotFill style={{fontSize: "var(--text-small)"}} />
                                    </>
                                )}
                            </p>
                            <p>{formatSocialMediaDate(created_at)}</p>
                        </div>
                    </div>
                    <div className={style["option"]}>
                        <Dropdown>
                            <DropdownTrigger asChild>
                                <BsThreeDots />
                            </DropdownTrigger>
                            {user_id === current_user_id ? (
                                <DropdownContent>
                                    <DropdownItem>Edit Post</DropdownItem>
                                    <DropdownItem>
                                        <button
                                            type="button"
                                            onClick={handleDeletePost}
                                        >
                                            Delete Post
                                        </button>
                                    </DropdownItem>
                                </DropdownContent>
                            ) : (
                                <DropdownContent>
                                    <DropdownItem>
                                        Follow @{username}
                                    </DropdownItem>
                                    <DropdownItem>
                                        Add Friend @{username}
                                    </DropdownItem>
                                    <DropdownItem>
                                        Block @{username}
                                    </DropdownItem>
                                    <DropdownItem>Report</DropdownItem>
                                </DropdownContent>
                            )}
                        </Dropdown>
                    </div>
                </section>
                {/* //todo: dropdown options for user */}
            </div>

            <div className={style["main-container"]}>
                <div className={style["text-container"]}>
                    <span
                        className={`${style["content"]} ${!open ? style["is-collapsed"] : ""}`}
                        ref={ref}
                    >
                        {content}
                    </span>
                    <div>
                        {showReadMoreButton && (
                            <button
                                type="button"
                                onClick={() => setOpen(!open)}
                                className={style["read-more-btn"]}
                            >
                                {open ? "Show less" : "Read more"}
                            </button>
                        )}
                    </div>
                </div>

                {/* //*--------------------has attachment cp---------------- */}
                {has_attachment && <MediaGroup media={media} />}
                {/* //todo: */}
                <ul className={style["subject-tag"]}>
                    {tag.map((item) => {
                        return (
                            <li key={item.tag_id}>
                                <p>{item.tag_name}</p>
                            </li>
                        );
                    })}
                </ul>
            </div>

            <div className={style["bottom-container"]}>
                <section className={style["interaction"]}>
                    <div style={{ userSelect: "none" }}>
                        {/* //todo: Add animation if possible*/}
                        <button
                            type="button"
                            style={{ cursor: "pointer" }}
                            onClick={async () => {
                                try {
                                    const nextLikeState = !likeState;

                                    console.log({
                                        id,
                                        is_liked: likeState,
                                        likeState,
                                        nextLikeState,
                                    });

                                    const response = await TogglePostLike(
                                        id,
                                        nextLikeState,
                                    );

                                    setLikeState(nextLikeState);
                                    setLikeCount(response.total_liked);
                                } catch (error) {
                                    console.error(
                                        "Failed to toggle like:",
                                        error,
                                    );
                                }
                            }}
                        >
                            <LuThumbsUp />
                        </button>
                        {/* //todo: onClick get panigation user liked on post */}
                        <button
                            className={style["has-hover"]}
                            style={{ cursor: "pointer" }}
                            type="button"
                        >
                            {likeCount || 0}
                        </button>
                    </div>
                    <div style={{ userSelect: "none", cursor: "pointer" }}>
                        {/* //todo: onClick pass to specific post and fetch comment */}
                        <button type="button">
                            <GoComment />
                        </button>
                        {total_comment || 0}
                    </div>
                </section>

                <section className={style["interaction"]}>
                    {/* //todo: dialog for share *if possible */}
                    <button type="button" style={{ cursor: "pointer" }}>
                        <DialogSharePost />
                    </button>
                    {/* //todo: bookmark ofc why not xdddddddddddd */}
                    <button type="button" style={{ cursor: "pointer" }}>
                        <IoBookmarkOutline />
                    </button>
                </section>
            </div>
        </section>
    );
};

export { Post };

export function DialogSharePost() {
    const [linkToCopy, setLinkToCopy] = useState(
        "asidnsadjasodaijdiajsidjasidjajdoiasjidjsadjiasjdiaj",
    );
    const [isCopied, setIsCopied] = useState(true);
    const [errorText, setErrorText] = useState("");

    async function copyToClipBoard() {
        try {
            await navigator.clipboard.writeText(linkToCopy);
            setIsCopied(true);
            setTimeout(() => setIsCopied(false), 2000);
        } catch (err) {
            setErrorText("Faield to copy link : " + err);
            setTimeout(() => setErrorText(""), 5000);
        }
    }

    const dialogRef = useRef<HTMLDivElement | null>(null);

    return (
        <Dialog overlayClassName={style["link-dialog-overlay"]}>
            <DialogTrigger asChild>
                <IoMdShare />
            </DialogTrigger>

            <DialogContent className={style["link-container"]}>
                <DialogHeading className={style["header"]}>
                    <DialogClose className={style["box"]}>
                        <IoClose />
                    </DialogClose>
                    <p className={style["title"]}>Share</p>
                    <div className={style["box"]}></div>
                </DialogHeading>
                <section className={style["main"]}>
                    <section className={style["clip-board"]}>
                        <div className={style["board"]} ref={dialogRef}>
                            {/* //fixme : tooltip somehow is showing behind z-index 999*/}
                            <Tooltip parent={dialogRef.current} open={isCopied}>
                                <TooltipTrigger asChild>
                                    {isCopied}
                                </TooltipTrigger>

                                <TooltipContent
                                    style={{
                                        zIndex: 9999,
                                        position: "relative",
                                        top: "-10px",
                                        left: "-10px",
                                    }}
                                >
                                    asdmadiaidsjdjsajdiasjdsaijdaijadss
                                </TooltipContent>
                            </Tooltip>

                            <input
                                type="text"
                                value={linkToCopy}
                                onChange={(e) => setLinkToCopy(e.target.value)}
                                onClick={copyToClipBoard}
                                readOnly
                            />
                            <div className={style["copy-button"]}>
                                <button type="button" onClick={copyToClipBoard}>
                                    {/* //todo: on complete changing icon */}
                                    <IoClipboardOutline />
                                </button>
                            </div>
                        </div>
                    </section>
                    <div>{errorText}</div>
                </section>
            </DialogContent>
        </Dialog>
    );
}
