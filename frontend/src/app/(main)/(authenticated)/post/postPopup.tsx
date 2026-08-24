"use client";

import Link from "next/link";
import { useEffect, useState } from "react";
import style from "./style.module.scss"
type PostPopupProps = {
    postId: string;
};

function FocusPostSkeleton() {
    return (
        <div>
            Loading
        </div>
    );
}

export default function PostPopup({ postId }: PostPopupProps) {
    const [isLoading, setIsLoading] = useState(false);

    useEffect(() => {
        const handlePopState = () => {
            if (!window.location.pathname.startsWith("/post/f/")) {
            }
        };

        window.addEventListener("popstate", handlePopState);

        return () => {
            window.removeEventListener("popstate", handlePopState);
        };
    }, []);

    return (
        <div className={style["post-popup-container"]}>
            {isLoading ? (
                <FocusPostSkeleton />
            ) : (
                <article>
                    <Link href={'/post'}>
                        Close
                    </Link>

                    Post ID: {postId}
                </article>
            )}
        </div>
    );
}