import { useRef, useState, useEffect } from "react";
import { deleteComment, likeComment } from "@/api/post/comments";
import PostHeader from "../post/header";
import { MediaGroup } from "../post/mediagroup";
import style from "./user-comment.module.scss";
import { InteractButton } from "../post/interaction";
import { FaHeart, FaRegHeart } from "react-icons/fa6";
import { commentProps } from "@/api/post/comments";

function UserComment({
    id,
    post_id,
    author,
    content,
    is_like,
    total_likes,
    has_attachment,
    attachments,
    created_at,
    is_followed,
    updated_at,
}: commentProps) {
    if (id === undefined) return null;

    const ref = useRef<HTMLSpanElement | null>(null);
    const [open, setOpen] = useState(false);
    const [isLiked, setIsLiked] = useState(is_like);
    const [likeCount, setLikeCount] = useState(total_likes);
    const [showReadMoreButton, setShowReadMoreButton] = useState(false);
    
    useEffect(() => {
        if (ref.current) {
            setShowReadMoreButton(
                ref.current.scrollHeight > ref.current.clientHeight
            );
        }
    }, [content]);
        
    const handleLike = async () => {
        try {
            const nextLikeState = !isLiked;
            await likeComment(post_id, id, nextLikeState);
            setIsLiked(nextLikeState);
            setLikeCount(prev => prev + (nextLikeState ? 1 : -1));
        } catch (error) {
            setLikeCount(prev => prev + (isLiked ? -1 : 1));
            setIsLiked(!isLiked);
        }
    }
    
    const [isDeleted, setIsDeleted] = useState(false);
    
    async function handleDeleteComment() {
        try{
            await deleteComment(post_id, id);
            setIsDeleted(true);
        }catch(error){
            console.error("Failed to delete comment:", error);
        }
    }

    return (
        !isDeleted && (
            <div key={id} className={style["container"]}>
                <PostHeader
                    author={{
                        id: author.id,
                        username: author.username,
                        display_name: author.display_name,
                        avatar: author.avatar,
                        avatar_thumbhash: author.avatar_thumbhash,
                    }}
                    is_followed={is_followed}
                    visibility={"public"}
                    created_at={created_at}
                    onDelete={handleDeleteComment}
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
                    <article className={style["media"]}>
                        {has_attachment && (
                            <MediaGroup
                                containerWidthRatio={2}
                                containerHeightRatio={1}
                                media={attachments}
                            />
                        )}
                    </article>
                </div>
                {/* like interaction */}
                <div className={style["separator"]} />
                <section className={style["interaction"]}>
                    <InteractButton
                        name="Like"
                        className={`${style["button"]} ${style["like-button"]}`}
                        onClick={async () => {
                            await handleLike();
                            setIsLiked(!isLiked);
                            setLikeCount(isLiked ? likeCount - 1 : likeCount + 1);
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
