import { useEffect, useRef, useState } from "react";

import type { HlsPlayerProps } from "./type";

import style from "./style.module.scss";

import Hls from "hls.js";
import { Image } from "../Image";
import {
    createHls,
    getHlsLevels,
    getMasterLevels,
    switchResolution,
    type HlsLevel,
} from "./hls";

import { formatTime, getSelectedResolution, makeFullURL } from "./video";
import { IoPause, IoPlay } from "react-icons/io5";
import {
    PiSpeakerSimpleHighFill,
    PiSpeakerSimpleLowFill,
    PiSpeakerSimpleSlashFill,
} from "react-icons/pi";
import { MdFullscreen, MdFullscreenExit } from "react-icons/md";
import { Tooltip, TooltipContent, TooltipTrigger } from "../tooltip";
import {
    Dropdown,
    DropdownContent,
    DropdownTrigger,
    DropdownItem,
} from "../dropdown";
import { FaCog } from "react-icons/fa";

const VOLUME_STORAGE_KEY = "hls-player-volume";
const VOLUME_CHANGE_EVENT = "hls-player-volume-change";

export const HlsPlayer = ({
    id,
    base_src,
    thumbhash,
    autoPlay,
    width,
    height,
    duration: initialDuration = 0,
}: HlsPlayerProps) => {
    const thumbnailSrc = `${base_src}t_${id}.png`;
    const src = makeFullURL(base_src, "hls/master.m3u8").trim();

    const videoRef = useRef<HTMLVideoElement | null>(null);
    const hlsRef = useRef<Hls | null>(null);
    const playerContainerRef = useRef<HTMLDivElement | null>(null);

    const [hlsLevels, setHlsLevels] = useState<HlsLevel[]>([]);
    const [selectedLevel, setSelectedLevel] = useState(-1);
    const [isPlaying, setIsPlaying] = useState(false);
    const [currentTime, setCurrentTime] = useState(0);
    const [duration, setDuration] = useState(initialDuration);
    const [volume, setVolume] = useState(1);
    const [isMuted, setIsMuted] = useState(false);
    const [isFullscreen, setIsFullscreen] = useState(false);
    const [isLoaded, setIsLoaded] = useState(false);

    const loadHls = () => {
        const video = videoRef.current;

        if (!video || !base_src.trim() || hlsRef.current) {
            return;
        }

        setIsLoaded(true);

        if (!Hls.isSupported()) {
            if (video.canPlayType("application/vnd.apple.mpegurl")) {
                video.src = src;
            }

            return;
        }

        const hls = createHls();

        hlsRef.current = hls;

        hls.on(Hls.Events.MANIFEST_PARSED, () => {
            setHlsLevels(getHlsLevels(hls));

            if (autoPlay) {
                video.play().catch(() => {});
            }
        });

        hls.on(Hls.Events.LEVEL_SWITCHED, (_event, data) => {
            setSelectedLevel(data.level);
        });

        hls.on(Hls.Events.ERROR, (_event, data) => {
            if (
                data.details === "bufferAppendNoProgress" ||
                data.details === "bufferSeekOverHole" ||
                data.details === "aborted"
            ) {
                return;
            }

            console.error("HLS error:", data);

            if (data.fatal) {
                hls.destroy();
                hlsRef.current = null;
                setIsLoaded(false);
                setHlsLevels([]);
                setSelectedLevel(-1);
            }
        });

        hls.loadSource(src);
        hls.attachMedia(video);
    };

    const playVideo = async () => {
        const video = videoRef.current;

        if (!video) {
            return;
        }

        if (video.paused) {
            if (!hlsRef.current && !isLoaded) {
                loadHls();
            }

            await video.play();
        } else {
            video.pause();
        }
    };

    const changeResolution = async (levelIndex: number) => {
        const video = videoRef.current;
        const currentHls = hlsRef.current;

        if (!video || !currentHls) {
            return;
        }

        if (levelIndex === -1) {
            currentHls.currentLevel = -1;
            setSelectedLevel(-1);
            return;
        }

        if (!currentHls.levels[levelIndex]) {
            return;
        }

        try {
            // * Preload the requested resolution, then swap the real player.
            const newHls = await switchResolution(
                video,
                currentHls,
                src,
                levelIndex,
            );

            hlsRef.current = newHls;

            setSelectedLevel(levelIndex);
        } catch (error) {
            console.error("Resolution switch failed:", error);
        }
    };

    useEffect(() => {
        return () => {
            hlsRef.current?.destroy();
            hlsRef.current = null;
        };
    }, [base_src]);

    useEffect(() => {
        let cancelled = false;

        const loadMasterLevels = async () => {
            if (!base_src.trim()) {
                return;
            }

            try {
                // * Only fetch/parse master.m3u8. No HLS instance and no media segments.
                const levels = await getMasterLevels(src);

                if (!cancelled) {
                    setHlsLevels(levels);
                }
            } catch (error) {
                if (!cancelled) {
                    console.error("Failed to load HLS master playlist:", error);
                }
            }
        };

        loadMasterLevels();

        return () => {
            cancelled = true;
        };
    }, [src, base_src]);

    const isSeekingRef = useRef(false);

    const seekFromPointer = (event: React.PointerEvent<HTMLDivElement>) => {
        const container = event.currentTarget;
        const rect = container.getBoundingClientRect();

        const progress = Math.min(
            Math.max((event.clientX - rect.left) / rect.width, 0),
            1,
        );

        seek(progress * duration);
    };

    const handleSeekPointerDown = (
        event: React.PointerEvent<HTMLDivElement>,
    ) => {
        if (!duration) {
            return;
        }

        isSeekingRef.current = true;

        event.currentTarget.setPointerCapture(event.pointerId);

        seekFromPointer(event);
    };

    const handleSeekPointerMove = (
        event: React.PointerEvent<HTMLDivElement>,
    ) => {
        if (!isSeekingRef.current) {
            return;
        }

        seekFromPointer(event);
    };

    const handleSeekPointerUp = (event: React.PointerEvent<HTMLDivElement>) => {
        if (!isSeekingRef.current) {
            return;
        }

        isSeekingRef.current = false;

        event.currentTarget.releasePointerCapture(event.pointerId);
    };

    useEffect(() => {
        if (!autoPlay) {
            return;
        }

        loadHls();
    }, [autoPlay, base_src]);

    useEffect(() => {
        const video = videoRef.current;

        if (!video) {
            return;
        }

        const savedVolume = localStorage.getItem(VOLUME_STORAGE_KEY);

        if (savedVolume !== null) {
            const saved = Number(savedVolume);

            if (Number.isFinite(saved)) {
                video.volume = Math.min(Math.max(saved, 0), 1);
            }
        }

        const handleVolumeChange = () => {
            const currentVolume = video.volume;

            localStorage.setItem(VOLUME_STORAGE_KEY, String(currentVolume));

            window.dispatchEvent(
                new CustomEvent(VOLUME_CHANGE_EVENT, {
                    detail: currentVolume,
                }),
            );
        };

        const handleSharedVolumeChange = (event: Event) => {
            const customEvent = event as CustomEvent<number>;
            const currentVolume = customEvent.detail;

            if (
                typeof currentVolume !== "number" ||
                !Number.isFinite(currentVolume)
            ) {
                return;
            }

            if (video.volume !== currentVolume) {
                video.volume = currentVolume;
            }
        };

        video.addEventListener("volumechange", handleVolumeChange);

        window.addEventListener(VOLUME_CHANGE_EVENT, handleSharedVolumeChange);

        return () => {
            video.removeEventListener("volumechange", handleVolumeChange);

            window.removeEventListener(
                VOLUME_CHANGE_EVENT,
                handleSharedVolumeChange,
            );
        };
    }, []);

    const [bufferedTime, setBufferedTime] = useState(0);

    useEffect(() => {
        const video = videoRef.current;

        if (!video) {
            return;
        }

        const updateBuffered = () => {
            const currentTime = video.currentTime;

            for (let i = 0; i < video.buffered.length; i++) {
                const start = video.buffered.start(i);
                const end = video.buffered.end(i);

                if (currentTime >= start && currentTime <= end) {
                    setBufferedTime(end);
                    return;
                }
            }

            setBufferedTime(0);
        };

        video.addEventListener("progress", updateBuffered);
        video.addEventListener("timeupdate", updateBuffered);
        video.addEventListener("loadedmetadata", updateBuffered);
        video.addEventListener("durationchange", updateBuffered);

        return () => {
            video.removeEventListener("progress", updateBuffered);
            video.removeEventListener("timeupdate", updateBuffered);
            video.removeEventListener("loadedmetadata", updateBuffered);
            video.removeEventListener("durationchange", updateBuffered);
        };
    }, []);

    useEffect(() => {
        const video = videoRef.current;

        if (!video) {
            return;
        }

        const handlePlay = () => setIsPlaying(true);

        const handlePause = () => setIsPlaying(false);

        const handleTimeUpdate = () => {
            setCurrentTime(video.currentTime);
        };

        const handleDurationChange = () => {
            if (Number.isFinite(video.duration)) {
                setDuration(video.duration);
            }
        };

        const handleVolumeChange = () => {
            setVolume(video.volume);
            setIsMuted(video.muted || video.volume === 0);
        };

        const handleFullscreenChange = () => {
            setIsFullscreen(
                document.fullscreenElement === playerContainerRef.current,
            );
        };

        video.addEventListener("play", handlePlay);
        video.addEventListener("pause", handlePause);
        video.addEventListener("timeupdate", handleTimeUpdate);
        video.addEventListener("durationchange", handleDurationChange);
        video.addEventListener("volumechange", handleVolumeChange);

        document.addEventListener("fullscreenchange", handleFullscreenChange);

        return () => {
            video.removeEventListener("play", handlePlay);
            video.removeEventListener("pause", handlePause);
            video.removeEventListener("timeupdate", handleTimeUpdate);
            video.removeEventListener("durationchange", handleDurationChange);
            video.removeEventListener("volumechange", handleVolumeChange);

            document.removeEventListener(
                "fullscreenchange",
                handleFullscreenChange,
            );
        };
    }, []);

    const seek = (value: number) => {
        const video = videoRef.current;

        if (!video) {
            return;
        }

        video.currentTime = value;
        setCurrentTime(value);
    };

    const changeVolume = (value: number) => {
        const video = videoRef.current;

        if (!video) {
            return;
        }

        video.volume = value;
        video.muted = value === 0;
    };

    const toggleMute = () => {
        const video = videoRef.current;

        if (!video) {
            return;
        }

        video.muted = !video.muted;
    };

    const toggleFullscreen = async () => {
        const container = playerContainerRef.current;

        if (!container) {
            return;
        }

        if (document.fullscreenElement) {
            await document.exitFullscreen();
            return;
        }

        await container.requestFullscreen();
    };

    return (
        <div
            ref={playerContainerRef}
            className={style["hls-player-container"]}
            style={{
                width,
                height,
            }}
        >
            <div className={style["html5-video-container"]}>
                <video
                    tabIndex={-1}
                    ref={videoRef}
                    id={id}
                    className={style["hls-player"]}
                    autoPlay={false}
                    controlsList="nodownload"
                    width={width}
                    height={height}
                />
                <Image
                    src={thumbnailSrc}
                    thumbhash={thumbhash || undefined}
                    style={{
                        opacity: isLoaded ? 0 : 1,
                        transition: "opacity 0.3s ease-in-out",
                    }}
                    alt="Video thumbnail"
                    width={width}
                    height={height}
                />
            </div>
            <div className={style["hls-overlay"]} onClick={playVideo} />
            <div className={style["hls-controls"]}>
                <div
                    className={`${style["hls-seek-container"]} ${isSeekingRef.current ? style["seeking"] : ""}`}
                    onPointerDown={handleSeekPointerDown}
                    onPointerMove={handleSeekPointerMove}
                    onPointerUp={handleSeekPointerUp}
                    onPointerCancel={handleSeekPointerUp}
                >
                    <div className={style["hls-player-bar"]} />

                    <div
                        className={style["hls-seek-buffered"]}
                        style={{
                            width: duration
                                ? `${(bufferedTime / duration) * 100}%`
                                : "0%",
                        }}
                    />
                    <div
                        className={style["hls-seek"]}
                        style={
                            {
                                "--seek-progress": duration
                                    ? `${(currentTime / duration) * 100}%`
                                    : "0%",
                            } as React.CSSProperties
                        }
                    />
                    <div
                        className={style["hls-seek-thumb"]}
                        style={{
                            left: duration
                                ? `${(currentTime / duration) * 100}%`
                                : "0%",
                        }}
                    />
                </div>
                <div className={style["hls-actions"]}>
                    <div className={style["hls-left-actions"]}>
                        <Tooltip gap={24} parent={playerContainerRef.current}>
                            <TooltipTrigger asChild>
                                <div
                                    className={style["hls-play-pause"]}
                                    onClick={playVideo}
                                >
                                    <button
                                        className={style["hls-icon-button"]}
                                        type="button"
                                    >
                                        {isPlaying ? <IoPause /> : <IoPlay />}
                                    </button>
                                </div>
                            </TooltipTrigger>
                            <TooltipContent>
                                {isPlaying ? "Pause" : "Play"}
                            </TooltipContent>
                        </Tooltip>

                        <div className={style["hls-time-container"]}>
                            <span className={style["hls-time"]}>
                                {formatTime(currentTime)} /{" "}
                                {formatTime(duration)}
                            </span>
                        </div>

                        <div className={style["hls-volume-container"]}>
                            <Tooltip
                                gap={24}
                                parent={playerContainerRef.current}
                            >
                                <TooltipTrigger asChild>
                                    <button
                                        className={style["hls-icon-button"]}
                                        type="button"
                                        onClick={toggleMute}
                                    >
                                        {isMuted || volume === 0 ? (
                                            <PiSpeakerSimpleSlashFill />
                                        ) : volume > 0.5 ? (
                                            <PiSpeakerSimpleHighFill />
                                        ) : (
                                            <PiSpeakerSimpleLowFill />
                                        )}
                                    </button>
                                </TooltipTrigger>
                                <TooltipContent>Volume</TooltipContent>
                            </Tooltip>

                            <input
                                type="range"
                                min={0}
                                max={1}
                                step={0.01}
                                value={isMuted ? 0 : volume}
                                onChange={(event) =>
                                    changeVolume(Number(event.target.value))
                                }
                                style={
                                    {
                                        "--volume-progress": `${isMuted ? 0 : volume * 100}%`,
                                    } as React.CSSProperties
                                }
                                className={style["hls-volume"]}
                            />
                        </div>
                    </div>

                    <div className={style["hls-right-actions"]}>
                        <Dropdown placement="top" offsetPlacement={24} containerRef={playerContainerRef}>
                            <DropdownTrigger asChild>
                                <button
                                    className={style["hls-icon-button"]}
                                    type="button"
                                >
                                    <FaCog />
                                </button>
                            </DropdownTrigger>
                            <DropdownContent>
                                <DropdownItem>
                                    <button
                                        type="button"
                                        onClick={() => changeResolution(-1)}
                                    >
                                        Auto
                                    </button>
                                </DropdownItem>
                                {hlsLevels.map((level) => (
                                    <DropdownItem key={level.index}>
                                        <button
                                            type="button"
                                            onClick={() =>
                                                changeResolution(level.index)
                                            }
                                        >
                                            {level.height}
                                        </button>
                                    </DropdownItem>
                                ))}
                            </DropdownContent>
                        </Dropdown>

                        <button
                            className={style["hls-icon-button"]}
                            type="button"
                            onClick={toggleFullscreen}
                        >
                            {isFullscreen ? (
                                <MdFullscreenExit />
                            ) : (
                                <MdFullscreen />
                            )}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    );
};
