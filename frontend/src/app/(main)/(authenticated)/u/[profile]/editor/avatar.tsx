import { PublicUserProfileResponse } from "@/api/user/profile";
import React from "react";

import style from "./avatar.module.scss";
import { ChromaImage } from "@/app/_components/ui/chromatic/chromaImage";
import { UserIdAvatar } from "@/app/_components/ui/chromatic/initialAvatar";
import { Image } from "@/app/_components/ui/chromatic/Image";

/**
 * prefix with "a_" indicates animated avatar
 */
function isAvatarAnimated(avatar: string): boolean {
    return avatar.startsWith("a_");
}

export function Avatar({
    is_owner,
    profile,
    setProfile,
}: {
    is_owner: boolean;
    profile: PublicUserProfileResponse;
    setProfile: React.Dispatch<
        React.SetStateAction<PublicUserProfileResponse | null>
    >;
}) {
    const [avatarSrc, setAvatarSrc] = React.useState<string | null>(null);

    React.useEffect(() => {
        if (profile && profile.avatar) {
            setAvatarSrc(`avatars/${profile.id}/${profile.avatar}`);
        }
    }, [profile]);

    return (
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
                setAvatarSrc(`avatars/${profile.id}/${profile.avatar}`);
            }}
        >
            {profile.avatar && avatarSrc ? (
                <Image
                    style={{ width: "180px", height: "180px" }}
                    src={avatarSrc}
                    alt={`${profile.display_name || profile.username}'s avatar`}
                    thumbhash={profile.avatar_thumbhash || undefined}
                    size={180}
                    optimizationType="animated_in_viewport"
                />
            ) : (
                <UserIdAvatar
                    userId={profile.id}
                    name={profile.display_name || profile.username}
                    size={180}
                />
            )}
        </div>
    );
}
