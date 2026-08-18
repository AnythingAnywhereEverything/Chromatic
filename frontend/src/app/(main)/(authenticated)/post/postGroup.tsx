"use client";

import { Post } from "@/app/_components/ui/chromatic/post";
import { useEffect, useState } from "react";
import { getUserFeed } from "@/api/getFeed";
import type { mediaPostProps, MediaReponse } from "@/api/getFeed";

export default function PostGroup() {
    const [feed, setFeed] = useState<mediaPostProps[]>([]);

    useEffect(() => {
        async function fetchFeed() {
            const res = await getUserFeed();

            setFeed(res);
        }

        fetchFeed();
    }, []);

    return (
        <div>
            {feed?.map((post) => (
                <Post
                    key={post.id}
                    ownerId={post.id}
                    id={post.id}
                    ownerName={post.id.toString()}
                    content={post.content}
                    like={post.total_likes}
                    comment={[]}
                    bookmark={false}
                />
            ))}
        </div>
    );
}