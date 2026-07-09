"use client";

import { Portal } from "@/app/_components/portal";
import { Tooltip, TooltipAnchor, TooltipContent, TooltipTrigger } from "@/app/_components/ui/tooltip";

import React from "react";

const TestPage = () => {
    const [inner, setInner] = React.useState(false);

    const showInner = (event: React.MouseEvent<HTMLButtonElement>) => {
        setInner(true);
    };

    const [rootContent, setRootContent] = React.useState<HTMLElement | null>(null);

    const innerRef = React.useRef<HTMLDivElement | null>(null);

    return (
        <section>
            <button onMouseOver={showInner} onMouseOut={() => setInner(false)}>
                Open Portal
            </button>
            {inner && <Portal target={innerRef.current}>Something</Portal>}
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
                    <div style={{ zIndex: 899, position: "fixed", top: 0, left: 0, backgroundColor: "white", padding: "10px" }}>
                        This is root content!
                    </div>
                </Portal>
            )}

            <div style={{ height: "300px" }}></div>

            <Tooltip>
                <TooltipTrigger asChild>
                    <button style={{width: "fit-content"}}>Hover me</button>
                </TooltipTrigger>
                <TooltipContent>
                    Hewoo
                </TooltipContent>
            </Tooltip>

            <section 
                className="longScreen"
                style={{backgroundColor: "lightgray", maxHeight: "300px", overflowY: "scroll" }}
            >
                <div style={{ height: "600px" }}></div>
                    <Tooltip>
                        <TooltipTrigger asChild>
                            <button style={{width: "fit-content"}}>Hover me</button>
                        </TooltipTrigger>
                        <TooltipContent>
                            Amazing
                        </TooltipContent>
                    </Tooltip>
            </section>

            <div>
                <Tooltip>
                    <TooltipTrigger>
                        <TooltipAnchor asChild>
                            <button>🔥</button>
                        </TooltipAnchor>
                        <button style={{width: "fit-content"}}>The tooltip will appear on the 🔥</button>
                    </TooltipTrigger>
                    <TooltipContent>
                        Amazing
                    </TooltipContent>
                </Tooltip>
            </div>
        </section>
    );
};

export default TestPage;
