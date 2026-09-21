"use client";

import style from "./comment.module.scss";
import React, { useEffect, useRef, useState } from "react";
import UserComment from "@/app/_components/ui/chromatic/userComment/userComment";
import CreateComment from "@/app/_components/ui/chromatic/userComment/createComment";
import { commentProps, getComments } from "@/api/post/comments";
import { UserResponse } from "@/api/user";
import { useUser } from "@/hooks/useUser";
type CommentSectionProps = {
    postId: string;
};

export const CommentSection = ({ postId }: CommentSectionProps) => {
    const [comments, setComments] = useState<commentProps[]>([]);
    const [user, setUser] = useState<UserResponse | null>(null);
    const currentUser = useUser();
    const observerRef = useRef<HTMLDivElement | null>(null);

    React.useEffect(() => {
        if (currentUser?.data) {
            setUser(currentUser.data);
        }
    }, [currentUser]);

    const loadingRef = React.useRef<HTMLDivElement | null>(null);
    const [beforeDate, setBeforeDate] = useState(new Date());
    const [hasMore, setHasMore] = useState(true);
    const [loading, setLoading] = useState(false);

    const loadMoreComments = async () => {
        if (loading || !hasMore) return;
        setLoading(true);

        try {
            const result = await getComments(postId, beforeDate.toISOString(), 10);
            if (!result || result.length === 0) {
                setHasMore(false);
                return;
            }
            setComments((current) => {
                const existingIds = new Set(
                    current.map((comment) => comment.id),
                );
                return [
                    ...current,
                    ...result.filter((comment) => !existingIds.has(comment.id)),
                ];
            });
            
            
            const oldestComment = result[result.length - 1];
            setBeforeDate(new Date(oldestComment.created_at));
            
            if (result.length < 10) {
                setHasMore(false);
            }
        } finally {
            setLoading(false);
        }
    };

    React.useEffect(() => {
        loadMoreComments();
    }, [postId]);

    // observer
    React.useEffect(() => {
        const observer = new IntersectionObserver(
            ([entry]) => {
                if (entry.isIntersecting) {
                    loadMoreComments();
                }
            },
            { rootMargin: "100px" },
        );
        const target = loadingRef.current;

        if (target) {
            observer.observe(target);
        }

        return () => {
            if (target) {
                observer.unobserve(target);
            }
        };
    }, [loadingRef, loadMoreComments, beforeDate, postId]);

    return (
        <div className={style["comment-layout"]}>
            {user && <CreateComment postId={postId} author={user} onCommentCreated={loadMoreComments} />}
            <div className={style["spacer"]} />
            <div className={style["comments-container"]}>
                {comments.map((comment) => (
                    <UserComment key={comment.id} {...comment} />
                ))}
            </div>
            {hasMore && <div ref={loadingRef}>{loading && "Loading..."}</div>}
        </div>
    );
};

export default CommentSection;
