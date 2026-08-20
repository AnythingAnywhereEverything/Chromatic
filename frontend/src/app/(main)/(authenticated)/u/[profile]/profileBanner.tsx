"use client";

import {
    getPublicUserProfile,
    PublicUserProfileResponse,
} from "@/api/user/profile";
import { ChromaImage } from "@/app/_components/ui/chromatic/chromaImage";
import getIdColor from "@lib/getIdColor";
import { useEffect, useState } from "react";

import style from "./style.module.scss";
import React from "react";
import { UserIdAvatar } from "@/app/_components/ui/chromatic/initialAvatar";
import { getCacheUserId } from "@/handler/token_handler";
import { FaCamera } from "react-icons/fa";
import { EditAvatarPopup } from "./editor/avatarEditor";

function ProfileBannerSkeleton() {
    return (
        <div className="profile-banner-skeleton">
            <div className="banner-skeleton">
                <div className="profile-info-skeleton">
                    <div className="avatar-skeleton"></div>
                    <div className="profile-details-skeleton">
                        <div className="name-skeleton"></div>
                        <div className="bio-skeleton"></div>
                    </div>
                </div>
            </div>
        </div>
    );
}

interface AvatarUploadItem {
    id: string;
    file: File;
    name: string;
    avatar: string;
}

function Banner({
    userId,
    banner,
    banner_thumbhash,
    width,
    height,
}: {
    userId: string;
    banner: string | null;
    banner_thumbhash: string | null;
    width: number; // read width from image Container
    height: number; // read height from image Container
}) {
    console.log(
        "Banner props:",
        userId,
        banner,
        banner_thumbhash,
        width,
        height,
    );
    if (banner) {
        return (
            <ChromaImage
                className={style["profile-banner"]}
                src={`banners/${userId}/${banner}`}
                alt="User Banner"
                width={width}
                height={height}
                thumbhash={banner_thumbhash || undefined}
            />
        );
    } else {
        let backgroundColor = getIdColor(userId);
        return (
            <div
                className={style["default-banner"]}
                style={{
                    backgroundColor,
                    width: `${width}px`,
                    height: `${height}px`,
                }}
            >
                {/* Default banner content */}
            </div>
        );
    }
}

function isAvatarAnimated(avatar: string): boolean {
    // prefix with "a_" indicates animated avatar
    return avatar.startsWith("a_");
}

function ProfileBanner({ params }: { params: { profile: string } }) {
    const [profile, setProfile] = useState<PublicUserProfileResponse | null>(
        null,
    );
    const [avatarSrc, setAvatarSrc] = useState<string | null>(null);

    useEffect(() => {
        const fetchProfile = async () => {
            const profileOf = params.profile;
            const response = await getPublicUserProfile(profileOf);

            console.log("ProfileBanner response:", response);

            setProfile(response);
            setAvatarSrc(
                response?.avatar
                    ? `avatars/${response.id}/${response.avatar}`
                    : null,
            );
        };
        fetchProfile();
    }, [params.profile]);

    // create banner container ref
    const [bannerContainerWidth, setBannerContainerWidth] = useState(600);
    const [bannerContainerHeight, setBannerContainerHeight] = useState(240);
    let bannerContainerRef = React.createRef<HTMLDivElement>();

    useEffect(() => {
        const handleResize = () => {
            console.log(
                "Banner container size:",
                bannerContainerWidth,
                bannerContainerHeight,
            );
            if (bannerContainerRef.current) {
                setBannerContainerWidth(bannerContainerRef.current.offsetWidth);
                setBannerContainerHeight(
                    (bannerContainerRef.current.offsetWidth / 5) * 2,
                ); // maintain aspect ratio 5 / 2
            }
        };

        // Initial size
        handleResize();

        window.addEventListener("resize", handleResize);
        return () => {
            window.removeEventListener("resize", handleResize);
        };
    }, [bannerContainerRef]);

    if (!profile) {
        return <ProfileBannerSkeleton />;
    }

    const is_owner = getCacheUserId() === profile?.id;

    return (
        <div className={style["profile-header"]}>
            {profile ? (
                <>
                    <div
                        ref={bannerContainerRef}
                        className={style["profile-banner-container"]}
                    >
                        <Banner
                            userId={profile.id}
                            banner={profile.banner}
                            banner_thumbhash={profile.banner_thumbhash}
                            width={bannerContainerWidth}
                            height={bannerContainerHeight}
                        />
                        {is_owner && (
                            <button className={style["edit-banner-button"]}>
                                <FaCamera />
                                Edit Banner
                            </button>
                        )}
                    </div>
                    <div className={style["profile-info"]}>
                        <div
                            className={style["profile-avatar"]}
                            onMouseEnter={() => {
                                if (isAvatarAnimated(profile.avatar || "")) {
                                    // trim .png and replace with .webp for animated avatar
                                    const animatedAvatarSrc = `avatars/${profile.id}/${profile.avatar?.replace(
                                        ".png",
                                        ".webp",
                                    )}`;
                                    setAvatarSrc(animatedAvatarSrc);
                                }
                            }}
                            onMouseLeave={() => {
                                setAvatarSrc(
                                    `avatars/${profile.id}/${profile.avatar}`,
                                );
                            }}
                        >
                            {profile.avatar && avatarSrc ? (
                                <ChromaImage
                                    style={{ width: "180px", height: "180px" }}
                                    src={avatarSrc}
                                    alt={`${profile.display_name || profile.username}'s avatar`}
                                    thumbhash={
                                        profile.avatar_thumbhash || undefined
                                    }
                                    size={180}
                                />
                            ) : (
                                <UserIdAvatar
                                    userId={profile.id}
                                    name={
                                        profile.display_name || profile.username
                                    }
                                    size={180}
                                />
                            )}
                            {is_owner && (
                                <EditAvatarPopup
                                    onUpdate={(res) => {
                                        setAvatarSrc(
                                            `avatars/${res.id}/${res.avatar}`,
                                        );
                                        setProfile((prevProfile) => {
                                            if (prevProfile) {
                                                return {
                                                    ...prevProfile,
                                                    avatar: res.avatar,
                                                    avatar_thumbhash:
                                                        res.avatar_thumbhash,
                                                };
                                            }
                                            return prevProfile;
                                        });
                                    }}
                                />
                            )}
                        </div>
                        <div className={style["profile-details"]}>
                            <h2>{profile.username}</h2>
                            <p>{profile.display_name}</p>
                            <p>{profile.bio}</p>
                            <div className={style["profile-stats"]}>
                                <p>
                                    <strong>{0 /* Need migration */}</strong>{" "}
                                    Posts
                                </p>
                                <p>
                                    <strong>{profile.followers_count}</strong>{" "}
                                    Followers
                                </p>
                                <p>
                                    <strong>{profile.following_count}</strong>{" "}
                                    Following
                                </p>
                            </div>
                        </div>
                    </div>
                </>
            ) : (
                <ProfileBannerSkeleton />
            )}
        </div>
    );
}

export default ProfileBanner;
