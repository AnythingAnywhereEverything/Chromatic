import style from "./mediagroup.module.scss";
import React from "react";
import { Image } from "../../Image";
import {
    calculateMediaRow,
    CalculatorProps,
} from "../helpers/calculateMediaRow";
import { HlsPlayer } from "../../hlsPlayer";
import { Media, MediaObjects } from "@/api/types/media";
import { IoIosArrowBack, IoIosArrowForward } from "react-icons/io";
import Link from "next/link";

// BIT MASKING LAYER
const MediaFlags = {
    IsAnimated: 1 << 0,
};

interface MediaGroupProps {
    containerWidthRatio?: number;
    containerHeightRatio?: number;
    media: Media[];
    // * When provided, media items link to /media/{index + 1} under this post path
    // * (e.g. `/u/{username}/f/{post_id}/media/2`).
    postUrl?: string;
}

// read from flag
function isAnimated(media: Media): boolean {
    return (media.flags & MediaFlags.IsAnimated) !== 0;
}

function makeStaticURL(objects: MediaObjects[]): string {
    if (objects === undefined || objects.length === 0) {
        return "";
    }
    // loop get all from media object, pioritize thumbnail
    for (let i = 0; i < objects.length; i++) {
        if (objects[i].kind === "Thumbnail") {
            return objects[i].storage_key + "/" + objects[i].name;
        }
    }
    for (let i = 0; i < objects.length; i++) {
        if (objects[i].kind === "Original") {
            return objects[i].storage_key + "/" + objects[i].name;
        }
    }
    return "";
}

const CONTAINER_WIDTH_RATIO = 1;
const CONTAINER_HEIGHT_RATIO = 1;

function GetAllMediaDimensions(
    media: Media[],
    cWidth: number,
    cHeight: number,
): Media[] {
    let calculatorProps: CalculatorProps = {
        containerWidth: cWidth,
        containerHeight: cHeight,
        medias: media.map((item, i) => ({
            w: item.media_object_metadata.width || 0,
            h: item.media_object_metadata.height || 0,
            order: i,
        })),
        gap: 8,
    };

    let fittedMedias = calculateMediaRow(calculatorProps);

    // make the media immutable by creating a new array with updated dimensions
    media = media.map((item, i) => ({
        ...item,
        media_object_metadata: {
            ...item.media_object_metadata,
            width: fittedMedias[i].w,
            height: fittedMedias[i].h,
        },
    }));

    return media;
}

interface ImageContainerSize {
    width: number;
    height: number;
}

