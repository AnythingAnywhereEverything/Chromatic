import { useRef, useState, useEffect } from "react";
import { deleteComment, likeComment } from "@/api/post/comments";
import PostHeader from "../post/header";
import { MediaGroup } from "../post/mediagroup";
import style from "./user-comment.module.scss";
import { InteractButton } from "../post/interaction";
import { FaHeart, FaRegHeart } from "react-icons/fa6";
import { commentProps } from "@/api/post/comments";
import { PostAvatar } from "../post/header/avatar";
import {
    Tooltip,
    TooltipArrow,
    TooltipContent,
    TooltipTrigger,
} from "../tooltip";
import Link from "next/dist/client/link";
import {
    formatFullDateWithExactTime,
    formatSocialMediaDate,
} from "../post/helpers/dateFormater";
import {
    Dropdown,
    DropdownContent,
    DropdownItem,
    DropdownTrigger,
} from "../dropdown";
import { BsThreeDots } from "react-icons/bs";
import { useUser } from "@/hooks/useUser";

function UserComment(props: commentProps) {
    const {
        id,
        post_id,
        author,
        content,
        is_liked,
        total_likes,
        created_at,
        updated_at,
    } = props;

    if (id === undefined) return null;

    const ref = useRef<HTMLSpanElement | null>(null);
    const [open, setOpen] = useState(false);
    const [isLiked, setIsLiked] = useState(is_liked);
    const [likeCount, setLikeCount] = useState(total_likes);
    const [showReadMoreButton, setShowReadMoreButton] = useState(false);

    const currentUserId = useUser().data?.id;

    useEffect(() => {
        if (ref.current) {
            setShowReadMoreButton(
                ref.current.scrollHeight > ref.current.clientHeight,
            );
        }
    }, [content]);

    const handleLike = async () => {
        try {
            const nextLikeState = !isLiked;
            await likeComment(post_id, id, nextLikeState);
            setIsLiked(nextLikeState);
            setLikeCount((prev) => prev + (nextLikeState ? 1 : -1));
        } catch (error) {
            setLikeCount((prev) => prev + (isLiked ? -1 : 1));
            setIsLiked(!isLiked);
        }
    };

    const [isDeleted, setIsDeleted] = useState(false);

    async function handleDeleteComment() {
        try {
            await deleteComment(post_id, id);
            setIsDeleted(true);
        } catch (error) {
            console.error("Failed to delete comment:", error);
        }
    }

    return (
        !isDeleted && (
            <div key={id} className={style["container"]}>
                <header className={style["header"]}>
                    <div className={style["author-container"]}>
                        <PostAvatar
                            userId={author.id}
                            username={author.username}
                            displayName={author.display_name}
                            avatar={author.avatar}
                            thumbhash={author.avatar_thumbhash || ""}
                            className={style["avatar"]}
                            width={38}
                            height={38}
                        />
                        <section>
                            <div className={style["comment-info"]}>
                                <Link
                                    href={`/u/${author.username}`}
                                    className={style["username"]}
                                >
                                    <p>
                                        {author.display_name || author.username}
                                    </p>
                                    <span>@{author.username}</span>
                                </Link>
                                <span>•</span>
                                <p className={style["comment-meta"]}>
                                    <Tooltip openDelayDuration={300}>
                                        <TooltipTrigger>
                                            <span>
                                                {formatSocialMediaDate(
                                                    created_at,
                                                )}
                                            </span>
                                        </TooltipTrigger>
                                        <TooltipContent>
                                            <p>
                                                {formatFullDateWithExactTime(
                                                    created_at,
                                                )}
                                            </p>
                                        </TooltipContent>
                                    </Tooltip>
                                </p>
                            </div>
                            <div className={style["text-container"]}>
                                <span
                                    style={{ whiteSpace: "pre-wrap" }}
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
                        </section>
                    </div>
                    <div className={style["option"]}>
                        <Dropdown>
                            <DropdownTrigger asChild>
                                <BsThreeDots />
                            </DropdownTrigger>
                            {author.id === currentUserId ? (
                                <DropdownContent>
                                    <DropdownItem>Edit Comment</DropdownItem>
                                    <DropdownItem>
                                        <button
                                            type="button"
                                            onClick={handleDeleteComment}
                                        >
                                            Delete Comment
                                        </button>
                                    </DropdownItem>
                                </DropdownContent>
                            ) : (
                                <DropdownContent>
                                    <DropdownItem>
                                        Follow @{author.username}
                                    </DropdownItem>
                                    <DropdownItem>
                                        Add Friend @{author.username}
                                    </DropdownItem>
                                    <DropdownItem>
                                        Block @{author.username}
                                    </DropdownItem>
                                    <DropdownItem>Report</DropdownItem>
                                </DropdownContent>
                            )}
                        </Dropdown>
                    </div>
                </header>
                <section className={style["interaction"]}>
                    <InteractButton
                        name="Like"
                        className={`${style["button"]} ${style["like-button"]}`}
                        onClick={async () => {
                            await handleLike();
                            setIsLiked(!isLiked);
                            setLikeCount(
                                isLiked ? likeCount - 1 : likeCount + 1,
                            );
                        }}
                    >
                        <i>{isLiked ? <FaHeart /> : <FaRegHeart />}</i>
                        {likeCount > 0 ? likeCount : null}
                    </InteractButton>
                </section>
            </div>
        )
    );
}

export default UserComment;
