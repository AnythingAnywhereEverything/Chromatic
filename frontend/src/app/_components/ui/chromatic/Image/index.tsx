"use client";

import React, { useEffect, useMemo, useRef, useState } from "react";

import type { ImageProps } from "./type";
import { OptimizationType } from "./type";

import { constructImageUrl, thumbhashB64ToDataURL } from "./helper";

export function Image({
    src,
    animated_src,
    width,
    height,
    size,
    format,
    thumbhash,
    style,
    onLoad,
    delay = 200,
    containerClassName,
    containerWidth,
    containerHeight,
    optimizationType = "static",
    viewportThreshold = 0.5,
    ...props
}: ImageProps) {
    const [loaded, setLoaded] = useState(false);
    const [animatedLoaded, setAnimatedLoaded] = useState(false);
    const [inViewport, setInViewport] = useState(false);
    const [hovered, setHovered] = useState(false);
    const [pageActive, setPageActive] = useState(true);

    const imageContainerRef = useRef<HTMLDivElement>(null);

    const imageUrl = useMemo(
        () => constructImageUrl(src, width, height, format, size),
        [src, width, height, format, size],
    );

    const animatedImageUrl = useMemo(
        () =>
            animated_src
                ? constructImageUrl(animated_src, width, height, format, size)
                : undefined,
        [animated_src, width, height, format, size],
    );

    useEffect(() => {
        const updatePageActive = () => {
            setPageActive(
                document.visibilityState === "visible" && document.hasFocus(),
            );
        };

        updatePageActive();

        document.addEventListener("visibilitychange", updatePageActive);
        window.addEventListener("focus", updatePageActive);
        window.addEventListener("blur", updatePageActive);

        return () => {
            document.removeEventListener("visibilitychange", updatePageActive);
            window.removeEventListener("focus", updatePageActive);
            window.removeEventListener("blur", updatePageActive);
        };
    }, []);

    const thumbhashUrl = useMemo(
        () => (thumbhash ? thumbhashB64ToDataURL(thumbhash) : undefined),
        [thumbhash],
    );

    useEffect(() => {
        if (optimizationType !== "animated_in_viewport") {
            return;
        }

        const element = imageContainerRef.current;

        if (!element) {
            return;
        }

        const observer = new IntersectionObserver(
            ([entry]) => {
                setInViewport(entry.intersectionRatio >= viewportThreshold);
            },
            {
                threshold: [0, viewportThreshold, 1],
            },
        );

        observer.observe(element);

        return () => observer.disconnect();
    }, [optimizationType, viewportThreshold]);

    const shouldLoadAnimated =
        !!animatedImageUrl &&
        (optimizationType === "animated_on_load"
            ? loaded
            : optimizationType === "animated_in_viewport"
              ? inViewport
              : optimizationType === "animated_on_hover"
                ? hovered
                : false);

    const showAnimated = shouldLoadAnimated && animatedLoaded && pageActive;

    return (
        <div
            ref={imageContainerRef}
            className={containerClassName}
            style={{
                position: "relative",
                width: `${containerWidth}px`,
                height: `${containerHeight}px`,
                aspectRatio: `${containerWidth} / ${containerHeight}`,
                overflow: "hidden",
            }}
            onMouseEnter={() => {
                if (optimizationType === "animated_on_hover") {
                    setHovered(true);
                }
            }}
            onMouseLeave={() => {
                if (optimizationType === "animated_on_hover") {
                    setHovered(false);
                }
            }}
        >
            {thumbhashUrl && (
                <img
                    src={thumbhashUrl}
                    aria-hidden
                    style={{
                        position: "absolute",
                        inset: 0,
                        width: "100%",
                        height: "100%",
                        objectFit: "cover",
                        opacity: loaded ? 0 : 1,
                        transition: `opacity ${delay}ms ease-out`,
                        pointerEvents: "none",
                    }}
                />
            )}

            <img
                {...props}
                src={imageUrl}
                width={containerWidth}
                height={containerHeight}
                onLoad={(event) => {
                    setLoaded(true);
                    onLoad?.(event);
                }}
                style={{
                    position: "relative",
                    display: "block",
                    width: "100%",
                    height: "100%",
                    objectFit: "cover",
                    opacity: showAnimated ? 0 : 1,
                    transition: `opacity ${delay}ms ease-out`,
                    ...style,
                }}
            />

            {shouldLoadAnimated && animatedImageUrl && pageActive && (
                <img
                    src={animatedImageUrl}
                    width={containerWidth}
                    height={containerHeight}
                    aria-hidden
                    onLoad={() => {
                        setAnimatedLoaded(true);
                    }}
                    style={{
                        position: "absolute",
                        inset: 0,
                        width: "100%",
                        height: "100%",
                        objectFit: "cover",
                        opacity: animatedLoaded ? 1 : 0,
                        transition: `opacity ${delay}ms ease-out`,
                        pointerEvents: "none",
                    }}
                />
            )}
        </div>
    );
}
