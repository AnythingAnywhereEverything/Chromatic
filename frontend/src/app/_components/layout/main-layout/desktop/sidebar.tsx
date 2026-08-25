"use client";

import React from "react";
import style from "./style.module.scss";
import {
    IoCompass,
    IoCompassOutline,
    IoHome,
    IoHomeOutline,
    IoChatboxEllipsesOutline,
    IoChatboxEllipses,
} from "react-icons/io5";
import { HiOutlineUserGroup, HiMiniUserGroup } from "react-icons/hi2";
import { FaBell, FaRegBell } from "react-icons/fa6";
import { usePathname } from "next/navigation"; // pages router
import Link from "next/link";
import { useUser } from "@/hooks/useUser";
import { Image } from "@/app/_components/ui/chromatic/Image";
import { UserIdAvatar } from "@/app/_components/ui/chromatic/initialAvatar";
// OR usePathname if app router

const SidebarNavigator: React.FC = () => {
    const pathname = usePathname();
    const firstPathSegment = pathname.split("/")[1];
    // Custom hook to get user data

    const handleMouseEnter = (event: React.MouseEvent<HTMLDivElement>) => {
        const item = event.currentTarget;
        item.classList.add(style["hovered"]);
    };

    const handleMouseLeave = (event: React.MouseEvent<HTMLDivElement>) => {
        const item = event.currentTarget;
        item.classList.remove(style["hovered"]);
    };

    //get user theme for logo
    const userTheme =
        typeof window !== "undefined" ? localStorage.getItem("theme") : null;

    return (
        <nav
            className={style["sidebar"]}
            onMouseEnter={handleMouseEnter}
            onMouseLeave={handleMouseLeave}
        >
            <div className={style["sidebar-logo"]}>
                <Image
                    src={
                        userTheme === "dark"
                            ? "/asset/icon-light.png"
                            : "/asset/icon-dark.png"
                    }
                    no_cdn
                    alt="Absolute Cinema"
                    containerWidth={40}
                    containerHeight={40}
                />
            </div>

            <div className={style["sidebar-items"]}>
                <SidebarItem
                    icon={<IoHomeOutline />}
                    iconEnable={<IoHome />}
                    active={firstPathSegment === ""}
                    label="Home"
                    href="/"
                />
                <SidebarItem
                    icon={<IoCompassOutline />}
                    iconEnable={<IoCompass />}
                    active={firstPathSegment === "explore"}
                    label="Explore"
                    href="/explore"
                />
                <SidebarItem
                    icon={<HiOutlineUserGroup />}
                    iconEnable={<HiMiniUserGroup />}
                    active={firstPathSegment === "groups"}
                    label="Groups"
                    href="/groups"
                />
                <SidebarItem
                    icon={<IoChatboxEllipsesOutline />}
                    iconEnable={<IoChatboxEllipses />}
                    active={firstPathSegment === "messages"}
                    label="Messages"
                    href="/messages"
                />
                <SidebarItem
                    icon={<FaRegBell />}
                    iconEnable={<FaBell />}
                    active={firstPathSegment === "notifications"}
                    label="Notifications"
                    href="/notifications"
                />
            </div>

            <div className={style["sidebar-footer"]}>
                <SidebarProfile />
            </div>
        </nav>
    );
};

function parseStaticImage(url: string): string {
    // remove the extension from the url
    if (url.startsWith("a_")) {
        const urlWithoutExtension = url.replace(/\.[^/.]+$/, "");
        return `${urlWithoutExtension}.png`;
    }
    return url;
}

function SidebarProfile() {
    const user = useUser();

    if (!user || !user.data) {
        return null;
    }
    const { id: userId, username, avatar, avatar_thumbhash } = user.data;

    // construct the avatar URL using the userId and avatar hash
    const avatarUrl = `avatars/${userId}/${parseStaticImage(avatar || "")}`;

    return (
        <div className={style["sidebar-profile"]}>
            <Link href={`/u/${username}`} className={style["profile-link"]}>
                <div className={style["profile-container"]}>
                    {avatar ? (
                        <Image
                            src={avatarUrl}
                            animated_src={avatarUrl.replace(".png", ".webp")}
                            alt={`${username}'s profile`}
                            thumbhash={avatar_thumbhash || undefined}
                            className={style["profile-image"]}
                            width={44}
                            height={44}
                            containerWidth={44}
                            containerHeight={44}
                        />
                    ) : (
                        <UserIdAvatar
                            userId={userId}
                            name={username}
                            size={44}
                        />
                    )}
                </div>
                <span className={style["label"]}>Profile</span>
            </Link>
        </div>
    );
}

function getFirstPathSegment(pathname: string): string {
    const segments = pathname.split("/").filter(Boolean);
    return segments.length > 0 ? segments[0] : "";
}

interface SidebarItemProps {
    icon: React.ReactNode;
    iconEnable?: React.ReactNode;
    active: boolean;
    label: string;
    href: string;
}

const SidebarItem: React.FC<SidebarItemProps> = ({
    icon,
    iconEnable,
    active,
    label,
    href,
}) => {
    return (
        <div className={style["item"]}>
            <Link href={href}>
                <div className={style["icon-container"]}>
                    {active && iconEnable ? iconEnable : icon}
                </div>

                <span className={style["label"]}>{label}</span>
            </Link>
        </div>
    );
};

export default SidebarNavigator;
