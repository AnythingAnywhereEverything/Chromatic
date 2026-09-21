"use client";

import style from "./comment.module.scss";
import React, { useEffect, useRef, useState } from "react";
import UserComment from "@/app/_components/ui/chromatic/userComment/userComment";
import CreateComment from "@/app/_components/ui/chromatic/userComment/createComment";
import { commentProps, getCommentsOnPost } from "@/api/post/comments";
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
    const [beforeDate, setBeforeDate] = useState(new Date());
    const [hasMore, setHasMore] = useState(true);
    const [loading, setLoading] = useState(false);

    React.useEffect(() => {
        if (currentUser?.data) {
            setUser(currentUser.data);
        }
    }, [currentUser]);

    // panigation comment section
    const loadmore = async () => {
        if (loading || !hasMore) return;
        setLoading(true);
        try {
            console.log("Load more comments");
            const comments = await getCommentsOnPost(
                postId,
                beforeDate,
                10,
            );
            console.log(comments);

            if (!comments || comments.length === 0) {
                setHasMore(false);
                return;
            }

            setComments((current) => {
                const existingIds = new Set(current.map((item) => item.id));
                return [
                    ...current,
                    ...comments.filter((item) => !existingIds.has(item.id)),
                ];
            });

            const oldestComment = comments[comments.length - 1];
            setBeforeDate(new Date(`${oldestComment.created_at}`));
            console.log("Updated beforeDate:", beforeDate);

            if (comments.length < 10) {
                setHasMore(false);
            }
        } finally {
            setLoading(false);
        }
    };

    // init fetch
    useEffect(() => {
        loadmore();
    }, [postId]);

    useEffect(() => {
        const observer = new IntersectionObserver(
            ([entry]) => {
                if (entry.isIntersecting) {
                    loadmore();
                }
            },
            {
                rootMargin: "300px",
            },
        );

        const target = observerRef.current;

        if (target) {
            observer.observe(target);
        }

        return () => {
            if (target) {
                observer.unobserve(target);
            }
        };
    }, [loading, loadmore, beforeDate, postId]);

    return (
        <div className={style["comment-layout"]}>
            {user && <CreateComment postId={postId} author={user} />}

            <div className={style["comments-container"]}>
                {comments.map((comment) => (
                    <UserComment key={comment.id} {...comment} />
                ))}
            </div>
        </div>
    );
};

export default CommentSection;
