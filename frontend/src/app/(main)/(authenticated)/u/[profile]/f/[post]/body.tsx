"use client";

import { getFocusedPost, PostProps } from "@/api/post/getFeed";
import { getCacheUserId } from "@/handler/token_handler";
import { useEffect, useRef, useState } from "react";
import { useUser } from "@/hooks/useUser";
import style from "./content.module.scss";
import { IoArrowBackCircleOutline, IoBookmarkOutline } from "react-icons/io5";
import { MediaGroup } from "@/app/_components/ui/chromatic/post/mediagroup";
import { TogglePostLike } from "@/api/post/like";
import { GoComment } from "react-icons/go";
import { LuThumbsUp } from "react-icons/lu";
import BottomPostInteraction from "@/app/_components/ui/chromatic/post/interaction";
import CommentSection from "./comment";
import PostHeader from "@/app/_components/ui/chromatic/post/header";
import { FaArrowLeft } from "react-icons/fa";
import { Post } from "@/app/_components/ui/chromatic/post";

function PostContentSkeleton() {
    return <div className={style["skeleton-container"]}>loading</div>;
}

function PostContentPage({ params }: { params: { post: string } }) {
    const [post, setPost] = useState<PostProps | null>(null);
    const [likeState, setLikeState] = useState(post?.is_liked || false);
    const [likeCount, setLikeCount] = useState(post?.total_likes || 0);
    const currentUserId = useUser().data?.id;

    useEffect(() => {
        const postFetch = async () => {
            const postOf = params.post;
            const response = await getFocusedPost(postOf);

            if (currentUserId === response.author.id) {
                setPost(response);
                return;
            }
            setPost(response);
        };
        postFetch();
    }, [params.post, currentUserId]);

    if (!post) {
        return <PostContentSkeleton />;
    }

    const handleBackButtonClick = () => {
        window.history.back();
    };

    console.log(post);
    return (
        <>
            <section className={style["header"]}>
                <button
                    type="button"
                    style={{ cursor: "pointer" }}
                    className={style["leave-btn"]}
                    onClick={handleBackButtonClick}
                >
                    <FaArrowLeft/>
                </button>
                <h2>Post</h2>
            </section>
            {post ? (
                <>
                    <Post {...post} />
                    { currentUserId ?
                        <CommentSection postId={post.post_id} />
                    :
                        <div className={style["login-warning"]}>
                            <p>You must be logged in to see the comments.</p>
                        </div>
                    }
                </>
            ) : (
                <PostContentSkeleton />
            )}
        </>
    );
}

export default PostContentPage;
