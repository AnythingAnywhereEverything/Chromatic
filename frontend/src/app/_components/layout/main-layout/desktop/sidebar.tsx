"use client";

import React from "react";
import style from "./style.module.scss";
import {
    HiOutlineUserGroup,
    HiMiniUserGroup,
    HiMiniChatBubbleLeftRight,
    HiMiniHome,
} from "react-icons/hi2";
import { FaBell, FaCompass, FaRegBell } from "react-icons/fa6";
import { usePathname } from "next/navigation"; // pages router
import Link from "next/link";
import { useUser } from "@/hooks/useUser";
import { Image } from "@/app/_components/ui/chromatic/Image";
import { UserIdAvatar } from "@/app/_components/ui/chromatic/initialAvatar";
// OR usePathname if app router

const SidebarNavigator: React.FC = () => {
    const pathname = usePathname();
    const firstPathSegment = pathname.split("/")[1];

    
    const user = useUser();
    
    if (!user || !user.data) {
        return null;
    }
    const profile_path = `/u/${user.data.username}`;

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
                    icon={<HiMiniHome />}
                    active={firstPathSegment === ""}
                    label="Home"
                    href="/"
                />
                <SidebarItem
                    icon={<FaCompass style={{ width: 22, height: 22 }} />}
                    active={firstPathSegment === "explore"}
                    label="Explore"
                    href="/explore"
                />
                <SidebarItem
                    icon={<HiMiniUserGroup />}
                    active={firstPathSegment === "groups"}
                    label="Groups"
                    href="/groups"
                />
                <SidebarItem
                    icon={<HiMiniChatBubbleLeftRight />}
                    active={firstPathSegment === "messages"}
                    label="Messages"
                    href="/messages"
                />
                <SidebarItem
                    icon={<FaBell />}
                    active={firstPathSegment === "notifications"}
                    label="Notifications"
                    href="/notifications"
                />
            </div>

            <div className={style["sidebar-footer"]}>
                <SidebarProfile
                    userData={user.data}
                    active={firstPathSegment === profile_path.split("/")[1]}
                />
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

function SidebarProfile({
    active,
    userData,
}: {
    active: boolean;
    userData: any;
}) {
    const { id: userId, username, avatar, avatar_thumbhash } = userData;

    // construct the avatar URL using the userId and avatar hash
    const avatarUrl = `avatars/${userId}/${parseStaticImage(avatar || "")}`;

    return (
            <Link href={`/u/${username}`} className={`${style["sidebar-profile"]} ${active ? style["active"] : ""}`}>
                <div className={style["profile-container"]}>
                    {avatar ? (
                        <Image
                            src={avatarUrl}
                            animated_src={avatarUrl.replace(".png", ".webp")}
                            alt={`${username}'s profile`}
                            thumbhash={avatar_thumbhash || undefined}
                            className={style["profile-image"]}
                            // scale up 4x for sharper quality
                            width={160}
                            height={160}
                            containerWidth={40}
                            containerHeight={40}
                        />
                    ) : (
                        <UserIdAvatar
                            userId={userId}
                            name={username}
                            size={40}
                        />
                    )}
                </div>
                <span className={style["label"]}>Profile</span>
            </Link>
    );
}

function getFirstPathSegment(pathname: string): string {
    const segments = pathname.split("/").filter(Boolean);
    return segments.length > 0 ? segments[0] : "";
}

interface SidebarItemProps {
    icon: React.ReactNode;
    active: boolean;
    label: string;
    href: string;
}

const SidebarItem: React.FC<SidebarItemProps> = ({
    icon,
    active,
    label,
    href,
}) => {
    return (
        <Link
            href={href}
            className={`${style["item"]} ${active ? style["active"] : ""}`}
        >
            <div className={style["icon-container"]}>{icon}</div>

            <span className={style["label"]}>{label}</span>
        </Link>
    );
};

export default SidebarNavigator;
