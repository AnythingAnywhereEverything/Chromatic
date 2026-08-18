"use client";

import React, { useEffect, useMemo, useState } from "react";
import { thumbHashToRGBA, thumbHashToDataURL } from "thumbhash";

type ChromaImageProps = Omit<
    React.ImgHTMLAttributes<HTMLImageElement>,
    "src" | "width" | "height"
> & {
    src: string;
    width: number;
    height: number;
    thumbhash?: string;
    format?: string;
    containerClassName?: string;
};

export const thumbhashBase64URLSAFEToBytes = (thumbhashBase64: string): Uint8Array => {
    const base64 = thumbhashBase64
        .replace(/-/g, '+')
        .replace(/_/g, '/');

    // * Restore Base64 padding
    const padded = base64 + '='.repeat((4 - base64.length % 4) % 4);
    const binary = atob(padded);
    return Uint8Array.from(binary, char => char.charCodeAt(0));
};

function thumbhashBase64ToDataURL(base64: string): string {
    const hash = thumbhashBase64URLSAFEToBytes(base64);
    return thumbHashToDataURL(hash);
}

export function ChromaImage({
    src,
    width,
    height,
    thumbhash,
    format = "webp",
    style,
    onLoad,
    ...props
}: ChromaImageProps) {
    const [loaded, setLoaded] = useState(false);

    const thumbhashUrl = useMemo(
        () => (thumbhash ? thumbhashBase64ToDataURL(thumbhash) : undefined),
        [thumbhash]
    );

    const final_src = process.env.NEXT_PUBLIC_CDN_URL ? `${process.env.NEXT_PUBLIC_CDN_URL}${src}` : src;

    const imageUrl = useMemo(() => {
        const url = new URL(final_src, window.location.origin);

        url.searchParams.set("width", String(width));
        url.searchParams.set("height", String(height));
        url.searchParams.set("format", format);

        return url.toString();
    }, [final_src, width, height, format]);

    return (
        <div
            className={props.containerClassName}
            style={{
                position: "relative",
                width: "100%",
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
                        width: "100%",
                        height: "100%",
                        objectFit: "cover",
                        opacity: loaded ? 0 : 1,
                        transition: "opacity 200ms ease",
                        ...style
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
                    width: "100%",
                    height: "100%",
                    objectFit: "cover",
                    ...style,
                }}
            />
        </div>
    );
}