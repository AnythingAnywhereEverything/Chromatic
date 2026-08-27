"use client";

import { commentProps, getCommentsOnPost } from "@/api/post/getFeed";
import style from "./comment.module.scss";
import React, { useState } from "react";
import EditableTextArea from "@/app/_components/ui/chromatic/textarea";
import { PostAvatar } from "@/app/_components/ui/chromatic/post/profile";
import {
  getCurrentProfile,
  getPublicUserProfile,
  PublicUserProfileResponse,
} from "@/api/user/profile";
import { getCacheUserId } from "@/handler/token_handler";
import { CiImageOn } from "react-icons/ci";
import EmojiPicker, { EmojiStyle, Theme } from "emoji-picker-react";
import { HiOutlineEmojiHappy } from "react-icons/hi";
import {
  FloatingFocusManager,
  useDismiss,
  useFloating,
  useInteractions,
} from "@floating-ui/react";
import {
  Dropdown,
  DropdownContent,
  DropdownItem,
  DropdownTrigger,
} from "@/app/_components/ui/chromatic/dropdown";
import { BsThreeDots } from "react-icons/bs";
import { formatSocialMediaDate } from "@/app/_components/ui/chromatic/post/dataformat";
import { GoComment, GoDotFill } from "react-icons/go";
import { MediaGroup } from "@/app/_components/ui/chromatic/post/mediagroup";
import { DialogSharePost } from "@/app/_components/ui/chromatic/post";
import { IoBookmarkOutline } from "react-icons/io5";
import { LuThumbsUp } from "react-icons/lu";
import { TogglePostLike } from "@/api/post/like";
import { FaRegPaperPlane } from "react-icons/fa6";
type CommentSectionProps = {
  postId: string;
};
type CommentPayload = {
  text: string;
  image?: File;
};
export const CommentSection = ({ postId }: CommentSectionProps) => {
  const [comments, setComments] = useState<commentProps[]>([]);

  React.useEffect(() => {
    async function getComments() {
      const res = await getCommentsOnPost(postId);
      console.log("This is all comments", res);
      setComments(res);
    }
    getComments();
  }, []);

  return (
    <section className={style["comment-layout"]}>
      <CreateComment />
      {/* for index,comment in comments */}
      {comments?.map((comment) => (
        <div key={comment.id}>
          <CommentFromUser {...comment} />
        </div>
      ))}
    </section>
  );
};

//  * -------------- CREATE COMMENT -----------------

const CreateComment: React.FC = () => {
  const [text, setText] = React.useState("");
  const [user, setUser] = React.useState<PublicUserProfileResponse | null>(
    null,
  );
  const maxLength = 1024;
  const remainingChars = maxLength - text.length;
  const progressPercentage = (text.length / maxLength) * 100;
  const [isOpenEmoji, setIsOpenEmoji] = React.useState(false);
  const [extended, isExtended] = React.useState(false);

  const { refs, context } = useFloating({
    open: isOpenEmoji,
    onOpenChange: setIsOpenEmoji,
  });

  const dismiss = useDismiss(context);
  const { getFloatingProps } = useInteractions([dismiss]);
  const ref = React.useRef<HTMLDivElement>(null);

  const textareaRef = React.useRef<HTMLTextAreaElement>(null);
  const warpperRef = React.useRef<HTMLDivElement>(null);

  const syncHeight = () => {
    const textarea = textareaRef.current;
    const wrapper = warpperRef.current;

    if (!textarea || !wrapper) {
      return;
    }
    textarea.style.height = "auto";
    wrapper.style.height = `${textarea.scrollHeight}px`;
    textarea.style.height = `${textarea.scrollHeight}px`;
  };
  React.useEffect(() => {
    syncHeight();
  }, [text]);

  React.useEffect(() => {
    const loadProfile = async () => {
      const current = await getCurrentProfile();
      setUser(current);
    };
    loadProfile();
  }, []);

  if (!user) {
    return;
  }

  return (
    <section className={style["create-comment"]}>
      <h3>Create Comment</h3>
      <section>
        <form action="#" className={style["container"]}>
          <section 
          className={style["comment-header"]}
          data-state={!extended}
          >
            <div className={style["profile"]} key={user.id}>
              <PostAvatar
                userId={user.id}
                username={user.username}
                displayName={user.display_name}
                avatar={`/avatars/${user.id}/${user.avatar}`}
                thumbhash={user.avatar_thumbhash}
                containerRef={ref}
              />
            </div>

            <div
              className={style["comment-wrapper"]}
              onClick={() => isExtended(true)}
              ref={warpperRef}
            >
              <textarea
                ref={textareaRef}
                className={style["content"]}
                placeholder="Enter your comment here."
                maxLength={maxLength}
                value={text}
                onChange={(e) => setText(e.target.value)}
              />
            </div>
            <div className={style["comment-btn"]}>
              <button 
              type="button" 
              onClick={() => console.log(text.length)}
              className={style["icon"]}
              >
                <span>Reply</span>
              </button>
                <div
                    className={`${style["bar-thingy"]} ${
                        remainingChars <= 256 ? style.visible : ""
                    }`}
                >
                    <label className={style["counter"]} htmlFor="limit-bar">
                        {text.length}
                    </label>
                  
                    <div id="limit-bar" className={style["limit-bar"]}>
                        <div
                            className={style["progess-bar"]}
                            style={{ width: `${progressPercentage}%` }}
                        />
                    </div>
                </div>
              </div>
          </section>
                <section 
                className={style["extended"]} 
                data-state={extended}>
                    <label style={{ cursor: "pointer" }}>
                      <input
                        type="file"
                        accept="image/gif, image/jpeg, image/png, image/webp"
                        hidden
                      />
                      <CiImageOn size={24} />
                    </label>
                    <label style={{ cursor: "pointer" }}>
                      <HiOutlineEmojiHappy
                        size={24}
                        onClick={() => setIsOpenEmoji(!isOpenEmoji)}
                      />
                      <FloatingFocusManager context={context}>
                        <div ref={refs.setFloating} {...getFloatingProps()}>
                          <EmojiPicker
                            open={isOpenEmoji}
                            emojiStyle={EmojiStyle.GOOGLE}
                            theme={Theme.AUTO}
                            lazyLoadEmojis={true}
                            autoFocusSearch
                            className={style["emoji-picker"]}
                            style={{ position: "absolute" }}
                          />
                        </div>
                      </FloatingFocusManager>
                    </label>
                </section>
          {/* //! accept ONLY images */}
        </form>
      </section>
    </section>
  );
};

