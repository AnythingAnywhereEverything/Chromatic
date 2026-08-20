"use client";

import React, { useMemo, useState } from "react";
import { thumbHashToDataURL } from "thumbhash";

type ChromaImageProps = Omit<
    React.ImgHTMLAttributes<HTMLImageElement>,
    "src" | "width" | "height"
> & urlParams & {
    src: string;
    thumbhash?: string;
    containerClassName?: string;
    delay?: number;
};

type urlParams = {
    width?: number;
    height?: number;
    format?: string;
    size?: number;
};

function thumbhashB64ToDataURL(
    thumbhashBase64: string,
): string {
    const base64 = thumbhashBase64.replace(/-/g, "+").replace(/_/g, "/");

    // * Restore Base64 padding
    const padded = base64 + "=".repeat((4 - (base64.length % 4)) % 4);
    const binary = atob(padded);
    const hash = Uint8Array.from(binary, (char) => char.charCodeAt(0));
    return thumbHashToDataURL(hash);
};

function constructImageUrl(
    src: string,
    width?: number,
    height?: number,
    format?: string,
    size?: number,
): string {
    if (!process.env.NEXT_PUBLIC_CDN_URL) {
        throw new Error("NEXT_PUBLIC_CDN_URL is not defined");
    }

    const url = new URL(`${process.env.NEXT_PUBLIC_CDN_URL}${src}`, window.location.origin);
    if (width) {
        url.searchParams.set("width", String(width));
    }
    if (height) {
        url.searchParams.set("height", String(height));
    }
    if (size && !width && !height) {
        url.searchParams.set("width", String(size));
        url.searchParams.set("height", String(size));
    }
    if (format) {
        url.searchParams.set("format", format);
    }

    return url.toString();
}

export function ChromaImage({
    src,

    width,
    height,
    size,
    format,
    
    thumbhash,
    style,
    onLoad,
    delay = 200,
    ...props
}: ChromaImageProps) {
    const [loaded, setLoaded] = useState(false);
    const thumbhashUrl = useMemo(
        () => (thumbhash ? thumbhashB64ToDataURL(thumbhash) : undefined),
        [thumbhash],
    );

    const imageUrl = useMemo(
        () => constructImageUrl(src, width, height, format, size),
        [src, width, height, format, size],
    );

    return (
        <div
            className={props.containerClassName}
            style={{
                position: "relative",
                width: `${width}px`,
                height: `${height}px`,
                aspectRatio: `${width} / ${height}`,
                overflow: "hidden",
            }}
        >
            {thumbhashUrl && (
                <img
                    {...props}
                    src={thumbhashUrl}
                    aria-hidden
                    style={{
                        position: "absolute",
                        inset: 0,
                        objectFit: "cover",
                        opacity: loaded ? 0 : 1,
                        transition: `opacity ${delay}ms ease-out`,
                        width: "100%",
                        height: "100%",
                        pointerEvents: "none",
                        ...style,
                    }}
                />
            )}

            <img
                {...props}
                src={imageUrl}
                width={width}
                height={height}
                onLoad={(event) => {
                    setLoaded(true);
                    onLoad?.(event);
                }}
                style={{
                    objectFit: "cover",
                    ...style,
                }}
            />
        </div>
    );
}
