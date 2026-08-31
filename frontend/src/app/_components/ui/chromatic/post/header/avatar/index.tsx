import React from "react";
import { useState } from "react";
import { Image } from "../../../Image";
import { UserIdAvatar } from "../../../initialAvatar";
interface PostAvatarProps {
    userId: string;
    username: string;
    displayName: string;
    avatar: string | null;
    thumbhash: string | null;
    width?: number;
    height?: number;
    className?: string;
}

const PostAvatar = ({
    userId,
    avatar,
    thumbhash,
    username,
    displayName,
    width,
    height,
    className,
}: PostAvatarProps) => {

    if (avatar) {
        const avatarSrc = `avatars/${userId}/${avatar.startsWith("a_") 
            ? avatar.replace(".webp", ".png") 
            : avatar}`;
        console.log("avatarSrc", avatarSrc);
        return (
            <Image
                src={avatarSrc}
                width={width || 40}
                height={height || 40}
                containerWidth={width || 40}
                containerHeight={height || 40}
                className={className}
                thumbhash={thumbhash || undefined} 
                draggable={false}
            />
        );
    } else {
        return (
            <UserIdAvatar
                size={width}
                userId={userId}
                className={className}
                name={displayName || username}
            />
        )
    }
};
        
export {PostAvatar}
