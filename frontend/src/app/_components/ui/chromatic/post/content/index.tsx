import { fetchOpenGraphData } from "@/api/opengraph";
import React, { useEffect, useMemo } from "react";
import styles from "./style.module.scss";

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
                {preview.title && <h3 title={preview.title} className={styles["opengraph-preview-title"]}>{preview.title}</h3>}

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

    const urls = useMemo(() => {
        return [...new Set(content.match(urlRegex) ?? [])];
    }, [content]);

    useEffect(() => {
        if (!onLinkOpenGraphPreview || urls.length === 0) {
            return;
        }

        let cancelled = false;

        const getOpenGraphData = async () => {
            const previews = await Promise.all(
                urls.map((url) => fetchOpenGraph(url)),
            );

            console.log(previews);

            if (!cancelled) {
                onLinkOpenGraphPreview(
                    previews.filter(
                        (preview): preview is OpenGraphPreview =>
                            preview !== null,
                    ),
                );
            }
        };

        getOpenGraphData();

        return () => {
            cancelled = true;
        };
    }, [urls, onLinkOpenGraphPreview]);

    const formatText = (input: string) => {
        return input.split(urlRegex).map((part, index) => {
            if (part.match(urlRegex)) {
                return (
                    <a
                        key={index}
                        href={part}
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        {part}
                    </a>
                );
            }

            return part;
        });
    };

    return (
        <span className={className} ref={ref} {...props}>
            {formatText(content)}
        </span>
    );
};

export { FormattedText, OpenGraphPreviewComponent };

export type { OpenGraphPreview };
