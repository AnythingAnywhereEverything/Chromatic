"use client";

import { useCallback, useEffect, useMemo, useState } from "react";
import { createPortal } from "react-dom";
import { useRouter } from "next/navigation";
import { IoIosArrowBack, IoIosArrowForward } from "react-icons/io";
import { IoClose } from "react-icons/io5";
import { Image } from "@/app/_components/ui/chromatic/Image";
import { HlsPlayer } from "@/app/_components/ui/chromatic/hlsPlayer";
import { Media, MediaObjects } from "@/api/types/media";
import { useFocusedPost } from "../_components/focusedPost";
import style from "./mediaViewer.module.scss";

const MediaFlags = {
    IsAnimated: 1 << 0,
};

function isAnimated(media: Media): boolean {
    return (media.flags & MediaFlags.IsAnimated) !== 0;
}

// * Prefer the original (full resolution) object for the focused view.
function makeViewerURL(objects: MediaObjects[]): string {
    if (objects === undefined || objects.length === 0) {
        return "";
    }
    for (let i = 0; i < objects.length; i++) {
        if (objects[i].kind === "Original") {
            return objects[i].storage_key + "/" + objects[i].name;
        }
    }
    for (let i = 0; i < objects.length; i++) {
        if (objects[i].kind === "Thumbnail") {
            return objects[i].storage_key + "/" + objects[i].name;
        }
    }
    return "";
}

interface FittedBox {
    width: number;
    height: number;
}

// * Fit a media item inside the available box while preserving its aspect ratio.
function fitMedia(
    containerWidth: number,
    containerHeight: number,
    width: number,
    height: number,
    padding: number,
): FittedBox {
    const boxWidth = Math.max(containerWidth - padding, 1);
    const boxHeight = Math.max(containerHeight - padding, 1);

    if (!width || !height) {
        return { width: boxWidth, height: boxHeight };
    }

    const scale = Math.min(boxWidth / width, boxHeight / height);

    return {
        width: Math.max(Math.round(width * scale), 1),
        height: Math.max(Math.round(height * scale), 1),
    };
}

function useViewportBox() {
    const [box, setBox] = useState({ width: 0, height: 0 });

    useEffect(() => {
        const update = () => {
            setBox({ width: window.innerWidth, height: window.innerHeight });
        };

        update();

        window.addEventListener("resize", update);

        return () => {
            window.removeEventListener("resize", update);
        };
    }, []);

    return box;
}

export default function MediaViewer({
    profile,
    index,
}: {
    profile: string;
    index: string;
}) {
    const router = useRouter();
    const { post } = useFocusedPost();
    const { width: viewportWidth, height: viewportHeight } = useViewportBox();

    const [mounted, setMounted] = useState(false);

    useEffect(() => {
        setMounted(true);
    }, []);

    useEffect(() => {
        const previousOverflow = document.body.style.overflow;

        document.body.style.overflow = "hidden";

        return () => {
            document.body.style.overflow = previousOverflow;
        };
    }, []);

    const indexParam = Number.parseInt(index, 10);

    const attachments = useMemo(() => post?.attachments ?? [], [post]);

    const currentIndex = indexParam - 1;
    const isValid = attachments.length > 0 && currentIndex >= 0;
    const inRange = isValid && currentIndex < attachments.length;

    const baseUrl = post ? `/u/${profile}/f/${post.post_id}` : "";

    const navigateTo = useCallback(
        (nextIndex: number) => {
            router.replace(`${baseUrl}/media/${nextIndex + 1}`);
        },
        [baseUrl, router],
    );

    const closeViewer = useCallback(() => {
        router.replace(baseUrl);
    }, [baseUrl, router]);

    useEffect(() => {
        if (baseUrl && !inRange) {
            router.replace(baseUrl);
        }
    }, [baseUrl, inRange, router]);

    const hasPrev = inRange && currentIndex > 0;
    const hasNext = inRange && currentIndex < attachments.length - 1;

    useEffect(() => {
        const handleKeyDown = (event: KeyboardEvent) => {
            if (event.key === "Escape") {
                closeViewer();
            }
            if (event.key === "ArrowRight" && hasNext) {
                navigateTo(currentIndex + 1);
            }
            if (event.key === "ArrowLeft" && hasPrev) {
                navigateTo(currentIndex - 1);
            }
        };

        window.addEventListener("keydown", handleKeyDown);

        return () => {
            window.removeEventListener("keydown", handleKeyDown);
        };
    }, [hasNext, hasPrev, navigateTo, closeViewer, currentIndex]);

    if (!post || !inRange) {
        return null;
    }

    if (!mounted) {
        return null;
    }

    const media = attachments[currentIndex];
    const total = attachments.length;

    const animated = isAnimated(media);
    const url = makeViewerURL(media.media_objects);
    const width = media.media_object_metadata.width || 0;
    const height = media.media_object_metadata.height || 0;

    // * Keep some breathing room between the media and the viewport edges.
    const padded =
        viewportWidth <= 640
            ? { width: viewportWidth, height: viewportHeight }
            : fitMedia(viewportWidth, viewportHeight, width, height, 96);

    return createPortal(
        <div
            className={style["media-viewer-overlay"]}
            role="dialog"
            aria-modal="true"
            aria-label="Media viewer"
        >
            <div className={style["media-viewer-box"]}>
                <header className={style["media-viewer-header"]}>
                    <span className={style["media-viewer-counter"]}>
                        {currentIndex + 1} / {total}
                    </span>
                    <button
                        type="button"
                        className={style["media-viewer-close"]}
                        aria-label="Close media viewer"
                        onClick={closeViewer}
                        autoFocus
                    >
                        <IoClose />
                    </button>
                </header>

                {hasPrev && (
                    <button
                        type="button"
                        className={`${style["media-viewer-controller"]} ${style["media-viewer-controller-left"]}`}
                        onClick={() => navigateTo(currentIndex - 1)}
                        aria-label="Previous media"
                    >
                        <IoIosArrowBack />
                    </button>
                )}

                <div
                    className={style["media-viewer-canvas"]}
                    onClick={closeViewer}
                >
                    <div
                        className={style["media-viewer-media"]}
                        onClick={(event) => event.stopPropagation()}
                    >
                        {media.file_type === "Hls" ? (
                            <HlsPlayer
                                id={media.id}
                                media={media}
                                width={padded.width}
                                height={padded.height}
                                containerWidth={padded.width}
                                containerHeight={padded.height}
                            />
                        ) : (
                            <Image
                                src={url}
                                containerWidth={padded.width}
                                containerHeight={padded.height}
                                width={width}
                                height={height}
                                objectFit="contain"
                                format={animated ? "webp" : undefined}
                                animated_src={animated ? url : undefined}
                                optimizationType={
                                    animated ? "animated_on_load" : "static"
                                }
                                thumbhash={
                                    media.media_objects[0]?.thumbhash ||
                                    undefined
                                }
                                alt="Media"
                            />
                        )}
                    </div>
                </div>

                {hasNext && (
                    <button
                        type="button"
                        className={`${style["media-viewer-controller"]} ${style["media-viewer-controller-right"]}`}
                        onClick={() => navigateTo(currentIndex + 1)}
                        aria-label="Next media"
                    >
                        <IoIosArrowForward />
                    </button>
                )}
            </div>
        </div>,
        document.body,
    );
}