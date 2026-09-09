import { getCommentsOnPost, getFocusedPost } from "@/api/post/getFeed";
import PostContentPage from "./body";
import type { Metadata } from "next";
import style from "./content.module.scss"
import PostRightLayout from "../../../../../feed/rightside";
export async function generateMetadata({
    params,
}: {
    params: Promise<{ post: string }>;
}): Promise<Metadata> {
    const resolvedParams = await params;
    const postOf = resolvedParams.post;

    try {
        const response = await getFocusedPost(postOf);

        if (!response) {
            return {
                title: "Post not found",
            };
        }

        const postImages = [];

        if (response.has_attachment) {
            const mediaObject = response.attachments[0]?.media_objects[0]; 
            postImages.push({
                url: `${process.env.NEXT_PUBLIC_CDN_URL}/${mediaObject?.storage_key}/${mediaObject?.name}`,
                width: 400,
                height: 400,
            });
        }

        return {
            title: `${response.author.display_name} (${response.author.username})'s post`,
            description: response.content,
            openGraph: {
                title: `${response.author.display_name} (${response.author.username})'s post`,
                description: response.content,
                url: `${process.env.NEXT_PUBLIC_URL}/f/${postOf}`,
                images: postImages,
                locale: "en-US",
                type: "website",
            },
        };
    } catch (error) {
        return {
            title: "Post not found",
        };
    }
}

export default async function PostPage({
    params,
}: {
    params: Promise<{ post: string }>;
}) {
    return (
        <div className={style["content-layout"]}>
            <div className={style["left-layout"]}>
            <PostContentPage params={await params} />
            </div>
            <PostRightLayout/>
        </div>
    );
}