"use client";

import { PostProps } from "@/api/post/getFeed";
import { GetUserPosts } from "@/api/post/post";
import { UserResponse } from "@/api/user";
import { PublicUserProfileResponse } from "@/api/user/profile";
import { CreatePostComponent } from "@/app/_components/ui/chromatic/createPost";
import { Post } from "@/app/_components/ui/chromatic/post";
import { useUser } from "@/hooks/useUser";
import React, { useEffect, useRef, useState } from "react";
import style from "./postgroup.module.scss";

export default function PostGroup({
    profile,
    isOwner,
}: {
    profile: PublicUserProfileResponse;
    isOwner: boolean;
}) {
    const [posts, setPosts] = useState<PostProps[]>([]);
    const [user, setUser] = useState<UserResponse | null>(null);

    const [beforeDate, setBeforeDate] = useState(new Date());
    const [loading, setLoading] = useState(false);
    const [hasMore, setHasMore] = useState(true);

    const currentUser = useUser();

    const observerRef = useRef<HTMLDivElement | null>(null);

    useEffect(() => {
        if (currentUser?.data) {
            setUser(currentUser.data);
        }
    }, [currentUser]);

    const loadMore = async () => {
        if (loading || !hasMore) return;

        setLoading(true);

        try {
            console.log("Loading more posts before date:", beforeDate);
            const post = await GetUserPosts(profile.id, beforeDate, 10);
            console.log("Fetched posts:", post);

            if (!post || post.length === 0) {
                setHasMore(false);
                return;
            }

            setPosts((current) => {
                const existingIds = new Set(
                    current.map((item) => item.post_id),
                );

                return [
                    ...current,
                    ...post.filter((item) => !existingIds.has(item.post_id)),
                ];
            });

            // * Use the oldest post as the cursor for the next request.
            const oldestPost = post[post.length - 1];

            setBeforeDate(new Date(`${oldestPost.created_at}Z`));

            console.log("Oldest post date:", oldestPost.created_at);

            // * Fewer than 10 means the server has no more posts.
            if (post.length < 10) {
                setHasMore(false);
            }
        } finally {
            setLoading(false);
        }
    };

    // * Initial fetch.
    useEffect(() => {
        loadMore();
    }, [profile.id]);

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
    }, [loading, hasMore, beforeDate, profile.id]);

    return (
        <div className={style["postgroup-layout"]}>
            {user && isOwner && (
                <CreatePostComponent
                    author={user}
                    onPostCreated={(res) => {
                        setPosts((current) => [res, ...current]);
                    }}
                />
            )}

            {posts.map((post) => (
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
