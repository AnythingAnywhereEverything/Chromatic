import { mediaPostAttechment } from "@/api/post/getFeed";
import style from "./mediagroup.module.scss";
import React from "react";
import { Image } from "../../Image";
import {
    calculateMediaRow,
    CalculatorProps,
} from "../helpers/calculateMediaRow";

interface MediaGroupProps {
    media: mediaPostAttechment[];
}

function isAnimated(media: mediaPostAttechment): boolean {
    return media.path.includes("a_");
}

const CONTAINER_WIDTH_RATIO = 1;
const CONTAINER_HEIGHT_RATIO = 1;

function GetAllMediaDimensions(
    media: mediaPostAttechment[],
    cWidth: number,
    cHeight: number,
): mediaPostAttechment[] {
    let calculatorProps: CalculatorProps = {
        containerWidth: cWidth,
        containerHeight: cHeight,
        medias: media.map((item, i) => ({
            w: item.width || 0,
            h: item.height || 0,
            order: i,
        })),
        gap: 8,
    };

    let fittedMedias = calculateMediaRow(calculatorProps);

    media.forEach((item, i) => {
        item.width = fittedMedias[i].w;
        item.height = fittedMedias[i].h;
    });

    return media;
}

function ImageGroup({ media }: { media: mediaPostAttechment[] }) {
    const [images, setImages] = React.useState<mediaPostAttechment[]>([]);
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
                width * (CONTAINER_HEIGHT_RATIO / CONTAINER_WIDTH_RATIO);

            console.log("Updating images with width:", width, "height:", height);
            setImages(GetAllMediaDimensions(media, width, height));
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

        /*
         * * Check the actual rendered size of the grid.
         * * If it does not exceed the group, there is
         * * nowhere to scroll.
         */
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
    }, [images, activeIndex]);

    const scrollMedia = (direction: "left" | "right") => {
        setActiveIndex((currentIndex) => {
            if (direction === "right") {
                return Math.min(currentIndex + 1, images.length - 1);
            }

            return Math.max(currentIndex - 1, 0);
        });
    };

    const showLeftController = hasOverflow && activeIndex > 0;

    const showRightController = hasOverflow && activeIndex < images.length - 1;

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
                    {images.map((item) => {
                        return (
                            <Image
                                containerClassName={style["image-item"]}
                                key={item.id}
                                src={item.path}
                                animated_src={
                                    isAnimated(item)
                                        ? item.path.replace(".png", ".webp")
                                        : undefined
                                }
                                alt="Media"
                                containerWidth={item.width}
                                containerHeight={item.height}
                                width={item.width}
                                height={item.height}
                                thumbhash={item.thumbhash || undefined}
                                optimizationType={
                                    isAnimated(item)
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

export const MediaGroup = ({ media }: MediaGroupProps) => {
    if (media.length === 0) {
        return null;
    }

    const imageMedia = React.useMemo(
        () => media.filter((item) => item.mime_type.startsWith("image/")),
        [media],
    );

    const Videos = React.useMemo(
        () => media.filter((item) => item.mime_type.startsWith("video/")),
        [media],
    );

    return (
        <div className={style["media-group"]}>
            {imageMedia.length > 0 && <ImageGroup media={imageMedia} />}
            <ul className={style["video-grid"]}>
                {Videos.map((item) => {
                    return (
                        <li key={item.id} className={style["media-item"]}>
                            Waiting
                        </li>
                    );
                })}
            </ul>
        </div>
    );
};
