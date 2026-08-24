import { getCommentsOnPost, getFocusedPost } from "@/api/post/getFeed";
import PostContentPage from "./body";
import type { Metadata } from "next";
import style from "./content.module.scss"
import PostRightLayout from "../../rightside";
export async function generateMetadata({
    params,
}: {
    params: Promise<{ post: string }>;
}): Promise<Metadata> {
    const resolvedParams = await params;
    const postOf = resolvedParams.post;

    const response = await getFocusedPost(postOf);

    if (!response) {
        return {
            title: "Post not found",
        };
    }

    const postImages = [];

    if (response.has_attachment) {
        postImages.push({
            url: `${process.env.NEXT_PUBLIC_CDN_URL}/${response.media[0]?.path}`,
            width: 400,
            height: 400,
        });
    }

    return {
        title: `${response.display_name} (${response.username})'s post`,
        description: response.content,
        openGraph: {
            title: `${response.display_name} (${response.username})'s post`,
            description: response.content,
            url: `${process.env.NEXT_PUBLIC_URL}/f/${postOf}`,
            images: postImages,
            locale: "en-US",
            type: "website",
        },
    };
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