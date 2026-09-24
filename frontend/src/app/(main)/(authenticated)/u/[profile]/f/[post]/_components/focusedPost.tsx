"use client";

import { getFocusedPost, PostProps } from "@/api/post/getFeed";
import { createContext, useCallback, useContext, useEffect, useState } from "react";
import { useUser } from "@/hooks/useUser";
import style from "../content.module.scss";
import { FaArrowLeft } from "react-icons/fa";
import { Post } from "@/app/_components/ui/chromatic/post";
import CommentSection from "../comment";

interface FocusedPostContextValue {
    post: PostProps | null;
    reload: () => void;
}

const FocusedPostContext = createContext<FocusedPostContextValue | null>(null);

export function useFocusedPost(): FocusedPostContextValue {
    const context = useContext(FocusedPostContext);

    if (!context) {
        throw new Error(
            "useFocusedPost must be used within <FocusedPostProvider />",
        );
    }

    return context;
}

export function FocusedPostProvider({
    postId,
    children,
}: {
    postId: string;
    children: React.ReactNode;
}) {
    const [post, setPost] = useState<PostProps | null>(null);
    const currentUserId = useUser().data?.id;

    const reload = useCallback(() => {
        const postFetch = async () => {
            const response = await getFocusedPost(postId);

            if (currentUserId === response.author.id) {
                setPost(response);
                return;
            }
            setPost(response);
        };
        postFetch();
    }, [postId, currentUserId]);

    useEffect(() => {
        reload();
    }, [reload]);

    return (
        <FocusedPostContext.Provider value={{ post, reload }}>
            {children}
        </FocusedPostContext.Provider>
    );
}

export function FocusedPostView() {
    const { post } = useFocusedPost();
    const currentUserId = useUser().data?.id;

    if (!post) {
        return <div className={style["skeleton-container"]}>loading</div>;
    }

    const postId = post.post_id;

    const handleBackButtonClick = () => {
        window.history.back();
    };

    return (
        <>
            <section className={style["header"]}>
                <button
                    type="button"
                    style={{ cursor: "pointer" }}
                    className={style["leave-btn"]}
                    onClick={handleBackButtonClick}
                >
                    <FaArrowLeft />
                </button>
                <h2>Post</h2>
            </section>
            <Post {...post} />
            {currentUserId ? (
                <CommentSection postId={postId} />
            ) : (
                <div className={style["login-warning"]}>
                    <p>You must be logged in to see the comments.</p>
                </div>
            )}
        </>
    );
}