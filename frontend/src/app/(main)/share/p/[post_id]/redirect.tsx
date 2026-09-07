"use client";

import { useEffect } from "react";
import { useRouter } from "next/navigation";

const RedirectTo = ({ response, i64 }: { response: any; i64: string }) => {
    const router = useRouter();

    useEffect(() => {
        if (!response?.author?.username || !i64) return;

        // * Navigation must happen after render
        router.replace(`/u/${response.author.username}/f/${i64}`);
    }, [router, response?.author?.username, i64]);

    return null;
};

export default RedirectTo;