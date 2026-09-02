"use client";

import { Post } from "@/app/_components/ui/chromatic/post";
import { useEffect, useState } from "react";
import { getUserFeed, PostProps } from "@/api/post/getFeed";
import style from "./style.module.scss";
import PostPopup from "./postPopup";
import { CreatePostComponent, CreatePostPopup } from "@/app/_components/ui/chromatic/createPost";
import { getCurrentProfile, PublicUserProfileResponse } from "@/api/user/profile";

export default function PostGroup() {
  const [feed, setFeed] = useState<PostProps[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [popupPostId, setPopupPostId] = useState<string | null>(null);
  const [currentUser, setCurrentUser] = useState<PublicUserProfileResponse>();

  useEffect(() => {
      const fetchUser = async () => {
          const user = await getCurrentProfile();
          if (user) {
              setCurrentUser(user);
          }
      };
    
      fetchUser();
  }, []);

  useEffect(() => {
    const fetchPost = async () => {
      const post = await getUserFeed();
        if (post){
          setFeed(post);
        }
    };

    fetchPost();
  }, [])


    // todo: onClick the post Push? to
    //   const changePathname = (id: string) => {
    //         const currentUrl =
    //             window.location.pathname +
    //             window.location.search +
    //             window.location.hash;

    //         const newUrl = `/post/f/${id}`;

    //         window.history.replaceState(
    //             {
    //                 ...window.history.state,
    //                 previousUrl: currentUrl,
    //             },
    //             "",
    //             newUrl
    //         );
    //             setPopupPostId(id);
    //     };

  return (
    <div className={style["feed-layout"]}>
      {currentUser && <CreatePostPopup author={currentUser}/> }
      {feed?.map((post) => (
        <div key={post.post_id}
        // onClick={() => changePathname(post.post_id)}
        >
        <Post
          key={post.post_id}
          {...post}
          />
          </div>
      ))}
    </div>
  );
}
