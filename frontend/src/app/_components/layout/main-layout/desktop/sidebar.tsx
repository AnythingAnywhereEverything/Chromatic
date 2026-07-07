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
// OR usePathname if app router

const SidebarNavigator: React.FC = () => {
    const pathname = usePathname();
    const firstPathSegment = pathname.split("/")[1];

    const handleMouseEnter = (event: React.MouseEvent<HTMLDivElement>) => {
        const item = event.currentTarget;
        item.classList.add(style["hovered"]);
    };

    const handleMouseLeave = (event: React.MouseEvent<HTMLDivElement>) => {
        const item = event.currentTarget;
        item.classList.remove(style["hovered"]);
    };

    return (
        <nav 
            className={style["sidebar"]}
            onMouseEnter={handleMouseEnter}
            onMouseLeave={handleMouseLeave}
        >
            <div className={style["sidebar-logo"]}>
                <h1>Absolute Cinema</h1>
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
                <SidebarProfile
                    username="username"
                    profileImageUrl="https://placehold.co/40"
                    label="Profile"
                />
            </div>
        </nav>
    );
};

function SidebarProfile({
    username,
    profileImageUrl,
    label
}: {
    username: string;
    profileImageUrl: string;
    label: string;
}) {
    return (
        <div className={style["sidebar-profile"]}>
            <Link href={`/${username}`} className={style["profile-link"]}>
                <div className={style["profile-container"]}>
                    <img
                        src={profileImageUrl}
                        alt={`${username}'s profile`}
                        className={style["profile-image"]}
                        width={40}
                        height={40}
                    />
                </div>
                <span className={style["label"]}>{label}</span>
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
