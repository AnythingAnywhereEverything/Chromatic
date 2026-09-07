"use client";

import React, { useState } from "react";
import style from "./style.module.scss";
import {
    HiOutlineUserGroup,
    HiMiniUserGroup,
    HiMiniChatBubbleLeftRight,
    HiMiniHome,
} from "react-icons/hi2";
import { FaBell, FaCompass, FaRegBell } from "react-icons/fa6";
import { usePathname, useRouter } from "next/navigation"; // pages router
import Link from "next/link";
import { useUser } from "@/hooks/useUser";
import { Image } from "@/app/_components/ui/chromatic/Image";
import { UserIdAvatar } from "@/app/_components/ui/chromatic/initialAvatar";
import { UserSetting } from "@/app/_components/ui/chromatic/setting";
import { IoSettingsSharp } from "react-icons/io5";
import { IoMdMenu } from "react-icons/io";
import {
    Dropdown,
    DropdownContent,
    DropdownItem,
    DropdownTrigger,
} from "@/app/_components/ui/chromatic/dropdown";
import { logout } from "@/api/auth";
import { useAuthService } from "@/hooks/useAuthService";
// OR usePathname if app router

const SidebarNavigator: React.FC = () => {
    const pathname = usePathname();
    const firstPathSegment = pathname.split("/")[1];
    const profilePathSegment = pathname.slice(3); //
    const router = useRouter();
    const authService = useAuthService();

    const user = useUser();

    const [hovering, setHovering] = useState(false);
    const [settingOpen, setSettingOpen] = useState(false);
    const [menuOpen, setMenuOpen] = useState(false);

    if (!user || !user.data) {
        return null;
    }

    const profile_path = `/u/${user.data.username}`;

    const handleLogout = () => {
        try {
            authService.logout();
            router.push("/auth/signin");
        } catch (error) {
            console.error("Logout failed:", error);
        }
    };

    return (
        <nav
            className={`${hovering || menuOpen ? style["hovered"] : ""} ${style["sidebar"]}`}
            onMouseEnter={() => setHovering(true)}
            onMouseLeave={() => setHovering(false)}
        >
            <div className={style["sidebar-logo"]}>
                <Image
                    src="/asset/logo_no_border.svg"
                    no_cdn
                    alt="Absolute Cinema"
                    containerWidth={40}
                    containerHeight={40}
                />
            </div>

            <div className={style["sidebar-items"]}>
                <SidebarPageItem
                    icon={<HiMiniHome />}
                    active={firstPathSegment === ""}
                    label="Home"
                    href="/"
                />
                <SidebarPageItem
                    icon={<FaCompass style={{ width: 22, height: 22 }} />}
                    active={firstPathSegment === "explore"}
                    label="Explore"
                    href="/explore"
                />
                <SidebarPageItem
                    icon={<HiMiniUserGroup />}
                    active={firstPathSegment === "groups"}
                    label="Groups"
                    href="/groups"
                />
                <SidebarPageItem
                    icon={<HiMiniChatBubbleLeftRight />}
                    active={firstPathSegment === "messages"}
                    label="Messages"
                    href="/messages"
                />
                <SidebarPageItem
                    icon={<FaBell />}
                    active={firstPathSegment === "notifications"}
                    label="Notifications"
                    href="/notifications"
                />
            </div>

            <div className={style["sidebar-footer"]}>
                <SidebarProfile
                    userData={user.data}
                    active={profilePathSegment === user.data.username}
                />
                <Dropdown
                    placement="top"
                    open={menuOpen}
                    onOpenChange={(open) => {
                        setMenuOpen(open);
                    }}
                >
                    <DropdownTrigger
                        asChild
                        onClick={() => setMenuOpen(!menuOpen)}
                    >
                        <SidebarButtonItem
                            icon={<IoMdMenu />}
                            active={false}
                            label="Settings"
                        />
                    </DropdownTrigger>
                    <DropdownContent className={style["dropdown-content"]}>
                        <DropdownItem>
                            <button
                                className={style["dropdown-button"]}
                                onClick={() => {
                                    setMenuOpen(!menuOpen);
                                    setSettingOpen(true);
                                }}
                                onBlur={() => setHovering(false)}
                            >
                                Open Settings
                            </button>
                        </DropdownItem>
                        <DropdownItem>
                            <button
                                className={`${style["dropdown-button"]} ${style["dropdown-logout"]}`}
                                onClick={handleLogout}
                            >
                                Logout
                            </button>
                        </DropdownItem>
                    </DropdownContent>
                </Dropdown>
            </div>
            <UserSetting
                open={settingOpen}
                onOpenChange={(open) => {
                    console.log("UserSetting open state changed:", open);
                    setHovering(open);
                    setSettingOpen(open);
                }}
            />
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
        <Link
            href={`/u/${username}`}
            className={`${style["sidebar-profile"]} ${active ? style["active"] : ""}`}
        >
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
                    <UserIdAvatar userId={userId} name={username} size={40} />
                )}
            </div>
            <span className={style["label"]}>Profile</span>
        </Link>
    );
}

interface SidebarItemProps {
    icon: React.ReactNode;
    active: boolean;
    label: string;
    href: string;
}

const SidebarPageItem: React.FC<SidebarItemProps> = ({
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

type SidebarButtonItemProps = {
    icon: React.ReactNode;
    active: boolean;
    label: string;
} & React.ButtonHTMLAttributes<HTMLButtonElement>;

const SidebarButtonItem: React.FC<SidebarButtonItemProps> = ({
    icon,
    active,
    label,
    ...props
}) => {
    return (
        <button
            {...props}
            className={`${style["item"]} ${active ? style["active"] : ""}`}
        >
            <div className={style["icon-container"]}>{icon}</div>
            <span className={style["label"]}>{label}</span>
        </button>
    );
};

export default SidebarNavigator;
