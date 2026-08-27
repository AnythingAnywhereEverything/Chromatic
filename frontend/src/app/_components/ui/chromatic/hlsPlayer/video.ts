// video.ts

export const formatTime = (seconds: number): string => {
    if (!Number.isFinite(seconds)) {
        return "0:00";
    }

    const hours = Math.floor(seconds / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);
    const remainingSeconds = Math.floor(seconds % 60);

    if (hours > 0) {
        return `${hours}:${String(minutes).padStart(2, "0")}:${String(
            remainingSeconds,
        ).padStart(2, "0")}`;
    }

    return `${minutes}:${String(remainingSeconds).padStart(2, "0")}`;
};

export const makeFullURL = (
    base_src: string,
    path: string,
): string =>
    `${process.env.NEXT_PUBLIC_CDN_URL}${base_src}${path}`;

export const getSelectedResolution = (
    levels: {
        index: number;
        height: number;
    }[],
    selectedLevel: number,
): string => {
    if (selectedLevel === -1) {
        return "Auto";
    }

    const level = levels.find(
        (level) => level.index === selectedLevel,
    );

    return level ? `${level.height}p` : "Auto";
};