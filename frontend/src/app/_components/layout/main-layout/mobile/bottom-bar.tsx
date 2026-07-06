"use client";

import { IoHome, IoCompass, IoHomeOutline, IoCompassOutline, IoChatboxEllipses, IoChatboxEllipsesOutline } from "react-icons/io5";
import Link from "next/link";
import { FaPlus } from "react-icons/fa6";
import { HiMiniUserGroup, HiOutlineUserGroup } from "react-icons/hi2";
import style from "./style.module.scss";
import { usePathname } from "next/navigation";

export default function BottomBar() {
    const pathname = usePathname();

    const firstPathSegment =
        pathname.split("/").filter(Boolean)[0] || "";

    const username = "username";

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
            <BottomBarProfile
                username={username}
                profileImageUrl="https://placehold.co/40"
            />
        </div>
    );
}

function getFirstPathSegment(pathname: string): string {
    const segments = pathname.split("/").filter(Boolean);
    return segments.length > 0 ? segments[0] : "";
}

interface BottomBarProfileProps {
    username: string;
    profileImageUrl: string;
}

function BottomBarProfile({ username, profileImageUrl }: BottomBarProfileProps) {
    return (
        <a href={`/${username}`} className={style["profile-link"]}>
            <img src={profileImageUrl} alt={`${username}'s profile`} className={style["profile-image"]} width={40} height={40} />
        </a>
    );
}

interface BottomBarButtonsProps {
    href: string;
    icon: React.ReactNode;
    iconEnabled: React.ReactNode;
    active: boolean;
}

function BottomBarButtons(
    { href, icon, iconEnabled, active }: BottomBarButtonsProps
) {
    return (
        <Link href={href} className={style["item"]}>
            {active ? iconEnabled : icon}
        </Link>
    );
}