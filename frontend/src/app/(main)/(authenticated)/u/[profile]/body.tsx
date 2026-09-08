"use client";
import { useEffect, useState } from "react";
import {
    PublicUserProfileResponse,
} from "@/api/user/profile";
import ProfileBanner from "./profileBanner";
import { useProfile } from "@/hooks/useProfile";
import PostGroup from "./posts/postgroup";
import { ProfileInfo } from "./profileInfo/profileinfo";

import style from "./style.module.scss";

interface ProfileBodyProps {
    profile: PublicUserProfileResponse;
}

export default function ProfileBody({ profile: initialProfile }: ProfileBodyProps) {
    const userProfile = useProfile();
    const [profile, setProfile] = useState(initialProfile);
    const isOwner = userProfile?.data?.id === profile?.id;

    // watch for profile service data update
    useEffect(() => {
        if (isOwner) {

            setProfile(userProfile?.data ?? initialProfile);
        }
    }, [userProfile?.data]);

    if (!profile) {
        return null;
    }

    return (
        <div>
            <ProfileBanner profile={profile} isOwner={isOwner} />
            <div className={style["profile-main-container"]}>
                <ProfileInfo profile={profile} isOwner={isOwner} /> 
                <PostGroup profile={profile} isOwner={isOwner} /> 
            </div>
        </div>
    );
}