function MediaLayout({
    media,
    containerWidthRatio,
    containerHeightRatio,
    postUrl,
}: MediaGroupProps) {
    const [medias, setMedia] = React.useState<Media[]>([]);

    const [imageContainers, setImageContainers] = React.useState<
        Record<string, ImageContainerSize>
    >({});

    const [activeIndex, setActiveIndex] = React.useState(0);
    const [translateX, setTranslateX] = React.useState(0);
    const [hasOverflow, setHasOverflow] = React.useState(false);

    const ImageGroupRef = React.useRef<HTMLDivElement | null>(null);
    const ImageGridRef = React.useRef<HTMLUListElement | null>(null);

    const initializedRef = React.useRef(false);

    React.useLayoutEffect(() => {
        const element = ImageGroupRef.current;

        if (!element) {
            return;
        }

        const widthRatio = containerWidthRatio || CONTAINER_WIDTH_RATIO;
        const heightRatio = containerHeightRatio || CONTAINER_HEIGHT_RATIO;
        const getContainerHeight = (width: number) =>
            width * (heightRatio / widthRatio);

        const initialWidth = element.clientWidth;

        if (initialWidth <= 0) {
            return;
        }

        if (!initializedRef.current) {
            initializedRef.current = true;
            const initialHeight = getContainerHeight(initialWidth);
            const initializedMedia = GetAllMediaDimensions(
                media,
                initialWidth,
                initialHeight,
            );

            setMedia(initializedMedia);
            setImageContainers(
                Object.fromEntries(
                    initializedMedia.map((item) => [
                        item.id,
                        {
                            width: item.media_object_metadata.width || 0,
                            height: item.media_object_metadata.height || 0,
                        },
                    ]),
                ),
            );

            setActiveIndex(0);
            setTranslateX(0);
        }

        const observer = new ResizeObserver((entries) => {
            const width = entries[0]?.contentRect.width;

            if (!width || width <= 0) {
                return;
            }

            setImageContainers((current) => {
                const next: Record<string, ImageContainerSize> = {};

                // recalculate container sizes using getAllMediaDimensions
                const recalculatedMedia = GetAllMediaDimensions(
                    media,
                    width,
                    getContainerHeight(width),
                );

                recalculatedMedia.forEach((item) => {
                    next[item.id] = {
                        width: item.media_object_metadata.width || 0,
                        height: item.media_object_metadata.height || 0,
                    };
                });

                return next;
            });
        });

        observer.observe(element);

        return () => {
            observer.disconnect();
        };
    }, [media, containerWidthRatio, containerHeightRatio]);

    React.useLayoutEffect(() => {
        const container = ImageGroupRef.current;
        const grid = ImageGridRef.current;

        if (!container || !grid) {
            return;
        }

        const items = Array.from(
            grid.querySelectorAll<HTMLElement>(`.${style["image-item"]}`),
        );

        const item = items[activeIndex];

        setHasOverflow(grid.scrollWidth > container.clientWidth + 1);

        if (!item) {
            setTranslateX(0);
            return;
        }

        const maxTranslate = Math.max(
            grid.scrollWidth - container.clientWidth,
            0,
        );

        const targetTranslate = Math.min(item.offsetLeft, maxTranslate);

        setTranslateX(targetTranslate);
    }, [medias, activeIndex, imageContainers]);

    const scrollMedia = (direction: "left" | "right") => {
        setActiveIndex((currentIndex) => {
            if (direction === "right") {
                return Math.min(currentIndex + 1, medias.length - 1);
            }

            return Math.max(currentIndex - 1, 0);
        });
    };

    const showLeftController = hasOverflow && activeIndex > 0;

    const showRightController = hasOverflow && activeIndex < medias.length - 1;

    return (
        <div className={style["image-group-wrapper"]}>
            {showLeftController && (
                <button
                    type="button"
                    className={`${style["media-controller"]} ${style["media-controller-left"]}`}
                    onClick={() => scrollMedia("left")}
                    aria-label="Previous media"
                >
                    <IoIosArrowBack />
                </button>
            )}

            <div className={style["image-group"]} ref={ImageGroupRef}>
                <ul
                    className={style["image-grid"]}
                    ref={ImageGridRef}
                    style={{
                        transform: `translate3d(-${translateX}px, 0, 0)`,
                    }}
                >
                    {medias.map((item, index) => {
                        const animated = isAnimated(item);
                        const url = makeStaticURL(item.media_objects);
                        const imageWidth = item.media_object_metadata.width;
                        const imageHeight = item.media_object_metadata.height;
                        const container = imageContainers[item.id];

                        const containerWidth = container?.width || imageWidth;
                        const containerHeight =
                            container?.height || imageHeight;

                        const href = postUrl
                            ? `${postUrl}/media/${index + 1}`
                            : undefined;

                        if (item.file_type === "Hls") {
                            return (
                                <li
                                    className={style["image-item"]}
                                    key={item.id}
                                    style={{
                                        width: containerWidth,
                                        height: containerHeight,
                                    }}
                                >
                                    {href ? (
                                        <Link
                                            href={href}
                                            className={style["image-link"]}
                                        >
                                            <HlsPlayer
                                                id={item.id}
                                                media={item}
                                                width={imageWidth}
                                                height={imageHeight}
                                                containerWidth={containerWidth}
                                                containerHeight={
                                                    containerHeight
                                                }
                                            />
                                        </Link>
                                    ) : (
                                        <HlsPlayer
                                            id={item.id}
                                            media={item}
                                            width={imageWidth}
                                            height={imageHeight}
                                            containerWidth={containerWidth}
                                            containerHeight={containerHeight}
                                        />
                                    )}
                                </li>
                            );
                        }

                        const image = (
                            <Image
                                containerClassName={style["image-item"]}
                                key={item.id}
                                src={url}
                                format={animated ? "webp" : undefined}
                                animated_src={animated ? url : undefined}
                                alt="Media"
                                containerWidth={containerWidth}
                                containerHeight={containerHeight}
                                width={imageWidth}
                                height={imageHeight}
                                thumbhash={
                                    item.media_objects[0]?.thumbhash ||
                                    undefined
                                }
                                optimizationType={
                                    animated ? "animated_in_viewport" : "static"
                                }
                                viewportThreshold={0.2}
                            />
                        );

                        return href ? (
                            <Link
                                key={item.id}
                                href={href}
                                className={style["image-link"]}
                            >
                                {image}
                            </Link>
                        ) : (
                            image
                        );
                    })}
                </ul>
            </div>

            {showRightController && (
                <button
                    type="button"
                    className={`${style["media-controller"]} ${style["media-controller-right"]}`}
                    onClick={() => scrollMedia("right")}
                    aria-label="Next media"
                >
                    <IoIosArrowForward />{" "}
                </button>
            )}
        </div>
    );
}

export const MediaGroup = ({
    media,
    containerWidthRatio,
    containerHeightRatio,
    postUrl,
}: MediaGroupProps) => {
    if (media.length === 0) {
        return null;
    }

    return (
        <div className={style["media-group"]}>
            <MediaLayout
                media={media}
                containerWidthRatio={containerWidthRatio}
                containerHeightRatio={containerHeightRatio}
                postUrl={postUrl}
            />
        </div>
    );
};
