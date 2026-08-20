"use client";

import {
    getPublicUserProfile,
    PublicUserProfileResponse,
} from "@/api/user/profile";
import { useEffect, useState } from "react";

import style from "./style.module.scss";
import { getCacheUserId } from "@/handler/token_handler";
import { Banner } from "./editor/banner";
import { Avatar } from "./editor/avatar";

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
    

    if (!profile) {
        return <ProfileBannerSkeleton />;
    }

    const is_owner = getCacheUserId() === profile?.id;

    return (
        <div className={style["profile-header"]}>
            {profile ? (
                <>
                    <Banner
                        userId={profile.id}
                        banner={profile.banner}
                        banner_thumbhash={profile.banner_thumbhash}
                        is_owner={is_owner}
                        setProfile={setProfile}
                    />
                    <div className={style["profile-info"]}>
                        <Avatar
                            is_owner={is_owner}
                            profile={profile}
                            setProfile={setProfile}
                        />
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
