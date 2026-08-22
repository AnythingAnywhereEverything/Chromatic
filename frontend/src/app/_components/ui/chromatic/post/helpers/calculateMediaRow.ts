const MIN_WIDTH_RATIO = 0.6;
const MAX_WIDTH_RATIO = 0.9;
const MIN_HEIGHT_RATIO = 0.3;
const SINGLE_MAX_HEIGHT_RATIO = 1.25;

export type Media = {
    w: number;
    h: number;
    order: number;
};

export type FittedMedia = {
    w: number;
    h: number;
    order: number;
};

export interface CalculatorProps {
    containerWidth: number;
    containerHeight: number;
    medias: Media[];
    gap: number;
}

export function clamp(value: number, minimum: number, maximum: number): number {
    return Math.max(minimum, Math.min(value, maximum));
}

function fitSingle(
    media: Media,
    containerWidth: number,
    containerHeight: number,
): FittedMedia {
    const minWidth = containerWidth * MIN_WIDTH_RATIO;
    const minHeight = containerHeight * MIN_HEIGHT_RATIO;
    const maxHeight = containerHeight * SINGLE_MAX_HEIGHT_RATIO;

    const ratio = media.w / media.h;

    /*
     * * Try minimum dimensions while retaining ratio.
     */
    const scale = Math.max(minWidth / media.w, minHeight / media.h);
    let width = media.w * scale;
    let height = media.h * scale;

    /*
     * * Ratio-preserving result fits.
     */
    if (width <= containerWidth && height <= maxHeight) {
        return {
            w: Math.round(width),
            h: Math.round(height),
            order: media.order,
        };
    }

    /*
     * * Too tall:
     * * use the maximum allowed single-image height
     * * while retaining the original ratio.
     */
    if (height > maxHeight) {
        height = maxHeight;
        width = height * ratio;
    }

    /*
     * * Too wide:
     * * fit the viewport width while retaining ratio.
     */
    if (width > containerWidth) {
        width = containerWidth;
        height = width / ratio;
    }

    /*
     * ! If the ratio-preserving dimensions now
     * ! fall below the minimum visual size,
     * ! intentionally break the ratio.
     */
    if (width < minWidth) {
        width = minWidth;
    }

    if (height < minHeight) {
        height = minHeight;
    }

    return {
        w: Math.round(width),
        h: Math.round(height),
        order: media.order,
    };
}

function fitOverflow(
    medias: Media[],
    containerWidth: number,
    containerHeight: number,
): FittedMedia[] {
    const minWidth = containerWidth * MIN_WIDTH_RATIO;

    const maxWidth = containerWidth * MAX_WIDTH_RATIO;

    const minHeight = containerHeight * MIN_HEIGHT_RATIO;

    const requiredHeights = medias.map((media) => {
        const ratio = media.w / media.h;

        return minWidth / ratio;
    });

    /*
     * * Let the thinnest image determine the
     * useful row height.
     */
    let rowHeight = Math.max(...requiredHeights);

    rowHeight = Math.max(rowHeight, minHeight);

    rowHeight = Math.min(rowHeight, containerHeight);

    return medias.map((media) => {
        const ratio = media.w / media.h;

        const naturalWidth = rowHeight * ratio;

        const width = clamp(naturalWidth, minWidth, maxWidth);

        return {
            w: Math.round(width),
            h: Math.round(rowHeight),
            order: media.order,
        };
    });
}

function tryFitTwo(
    medias: Media[],
    containerWidth: number,
    containerHeight: number,
    gap: number,
): FittedMedia[] | null {
    const [first, second] = medias;

    const firstRatio = first.w / first.h;

    const secondRatio = second.w / second.h;

    const availableWidth = containerWidth - gap;

    const combinedRatio = firstRatio + secondRatio;

    /*
     * Height needed for both images to
     * exactly fill the available width.
     */
    const rowHeight = availableWidth / combinedRatio;

    const minimumHeight = containerHeight * MIN_HEIGHT_RATIO;

    /*
     * * The two-image "fit together" layout
     * must be visually useful as well as valid.
     *
     * Too tall  -> reject.
     * Too short -> reject.
     */
    if (rowHeight > containerHeight || rowHeight < minimumHeight) {
        return null;
    }

    return medias.map((media) => {
        const ratio = media.w / media.h;

        return {
            w: Math.round(rowHeight * ratio),
            h: Math.round(rowHeight),
            order: media.order,
        };
    });
}

export function calculateMediaRow({
    containerWidth,
    containerHeight,
    medias,
    gap,
}: CalculatorProps): FittedMedia[] {
    if (medias.length === 0) {
        return [];
    }

    if (medias.length === 1) {
        return [fitSingle(medias[0], containerWidth, containerHeight)];
    }

    /*
     * * Two images get one attempt to fit
     * together while preserving their ratios.
     */
    if (medias.length === 2) {
        const fitted = tryFitTwo(medias, containerWidth, containerHeight, gap);

        if (fitted) {
            return fitted;
        }
    }

    /*
     * * Failed two-image fit and 3+ images
     * use the same visual overflow algorithm.
     */
    return fitOverflow(medias, containerWidth, containerHeight);
}
