import { getFocusedPost } from "@/api/post/getFeed";
import { b64ToBn, urlBase64ToBase64 } from "@lib/base64";

import { cache } from "react";
import { Metadata } from "next";

import RedirectTo from "./redirect";

const getPost = cache(async (postId: string) => {
    return getFocusedPost(postId);
});

const getPostId = (post_id: string) => {
    return b64ToBn(urlBase64ToBase64(post_id)).toString();
};

export async function generateMetadata({
    params,
}: {
    params: Promise<{ post_id: string }>;
}): Promise<Metadata> {
    const { post_id } = await params;
    const postOf = getPostId(post_id);

    const response = await getPost(postOf);

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
}

export default async function SharedPostReroutePage({
    params,
}: {
    params: Promise<{ post_id: string }>;
}) {
    const { post_id } = await params;
    const postOf = getPostId(post_id);

    const response = await getPost(postOf);

    return <RedirectTo response={response} i64={postOf} />;
}
