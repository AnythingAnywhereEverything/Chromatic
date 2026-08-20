import { getUserFeed } from "@/api/post/getFeed";
import { useInfiniteQuery } from "@tanstack/react-query";
import { useEffect, useRef } from "react";

export const useGetUserFeed = (userId: string, interest_tags: string[]) => {
    const {
        data,
        error,
        status,
        fetchNextPage,
        isFetchingNextPage
    } = useInfiniteQuery({
        queryKey: ['post-feed', 'for-user'],
        queryFn: ({ pageParam }) => getUserFeed(pageParam),
        initialPageParam: 1,
        // * matches getUserFeed pageParam default (1), was 0 before
        getNextPageParam: (lastPage, allPages) =>
           lastPage.media.length < 15 ? undefined : lastPage.currentPage + 1,
    });

    const loadMoreRef = useRef(null);

    useEffect(()=>{
    const el = loadMoreRef.current;
      if (!el) return;

      const observer = new IntersectionObserver((entries) => {
          if (entries[0].isIntersecting && !isFetchingNextPage) {  
            fetchNextPage();
          }
      });
      console.log(data)
      observer.observe(el);
      return () => observer.disconnect();
    }, [fetchNextPage, isFetchingNextPage, status]);


    return {
        data,
        error,
        status,
        fetchNextPage,
        isFetchingNextPage
    }
}

// todo: impl to home page