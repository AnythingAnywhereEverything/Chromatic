"use client";
import { GetAllTagAttachments, TagRow } from "@/api/tags/tags";
import { PostProps } from "@/api/post/getFeed";
import { GetExplorePosts } from "@/api/post/post";
import { Post } from "@/app/_components/ui/chromatic/post";
import PostTags from "@/app/_components/ui/chromatic/createPost/tags";
import style from "./explore.module.scss";
import { useEffect, useRef, useState } from "react";

const LIMIT = 10;

function ExploreBody() {
    const [tags, setTags] = useState<TagRow[]>([]);
    const [selectedTag, setSelectedTag] = useState<TagRow | null>(null);

    const [posts, setPosts] = useState<PostProps[]>([]);
    const [loading, setLoading] = useState(false);
    const [hasMore, setHasMore] = useState(true);
    const [before, setBefore] = useState<string | undefined>();
    const [beforeId, setBeforeId] = useState<string | undefined>();

    const observerRef = useRef<HTMLDivElement | null>(null);
    const loadingRef = useRef(false);
    const epochRef = useRef(0);

    useEffect(() => {
        GetAllTagAttachments().then((fetchedTags) => {
            if (fetchedTags) {
                setTags(fetchedTags);
            }
        });
    }, []);

    const loadMore = async (
        cursorBefore: string | undefined,
        cursorBeforeId: string | undefined,
        tag: TagRow | null,
        more: boolean,
    ) => {
        if (loadingRef.current || !more) return;
        loadingRef.current = true;
        setLoading(true);

        const epoch = epochRef.current;
        try {
            const result = await GetExplorePosts(
                cursorBefore ? new Date(cursorBefore) : undefined,
                cursorBeforeId,
                tag?.id,
                LIMIT,
            );

            if (epoch !== epochRef.current) return;

            if (!result || result.length === 0) {
                setHasMore(false);
                return;
            }

            setPosts((current) => {
                const existing = new Set(current.map((post) => post.post_id));
                return [
                    ...current,
                    ...result.filter((post) => !existing.has(post.post_id)),
                ];
            });

            const oldestPost = result[result.length - 1];
            setBefore(oldestPost.created_at);
            setBeforeId(oldestPost.post_id);

            if (result.length < LIMIT) {
                setHasMore(false);
            }
        } finally {
            loadingRef.current = false;
            setLoading(false);
        }
    };

    // * Reset and load the first page when the tag changes (or on mount).
    useEffect(() => {
        epochRef.current += 1;
        loadingRef.current = false;
        setPosts([]);
        setBefore(undefined);
        setBeforeId(undefined);
        setHasMore(true);
        loadMore(undefined, undefined, selectedTag, true);
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [selectedTag]);

    // * Watch the bottom sentinel for infinite scroll.
    useEffect(() => {
        const observer = new IntersectionObserver(
            ([entry]) => {
                if (entry.isIntersecting) {
                    loadMore(before, beforeId, selectedTag, hasMore);
                }
            },
            { rootMargin: "300px" },
        );
        const target = observerRef.current;
        if (target) {
            observer.observe(target);
        }
        return () => {
            if (target) {
                observer.unobserve(target);
            }
        };
    }, [loading, hasMore, before, beforeId, selectedTag]);

    return (
        <section className={style["explore-body"]}>
            <div className={style["explore-filter"]}>
                <div className={style["explore-top"]}>
                    <input
                        className={style["explore-filter-input"]}
                        placeholder="Search posts..."
                        type="text"
                    />
                </div>
                <div className={style["explore-tags"]}> 
                <PostTags
                    tags={tags}
                    selectedTag={selectedTag}
                    onChangeTag={setSelectedTag}
                />
                </div>
            </div>

            <div className={style["explore-posts"]}>
                {posts.map((post) => (
                    <Post key={post.post_id} {...post} />
                ))}

                {hasMore && (
                    <div ref={observerRef} style={{ minHeight: "1px" }}>
                        {loading && "Loading..."}
                    </div>
                )}

                {!hasMore && posts.length === 0 && !loading && (
                    <p className={style["explore-empty"]}>No posts found.</p>
                )}
            </div>
        </section>
    );
}

export default ExploreBody;
