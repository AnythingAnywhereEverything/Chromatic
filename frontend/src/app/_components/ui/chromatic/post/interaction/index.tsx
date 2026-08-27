"use client";

import { IoBookmarkOutline } from "react-icons/io5";
import style from "./interaction.module.scss";
import { DialogSharePost } from "..";
import { GoComment } from "react-icons/go";
import { LuThumbsUp } from "react-icons/lu";
import { TogglePostLike } from "@/api/post/like";
import { mediaPostProps } from "@/api/post/getFeed";
import { useState } from "react";
import React from "react";

const BottomPostInteraction = React.memo(function BottomPostInteraction  ({
    id,
    is_liked, 
    total_likes,
    total_comment,
}:mediaPostProps) {
    const [openOption, setOpenOption] = useState(false);
    const [likeState, setLikeState] = useState(is_liked);
    const [likeCount, setLikeCount] = useState(total_likes);
  return(
    <section>
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
                  const response = await TogglePostLike(id, nextLikeState);
                  setLikeState(nextLikeState);
                  setLikeCount(response.total_liked);
                } catch (error) {
                  console.error("Failed to toggle like:", error);
                }
              }}
            >
              <LuThumbsUp />
            </button>
            <button
              className={style["has-hover"]}
              style={{ cursor: "pointer" }}
              type="button"
            >
              {likeCount || 0}
            </button>
          </div>
          <div style={{ userSelect: "none", cursor: "pointer" }}>
            <button type="button">
              <GoComment />
            </button>
            {total_comment || 0}
          </div>
        </section>
        <section className={style["interaction"]}>
          <button type="button" style={{ cursor: "pointer" }}>
            <DialogSharePost />
          </button>
          <button type="button" style={{ cursor: "pointer" }}>
            <IoBookmarkOutline />
          </button>
        </section>
      </div>
    </section>
    )
});

export default BottomPostInteraction;