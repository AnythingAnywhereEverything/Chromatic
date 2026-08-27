import React from "react";
import { useState } from "react";
import { Image } from "../../Image";
import { UserIdAvatar } from "../../initialAvatar";
interface PostAvatarProps {
    userId: string;
    username: string;
    displayName: string;
    avatar: string | null;
    thumbhash: string | null;
    containerRef: React.RefObject<HTMLDivElement | null>;
    containerWidth?: number
    containerHeight?: number
}

const PostAvatar = ({
    userId,
    avatar,
    thumbhash,
    username,
    displayName,
    containerRef,
    containerWidth,
    containerHeight
}: PostAvatarProps) => {
    if (!containerRef) {
        return null;
    }

    const [isInit, setIsInit] = useState(false);
    const [avatarInitWidth, setAvatarInitWidth] = useState(400);
    const [avatarInitHeight, setAvatarInitHeight] = useState(400);
    const [avatarContainerWidth, setAvatarContainerWidth] = useState(containerWidth || 40);
    const [avatarContainerHeight, setAvatarContainerHeight] = useState(containerHeight || 40);

    const handdleResize = () => {
        if (containerRef.current) {
            setAvatarContainerWidth(
                Math.floor(containerRef.current.offsetWidth),
            );
            setAvatarContainerHeight(
                Math.floor(containerRef.current.offsetWidth),
            );
        }
    };

    React.useEffect(() => {
            if (containerRef.current && !isInit) {
                setIsInit(true);
                setAvatarInitWidth(
                    Math.floor(containerRef.current.offsetWidth),
                );
                setAvatarInitHeight(
                    Math.floor(containerRef.current.offsetWidth),
                );
            }
    
            const handleResize = () => {
                if (containerRef.current) {
                    setAvatarContainerWidth(
                        Math.floor(containerRef.current.offsetWidth),
                    );
                    setAvatarContainerHeight(
                        Math.floor(containerRef.current.offsetWidth),
                    );
                }
            };
    
            handleResize();
    
            window.addEventListener("resize", handleResize);
            return () => {
                window.removeEventListener("resize", handleResize);
            };
        }, [containerRef.current]);

    if (avatar) {
        const avatarSrc = avatar.startsWith("a_") 
            ? avatar.replace(".webp", ".png") 
            : avatar;
        console.log(avatarSrc);
        return (
            <Image
                style={{borderRadius: "50%"}}
                src={avatarSrc}
                width={avatarInitWidth}
                height={avatarInitHeight}
                containerWidth={avatarContainerWidth}
                containerHeight={avatarContainerHeight}
                thumbhash={thumbhash || undefined} 
            />
        );
    } else {
        return (
            <UserIdAvatar
                size={avatarContainerWidth}
                userId={userId}
                name={displayName || username}
            />
        )
    }
};
        
export {PostAvatar}