//  * -------------- GET COMMENT -----------------

const CommentFromUser: React.FC<commentProps> = ({ ...comment }) => {
  const [open, setOpen] = useState(false);
  const [showReadMoreButton, setShowReadMoreButton] = useState(false);
  const containerRefs = React.useRef<HTMLDivElement>(null);
  const hasDisplayName = comment.display_name || null;
  const ref = React.useRef<HTMLSpanElement | null>(null);
  const [likeState, setLikeState] = useState(comment.is_liked);
  const [likeCount, setLikeCount] = useState(comment.total_likes);

  return (
    <section className={style["comment-from-user"]}>
      <div className={style["header"]}>
        <div className={style["profile-header"]}>
          <div className={style["avatar"]}>
            <PostAvatar
              userId={comment.id}
              username={comment.username}
              displayName={comment.display_name}
              avatar={comment.avatar_path}
              thumbhash={comment.avatar_thumbhash}
              containerRef={containerRefs}
              containerWidth={36}
              containerHeight={36}
            />
          </div>

          <div className={style["user-info"]}>
            <div className={style["username"]}>
              <p style={{ fontSize: "var(--text-base--line-height)" }}>
                {comment.display_name || comment.username}
              </p>
            </div>
            <div className={style["post-date"]}>
              <p>
                {hasDisplayName && (
                  <>
                    {comment.username} <GoDotFill size={10} />
                  </>
                )}
              </p>
              <p style={{ fontSize: "var(--text)" }}>
                {formatSocialMediaDate(comment.created_at)}
              </p>
            </div>
          </div>
          <div className={style["option"]}>
            <Dropdown>
              <DropdownTrigger asChild>
                <BsThreeDots />
              </DropdownTrigger>
              {comment.user_id === comment.current_user_id ? (
                <DropdownContent>
                  <DropdownItem>Edit Post</DropdownItem>
                  <DropdownItem>
                    <button type="button" onClick={() => {}}>
                      Delete Post
                    </button>
                  </DropdownItem>
                </DropdownContent>
              ) : (
                <DropdownContent>
                  <DropdownItem>Follow @{comment.username}</DropdownItem>
                  <DropdownItem>Add Friend @{comment.username}</DropdownItem>
                  <DropdownItem>Block @{comment.username}</DropdownItem>
                  <DropdownItem>Report</DropdownItem>
                </DropdownContent>
              )}
            </Dropdown>
          </div>
        </div>
      </div>

      <article className={style["comment-user-container"]}>
        <div className={style["comment"]}>
          <div className={style["text-container"]}>
            <span
              className={`${style["content"]} ${!open ? style["is-collapsed"] : ""}`}
              ref={ref}
            >
              {comment.content}
            </span>
            <div>
              {showReadMoreButton && (
                <button
                  type="button"
                  onClick={() => setOpen(!open)}
                  className={style["read-more-btn"]}
                >
                  {open ? "Show less" : "Read more"}
                </button>
              )}
            </div>
          </div>
        </div>
        {comment.has_attachment && (
          <MediaGroup
            media={comment.media}
            containerHeightRatio={6}
            containerWidthRatio={9}
          />
        )}
      </article>

      <div className={style["bottom-container"]}>
        <section className={style["interaction"]}>
          <div style={{ userSelect: "none" }}>
            {/* //todo: Add animation if possible*/}
            <button
              type="button"
              style={{ cursor: "pointer" }}
              onClick={async () => {
                try {
                  const nextLikeState = !likeState;
                  const response = await TogglePostLike(
                    comment.id,
                    nextLikeState,
                  );
                  setLikeState(nextLikeState);
                  setLikeCount(response.total_liked);
                } catch (error) {
                  console.error("Failed to toggle like:", error);
                }
              }}
            >
              <LuThumbsUp />
            </button>
            {/* //todo: onClick get panigation user liked on post */}
            <button
              className={style["has-hover"]}
              style={{ cursor: "pointer" }}
              type="button"
            >
              {likeCount || 0}
            </button>
          </div>
        </section>

        <section className={style["interaction"]}>
          {/* //todo: dialog for share *if possible */}
          <button type="button" style={{ cursor: "pointer" }}>
            <DialogSharePost />
          </button>
          {/* //todo: bookmark ofc why not xdddddddddddd */}
          <button type="button" style={{ cursor: "pointer" }}>
            <IoBookmarkOutline />
          </button>
        </section>
      </div>
    </section>
  );
};
export default CommentSection;
