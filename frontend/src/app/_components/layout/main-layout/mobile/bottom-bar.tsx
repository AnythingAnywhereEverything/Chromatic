"use client";

import {
    IoHome,
    IoCompass,
    IoHomeOutline,
    IoCompassOutline,
    IoChatboxEllipses,
    IoChatboxEllipsesOutline,
} from "react-icons/io5";
import Link from "next/link";
import { FaPlus } from "react-icons/fa6";
import { HiMiniUserGroup, HiOutlineUserGroup } from "react-icons/hi2";
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
                icon={<IoHomeOutline />}
                iconEnabled={<IoHome />}
                active={firstPathSegment === ""}
            />
            <BottomBarButtons
                href="/explore"
                icon={<IoCompassOutline />}
                iconEnabled={<IoCompass />}
                active={firstPathSegment === "explore"}
            />
            <BottomBarButtons
                href="/groups"
                icon={<HiOutlineUserGroup />}
                iconEnabled={<HiMiniUserGroup />}
                active={firstPathSegment === "groups"}
            />
            <BottomBarButtons
                href="/create"
                icon={<FaPlus />}
                iconEnabled={<FaPlus />}
                active={firstPathSegment === "create"}
            />
            <BottomBarButtons
                href="/messages"
                icon={<IoChatboxEllipsesOutline />}
                iconEnabled={<IoChatboxEllipses />}
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

    // construct the avatar URL using the userId and avatar hash
    const avatarUrl = `avatars/${userId}/${parseStaticImage(avatar || "")}`;

    return (
        <Link href={`/u/${username}`} className={style["profile-link"]}>
            {avatar ? (
                <Image
                    src={avatarUrl}
                    animated_src={avatarUrl.replace(".png", ".webp")}
                    alt={`${username}'s profile`}
                    thumbhash={avatar_thumbhash || undefined}
                    className={style["profile-image"]}
                    width={32}
                    height={32}
                    containerWidth={32}
                    containerHeight={32}
                />
            ) : (
                <UserIdAvatar userId={userId} name={username} size={32} />
            )}
        </Link>
    );
}

interface BottomBarButtonsProps {
    href: string;
    icon: React.ReactNode;
    iconEnabled: React.ReactNode;
    active: boolean;
}

function BottomBarButtons({
    href,
    icon,
    iconEnabled,
    active,
}: BottomBarButtonsProps) {
    return (
        <Link href={href} className={style["item"]}>
            {active ? iconEnabled : icon}
        </Link>
    );
}
