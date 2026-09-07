"use client";
import {
    getPublicUserProfile,
    PublicUserProfileResponse,
} from "@/api/user/profile";
import ProfileBanner from "./profileBanner";
import { useState, useEffect } from "react";
import { useProfile } from "@/hooks/useProfile";
import PostGroup from "./posts/postgroup";
import { ProfileInfo } from "./profileInfo/profileinfo";

import style from "./style.module.scss";

interface ProfileBodyProps {
    params: {
        profile: string;
    };
}

export default function ProfileBody({ params }: ProfileBodyProps) {
    const [profile, setProfile] = useState<PublicUserProfileResponse | null>(
        null,
    );
    const userProfile = useProfile();
    const [isOwner, setIsOwner] = useState(false);

    useEffect(() => {
        const fetchProfile = async () => {
            const profileOf = params.profile;
            const response = await getPublicUserProfile(profileOf);

            // if owner, use profile from user service
            if (userProfile?.data?.id === response?.id && userProfile?.data) {
                console.log("Using profile from user service");
                setProfile(userProfile.data);
                setIsOwner(true);
                return;
            }

            setProfile(response);
        };
        fetchProfile();
    }, [params.profile, userProfile?.data]);

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
