"use client";

import { getFocusedPost, mediaPostProps } from "@/api/post/getFeed";
import { getCacheUserId } from "@/handler/token_handler";
import { useEffect, useRef, useState } from "react";
import style from "./content.module.scss";
import { IoArrowBackCircleOutline, IoBookmarkOutline } from "react-icons/io5";
import PostHeader from "./header";
import { MediaGroup } from "@/app/_components/ui/chromatic/post/mediagroup";
import { TogglePostLike } from "@/api/post/like";
import { GoComment } from "react-icons/go";
import { LuThumbsUp } from "react-icons/lu";
import { DialogSharePost } from "@/app/_components/ui/chromatic/post";

function PostContentSkeleton() {
  return <div className={style["skeleton-container"]}>loading</div>;
}

function PostContentPage({ params }: { params: { post: string } }) {
  const [post, setPost] = useState<mediaPostProps | null>(null);
  useEffect(() => {
    const postFetch = async () => {
      const postOf = params.post;
      const response = await getFocusedPost(postOf);

      if (getCacheUserId() === response?.user_id) {
        console.log("Owner of this post");
        setPost(response);
        return;
      }
      setPost(response);
    };
    postFetch();
  }, [params.post]);
      
  if (!post) {
    return <PostContentSkeleton />;
  }

    const [likeState, setLikeState] = useState(post.is_liked);
    const [likeCount, setLikeCount] = useState(post.total_likes);
    const ref = useRef<HTMLSpanElement | null>(null);
    

  console.log(post);
  return (
    <article className={style["layout"]} key={post.id}>
      {post ? (
        <section className={style["main-post-container"]}>
          {/*  */}
          <section className={style["back-button"]}>
            <button type="button">
              <IoArrowBackCircleOutline />
            </button>
            <h3>Post</h3>
          </section>
          <PostHeader {...post} />
          <section className={style["main"]}>
            <div className={style["context"]}>
              <span>{post.content}</span>
            </div>
            <article className={style[""]}>
              {post.has_attachment && <MediaGroup media={post.media} />}
              <ul className={style["subject-tag"]}>
                {post.tag.map((item) => {
                  return (
                    <li key={item.tag_id}>
                      <p>{item.tag_name}</p>
                    </li>
                  );
                })}
              </ul>
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
                            post.id,
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
                  <div style={{ userSelect: "none", cursor: "pointer" }}>
                    {/* //todo: onClick pass to specific post and fetch comment */}
                    <button type="button">
                      <GoComment />
                    </button>
                    {post.total_comment || 0}
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
            </article>
          </section>
        </section>
      ) : (
        <PostContentSkeleton />
      )}
    </article>
  );
}

export default PostContentPage;
