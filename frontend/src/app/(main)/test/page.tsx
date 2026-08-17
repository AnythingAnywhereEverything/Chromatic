"use client";

import { Portal } from "@/app/_components/portal";
import { AlertDialogue} from "@/app/_components/ui/chromatic/confirmation";
import { CreateCommunityBtn } from "@/app/_components/ui/chromatic/createCom";
import { CreatePost } from "@/app/_components/ui/chromatic/createPost";
import { PostStatus, PostVisibility } from "@/app/_components/ui/chromatic/createPost/status";
import {  Dialog, DialogClose, DialogContent, DialogDescription, DialogHeading,DialogTrigger } from "@/app/_components/ui/chromatic/dialogue";
import { Dropdown, DropdownContent, DropdownItem, DropdownTrigger } from "@/app/_components/ui/chromatic/dropdown";
import { Post } from "@/app/_components/ui/chromatic/post";
import {
    Tooltip,
    TooltipAnchor,
    TooltipArrow,
    TooltipContent,
    TooltipTrigger,
} from "@/app/_components/ui/chromatic/tooltip";

import React, { useState } from "react";

const TestPage = () => {
    const [inner, setInner] = React.useState(false);

    const showInner = (event: React.MouseEvent<HTMLButtonElement>) => {
        setInner(true);
    };

    const [rootContent, setRootContent] = React.useState<HTMLElement | null>(
        null,
    );

    const innerRef = React.useRef<HTMLDivElement | null>(null);
    const [testStatus, setTestStatus] = useState<PostVisibility>(PostVisibility.Everyone);
    
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
                    <div
                        style={{
                            zIndex: 899,
                            position: "fixed",
                            top: 0,
                            left: 0,
                            backgroundColor: "white",
                            padding: "10px",
                        }}
                    >
                        This is root content!
                    </div>
                </Portal>
            )}

            <div style={{ height: "300px" }}></div>

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
                <div style={{ height: "600px" }}></div>
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
                    <DialogDescription>My dialog description</DialogDescription>
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
                <CreatePost
                ownerId="1"
                ownerName="Username"
                ownerPfp="#"
                />
            </section>

            <section>
                {/* <PostStatus
                    visibility={testStatus}
                    onChange={(value) => setStatus(value)}
                /> */}
            </section>
            <section>
                <Dropdown>
                    <DropdownTrigger>
                        Hello
                    </DropdownTrigger>
                    <DropdownContent>
                        {Array.from({ length: 10 }, (_, i) => (
                          <DropdownItem key={i}>Item {i + 1}</DropdownItem>
                        ))}
                    </DropdownContent>
                </Dropdown>
            </section>

            <section>
                <PostStatus
                visibility={testStatus}
                onChange={(value) => setTestStatus(value)}
                />
            </section>
            {/* Testing purpose */}
            <section style={{width: "640px"}}>
              <Post
                id={""}
                ownerId={""}
                ownerName={""}
                content={`
                    Lorem ipsum dolor sit amet, consectetur adipiscing elit. Morbi at consequat sem. Donec tincidunt auctor nisl iaculis ultrices. Maecenas a velit purus. Nulla sit amet sem nulla. Morbi tincidunt purus sit amet eros bibendum, quis lobortis eros molestie. In accumsan bibendum velit non dapibus. Cras id varius lacus. Donec faucibus eget lorem mollis rutrum. Donec ante quam, pretium eget felis vitae, pulvinar efficitur nisi. Phasellus eget risus sollicitudin, posuere enim nec, placerat eros. Praesent maximus bibendum velit ut pretium. Praesent arcu turpis, euismod eu ex porta, scelerisque iaculis est. Nulla semper felis tortor, ut mattis elit viverra id. Duis vel hendrerit sem.
                    Nullam posuere lectus nec lectus tempor dignissim. Nulla magna tortor, facilisis ac nisl vel, rutrum congue neque. Fusce a justo porta, venenatis quam et, euismod lorem. Duis sit amet maximus nunc. Quisque sed mattis mauris, non gravida metus. Ut faucibus erat lectus, eget tempus ex rutrum sed. Ut mollis ante nisl, et egestas risus sodales consectetur. Vivamus et arcu scelerisque, semper lorem et, feugiat tellus. Morbi sagittis eros sed iaculis consequat.
                    Etiam sit amet pulvinar lectus. Aenean consectetur libero sollicitudin feugiat consequat. Ut ante lorem, dignissim ut massa sit amet, lacinia varius est. Pellentesque id est vitae magna sagittis tincidunt vel in nibh. Suspendisse ac congue sem. Praesent at mauris turpis. Morbi eleifend facilisis metus.
                `}
                like={0}
                comment={[]} 
                bookmark={false}/>
            </section>

            <section>
                <CreateCommunityBtn/>
            </section>
        </section>
    );
};

export default TestPage;
