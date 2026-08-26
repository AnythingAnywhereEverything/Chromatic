import { PublicUserProfileResponse } from "@/api/user/profile";

import style from "./avatar.module.scss";
import { UserIdAvatar } from "@/app/_components/ui/chromatic/initialAvatar";
import { Image } from "@/app/_components/ui/chromatic/Image";

/**
 * prefix with "a_" indicates animated avatar
 */
function isAvatarAnimated(avatar: string): boolean {
    return avatar.startsWith("a_");
}

export function Avatar({ profile }: { profile: PublicUserProfileResponse }) {
    return (
        <div className={style["profile-avatar"]}>
            {profile.avatar ? (
                <Image
                    style={{ width: "180px", height: "180px" }}
                    src={`avatars/${profile.id}/${profile.avatar}`}
                    animated_src={
                        isAvatarAnimated(profile.avatar)
                            ? `avatars/${profile.id}/${profile.avatar.replace(".png", ".webp")}`
                            : undefined
                    }
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
