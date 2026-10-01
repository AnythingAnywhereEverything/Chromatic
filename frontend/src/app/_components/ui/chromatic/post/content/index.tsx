import { fetchOpenGraphData } from "@/api/opengraph";
import React, { useEffect, useMemo } from "react";
import styles from "./style.module.scss";
import Link from "next/link";

interface FormattedTextProps {
    content: string;
    className?: string;
    ref?: React.Ref<HTMLSpanElement>;
    onLinkOpenGraphPreview?: (previews: OpenGraphPreview[]) => void;
}

interface OpenGraphPreview {
    url: string;
    title?: string;
    description?: string;
    image?: string;
}

const openGraphCache = new Map<string, Promise<OpenGraphPreview | null>>();

const fetchOpenGraph = (url: string): Promise<OpenGraphPreview | null> => {
    const cached = openGraphCache.get(url);
    if (cached) {
        return cached;
    }
    const request = fetchOpenGraphData(url)
        .then((response) => {
            if (!response) {
                return null;
            }

            return {
                url,
                title: response.title,
                description: response.description,
                image: response.image,
            };
        })
        .catch(() => null);
    openGraphCache.set(url, request);
    return request;
};

const OpenGraphPreviewComponent: React.FC<{
    preview: OpenGraphPreview;
}> = ({ preview }) => {
    if (!preview.description && !preview.image) {
        return null;
    }

    return (
        <a
            className={styles["opengraph-preview"]}
            href={preview.url}
            target="_blank"
            rel="noopener noreferrer"
        >
            <div className={styles["opengraph-preview-content"]}>
                {preview.title && (
                    <h3
                        title={preview.title}
                        className={styles["opengraph-preview-title"]}
                    >
                        {preview.title}
                    </h3>
                )}

                {preview.description && (
                    <p className={styles["opengraph-preview-description"]}>
                        {preview.description}
                    </p>
                )}
            </div>
            {preview.image && (
                <img
                    className={styles["opengraph-preview-image"]}
                    src={`/external?url=${preview.image}`}
                    alt={preview.title ?? "OpenGraph preview"}
                />
            )}
        </a>
    );
};

const FormattedText: React.FC<
    FormattedTextProps & React.HTMLAttributes<HTMLSpanElement>
> = ({ content, className, ref, onLinkOpenGraphPreview, ...props }) => {
    const urlRegex = /(https?:\/\/[^\s]+)/g;
    const tagRegex = /#[^\s#]+/g;
    const tokenRegex = /(https?:\/\/[^\s]+)|(#[^\s#]+)/g;

    const urls = useMemo(() => {
        return [...new Set(content.match(urlRegex) ?? [])];
    }, [content]);

    useEffect(() => {
        if (!onLinkOpenGraphPreview || urls.length === 0) {
            return;
        }

        let cancelled = false;

        Promise.all(urls.map(fetchOpenGraph)).then((previews) => {
            if (cancelled) {
                return;
            }

            onLinkOpenGraphPreview(
                previews.filter(
                    (preview): preview is OpenGraphPreview => preview !== null,
                ),
            );
        });

        return () => {
            cancelled = true;
        };
    }, [urls, onLinkOpenGraphPreview]);

    const formatText = (input: string) => {
        const parts: React.ReactNode[] = [];
        let lastIndex = 0;

        for (const match of input.matchAll(tokenRegex)) {
            const value = match[0];
            const index = match.index ?? 0;

            // * Preserve normal text before the token.
            if (index > lastIndex) {
                parts.push(input.slice(lastIndex, index));
            }

            if (value.match(urlRegex)) {
                parts.push(
                    <a
                        className={styles["opengraph-preview-link"]}
                        key={`url-${index}`}
                        href={value}
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        {value}
                    </a>,
                );
            } else if (value.match(tagRegex)) {
                const tag = value.substring(1);

                parts.push(
                    <Link
                        prefetch={false}
                        className={styles["tag"]}
                        key={`tag-${index}`}
                        href={`/explore?q=${encodeURIComponent(value)}`}
                    >
                        {value}
                    </Link>,
                );
            }

            lastIndex = index + value.length;
        }

        // * Preserve any text after the last token.
        if (lastIndex < input.length) {
            parts.push(input.slice(lastIndex));
        }

        return parts;
    };
    return (
        <span className={className} ref={ref} {...props}>
            {formatText(content)}
        </span>
    );
};

export { FormattedText, OpenGraphPreviewComponent };

export type { OpenGraphPreview };
