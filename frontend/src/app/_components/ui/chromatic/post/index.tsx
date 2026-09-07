"use client";

import { useEffect, useRef, useState } from "react";
import style from "./style.module.scss";
import { LuThumbsUp } from "react-icons/lu";
import { GoComment } from "react-icons/go";
import { IoMdShare } from "react-icons/io";
import {
    IoBookmarkOutline,
    IoClipboardOutline,
    IoClose,
} from "react-icons/io5";
import { PostProps } from "@/api/post/getFeed";
import {
    Dialog,
    DialogClose,
    DialogContent,
    DialogHeading,
    DialogTrigger,
} from "../dialogue";
import { Tooltip, TooltipContent, TooltipTrigger } from "../tooltip";
import { TogglePostLike } from "@/api/post/like";

import { MediaGroup } from "./mediagroup";
import { useUser } from "@/hooks/useUser";
import PostHeader from "./header";
import BottomPostInteraction from "./interaction";

// NOTE: Add support for community posts, custom popup to display and fetch comments.
const Post: React.FC<PostProps> = ({
    post_id,
    author, // post owner
    content,
    total_comments,
    total_likes,
    visibility,
    is_reposted,
    reposted_post,
    has_attachment,
    created_at,
    updated_at,
    attachments = [],
    tag = [],
    is_liked = false,
}) => {
    const [open, setOpen] = useState(false);
    const [showReadMoreButton, setShowReadMoreButton] = useState(false);
    const ref = useRef<HTMLSpanElement | null>(null);
    const [likeState, setLikeState] = useState(is_liked);
    const [likeCount, setLikeCount] = useState(total_likes);
    const rootRef = useRef<HTMLDivElement>(null);

    let currentUserId = useUser().data?.id;
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

    return (
        <section
            className={style["container"]}
            //  ! remove before push
            onClick={() => {
                console.log(post_id);
            }}
        >
            <PostHeader
                author={{
                    id: author.id,
                    username: author.username,
                    display_name: author.display_name,
                    avatar: author.avatar,
                    avatar_thumbhash: author.avatar_thumbhash,
                }}
                created_at={created_at}
                postId={post_id}
                visibility={visibility}
            />

            <div className={style["main-container"]}>
                {content && content.length > 0 && (
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
                )}

                {/* //*--------------------has attachment cp---------------- */}
                {has_attachment && <MediaGroup media={attachments} />}
                {/* //todo: */}
                <ul className={style["subject-tag"]}>
                    {tag.map((item) => {
                        return (
                            <li
                                key={item.tag_id}
                                style={{ backgroundColor: `${item.tag_color}` }}
                            >
                                <p>{item.tag_name}</p>
                            </li>
                        );
                    })}
                </ul>
            </div>

            <div className={style["separator"]} />

            <BottomPostInteraction
                {...{
                    username: author.username,
                    post_id,
                    is_liked,
                    total_likes,
                    total_comments,
                }}
            />
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
function useNavigate() {
    throw new Error("Function not implemented.");
}
