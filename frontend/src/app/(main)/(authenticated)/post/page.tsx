"use client"
import { getUserFeed } from "@/api/getFeed";
import { Post } from "@/app/_components/ui/chromatic/post";
import { useEffect, useState } from "react";
// export const metadata = {
//     title: "Post Page", // Let next js handle title and description for SEO purposes
//     description: "This is the post page",
// };

export default async function PostPage({
    children,
}: {
    children: React.ReactNode;
}) {    
    const [isMounted, setIsMounted] = useState(false);

    useEffect(() => {
        setIsMounted(true);
    }, []);

    if (!isMounted) {
        return null; 
    }
    const res = await getUserFeed();
    // Strictly import and use the components hete
    // due to it being a SSR page, and not a client component. This is to avoid hydration errors.
    return <div>
        {children}
        </div>;
}