"use client";

import { Post } from "@/app/_components/ui/chromatic/post";
import { useEffect, useRef, useState } from "react";
import { getUserFeed, PostProps } from "@/api/post/getFeed";
import style from "./style.module.scss";
import { CreatePostComponent } from "@/app/_components/ui/chromatic/createPost";
import { useUser } from "@/hooks/useUser";
import type { UserResponse } from "@/api/user";

const LIMIT = 8;

export default function PostGroup() {
    const [feed, setFeed] = useState<PostProps[]>([]);
    const [user, setUser] = useState<UserResponse | null>(null);

    const [beforeDate, setBeforeDate] = useState(new Date());
    const [beforeId, setBeforeId] = useState<string | undefined>(undefined);
    const [loading, setLoading] = useState(false);
    const [hasMore, setHasMore] = useState(true);

    const observerRef = useRef<HTMLDivElement | null>(null);

    const currentUser = useUser();

    useEffect(() => {
        if (currentUser && currentUser.data) {
            setUser(currentUser.data);
        }
    }, [currentUser, currentUser.data]);

    const loadMore = async () => {
        if (loading || !hasMore) return;

        setLoading(true);

        try {
            const post = await getUserFeed(beforeDate, beforeId, LIMIT);

            if (!post || post.length === 0) {
                setHasMore(false);
                return;
            }

            setFeed((current) => {
                const existingIds = new Set(
                    current.map((item) => item.post_id),
                );

                return [
                    ...current,
                    ...post.filter((item) => !existingIds.has(item.post_id)),
                ];
            });

            const oldestPost = post[post.length - 1];
            setBeforeDate(new Date(`${oldestPost.created_at}`));
            setBeforeId(oldestPost.post_id);

            if (post.length < LIMIT) {
                setHasMore(false);
            }
        } finally {
            setLoading(false);
        }
    };

    // * Initial fetch.
    useEffect(() => {
        loadMore();
    }, []);

    // * Watch the bottom sentinel.
    useEffect(() => {
        const observer = new IntersectionObserver(
            ([entry]) => {
                if (entry.isIntersecting) {
                    loadMore();
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
    }, [loading, hasMore, beforeDate, beforeId]);

    return (
        <div className={style["feed-layout"]}>
            {user && (
                <CreatePostComponent
                    author={user}
                    onPostCreated={(res) => {
                        setFeed((current) => [res, ...current]);
                    }}
                />
            )}
            {feed?.map((post) => (
                <Post key={post.post_id} {...post} />
            ))}

            {hasMore && (
                <div ref={observerRef} style={{ minHeight: "1px" }}>
                    {loading && "Loading..."}
                </div>
            )}
        </div>
    );
}