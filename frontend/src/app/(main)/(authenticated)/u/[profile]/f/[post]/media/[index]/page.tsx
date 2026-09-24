import { getFocusedPost } from "@/api/post/getFeed";
import { redirect } from "next/navigation";
import type { Metadata } from "next";
import style from "../../content.module.scss";
import {
    FocusedPostProvider,
    FocusedPostView,
} from "../../_components/focusedPost";
import MediaViewer from "../mediaViewer";

function resolveMediaStorageUrl(
    post: NonNullable<Awaited<ReturnType<typeof getFocusedPost>>>,
    index: number,
): string | null {
    const attachments = post.attachments ?? [];

    if (attachments.length === 0 || index < 1 || index > attachments.length) {
        return null;
    }

    const mediaObject = attachments[index - 1].media_objects?.[0];

    if (!mediaObject) {
        return null;
    }

    return `${mediaObject.storage_key}/${mediaObject.name}`;
}

export async function generateMetadata({
    params,
}: {
    params: Promise<{ profile: string; post: string; index: string }>;
}): Promise<Metadata> {
    const resolvedParams = await params;
    const index = Number.parseInt(resolvedParams.index, 10);
    const postOf = resolvedParams.post;

    try {
        const response = await getFocusedPost(postOf);

        if (!response) {
            return { title: "Post not found" };
        }

        const resolved = resolveMediaStorageUrl(response, index);
        const postImages = resolved
            ? [
                  {
                      url: `${process.env.NEXT_PUBLIC_CDN_URL}/${resolved}`,
                      width: 400,
                      height: 400,
                  },
              ]
            : [];

        return {
            title: `${response.author.display_name} (@${response.author.username})'s photo`,
            description: response.content,
            openGraph: {
                title: `${response.author.display_name} (@${response.author.username})'s photo`,
                description: response.content,
                url: `${process.env.NEXT_PUBLIC_URL}/u/${resolvedParams.profile}/f/${postOf}/media/${resolvedParams.index}`,
                images: postImages,
                locale: "en-US",
                type: "website",
            },
        };
    } catch (error) {
        return { title: "Post not found" };
    }
}

export default async function MediaPage({
    params,
}: {
    params: Promise<{ profile: string; post: string; index: string }>;
}) {
    const resolvedParams = await params;
    const index = Number.parseInt(resolvedParams.index, 10);

    if (!Number.isInteger(index) || index < 1) {
        redirect(`/u/${resolvedParams.profile}/f/${resolvedParams.post}`);
    }

    return (
        <div className={style["post-layout"]}>
            <div className={style["container-wrapper"]}>
                <FocusedPostProvider postId={resolvedParams.post}>
                    <FocusedPostView />
                    <MediaViewer
                        profile={resolvedParams.profile}
                        index={resolvedParams.index}
                    />
                </FocusedPostProvider>
            </div>
        </div>
    );
}