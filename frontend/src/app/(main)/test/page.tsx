"use client";

import { Portal } from "@/app/_components/portal";

import { AlertDialogue } from "@/app/_components/ui/chromatic/confirmation";

import { CreateCommunityBtn } from "@/app/_components/ui/chromatic/createCom";


import {
    PostStatus,
} from "@/app/_components/ui/chromatic/createPost/status";

import {
    Dialog,
    DialogClose,
    DialogContent,
    DialogDescription,
    DialogHeading,
    DialogTrigger,
} from "@/app/_components/ui/chromatic/dialogue";

import {
    Dropdown,
    DropdownContent,
    DropdownItem,
    DropdownTrigger,
} from "@/app/_components/ui/chromatic/dropdown";

import { Post } from "@/app/_components/ui/chromatic/post";

import {
    Tooltip,
    TooltipAnchor,
    TooltipArrow,
    TooltipContent,
    TooltipTrigger,
} from "@/app/_components/ui/chromatic/tooltip";

import React, { useRef, useState } from "react";
import Hls from "hls.js";

const TestPage = () => {
    const [inner, setInner] = React.useState(false);

    const showInner = (event: React.MouseEvent<HTMLButtonElement>) => {
        setInner(true);
    };

    const [rootContent, setRootContent] = React.useState<HTMLElement | null>(
        null,
    );

    const innerRef = React.useRef<HTMLDivElement | null>(null);



    const videoRef = useRef<HTMLVideoElement | null>(null);
    const hlsRef = useRef<Hls | null>(null);

    const [hlsUrl, setHlsUrl] = useState(
        "/cdn/dev_uploads/85265158980505601/85265158980505600/hls/master.m3u8",
    );

    const [hlsLevels, setHlsLevels] = useState<
        {
            index: number;
            width: number;
            height: number;
        }[]
    >([]);

    const [selectedLevel, setSelectedLevel] = useState(-1);

    const loadHls = () => {
        const video = videoRef.current;

        if (!video || !hlsUrl.trim()) {
            return;
        }

        // * Destroy the previous HLS instance before loading another manifest.
        if (hlsRef.current) {
            hlsRef.current.destroy();
            hlsRef.current = null;
        }

        setHlsLevels([]);
        setSelectedLevel(-1);

        const src = hlsUrl.trim();

        if (!Hls.isSupported()) {
            if (video.canPlayType("application/vnd.apple.mpegurl")) {
                video.src = src;
            }

            return;
        }

        const hls = new Hls();

        hlsRef.current = hls;

        hls.loadSource(src);
        hls.attachMedia(video);

        hls.on(Hls.Events.MANIFEST_PARSED, () => {
            const levels = hls.levels.map((level, index) => ({
                index,
                width: level.width,
                height: level.height,
            }));

            setHlsLevels(levels);
        });

        hls.on(Hls.Events.LEVEL_SWITCHED, (_event, data) => {
            console.log("Playing level:", data.level);
        });

        hls.on(Hls.Events.ERROR, (_event, data) => {
            if (data.details === "bufferAppendNoProgress") {
                return;
            }

            console.error("HLS error:", data);
        });
    };

    const changeResolution = (level: number) => {
        const hls = hlsRef.current;

        if (!hls) {
            return;
        }

        if (level === -1) {
            // * Auto / ABR.
            hls.currentLevel = -1;
            hls.nextLevel = -1;
            hls.loadLevel = -1;

            console.log("Available levels:", hls.levels);

            setSelectedLevel(-1);

            return;
        }

        // * Manual quality selection.
        // * nextLevel lets the current buffer continue playing.
        hls.nextLevel = level;

        setSelectedLevel(level);
    };

    React.useEffect(() => {
        return () => {
            // * Clean up HLS when leaving the page.
            hlsRef.current?.destroy();
            hlsRef.current = null;
        };
    }, []);

    return (
        <section>
            <button onMouseOver={showInner} onMouseOut={() => setInner(false)}>
                Open Portal
            </button>

            {inner && <Portal container={innerRef.current}>Something</Portal>}

            <div>
                <p>Hover over the button to open the portal.</p>

                <div>
                    <p>Portal will be rendered here:</p>

                    <div
                        ref={innerRef}
                        style={{
                            border: "1px solid white",
                            padding: "10px",
                            marginTop: "10px",
                        }}
                    />
                </div>
            </div>

            <button
                onMouseOver={(event) => setRootContent(event.currentTarget)}
                onMouseOut={() => setRootContent(null)}
            >
                Show Root Content
            </button>

            {rootContent && (
                <Portal>
                    <div
                        style={{
                            zIndex: 899,
                            position: "fixed",
                            top: 0,
                            left: 0,
                            backgroundColor: "grey",
                            padding: "10px",
                        }}
                    >
                        This is root content!
                    </div>
                </Portal>
            )}

            <div style={{ height: "300px" }} />

            <Tooltip>
                <TooltipTrigger asChild>
                    <button style={{ width: "fit-content" }}>Hover me</button>
                </TooltipTrigger>

                <TooltipContent>Hewoo</TooltipContent>
            </Tooltip>

            <section
                className="longScreen"
                style={{
                    backgroundColor: "lightgray",
                    maxHeight: "300px",
                    overflowY: "scroll",
                }}
            >
                <div style={{ height: "600px" }} />

                <Tooltip>
                    <TooltipTrigger asChild>
                        <button style={{ width: "fit-content" }}>
                            Hover me
                        </button>
                    </TooltipTrigger>

                    <TooltipContent>Amazing</TooltipContent>
                </Tooltip>
            </section>

            <div>
                <Tooltip
                    allowHovering
                    openDelayDuration={500}
                    closeDelayDuration={50000}
                >
                    <TooltipTrigger asChild>
                        <div>
                            <TooltipAnchor>
                                <button>🔥</button>
                            </TooltipAnchor>

                            <button style={{ width: "fit-content" }}>
                                The tooltip will appear on the 🔥
                            </button>
                        </div>
                    </TooltipTrigger>

                    <TooltipContent>
                        Amazing
                        <TooltipArrow />
                    </TooltipContent>
                </Tooltip>
            </div>

            <Dialog>
                <DialogTrigger>My trigger</DialogTrigger>

                <DialogContent>
                    <DialogHeading>My dialog heading</DialogHeading>
                    <DialogDescription>
                        My dialog description
                    </DialogDescription>
                    <DialogClose>Close</DialogClose>
                </DialogContent>
            </Dialog>

            <section>
                <AlertDialogue
                    type="info"
                    title="My Alert Dialog: Info"
                    message="Hello world"
                    triggerName="Normal variant"
                />
            </section>

            <section>
                <AlertDialogue
                    type="warning"
                    title="My Alert Dialog: Warning"
                    message="Are you sure?"
                    triggerName="Warning with checkbox"
                    hasButton
                    onConfirm={() => console.log("Your function here")}
                />
            </section>

            <section>
                <AlertDialogue
                    type="destructive"
                    title="My Alert Dialog: Destrcutive"
                    message="Are you sure?"
                    triggerName="Destructive button"
                    hasButton
                    typeCheck
                    checkTextValue="Write me"
                    onConfirm={() => console.log("Your function here")}
                />
            </section>

            <section>
                <AlertDialogue
                    type="confirm"
                    title="My Alert Dialog: Confirm"
                    message="Are you sure?"
                    triggerName="Confirm with checkbox"
                    hasButton
                    onConfirm={() => console.log("Your function here")}
                />
            </section>


            <section>
                <Dropdown>
                    <DropdownTrigger>Hello</DropdownTrigger>

                    <DropdownContent>
                        {Array.from({ length: 10 }, (_, i) => (
                            <DropdownItem key={i}>Item {i + 1}</DropdownItem>
                        ))}
                    </DropdownContent>
                </Dropdown>
            </section>


            <section
                style={{
                    width: "640px",
                    marginTop: "30px",
                }}
            >
                <h2>HLS Test</h2>

                <div
                    style={{
                        display: "flex",
                        gap: "8px",
                        marginBottom: "10px",
                    }}
                >
                    <input
                        type="text"
                        value={hlsUrl}
                        onChange={(event) => setHlsUrl(event.target.value)}
                        onKeyDown={(event) => {
                            if (event.key === "Enter") {
                                loadHls();
                            }
                        }}
                        placeholder="Enter HLS master.m3u8 URL"
                        style={{
                            flex: 1,
                            padding: "8px 10px",
                        }}
                    />

                    <button
                        type="button"
                        onClick={loadHls}
                        style={{
                            padding: "8px 14px",
                        }}
                    >
                        Load HLS
                    </button>
                </div>

                <video
                    ref={videoRef}
                    controls
                    playsInline
                    style={{
                        width: "100%",
                        display: "block",
                    }}
                />

                <div
                    style={{
                        display: "flex",
                        gap: "5px",
                        marginTop: "10px",
                    }}
                >
                    <button
                        onClick={() => changeResolution(-1)}
                        style={{
                            padding: "5px 10px",
                            fontWeight:
                                selectedLevel === -1 ? "bold" : "normal",
                        }}
                    >
                        Auto
                    </button>

                    {hlsLevels.map((level) => (
                        <button
                            key={level.index}
                            onClick={() => changeResolution(level.index)}
                            style={{
                                padding: "5px 10px",
                                fontWeight:
                                    selectedLevel === level.index
                                        ? "bold"
                                        : "normal",
                            }}
                        >
                            {Math.round(level.height)}p
                        </button>
                    ))}
                </div>
            </section>

            <section>
                <CreateCommunityBtn />
            </section>
        </section>
    );
};

export default TestPage;