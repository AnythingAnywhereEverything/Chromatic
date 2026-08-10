"use client"

import { useInfiniteQuery } from "@tanstack/react-query"
import { fetchItems } from './testapi';
import { useEffect, useRef } from "react";
import style from "./style.module.scss";
const HomeFeed = () => {
    const { data, error, status, fetchNextPage, isFetchingNextPage } =
    useInfiniteQuery({
      queryKey: ['items'],
      queryFn: fetchItems,
      initialPageParam: 0,
      getNextPageParam: (lastPage) => lastPage.nextPage,
    });

    const loadMoreRef = useRef(null);

    useEffect(() => {
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

    return(
    <section>
    {status === "pending" ? (
        <p>Loading...</p>
    ) : status === "error" ? (
        <p>{error.message}</p>
    ) : (
        <>
            {data.pages.map((page, i) => (
                <div key={i}>
                    {page.data.map((item) => (
                        <div className={style['post-card']} key={item.id}>
                          <p>{item.name}</p>
                        </div>
                    ))}
                </div>
            ))}
            <div ref={loadMoreRef} />
            {isFetchingNextPage && <p>Loading more...</p>}
        </>
    )}
</section>
    )
}
export default HomeFeed;