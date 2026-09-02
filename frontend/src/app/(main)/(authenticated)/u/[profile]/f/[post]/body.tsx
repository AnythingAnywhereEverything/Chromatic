"use client";

import {
    commentProps,
    getFocusedPost,
    PostProps,
} from "@/api/post/getFeed";
import { getCacheUserId } from "@/handler/token_handler";
import { useEffect, useRef, useState } from "react";
import { useUser } from "@/hooks/useUser";
import style from "./content.module.scss";
import { IoArrowBackCircleOutline, IoBookmarkOutline } from "react-icons/io5";
import PostHeader from "./header";
import { MediaGroup } from "@/app/_components/ui/chromatic/post/mediagroup";
import { TogglePostLike } from "@/api/post/like";
import { GoComment } from "react-icons/go";
import { LuThumbsUp } from "react-icons/lu";
import { DialogSharePost } from "@/app/_components/ui/chromatic/post";
import BottomPostInteraction from "@/app/_components/ui/chromatic/post/interaction";
import CommentSection from "./comment";

function PostContentSkeleton() {
    return <div className={style["skeleton-container"]}>loading</div>;
}

function PostContentPage({ params }: { params: { post: string } }) {
    const [post, setPost] = useState<PostProps | null>(null);
    const [likeState, setLikeState] = useState(post?.is_liked || false);
    const [likeCount, setLikeCount] = useState(post?.total_likes || 0);
    const ref = useRef<HTMLSpanElement | null>(null);

    const [comment, setComment] = useState<commentProps | null>(null);

    const currentUserId = useUser().data?.id;

    useEffect(() => {
        const postFetch = async () => {
            const postOf = params.post;
            const response = await getFocusedPost(postOf);

            if (currentUserId === response.author.id) {
                console.log("Owner of this post");
                setPost(response);
                return;
            }
            setPost(response);
            setLikeState(response?.is_liked);
            setLikeCount(response?.total_likes);
        };
        postFetch();
    }, [params.post]);

    if (!post) {
        return <PostContentSkeleton />;
    }

    console.log(post);
    return (
        <article className={style["layout"]} key={post.post_id}>
            {post ? (
                <section className={style["main-post-container"]}>
                    {/*  */}
                    <section className={style["back-button"]}>
                        <button
                            type="button"
                            style={{ cursor: "pointer" }}
                            className={style["leave-btn"]}
                        >
                            <IoArrowBackCircleOutline size={36} />
                        </button>
                        <h2>Post</h2>
                    </section>
                    <PostHeader {...post} />
                    <section className={style["main"]}>
                        <div className={style["context"]}>
                            <span style={{ fontSize: "var(--text-small)" }}>
                                {post.content}
                            </span>
                        </div>
                        <article className={style["media"]}>
                            {post.has_attachment && (
                                <MediaGroup media={post.attachments} />
                            )}
                            <ul className={style["subject-tag"]}>
                                {post.tag && post.tag.map((item) => {
                                    return (
                                        <li
                                            key={item.tag_id}
                                            style={{
                                                backgroundColor: `${item.tag_color}`,
                                            }}
                                        >
                                            <p>{item.tag_name}</p>
                                        </li>
                                    );
                                })}
                            </ul>
                        </article>
                        <BottomPostInteraction {...post} />

                        <CommentSection postId={params.post} />
                    </section>
                </section>
            ) : (
                <PostContentSkeleton />
            )}
        </article>
    );
}

export default PostContentPage;
