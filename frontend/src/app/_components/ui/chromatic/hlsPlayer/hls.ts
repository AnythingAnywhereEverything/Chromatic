// hls.ts

import Hls from "hls.js";

export type HlsLevel = {
    index: number;
    width: number;
    height: number;
}

export const getMasterLevels = async (
    src: string,
): Promise<HlsLevel[]> => {
    const response = await fetch(src);

    if (!response.ok) {
        throw new Error(`Failed to load master playlist: ${response.status}`);
    }

    const manifest = await response.text();

    const levels: HlsLevel[] = [];
    const lines = manifest.split(/\r?\n/);

    let levelIndex = 0;

    for (let i = 0; i < lines.length; i++) {
        const line = lines[i].trim();

        if (!line.startsWith("#EXT-X-STREAM-INF:")) {
            continue;
        }

        const resolution = line.match(
            /(?:^|,)RESOLUTION=(\d+)x(\d+)/,
        );

        if (!resolution) {
            continue;
        }

        levels.push({
            index: levelIndex,
            width: Number(resolution[1]),
            height: Number(resolution[2]),
        });

        levelIndex++;
    }

    return levels
        .filter(
            (level, index, array) =>
                array.findIndex(
                    (item) =>
                        item.width === level.width &&
                        item.height === level.height,
                ) === index,
        )
        .sort((a, b) => b.height - a.height);
};

export const createHls = () =>
    new Hls({
        autoStartLoad: true,
        maxBufferLength: 12,
        maxMaxBufferLength: 20,
        maxBufferSize: 30 * 1000 * 1000,
        maxBufferHole: 0.5,
        startLevel: -1,
    });

export const getHlsLevels = (hls: Hls): HlsLevel[] =>
    hls.levels
        .map((level, index) => ({
            index,
            width: level.width,
            height: level.height,
        }))
        .filter(
            (level, index, array) =>
                array.findIndex(
                    (item) =>
                        item.width === level.width &&
                        item.height === level.height,
                ) === index,
        )
        .sort((a, b) => b.height - a.height);

export const preloadResolution = async (
    src: string,
    levelIndex: number,
): Promise<() => void> => {
    const preloadVideo = document.createElement("video");

    preloadVideo.muted = true;
    preloadVideo.playsInline = true;
    preloadVideo.preload = "auto";

    preloadVideo.style.position = "fixed";
    preloadVideo.style.width = "1px";
    preloadVideo.style.height = "1px";
    preloadVideo.style.opacity = "0";
    preloadVideo.style.pointerEvents = "none";

    document.body.appendChild(preloadVideo);

    const preloadHls = createHls();

    try {
        preloadHls.loadSource(src);
        preloadHls.attachMedia(preloadVideo);

        await new Promise<void>((resolve, reject) => {
            let resolved = false;

            const timeout = window.setTimeout(() => {
                if (resolved) {
                    return;
                }

                resolved = true;
                reject(new Error("Resolution preload timeout"));
            }, 10000);

            preloadHls.on(Hls.Events.MANIFEST_PARSED, () => {
                if (resolved) {
                    return;
                }

                preloadHls.loadLevel = levelIndex;
                preloadHls.nextLevel = levelIndex;
            });

            preloadHls.on(Hls.Events.FRAG_BUFFERED, (_event, data) => {
                if (resolved || data.frag.level !== levelIndex) {
                    return;
                }

                resolved = true;
                window.clearTimeout(timeout);
                resolve();
            });

            preloadHls.on(Hls.Events.ERROR, (_event, data) => {
                if (resolved || !data.fatal) {
                    return;
                }

                resolved = true;
                window.clearTimeout(timeout);
                reject(new Error(data.details));
            });
        });

        return () => {
            preloadHls.destroy();
            preloadVideo.remove();
        };
    } catch (error) {
        preloadHls.destroy();
        preloadVideo.remove();

        throw error;
    }
};

export const switchResolution = async (
    video: HTMLVideoElement,
    currentHls: Hls,
    src: string,
    levelIndex: number,
): Promise<Hls> => {
    const currentTime = video.currentTime;
    const wasPlaying = !video.paused && !video.ended;

    // * Preload the requested rendition before touching the real player.
    const cleanupPreload = await preloadResolution(src, levelIndex);

    try {
        video.pause();

        currentHls.destroy();

        const newHls = createHls();

        newHls.loadSource(src);
        newHls.attachMedia(video);

        await new Promise<void>((resolve, reject) => {
            let resolved = false;

            const timeout = window.setTimeout(() => {
                if (resolved) {
                    return;
                }

                resolved = true;
                reject(new Error("Resolution switch timeout"));
            }, 10000);

            newHls.on(Hls.Events.MANIFEST_PARSED, () => {
                if (resolved) {
                    return;
                }

                // * Start the real player directly at the already-preloaded level.
                newHls.loadLevel = levelIndex;
                newHls.nextLevel = levelIndex;

                resolved = true;
                window.clearTimeout(timeout);
                resolve();
            });

            newHls.on(Hls.Events.ERROR, (_event, data) => {
                if (resolved || !data.fatal) {
                    return;
                }

                resolved = true;
                window.clearTimeout(timeout);
                reject(new Error(data.details));
            });
        });

        video.currentTime = currentTime;

        if (wasPlaying) {
            await video.play().catch(() => {});
        }

        return newHls;
    } finally {
        cleanupPreload();
    }
};