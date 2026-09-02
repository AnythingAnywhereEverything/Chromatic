"use client";

import { RxCross2 } from "react-icons/rx";
import { PublicUserProfileResponse } from "@/api/user/profile";
import style from "./style.module.scss";
import React, { useState } from "react";
import { CiImageOn, CiPaperplane } from "react-icons/ci";
import { HiOutlineEmojiHappy } from "react-icons/hi";
import EmojiPicker from "emoji-picker-react";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogHeading,
  DialogTrigger,
} from "../dialogue";
import Form from "next/form";
import { Portal } from "@/app/_components/portal";
import { CreatePost } from "@/api/post/post";
import { PostAvatar } from "../post/header/avatar";
import { PostStatus, Visibility } from "./status";
function CreatePostComponent({
  author,
}: {
  author: PublicUserProfileResponse;
}) {
  return (
    <section className={style["create-container"]}>
      <article className={style["header"]}>
        <div className={style["profile"]} key={author.id}>
          <PostAvatar
            userId={author.id}
            username={author.username}
            displayName={author.display_name || author.username}
            thumbhash={author.avatar_thumbhash ?? null}
            avatar={author.avatar ?? null}
          />
          <div className={style["text-container"]}>
            <textarea
              style={{ cursor: "pointer" }}
              placeholder="What's on your mind ?"
              readOnly
            ></textarea>
          </div>
          <div className={style["btn"]}>
            <CiPaperplane size={20} />
          </div>
        </div>

        <div className={style["extended"]}>
          <label style={{ cursor: "pointer" }}>
            <input
              type="file"
              disabled
              accept="image/gif, image/jpeg, image/png, image/webp"
              hidden
            />
            <CiImageOn size={24} />
          </label>
          <label style={{ cursor: "pointer" }}>
            <HiOutlineEmojiHappy size={24} />
          </label>
        </div>
      </article>
    </section>
  );
}

export type CreatePostPayload = {
  content?: string;
  media_src?: File[];
  visability: Visibility;
  tags?: string[];
};

interface MediaFileProps {
  file: File;
  url: string;
  type: "image" | "video";
}
const MAX_MEDIA_FILES = 5;

// todo: do some loading anim
// todo: repage? or show the post

