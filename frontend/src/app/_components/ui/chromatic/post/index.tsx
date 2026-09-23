"use client";

import { useEffect, useRef, useState } from "react";
import style from "./style.module.scss";
import { PostProps } from "@/api/post/getFeed";

import { MediaGroup } from "./mediagroup";
import { useUser } from "@/hooks/useUser";
import PostHeader from "./header";
import BottomPostInteraction from "./interaction";
import { deletePost } from "@/api/post/post";

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
    is_followed,
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
    const [isDeleted, setIsDeleted] = useState(false);
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

    const markPostAsDeleted = () => {
        setIsDeleted(true);
    };

    const handleDeletePost = async () => {
        try {
            await deletePost(post_id);
            markPostAsDeleted();
        } catch (error) {
            console.error("Failed to delete post:", error);
        }
    };

    return (
        <>
            {!isDeleted && (
            <section
                className={style["container"]}

            >
                <PostHeader
                    author={{
                        id: author.id,
                        username: author.username,
                        display_name: author.display_name,
                        avatar: author.avatar,
                        avatar_thumbhash: author.avatar_thumbhash,
                    }}
                    is_followed={is_followed}
                    created_at={created_at}
                    visibility={visibility}
                    onDelete={handleDeletePost}
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
            )}
        </>
    );
};

export { Post };