import style from "./mediagroup.module.scss";
import React from "react";
import { Image } from "../../Image";
import {
    calculateMediaRow,
    CalculatorProps,
} from "../helpers/calculateMediaRow";
import { HlsPlayer } from "../../hlsPlayer";
import { Media, MediaObjects } from "@/api/types/media";

// BIT MASKING LAYER
const MediaFlags = {
    IsAnimated: 1 << 0,
};

interface MediaGroupProps {
    containerWidthRatio?: number;
    containerHeightRatio?: number;
    media: Media[];
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

    media.forEach((item, i) => {
        item.media_object_metadata.width = fittedMedias[i].w;
        item.media_object_metadata.height = fittedMedias[i].h;
    });

    return media;
}

function MediaLayout({ media, containerWidthRatio ,containerHeightRatio }: MediaGroupProps ) {
    const [medias, setMedia] = React.useState<Media[]>([]);
    const [activeIndex, setActiveIndex] = React.useState(0);
    const [translateX, setTranslateX] = React.useState(0);
    const [hasOverflow, setHasOverflow] = React.useState(false);

    const ImageGroupRef = React.useRef<HTMLDivElement | null>(null);
    const ImageGridRef = React.useRef<HTMLUListElement | null>(null);

    React.useEffect(() => {
        const element = ImageGroupRef.current;

        if (!element) {
            return;
        }

        const updateImages = (width: number) => {
            if (width <= 0) {
                return;
            }
            
            const height =
                width * ((containerHeightRatio || CONTAINER_HEIGHT_RATIO) / (containerWidthRatio|| CONTAINER_WIDTH_RATIO));

            setMedia(GetAllMediaDimensions(media, width, height));
            setActiveIndex(0);
            setTranslateX(0);
        };

        updateImages(element.clientWidth);

        const observer = new ResizeObserver((entries) => {
            const width = entries[0]?.contentRect.width;

            if (width) {
                updateImages(width);
            }
        });

        observer.observe(element);

        return () => {
            observer.disconnect();
        };
    }, [media]);

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
    }, [medias, activeIndex]);

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
                    ‹
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
                    {medias.map((item) => {
                        const isAnimated = item.flags & MediaFlags.IsAnimated;
                        let url = makeStaticURL(item.media_objects);

                        if (item.file_type === "Hls") {
                            return (
                                <li
                                    className={style["image-item"]}
                                    key={item.id}
                                >
                                    <HlsPlayer
                                        id={item.id}
                                        media={item}
                                        width={item.media_object_metadata.width}
                                        height={item.media_object_metadata.height}
                                    />
                                </li>
                            );
                        }

                        return (
                            <Image
                                containerClassName={style["image-item"]}
                                key={item.id}
                                src={url}
                                format={isAnimated ? "webp" : undefined}
                                animated_src={
                                    isAnimated
                                        ? url
                                        : undefined
                                }
                                alt="Media"
                                containerWidth={item.media_object_metadata.width}
                                containerHeight={item.media_object_metadata.height}
                                width={item.media_object_metadata.width}
                                height={item.media_object_metadata.height}
                                thumbhash={item.media_objects[0]?.thumbhash || undefined}
                                optimizationType={
                                    isAnimated
                                        ? "animated_in_viewport"
                                        : "static"
                                }
                                viewportThreshold={0.2}
                            />
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
                    ›
                </button>
            )}
        </div>
    );
}

export const MediaGroup = ({ media, containerWidthRatio, containerHeightRatio }: MediaGroupProps) => {
    if (media.length === 0) {
        return null;
    }

    return (
        <div className={style["media-group"]}>
            <MediaLayout media={media} 
            containerWidthRatio={containerWidthRatio} 
            containerHeightRatio={containerHeightRatio}
            />
        </div>
    );
};
