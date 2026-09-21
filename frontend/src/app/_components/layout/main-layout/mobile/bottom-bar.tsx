"use client";

import Link from "next/link";
import { FaCompass } from "react-icons/fa6";
import {
    HiMiniChatBubbleLeftRight,
    HiMiniHome,
    HiMiniUserGroup,
    HiPlus,
} from "react-icons/hi2";
import style from "./style.module.scss";
import { usePathname } from "next/navigation";
import { useUser } from "@/hooks/useUser";
import { UserIdAvatar } from "@/app/_components/ui/chromatic/initialAvatar";
import { Image } from "@/app/_components/ui/chromatic/Image";

export default function BottomBar() {
    const pathname = usePathname();

    const firstPathSegment = pathname.split("/").filter(Boolean)[0] || "";

    return (
        <div className={style["bottom-bar"]}>
            <BottomBarButtons
                href="/"
                icon={<HiMiniHome />}
                active={firstPathSegment === ""}
            />
            <BottomBarButtons
                href="/explore"
                // force resize since compass is bigger than the other icons
                icon={<FaCompass style={{ width: 22, height: 22 }} />}
                active={firstPathSegment === "explore"}
            />
            <BottomBarButtons
                href="/create"
                icon={<HiPlus />}
                active={firstPathSegment === "create"}
            />
            <BottomBarButtons
                href="/messages"
                icon={<HiMiniChatBubbleLeftRight />}
                active={firstPathSegment === "messages"}
            />
            <BottomBarProfile />
        </div>
    );
}

function parseStaticImage(url: string): string {
    // remove the extension from the url
    if (url.startsWith("a_")) {
        const urlWithoutExtension = url.replace(/\.[^/.]+$/, "");
        return `${urlWithoutExtension}.png`;
    }
    return url;
}

function BottomBarProfile() {
    const user = useUser();

    if (!user || !user.data) {
        return null;
    }
    const { id: userId, username, avatar, avatar_thumbhash } = user.data;

    const handlePointerDown = (e: React.PointerEvent<HTMLAnchorElement>) => {
        const button = e.currentTarget;
        const bar = button.parentElement;

        if (!bar) return;

        const buttonRect = button.getBoundingClientRect();
        const barRect = bar.getBoundingClientRect();

        const effect = document.createElement("span");

        effect.className = style["interaction-effect"];

        effect.style.left = `${buttonRect.left - barRect.left}px`;
        effect.style.top = `${buttonRect.top - barRect.top}px`;
        effect.style.width = `${buttonRect.width}px`;
        effect.style.height = `${buttonRect.height}px`;

        bar.appendChild(effect);

        effect.addEventListener("animationend", () => {
            effect.remove();
        });
    };

    // construct the avatar URL using the userId and avatar hash
    const avatarUrl = `avatars/${userId}/${parseStaticImage(avatar || "")}`;

    return (
        <Link href={`/u/${username}`} className={style["profile-link"]} onPointerDown={handlePointerDown}>
            {avatar ? (
                <Image
                    src={avatarUrl}
                    animated_src={avatarUrl.replace(".png", ".webp")}
                    alt={`${username}'s profile`}
                    thumbhash={avatar_thumbhash || undefined}
                    className={style["profile-image"]}
                    // Give image 4x higher resolution so it looks sharp on high-DPI screens
                    width={128}
                    height={128}
                    containerWidth={32}
                    containerHeight={32}
                />
            ) : (
                <UserIdAvatar
                    userId={userId}
                    name={username}
                    className={style["profile-image"]}
                    size={32}
                />
            )}
        </Link>
    );
}

interface BottomBarButtonsProps {
    href: string;
    icon: React.ReactNode;
    active: boolean;
}

function BottomBarButtons({
    href,
    icon,
    active,
}: BottomBarButtonsProps) {
    const handlePointerDown = (e: React.PointerEvent<HTMLAnchorElement>) => {
        const button = e.currentTarget;
        const bar = button.parentElement;

        if (!bar) return;

        const buttonRect = button.getBoundingClientRect();
        const barRect = bar.getBoundingClientRect();

        const effect = document.createElement("span");

        effect.className = style["interaction-effect"];

        effect.style.left = `${buttonRect.left - barRect.left}px`;
        effect.style.top = `${buttonRect.top - barRect.top}px`;
        effect.style.width = `${buttonRect.width}px`;
        effect.style.height = `${buttonRect.height}px`;

        bar.appendChild(effect);

        effect.addEventListener("animationend", () => {
            effect.remove();
        });
    };

    return (
        <Link
            href={href}
            className={`${style["item"]} ${active ? style["active"] : ""}`}
            onPointerDown={handlePointerDown}
        >
            {icon}
        </Link>
    );
}
