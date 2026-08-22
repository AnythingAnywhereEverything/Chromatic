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
import { ViewProfile } from "@/app/_components/ui/chromatic/viewProfile";
import { useProfile } from "@/hooks/useProfile";

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

    const userProfile = useProfile();

    useEffect(() => {
        const fetchProfile = async () => {
            const profileOf = params.profile;
            const response = await getPublicUserProfile(profileOf);

            // if owner, use profile from user service
            if (getCacheUserId() === response?.id && userProfile?.data) {
                console.log("Using profile from user service");
                setProfile(userProfile.data);
                return;
            }

            setProfile(response);
        };
        fetchProfile();
    }, [params.profile, userProfile?.data]); // refetch when the profile param changes or when the user profile data changes

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
                            profile={profile}
                        />
                        <div className={style["profile-details"]}>
                            <h2>{profile.display_name ? profile.display_name : profile.username}</h2>
                            <p>{profile.username}</p>
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
                        <ViewProfile username={profile.username}>
                            <button>
                                edit profile
                            </button>
                        </ViewProfile>
                    </div>
                </>
            ) : (
                <ProfileBannerSkeleton />
            )}
        </div>
    );
}

export default ProfileBanner;
