import { useState, useRef, useEffect } from "react";
import { Image } from "@/app/_components/ui/chromatic/Image";
import style from "./avatar.module.scss";
import { ImageEditor, imageUploadProps } from "../editor";
import { FaPen } from "react-icons/fa";
import { ImageProcessor } from "@lib/cropImage";
import {
    Dropdown,
    DropdownContent,
    DropdownItem,
    DropdownTrigger,
} from "../../dropdown";
import { UserIdAvatar } from "../../initialAvatar";

export type AvatarPayload = {
    file?: File;
    remove?: boolean;
};

interface AvatarProps {
    username: string;
    userId: string;
    is_owner?: boolean;
    avatar: string | null;
    blobUrl: string | null;
    thumbhash: string | null;
    onChange?: (payload: AvatarPayload) => void;
    // Top most parent container
    containerRef?: React.RefObject<HTMLDivElement | null>;
}

interface AvatarPreviewProps {
    username: string;
    userId: string;
    avatar: string | null;
    blobUrl: string | null;
    thumbhash: string | null;
    avatarContainerRef?: React.RefObject<HTMLDivElement | null>;
}
/// move function from below to preview
const AvatarPreview = ({
    username,
    userId,
    avatar,
    blobUrl,
    thumbhash,
    avatarContainerRef,
}: AvatarPreviewProps) => {
    if (!avatarContainerRef) {
        return null;
    }

    const [isInit, setIsInit] = useState(false);
    const [avatarInitWidth, setAvatarInitWidth] = useState(600);
    const [avatarInitHeight, setAvatarInitHeight] = useState(240);
    const [avatarContainerWidth, setAvatarContainerWidth] = useState(600);
    const [avatarContainerHeight, setAvatarContainerHeight] = useState(240);

    useEffect(() => {
        if (avatarContainerRef.current && !isInit) {
            setIsInit(true);
            setAvatarInitWidth(
                Math.floor(avatarContainerRef.current.offsetWidth),
            );
            setAvatarInitHeight(
                Math.floor(avatarContainerRef.current.offsetWidth),
            );
        }

        const handleResize = () => {
            if (avatarContainerRef.current) {
                setAvatarContainerWidth(
                    Math.floor(avatarContainerRef.current.offsetWidth),
                );
                setAvatarContainerHeight(
                    Math.floor(avatarContainerRef.current.offsetWidth),
                );
            }
        };

        handleResize();

        window.addEventListener("resize", handleResize);
        return () => {
            window.removeEventListener("resize", handleResize);
        };
    }, [avatarContainerRef.current]);

    if (blobUrl) {
        return (
            <img
                src={blobUrl}
                alt="Avatar"
                style={{
                    width: `${avatarContainerWidth}px`,
                    height: `${avatarContainerHeight}px`,
                }}
            />
        );
    } else if (avatar) {
        const avatarSrc = `avatars/${userId}/${avatar}`;
        const animatedSrc = () => {
            if (avatar.startsWith("a_")) {
                return avatarSrc.replace(".png", ".webp");
            }
            return undefined;
        };
        return (
            <Image
                src={avatarSrc}
                animated_src={animatedSrc()}
                alt="User Avatar"
                width={avatarInitWidth}
                height={avatarInitHeight}
                containerWidth={avatarContainerWidth}
                containerHeight={avatarContainerHeight}
                thumbhash={thumbhash || undefined}
                optimizationType={
                    animatedSrc()
                        ? "animated_in_viewport"
                        : "static"
                }
            />
        );
    } else {
        return (
            <UserIdAvatar
                size={avatarContainerWidth}
                userId={userId}
                name={username}
            />
        );
    }
};

const Avatar = ({
    userId,
    avatar,
    blobUrl,
    thumbhash,
    is_owner,
    onChange,
    username,
    containerRef,
}: AvatarProps) => {
    let avatarContainerRef = useRef<HTMLDivElement>(null);

    const [editorOpen, setEditorOpen] = useState(false);
    const [dropdownOpen, setDropdownOpen] = useState(false);
    const handleUpload = async (data: imageUploadProps) => {
        // crop the image and get the blob url
        const processor = await ImageProcessor.create();

        const result = await processor.transform(
            data.File,
            {
                type: "ratio",
                width: 1,
                height: 1,
                scale: data.Scale,
            },
            {
                x: data.PositionX,
                y: data.PositionY,
            },
        );
        const avatar_file = new File([result.final], data.File.name, {
            type: data.File.type,
        });
        if (onChange) {
            console.log("Avatar: handleUpload", avatar_file);
            onChange({ file: avatar_file, remove: false });
        }
    };

    const handleRemove = () => {
        if (onChange) {
            onChange({ file: undefined, remove: true });
        }
    };

    const handleReset = () => {
        if (onChange) {
            onChange({ file: undefined, remove: undefined });
        }
    }

    return (
        <div className={style["avatar-container"]} ref={avatarContainerRef}>
            <div className={style["avatar"]}>
                <AvatarPreview
                    username={username}
                    userId={userId}
                    avatar={avatar}
                    blobUrl={blobUrl}
                    thumbhash={thumbhash}
                    avatarContainerRef={avatarContainerRef}
                />
            </div>
            {is_owner && (
                <>
                    <ImageEditor
                        ratio={[1, 1]}
                        isOpen={editorOpen}
                        onOpenChange={setEditorOpen}
                        onUpload={handleUpload}
                        containerRef={containerRef}
                    />
                    <Dropdown
                        placement="right-end"
                        open={dropdownOpen}
                        onOpenChange={setDropdownOpen}
                        offsetPlacement={-25}
                    >
                        <DropdownTrigger asChild>
                            <button
                                className={`${style["edit-overlay"]} ${
                                    dropdownOpen ? style["dropdown-open"] : ""
                                }`}
                                onClick={() => setDropdownOpen(!dropdownOpen)}
                            >
                                <div className={style["edit-button"]}>
                                    <FaPen />
                                </div>
                            </button>
                        </DropdownTrigger>
                        <DropdownContent className={style["dropdown-content"]}>
                            <DropdownItem>
                                <button
                                    className={style["edit-avatar-button"]}
                                    onClick={() => setEditorOpen(true)}
                                >
                                    Change Avatar
                                </button>
                            </DropdownItem>
                            {
                                blobUrl && (
                                    <DropdownItem>
                                        <button
                                            className={style["reset-avatar-button"]}
                                            onClick={handleReset}
                                        >
                                            Reset Avatar
                                        </button>
                                    </DropdownItem>
                                )
                            }
                            <DropdownItem>
                                <button
                                    className={style["remove-avatar-button"]}
                                    onClick={handleRemove}
                                >
                                    Remove Avatar
                                </button>
                            </DropdownItem>
                        </DropdownContent>
                    </Dropdown>
                </>
            )}
        </div>
    );
};

export default Avatar;
