import { thumbHashToDataURL } from "thumbhash";

function isAnimatedImage(url: string): boolean {
    const animatedExtensions = [".gif", ".webp", ".apng"];
    return animatedExtensions.some((ext) => url.toLowerCase().endsWith(ext));
}

function isValidImageUrl(url: string): boolean {
    try {
        const parsedUrl = new URL(url);
        return parsedUrl.protocol === "http:" || parsedUrl.protocol === "https:";
    } catch (e) {
        return false;
    }
}

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

    if (size && !height && !width) {
        url.searchParams.set("width", String(size));
        url.searchParams.set("height", String(size));
    }

    if (format) {
        url.searchParams.set("format", format);
    }

    return url.toString();
}

export { isAnimatedImage, isValidImageUrl, thumbhashB64ToDataURL, constructImageUrl };