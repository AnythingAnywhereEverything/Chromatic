"use client";

import {
    getPublicUserProfile,
    PublicUserProfileResponse,
} from "@/api/user/profile";
import { useEffect, useState } from "react";

import style from "./style.module.scss";
import { Banner } from "./header/banner";
import { Avatar } from "./header/avatar";
import { ViewProfile } from "@/app/_components/ui/chromatic/viewProfile";
import { useProfile } from "@/hooks/useProfile";
import {
    Dropdown,
    DropdownContent,
    DropdownItem,
    DropdownTrigger,
} from "@/app/_components/ui/chromatic/dropdown";
import { BsThreeDots } from "react-icons/bs";
import { followUser, unfollowUser } from "@/api/user/follow";

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

function parseStaticImage(url: string) {
    // remove the extension from the url
    if (url.startsWith("a_")) {
        const urlWithoutExtension = url.replace(/\.[^/.]+$/, "");
        return `${urlWithoutExtension}.png`;
    }
    return url;
}

function ProfileBanner({
    profile,
    isOwner,
}: {
    profile: PublicUserProfileResponse;
    isOwner: boolean;
}) {
    const [profileData, setProfileData] =
        useState<PublicUserProfileResponse | null>(null);

    if (!profile) {
        return <ProfileBannerSkeleton />;
    }

    const [following, setFollowing] = useState(false);
    useEffect(() => {
        if (profile) {
            setFollowing(profile.is_following);
        }
    }, [profile]);

    const handleFollowUser = async () => {
        setFollowing(!following);
        if (!following) {
            await followUser(profile.id);
        } else {
            await unfollowUser(profile.id);
        }
    };

    return (
        <div className={style["profile-header"]}>
            {profile ? (
                <>
                    <Banner
                        userId={profile.id}
                        banner={parseStaticImage(
                            profile.banner ? profile.banner : "",
                        )}
                        banner_thumbhash={profile.banner_thumbhash}
                    />
                    <div className={style["profile-info"]}>
                        <div className={style["profile-details-container"]}>
                            <Avatar profile={profile} />
                            <div className={style["profile-details"]}>
                                <h2>
                                    {profile.display_name
                                        ? profile.display_name
                                        : profile.username}
                                </h2>
                                <p>@{profile.username}</p>
                                <div className={style["profile-stats"]}>
                                    <p>
                                        <strong>{profile.posts_count}</strong>{" "}
                                        Posts
                                    </p>
                                    <p>
                                        <strong>
                                            {profile.followers_count}
                                        </strong>{" "}
                                        Followers
                                    </p>
                                    <p>
                                        <strong>
                                            {profile.following_count}
                                        </strong>{" "}
                                        Following
                                    </p>
                                </div>
                            </div>
                        </div>
                        {isOwner ? (
                            // if owner, show view profile button
                            <div className={style["profile-actions"]}>
                                <ViewProfile username={profile.username}>
                                    <button
                                        className={style["view-profile-button"]}
                                    >
                                        Edit Profile
                                    </button>
                                </ViewProfile>
                            </div>
                        ) : (
                            <div className={style["profile-actions"]}>
                                <Dropdown placement="bottom-end">
                                    <DropdownTrigger asChild>
                                        <button
                                            className={
                                                style["dropdown-trigger-button"]
                                            }
                                        >
                                            <BsThreeDots />
                                        </button>
                                    </DropdownTrigger>
                                    <DropdownContent>
                                        {!isOwner && (
                                            <>
                                                {following ? (
                                                    <DropdownItem>
                                                        <button
                                                            type="button"
                                                            onClick={
                                                                handleFollowUser
                                                            }
                                                        >
                                                            Unfollow User
                                                        </button>
                                                    </DropdownItem>
                                                ) : (
                                                    <DropdownItem>
                                                        <button
                                                            type="button"
                                                            onClick={
                                                                handleFollowUser
                                                            }
                                                        >
                                                            Follow User
                                                        </button>
                                                    </DropdownItem>
                                                )}
                                                <DropdownItem>
                                                    Report User
                                                </DropdownItem>
                                            </>
                                        )}
                                    </DropdownContent>
                                </Dropdown>
                            </div>
                        )}
                    </div>
                </>
            ) : (
                <ProfileBannerSkeleton />
            )}
        </div>
    );
}

export default ProfileBanner;
