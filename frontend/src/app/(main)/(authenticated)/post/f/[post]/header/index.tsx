import { deletePost, PostProps } from "@/api/post/getFeed";
import style from "./header.module.scss";
import { PostAvatar } from "@/app/_components/ui/chromatic/post/profile";
import { useRef } from "react";
import { formatdatemonthyear, formatSocialMediaDate } from "@/app/_components/ui/chromatic/post/dataformat";
import {
  Dropdown,
  DropdownContent,
  DropdownItem,
  DropdownTrigger,
} from "@/app/_components/ui/chromatic/dropdown";
import { BsThreeDots } from "react-icons/bs";
import { getCacheUserId } from "@/handler/token_handler";
import { GoDotFill } from "react-icons/go";

export default function PostHeader({
  ...media
}: PostProps) {
  const rootRef = useRef<HTMLDivElement>(null);
  const userId = getCacheUserId();
  const hasDisplayName = media.author.display_name || null;
  const handleDeletePost = async () => {
    try {
      console.log("Deleting post with ID:", media.post_id);
      await deletePost(media.post_id);
      // Optionally, you can add a callback to remove the post from the UI after deletion
    } catch (error) {
      console.error("Failed to delete post:", error);
    }
  };
  return (
    <section className={style["header"]}>
      <section className={style["profile-header"]} key={media.author.id}>
        <div className={style["avatar"]}>
          <PostAvatar
            userId={media.author.id}
            username={media.author.username}
            displayName={media.author.display_name}
            avatar={media.author.avatar}
            thumbhash={media.author.avatar_thumbhash}
            containerRef={rootRef}
          />
        </div>
        <div className={style["user-info"]}>
            <div className={style["username"]}>
              <p style={{fontSize: "var(--text-large)"}}>{media.author.display_name || media.author.username}</p>
            </div>
            <div className={style["post-date"]}>
              <p>
                {hasDisplayName && (
                  <>
                      <span>{media.author.username + " "}</span>
                      <GoDotFill size={8}  />
                  </>
                )}
              </p>
              <p style={{fontSize: "var(--text)"}}>{formatSocialMediaDate(media.updated_at)}</p>
            </div>
          </div>
        <div className={style["option"]}>
          <Dropdown>
            <DropdownTrigger asChild>
              <BsThreeDots />
            </DropdownTrigger>
            {media.author.id === userId ? (
              <DropdownContent>
                <DropdownItem>Edit Post</DropdownItem>
                <DropdownItem>
                  <button type="button" onClick={handleDeletePost}>
                    Delete Post
                  </button>
                </DropdownItem>
              </DropdownContent>
            ) : (
              <DropdownContent>
                <DropdownItem>Follow @{media.author.username}</DropdownItem>
                <DropdownItem>Add Friend @{media.author.username}</DropdownItem>
                <DropdownItem>Block @{media.author.username}</DropdownItem>
                <DropdownItem>Report</DropdownItem>
              </DropdownContent>
            )}
          </Dropdown>
        </div>
      </section>
    </section>
  );
}