function CreatePostPopup({ author }: { author: PublicUserProfileResponse }) {
  const [text, setText] = React.useState("");
  const [visibility, setVisibility] = useState<Visibility>(Visibility.Everyone);

  const [open, setOpen] = React.useState(false);

  const [hasEdited, setHasEdited] = React.useState(false);
  const [isEmojiOpen, setIsEmojiOpen] = React.useState(false);
  const [isPending, setIsPending] = React.useState(false);

  // media
  const [media, setMedia] = React.useState<MediaFileProps[]>([]);
  const [activeIndex, setActiveIndex] = React.useState(0);
  const [hasOverflow, setHasOverflow] = React.useState(false);
  const [translateX, setTranslateX] = React.useState(0);
  // Ref
  const textareaRef = React.useRef<HTMLTextAreaElement | null>(null);
  const groupRef = React.useRef<HTMLUListElement | null>(null);
  const dialogRef = React.useRef<HTMLDivElement | null>(null);

  const mediaRef = React.useRef<MediaFileProps[]>([]);
  const MediaContainerRef = React.useRef<HTMLDivElement | null>(null);

  const [error, setError] = React.useState("");

  const [isDiscardDialogOpen, setIsDiscardDialogOpen] = React.useState(false);
  const handleDiscard = () => {
    setText("");
    setMedia([]);
    setVisibility(Visibility.Everyone);
    setIsEmojiOpen(false);
    setError("");
    setHasEdited(false);

    setIsDiscardDialogOpen(false);
    setOpen(false);
  };

  const handleDialogClose = () => {
    if (!hasEdited) {
      return;
    }
    setIsDiscardDialogOpen(true);

    return false;
  };

  React.useLayoutEffect(() => {
    if (textareaRef.current) {
      textareaRef.current.style.height = "auto";

      if (textareaRef.current.scrollHeight > 325) {
        textareaRef.current.style.height = "325px";
        textareaRef.current.style.overflowY = "auto";
      } else {
        textareaRef.current.style.height =
          textareaRef.current.scrollHeight + "px";
        textareaRef.current.style.overflowY = "hidden";
      }
    }
  }, [text]);

  React.useEffect(() => {
    setHasEdited(text.length > 0 || media.length > 0);
  }, [text, media]);

  // Media
  React.useEffect(() => {
    return () => {
      media.forEach((item) => URL.revokeObjectURL(item.url));
    };
  }, []);

  React.useEffect(() => {
    return () => {
      mediaRef.current.forEach((item) => {
        URL.revokeObjectURL(item.url);
      });
    };
  }, []);

  const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const selectedFiles = Array.from(event.target.files ?? []);
    const remaining = MAX_MEDIA_FILES - media.length;

    if (remaining <= 0) {
      return;
    }

    const newMedia = selectedFiles
      .slice(0, remaining)
      .filter(
        (file) =>
          file.type.startsWith("image/") || file.type.startsWith("video/"),
      )
      // * createObjectURL returns a fresh unique url every call, safe to use directly as the id
      .map((file) => ({
        file,
        url: URL.createObjectURL(file),
        type: file.type.startsWith("video/")
          ? ("video" as const)
          : ("image" as const),
      }));

    setMedia((current) => [...current, ...newMedia]);
    event.target.value = "";
  };

  const handleRemove = (url: string) => {
    setMedia((current) => {
      URL.revokeObjectURL(url);
      return current.filter((item) => item.url !== url);
    });
  };

  React.useLayoutEffect(() => {
    const container = MediaContainerRef.current;
    const group = groupRef.current;

    if (!container || !group) {
      return;
    }

    const items = Array.from(
      group.querySelectorAll<HTMLElement>(`.${style["media-item"]}`),
    );

    setHasOverflow(group.scrollWidth > container.clientWidth + 1);

    const item = items[activeIndex];

    if (!item) {
      setTranslateX(0);
      return;
    }

    const maxTranslate = Math.max(group.scrollWidth - container.clientWidth, 0);

    const targetTranslate = Math.min(item.offsetLeft, maxTranslate);

    setTranslateX(targetTranslate);
  }, [media, activeIndex]);

  React.useEffect(() => {
    const container = MediaContainerRef.current;

    if (!container) {
      return;
    }

    const observer = new ResizeObserver(() => {
      const group = groupRef.current;

      if (!group) {
        return;
      }

      setHasOverflow(group.scrollWidth > container.clientWidth + 1);

      const items = Array.from(
        group.querySelectorAll<HTMLElement>(`.${style["media-item"]}`),
      );

      const item = items[activeIndex];

      if (!item) {
        setTranslateX(0);
        return;
      }

      const maxTranslate = Math.max(
        group.scrollWidth - container.clientWidth,
        0,
      );

      setTranslateX(Math.min(item.offsetLeft, maxTranslate));
    });

    observer.observe(container);

    return () => observer.disconnect();
  }, [activeIndex]);

  const slideMedia = (direction: "left" | "right") => {
    setActiveIndex((currentIndex) => {
      if (direction === "right") {
        return Math.min(currentIndex + 1, media.length - 1);
      }

      return Math.max(currentIndex - 1, 0);
    });
  };

  const showLeftController = hasOverflow && activeIndex > 0;
  //   *it's quite buggy for some reason
  const showRightController = hasOverflow && activeIndex < media.length - 2;

  const handleSubmit = async () => {
    if (isPending) {
      return;
    }
    const allMedia: File[] = media.map((item) => item.file);

    const formData = new FormData();
    if (text != undefined) {
      formData.append("content", text);
    }
    if (allMedia != undefined) {
      allMedia.forEach((file) => {
        formData.append("media_src", file);
      });
    }
    if (visibility != undefined) {
      formData.append("visibility", visibility);
    }

    try {
      setIsPending(true);

      const res = await CreatePost(formData);

      if (res) {
        setText("");
        setMedia([]);
        setHasEdited(false);
        // close the dialog on success
        setOpen(false);
      }
    } catch (err) {
      console.error(err);
    } finally {
      setIsPending(false);
    }
  };

  return (
    <>
      <Dialog
        open={open}
        onOpenChange={setOpen}
        outsidePress={!hasEdited}
        onClose={handleDialogClose}
      >
        <DialogTrigger asChild onClick={() => setOpen(true)}>
          <div style={{ width: "100%" }}>
            <CreatePostComponent author={author} />
          </div>
        </DialogTrigger>

        <DialogContent
          ref={dialogRef}
          className={style["create-container-dialog"]}
        >
          <DialogHeading className={style["exit-btn"]}>
            <DialogClose type="button" className={style["leave-btn"]}>
              <RxCross2 size={24} />
            </DialogClose>
            <span>Post</span>
          </DialogHeading>

          <Form action={handleSubmit}>
            <section className={style["container-dialog"]}>
              <article className={style["header"]}>
                <div className={style["profile"]} key={author.id}>
                  <div style={{ position: "relative", width: "40px" }}>
                    <PostAvatar
                      userId={author.id}
                      username={author.username}
                      displayName={author.display_name || author.username}
                      thumbhash={author.avatar_thumbhash ?? null}
                      avatar={author.avatar ?? null}
                    />
                  </div>
                  <div className={style["text-container"]}>
                    <textarea
                      ref={textareaRef}
                      maxLength={2500}
                      value={text}
                      placeholder="What's on your mind ?"
                      onChange={(e) => setText(e.target.value)}
                      onBlur={(e) => setText(e.target.value.trim())}
                    ></textarea>
                  </div>
                  <button
                    className={style["btn"]}
                    disabled={!hasEdited}
                    onClick={handleSubmit}
                  >
                    <CiPaperplane size={20} />
                  </button>
                </div>

                <div className={style["extended"]}>
                  <div className={style["visibility"]}>
                    <PostStatus
                      visibility={visibility}
                      onChange={(value) => setVisibility(value)}
                    />
                  </div>
                  <div className={style["panel"]}>
                    <label style={{ cursor: "pointer" }}>
                      <input
                        type="file"
                        accept="image/*, video/*"
                        hidden
                        multiple
                        onChange={handleFileChange}
                      />
                      <CiImageOn size={24} />
                    </label>
                    <label style={{ cursor: "pointer" }}>
                      <HiOutlineEmojiHappy
                        size={24}
                        onClick={() => setIsEmojiOpen(!isEmojiOpen)}
                      />
                      <Portal container={dialogRef?.current}>
                        <EmojiPicker open={isEmojiOpen} lazyLoadEmojis />
                      </Portal>
                    </label>
                  </div>
                </div>
              </article>
            </section>

            <section className={style["media-container"]}>
              {showLeftController && (
                <button
                  type="button"
                  className={`${style["slide-left"]} ${style["media-controller"]}`}
                  onClick={() => slideMedia("left")}
                  aria-label="Previous media"
                >
                  ‹
                </button>
              )}

              <div className={style["media-viewport"]} ref={MediaContainerRef}>
                <ul
                  className={style["media-group"]}
                  ref={groupRef}
                  style={{
                    transform: `translate3d(-${translateX}px, 0, 0)`,
                  }}
                >
                  {media.map((item) => (
                    <li key={item.url} className={style["media-item"]}>
                      {item.type === "image" ? (
                        <img
                          src={item.url}
                          alt={item.file.name}
                          className={style["media-preview"]}
                        />
                      ) : (
                        <video
                          controls
                          disablePictureInPicture
                          disableRemotePlayback
                          preload="none"
                          className={style["media-video"]}
                          style={{ pointerEvents: "none" }}
                        >
                          <source src={item.url} type={item.file.type} />
                        </video>
                      )}

                      <button
                        type="button"
                        className={style["media-remove"]}
                        onClick={() => handleRemove(item.url)}
                      >
                        <RxCross2 size={18} />
                      </button>
                    </li>
                  ))}
                </ul>
              </div>

              {showRightController && (
                <button
                  type="button"
                  className={`${style["slide-right"]} ${style["media-controller"]}`}
                  onClick={() => slideMedia("right")}
                  aria-label="Next media"
                >
                  ›
                </button>
              )}
            </section>
          </Form>
        </DialogContent>
      </Dialog>

      {/* Discard post */}
      <Dialog open={isDiscardDialogOpen} onOpenChange={setIsDiscardDialogOpen}>
        <DialogContent className={style["discard-container"]}>
          <DialogHeading className={style["header"]}>
            Unsaved changed
          </DialogHeading>
          <div className={style["description"]}>
            <DialogDescription>
              You have an unsaved change.<br />
              Are you sure you want to discard them?
            </DialogDescription>
          </div>

          <div className={style["btn"]}>
            <DialogClose className={style["continue"]} type="button">
              Continue
            </DialogClose>
            <button
              type="button"
              onClick={handleDiscard}
              className={style["discard"]}
            >
              Discard
            </button>
          </div>
        </DialogContent>
      </Dialog>
    </>
  );
}

export { CreatePostComponent, CreatePostPopup };
