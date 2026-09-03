const MIN_WIDTH_RATIO = 0.6;
const MAX_WIDTH_RATIO = 0.9;
const MIN_HEIGHT_RATIO = 0.4;
const SINGLE_MAX_HEIGHT_RATIO = 1.10;

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

    // fill the container width while retaining ratio.
    let width = containerWidth;
    let height = width / ratio;

    // If height exceeds the single-image height allowance,
    // scale down while retaining ratio.
    if (height > maxHeight) {
        height = maxHeight;
        width = height * ratio;
    }

    // If width is still below the minimum visual width,
    // increase width and intentionally break the ratio.
    if (width < minWidth) {
        width = minWidth;
        height = Math.min(height, maxHeight);
    }

    // If height is still below the minimum visual height,
    // increase height and intentionally break the ratio.
    if (height < minHeight) {
        height = minHeight;
        width = Math.min(width, containerWidth);
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
     * Let the thinnest image determine the
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

    // * This is the largest row that can preserve both aspect ratios
    // * while fitting exactly inside the available width.
    const rowHeight = availableWidth / combinedRatio;

    const firstWidth = rowHeight * firstRatio;
    const secondWidth = rowHeight * secondRatio;

    // * The row must fit vertically inside the container.
    if (rowHeight > containerHeight) {
        return null;
    }

    // * Keep the minimum height rule, but never enlarge the row to satisfy it.
    const minHeight = containerHeight * MIN_HEIGHT_RATIO;

    if (rowHeight < minHeight) {
        return null;
    }

    // * Final fit check. Do not allow rounding to push the row outside
    // * the container.
    const fittedWidth = Math.round(firstWidth) + gap + Math.round(secondWidth);
    const fittedHeight = Math.round(rowHeight);

    if (
        fittedWidth > containerWidth ||
        fittedHeight > containerHeight
    ) {
        return null;
    }

    return [
        {
            w: Math.round(firstWidth),
            h: fittedHeight,
            order: first.order,
        },
        {
            w: Math.round(secondWidth),
            h: fittedHeight,
            order: second.order,
        },
    ];
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

    // * Two images may be able to fit together in a visually useful way.
    if (medias.length === 2) {
        const fitted = tryFitTwo(medias, containerWidth, containerHeight, gap);

        if (fitted) {
            return fitted;
        }
    }


    // * More than two images, or two images that failed to fit together
    return fitOverflow(medias, containerWidth, containerHeight);
}
