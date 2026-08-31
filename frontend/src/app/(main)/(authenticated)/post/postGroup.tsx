"use client";

import { Post } from "@/app/_components/ui/chromatic/post";
import { useEffect, useState } from "react";
import { getUserFeed, PostProps } from "@/api/post/getFeed";
import style from "./style.module.scss";
import PostPopup from "./postPopup";

export default function PostGroup() {
    const [feed, setFeed] = useState<PostProps[]>([]);
    const [isLoading, setIsLoading] = useState(true);
    const [popupPostId, setPopupPostId] = useState<string | null>(null);

    // todo: onClick the post Push? to
    //   const changePathname = (id: string) => {
    //         const currentUrl =
    //             window.location.pathname +
    //             window.location.search +
    //             window.location.hash;

    //         const newUrl = `/post/f/${id}`;

    //         window.history.replaceState(
    //             {
    //                 ...window.history.state,
    //                 previousUrl: currentUrl,
    //             },
    //             "",
    //             newUrl
    //         );
    //             setPopupPostId(id);
    //     };

    useEffect(() => {
        async function fetchFeed() {
            const res = await getUserFeed();

            setFeed(res);
            setIsLoading(false);
        }

        fetchFeed();
    }, []);

    return (
        <div className={style["feed-layout"]}>
            {feed?.map((post) => (
                <div key={post.post_id}>
                    <Post key={post.post_id} {...post} />
                </div>
            ))}
        </div>
    );
}
