"use client";

import {
    IoBookmarkOutline,
    IoClipboardOutline,
    IoClose,
    IoLogoFacebook,
    IoLogoTwitter,
} from "react-icons/io5";
import style from "./interaction.module.scss";
import { LuShare2, LuThumbsUp } from "react-icons/lu";
import { TogglePostLike } from "@/api/post/like";
import { useState } from "react";
import React, { useRef } from "react";
import {
    Dialog,
    DialogClose,
    DialogContent,
    DialogHeading,
    DialogTrigger,
} from "../../dialogue";
import { Tooltip, TooltipContent, TooltipTrigger } from "../../tooltip";
import { MdOutlineChatBubbleOutline } from "react-icons/md";
import { base64ToUrlBase64, bnToB64 } from "@lib/base64";
import { FaLink } from "react-icons/fa";
import { FaXTwitter } from "react-icons/fa6";

interface BottomPostInteractionProps {
    username: string;
    post_id: string;
    is_liked: boolean;
    total_likes: number;
    total_comments: number;
}

const BottomPostInteraction = React.memo(function BottomPostInteraction({
    username,
    post_id,
    is_liked,
    total_likes,
    total_comments,
}: BottomPostInteractionProps) {
    const [likeState, setLikeState] = useState(is_liked);
    const [likeCount, setLikeCount] = useState(total_likes);

    const handleLike = async () => {
        try {
            const nextLikeState = !likeState;
            setLikeCount(likeCount + (nextLikeState ? 1 : -1));
            setLikeState(nextLikeState);
            await TogglePostLike(post_id, nextLikeState);
        } catch (error) {
            setLikeCount(likeCount + (likeState ? -1 : 1));
            setLikeState(!likeState);
        }
    };

    return (
        <section>
            <div className={style["bottom-container"]}>
                <section className={style["interaction"]}>
                    <InteractButton
                        name="Like"
                        className={`${style["button"]} ${style["like-button"]}`}
                        onClick={handleLike}
                    >
                        <i>
                            <LuThumbsUp />
                        </i>
                        {likeCount > 0 ? likeCount : null}
                    </InteractButton>
                    <InteractButton
                        name="Comment"
                        className={`${style["button"]} ${style["comment-button"]}`}
                    >
                        <i>
                            <MdOutlineChatBubbleOutline />
                        </i>
                        {total_comments > 0 ? total_comments : null}
                    </InteractButton>
                </section>
                <section className={style["interaction"]}>
                    <InteractButton
                        name="Share"
                        className={`${style["button"]} ${style["share-button"]}`}
                    >
                        <DialogSharePost
                            username={username}
                            post_id={post_id}
                        />
                    </InteractButton>
                    <InteractButton
                        name="Bookmark"
                        className={`${style["button"]} ${style["bookmark-button"]}`}
                    >
                        <i>
                            <IoBookmarkOutline />
                        </i>
                    </InteractButton>
                </section>
            </div>
        </section>
    );
});

type InteractButtonTooltip = {
    name: string;
    children: React.ReactNode;
} & React.HTMLAttributes<HTMLButtonElement>;

function InteractButton({ name, children, ...props }: InteractButtonTooltip) {
    return (
        <Tooltip openDelayDuration={500}>
            <TooltipTrigger {...props}>{children}</TooltipTrigger>
            <TooltipContent>
                <p>{name}</p>
            </TooltipContent>
        </Tooltip>
    );
}

type PopupInteractiveButtonTypes = {
    name: string;
    children: React.ReactNode;
    open?: boolean;
    onOpenChange?: (open: boolean) => void;
} & React.HTMLAttributes<HTMLButtonElement>;

function PopupInteractiveButton({
    name,
    children,
    open,
    onOpenChange,
    ...props
}: PopupInteractiveButtonTypes) {
    return (
        <Tooltip
            openDelayDuration={500}
            placement="top"
            open={open}
            onOpenChange={onOpenChange}
        >
            <TooltipTrigger asChild {...props}>
                {children}
            </TooltipTrigger>
            <TooltipContent className={style["popup-interactive-tooltip"]}>
                <p>{name}</p>
            </TooltipContent>
        </Tooltip>
    );
}

interface DialogSharePostProps {
    username: string;
    post_id: string;
}

function DialogSharePost({ username, post_id }: DialogSharePostProps) {
    // make base 64 from i64 post_id using bigint and encode from bigint
    const route = base64ToUrlBase64(bnToB64(BigInt(post_id)));
    const routeLink = `${window.location.origin}/share/p/${route}`;

    const shareToFacebook = `https://www.facebook.com/sharer/sharer.php?u=${routeLink}`;
    const shareToTwitter = `https://twitter.com/intent/tweet?url=${routeLink}`;
    const [isCopied, setIsCopied] = useState(false);
    const [isCopyOpen, setIsCopyOpen] = useState(false);

    async function copyToClipBoard() {
        await navigator.clipboard.writeText(routeLink);
        setIsCopied(true);
        setTimeout(() => setIsCopied(false), 1000);
    }

    const handleCopyLinkHover = () => {
        setIsCopyOpen(true);
    };

    const handleCopyLinkLeave = () => {
        // set isCopyOpen to false when no longer hovering over the copy link button
        setIsCopyOpen(false);
    };

    return (
        <Dialog overlayClassName={style["share-dialog-overlay"]}>
            <DialogTrigger asChild>
                <i>
                    <LuShare2 />
                </i>
            </DialogTrigger>

            <DialogContent className={style["share-container"]}>
                <DialogHeading className={style["share-header"]}>
                    <p className={style["share-title"]}>Share</p>
                    <DialogClose>
                        <IoClose />
                    </DialogClose>
                </DialogHeading>
                <div className={style["share-divider"]} />
                <main className={style["share-main"]}>
                    <p>Share this post with your friends!</p>
                    <p>
                        This area is marked as working in progress until friend
                        and chat feature is complete.
                    </p>
                </main>
                <div className={style["share-divider"]} />
                <footer className={style["share-footer"]}>
                    <PopupInteractiveButton
                        name={isCopied ? "Link copied!" : "Copy link"}
                        open={(isCopied && isCopyOpen) || isCopyOpen}
                        onOpenChange={setIsCopyOpen}
                    >
                        <button
                            className={style["button"]}
                            type="button"
                            onClick={copyToClipBoard}
                            onMouseEnter={handleCopyLinkHover}
                            onMouseLeave={handleCopyLinkLeave}
                        >
                            <FaLink />
                        </button>
                    </PopupInteractiveButton>
                    <PopupInteractiveButton name="Share to Facebook">
                        <a
                            className={style["button"]}
                            href={shareToFacebook}
                            target="_blank"
                            rel="noopener noreferrer"
                        >
                            <IoLogoFacebook />
                        </a>
                    </PopupInteractiveButton>
                    <PopupInteractiveButton name="Share to X(Formally Twitter)">
                        <a
                            className={style["button"]}
                            href={shareToTwitter}
                            target="_blank"
                            rel="noopener noreferrer"
                        >
                            <FaXTwitter />
                        </a>
                    </PopupInteractiveButton>
                </footer>
            </DialogContent>
        </Dialog>
    );
}

export default BottomPostInteraction;
