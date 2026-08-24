import { deletePost, mediaPostProps } from "@/api/post/getFeed";
import style from "./header.module.scss";
import { PostAvatar } from "@/app/_components/ui/chromatic/post/profile";
import { useRef } from "react";
import { formatdatemonthyear } from "@/app/_components/ui/chromatic/post/dataformat";
import {
  Dropdown,
  DropdownContent,
  DropdownItem,
  DropdownTrigger,
} from "@/app/_components/ui/chromatic/dropdown";
import { BsThreeDots } from "react-icons/bs";
import { getCacheUserId } from "@/handler/token_handler";

export default function PostHeader({
  id,
  user_id,
  username,
  display_name,
  avatar_path,
  avatar_thumbhash,
  updated_at,  
  current_user_id // for some reason it's null
}: mediaPostProps) {
  const rootRef = useRef<HTMLDivElement>(null);
  const userId = getCacheUserId();
  const handleDeletePost = async () => {
    try {
      console.log("Deleting post with ID:", id);
      await deletePost(id);
      // Optionally, you can add a callback to remove the post from the UI after deletion
    } catch (error) {
      console.error("Failed to delete post:", error);
    }
  };
  return (
    <section className={style["profile"]} key={user_id}>
      <div className={style["avatar"]}>
        <PostAvatar
          userId={id}
          username={username}
          displayName={display_name}
          avatar={avatar_path}
          thumbhash={avatar_thumbhash}
          containerRef={rootRef}
        />
      </div>
      <div className={style["username"]}>
        <p>
          {display_name} {username}
        </p>
        <span>{formatdatemonthyear(updated_at)}</span>
      </div>
      <div className={style["option"]}>
        <Dropdown>
          <DropdownTrigger asChild>
            <BsThreeDots />
          </DropdownTrigger>
          {user_id === userId ? (
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
              <DropdownItem>Follow @{username}</DropdownItem>
              <DropdownItem>Add Friend @{username}</DropdownItem>
              <DropdownItem>Block @{username}</DropdownItem>
              <DropdownItem>Report</DropdownItem>
            </DropdownContent>
          )}
        </Dropdown>
      </div>
    </section>
  );
}
