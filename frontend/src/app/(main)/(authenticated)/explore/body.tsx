"use client";

import { PostProps } from "@/api/post/getFeed";
import { Post } from "@/app/_components/ui/chromatic/post";
import style from "./explore.module.scss";
import { useCallback, useEffect, useRef, useState } from "react";
import Form from "next/form";
import { useSearchParams } from "next/navigation";
import { searchExplorePosts } from "@/api/search";

const LIMIT = 10;

function ExploreBody() {
    const searchParams = useSearchParams();
    const urlQuery = searchParams.get("q") ?? "";

    const [query, setQuery] = useState(urlQuery);

    const [posts, setPosts] = useState<PostProps[]>([]);
    const [loading, setLoading] = useState(false);
    const [hasMore, setHasMore] = useState(true);

    const [before, setBefore] = useState<string | undefined>();
    const [beforeId, setBeforeId] = useState<string | undefined>();

    const observerRef = useRef<HTMLDivElement | null>(null);
    const loadingRef = useRef(false);
    const epochRef = useRef(0);

    const loadMore = useCallback(
        async (
            cursorBefore: string | undefined,
            searchQuery: string,
            more: boolean,
        ) => {
            if (loadingRef.current || !more) return;

            loadingRef.current = true;
            setLoading(true);

            const epoch = epochRef.current;

            try {
                const result = await searchExplorePosts({
                    limit: LIMIT,
                    before: cursorBefore
                        ? new Date(cursorBefore).toISOString()
                        : undefined,
                    query: searchQuery || undefined,
                });

                // * Ignore results belonging to an older search.
                if (epoch !== epochRef.current) return;

                if (!result || result.length === 0) {
                    setHasMore(false);
                    return;
                }

                setPosts((current) => {
                    const existing = new Set(
                        current.map((post) => post.post_id),
                    );

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
        },
        [],
    );

    // * React to both normal searches and Link navigation such as /explore?q=%23typescript.
    useEffect(() => {
        epochRef.current += 1;
        loadingRef.current = false;

        setQuery(urlQuery);
        setPosts([]);
        setBefore(undefined);
        setBeforeId(undefined);
        setHasMore(true);

        loadMore(undefined, urlQuery, true);
    }, [urlQuery, loadMore]);

    useEffect(() => {
        const observer = new IntersectionObserver(
            ([entry]) => {
                if (entry.isIntersecting) {
                    loadMore(before, urlQuery, hasMore);
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
    }, [before, beforeId, hasMore, loadMore, urlQuery]);

    const handleSearch = (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault();

        const formData = new FormData(event.currentTarget);
        const searchQuery = formData.get("query")?.toString().trim() || "";

        const params = new URLSearchParams(searchParams.toString());

        if (searchQuery) {
            params.set("q", searchQuery);
        } else {
            params.delete("q");
        }

        const newUrl = params.toString()
            ? `${window.location.pathname}?${params.toString()}`
            : window.location.pathname;

        window.history.pushState({}, "", newUrl);

        // * pushState alone does not notify Next's useSearchParams.
        // * Force the URL-driven state to update.
        window.dispatchEvent(new PopStateEvent("popstate"));
    };

    return (
        <section className={style["explore-body"]}>
            <Form
                action="#"
                className={style["explore-top"]}
                onSubmit={handleSearch}
            >
                <input
                    name="query"
                    className={style["explore-filter-input"]}
                    placeholder="Search posts..."
                    type="text"
                    value={query}
                    onChange={(e) => setQuery(e.target.value)}
                />

                <button type="submit">Search</button>
            </Form>

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
