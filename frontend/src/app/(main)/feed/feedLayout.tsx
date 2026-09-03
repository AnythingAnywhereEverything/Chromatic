"use client";

import { Post } from "@/app/_components/ui/chromatic/post";
import { useEffect, useState } from "react";
import { getUserFeed, PostProps } from "@/api/post/getFeed";
import style from "./style.module.scss";
import { CreatePostComponent } from "@/app/_components/ui/chromatic/createPost";
import { useUser } from "@/hooks/useUser";
import type { UserResponse } from "@/api/user";

export default function PostGroup() {
    const [feed, setFeed] = useState<PostProps[]>([]);
    const [user, setUser] = useState<UserResponse | null>(null);

    const currentUser = useUser();

    useEffect(() => {
        if (currentUser && currentUser.data) {
            setUser(currentUser.data);
        }
    }, [currentUser, currentUser.data]);

    useEffect(() => {
        const fetchPost = async () => {
            const post = await getUserFeed();
            if (post) {
                setFeed(post);
            }
        };

        fetchPost();
    }, []);

    return (
        <div className={style["feed-layout"]}>
            {user && <CreatePostComponent author={user} onPostCreated={(res) => { setFeed((current) => [res, ...current]); }} />}
            {feed?.map((post) => (
                <Post key={post.post_id} {...post} />
            ))}
        </div>
    );
}
