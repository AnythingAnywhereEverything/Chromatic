"use client";

import { Post } from "@/app/_components/ui/chromatic/post";
import { useEffect, useState } from "react";
import { getUserFeed } from "@/api/post/getFeed";
import type { mediaPostProps, MediaReponse } from "@/api/post/getFeed";
import style from "./style.module.scss"
export default function PostGroup() {
    const [feed, setFeed] = useState<mediaPostProps[]>([]);
    const [isLoading, setIsLoading] = useState(true);

    useEffect(() => {
        async function fetchFeed() {
            const res = await getUserFeed();
            console.log(res)
            setFeed(res);
        }

        fetchFeed();
    }, []);

    return (
        <div className={style["feedLayout"]}>
            {feed?.map((post) => (
                <Post
                    key={post.id}
                    id={post.id}
                    user_id={post.user_id}
                    username={post.username}
                    content={post.content}
                    total_comment={post.total_comment}
                    total_likes={post.total_likes}
                    visibility={post.visibility}
                    repost_from={post.repost_from}
                    is_repost={post.is_repost}
                    has_attachment={post.has_attachment}
                    created_at={post.created_at}
                    updated_at={post.updated_at}
                    tag={post.tag}
                    media={post.media}
                    bookmark={post.bookmark}
                    is_liked={post.is_liked}
                />
            ))}
        </div>
    );
}