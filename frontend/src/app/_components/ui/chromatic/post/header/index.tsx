interface PostHeaderProps {
    author: {
        id: string;
        username: string;
        display_name: string;
        avatar: string | null;
        avatar_thumbhash: string | null;
    };
    created_at: string;
    visibility: string;
    is_followed: boolean;
    onDelete: () => void;
}
import { useUser } from "@/hooks/useUser";
import { useEffect, useState } from "react";
import {
    formatSocialMediaDate,
    formatFullDateWithExactTime,
} from "../helpers/dateFormater";
import {
    Dropdown,
    DropdownContent,
    DropdownItem,
    DropdownTrigger,
} from "../../dropdown";

import style from "./style.module.scss";
import { BsThreeDots } from "react-icons/bs";
import { PostAvatar } from "./avatar";
import Link from "next/link";
import {
    Tooltip,
    TooltipArrow,
    TooltipContent,
    TooltipTrigger,
} from "../../tooltip";
import { FaGlobeAmericas, FaLock, FaUserFriends } from "react-icons/fa";
import { followUser, unfollowUser } from "@/api/user/follow";

const PostHeader: React.FC<PostHeaderProps> = ({
    author,
    created_at,
    visibility,
    is_followed,
    onDelete,
}) => {
    const currentUserId = useUser().data?.id;
    const hasDisplayName = author.display_name || null;

    // * all props should re-render on actions but this is not.
    const [isFollowing, setIsFollowing] = useState(is_followed);
    const handleFollow = async () => {
        console.log("Following user...");
        try {
            const data = await followUser(author.id);
            if (data) {
                console.log("Follow successful:", data);
                setIsFollowing(true);
            }
        } catch (error) {
            console.error(error);
        }
    };

    const handleUnfollow = async () => {
        console.log("Unfollowing user...");
        try {
            await unfollowUser(author.id);
            console.log("Unfollow successful");
            setIsFollowing(false);
        } catch (error) {
            console.error(error);
        }
    };

    return (
        <header className={style["header"]}>
            <Tooltip placement="top-start" allowHovering={true} offset={8}>
                <div className={style["author-container"]}>
                    <TooltipTrigger>
                        <PostAvatar
                            userId={author.id}
                            username={author.username}
                            displayName={author.display_name}
                            avatar={author.avatar}
                            thumbhash={author.avatar_thumbhash || ""}
                            className={style["avatar"]}
                            width={40}
                            height={40}
                        />
                    </TooltipTrigger>
                    <div className={style["post-info"]}>
                        <Link
                            href={`/u/${author.username}`}
                            className={style["username"]}
                        >
                            <p>{author.display_name || author.username}</p>
                        </Link>
                        <p className={style["post-meta"]}>
                            {hasDisplayName && (
                                <>
                                    <span>{author.username}</span>
                                    <span>•</span>
                                </>
                            )}
                            <Tooltip>
                                <TooltipTrigger>
                                    <span>
                                        {formatSocialMediaDate(created_at)}
                                    </span>
                                </TooltipTrigger>
                                <TooltipContent
                                    className={style["tooltip-content"]}
                                >
                                    <p>
                                        {formatFullDateWithExactTime(
                                            created_at,
                                        )}
                                    </p>
                                </TooltipContent>
                            </Tooltip>
                            <Tooltip>
                                <TooltipTrigger>
                                    <span>
                                        {visibility === "everyone" ? (
                                            <FaGlobeAmericas />
                                        ) : visibility === "friend" ? (
                                            <FaUserFriends />
                                        ) : (
                                            <FaLock />
                                        )}
                                    </span>
                                </TooltipTrigger>
                                <TooltipContent
                                    className={style["tooltip-content"]}
                                >
                                    <p>{visibility}</p>
                                </TooltipContent>
                            </Tooltip>
                        </p>
                    </div>
                </div>
                <TooltipContent>
                    <TooltipArrow />
                    <p>User: @{author.username}</p>
                </TooltipContent>
            </Tooltip>
            <div className={style["option"]}>
                <Dropdown>
                    <DropdownTrigger asChild>
                        <BsThreeDots />
                    </DropdownTrigger>
                    {author.id === currentUserId ? (
                        <DropdownContent>
                            <DropdownItem>Edit Post</DropdownItem>
                            <DropdownItem>
                                <button type="button" onClick={onDelete}>
                                    Delete Post
                                </button>
                            </DropdownItem>
                        </DropdownContent>
                    ) : (
                        <DropdownContent>
                            {!isFollowing ? (
                                <DropdownItem>
                                    <button
                                        type="button"
                                        onClick={handleFollow}
                                    >
                                        Follow {author.username}
                                    </button>
                                </DropdownItem>
                            ) : (
                                <DropdownItem>
                                    <button
                                        type="button"
                                        onClick={handleUnfollow}
                                    >
                                        Unfollow {author.username}
                                    </button>
                                </DropdownItem>
                            )}
                            <DropdownItem>
                                Block {author.username}
                            </DropdownItem>
                            <DropdownItem>Report</DropdownItem>
                        </DropdownContent>
                    )}
                </Dropdown>
            </div>
        </header>
    );
};

export default PostHeader;
