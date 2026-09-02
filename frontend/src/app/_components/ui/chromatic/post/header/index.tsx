interface PostHeaderProps {
    author: {
        id: string;
        username: string;
        display_name: string;
        avatar: string | null;
        avatar_thumbhash: string | null;
    };
    created_at: string;
    postId: string;
}
import { useRef } from "react";
import { useUser } from "@/hooks/useUser";
import { formatSocialMediaDate, formatFullDateWithExactTime } from "../helpers/dateFormater";
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
import { Tooltip, TooltipContent, TooltipTrigger } from "../../tooltip";
import { deletePost } from "@/api/post/getFeed";

const PostHeader: React.FC<PostHeaderProps> = ({ author, created_at, postId }) => {
    const currentUserId = useUser().data?.id;
    const hasDisplayName = author.display_name || null;

    const handleDeletePost = async () => {
        try {
            console.log("Deleting post with ID:", postId);
            await deletePost(postId);
        } catch (error) {
            console.error("Failed to delete post:", error);
        }
    };

    return (
        <header className={style["header"]}>
            <div className={style["author-container"]}>
                <div>
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
                </div>
                <div className={style["post-info"]}>
                    <Link href={`/u/${author.username}`} className={style["username"]}>
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
                                <span>{formatSocialMediaDate(created_at)}</span>
                            </TooltipTrigger>
                            <TooltipContent className={style["tooltip-content"]}>
                                <p>{formatFullDateWithExactTime(created_at)}</p>
                            </TooltipContent>
                        </Tooltip>
                    </p>
                </div>
            </div>
            <div className={style["option"]}>
                <Dropdown>
                    <DropdownTrigger asChild>
                        <BsThreeDots />
                    </DropdownTrigger>
                    {author.id === currentUserId ? (
                        <DropdownContent>
                            <DropdownItem>Edit Post</DropdownItem>
                            <DropdownItem>
                                <button 
                                type="button" 
                                onClick={handleDeletePost}>
                                    Delete Post
                                </button>
                            </DropdownItem>
                        </DropdownContent>
                    ) : (
                        <DropdownContent>
                            <DropdownItem>
                                Follow @{author.username}
                            </DropdownItem>
                            <DropdownItem>
                                Add Friend @{author.username}
                            </DropdownItem>
                            <DropdownItem>
                                Block @{author.username}
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
