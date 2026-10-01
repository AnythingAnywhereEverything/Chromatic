"use client";

import { useQuery } from "@tanstack/react-query";
import { useDeferredValue, useState } from "react";
import { HiMiniPlus } from "react-icons/hi2";
import { FaAngleDown } from "react-icons/fa6";
import { getGuilds } from "@/api/guild/guilds";
import { Guild } from "@/api/guild/types";
import GuildCard from "./guildCard";
import CreateGuildDialog from "./createGuildDialog";
import style from "./community.module.scss";

/**
 * The public community list.
 *
 * Page 1 comes from react-query; later keyset pages accumulate into `loaded`
 * below it, advancing the cursor to the last row seen. This is the pattern
 * `RoleDialog`'s member list uses — deliberately not `useInfiniteQuery`, which
 * nothing else in this app uses.
 */
export default function CommunityBody() {
    const [search, setSearch] = useState("");
    const deferredSearch = useDeferredValue(search);
    const [creating, setCreating] = useState(false);

    const { data: page, isLoading, error } = useQuery({
        queryKey: ["guilds"],
        queryFn: () => getGuilds({ limit: 20 }),
        staleTime: 60 * 1000,
        retry: false,
    });

    // Pages after the first, accumulated. The cursor is derived from the last
    // row in `loaded` rather than stored, so it cannot drift out of sync with
    // the rows it was supposed to point past.
    const [loaded, setLoaded] = useState<Guild[]>([]);
    const [loadingMore, setLoadingMore] = useState(false);
    const [moreError, setMoreError] = useState<string | null>(null);
    // Whether a page beyond what the most recent fetch returned exists. Set
    // from page 1 before any "load more", then replaced by each later page.
    const [lastHasMore, setLastHasMore] = useState<boolean | null>(null);

    const guilds = [...(page?.rows ?? []), ...loaded];
    // Until a second page has been fetched, page 1 is the authority on whether
    // more exist. After that the later page is, since only it knows what
    // follows it — deriving from a row count instead would keep offering the
    // button after the last page.
    const hasMore = lastHasMore ?? Boolean(page?.has_more);

    const loadMore = async () => {
        const last = guilds[guilds.length - 1];
        if (!last || loadingMore) return;

        setLoadingMore(true);
        setMoreError(null);
        try {
            const next = await getGuilds({
                before: last.created_at,
                before_id: last.id,
                limit: 20,
            });
            setLoaded((prev) => [...prev, ...next.rows]);
            setLastHasMore(next.has_more);
        } catch (err) {
            setMoreError(
                err instanceof Error ? err.message : "Could not load more communities.",
            );
        } finally {
            setLoadingMore(false);
        }
    };

    // The browse endpoint has no `q` param (that exists only on the admin
    // list), so search narrows what is loaded rather than asking the server.
    // It is therefore scoped to the pages fetched so far, not the whole table.
    const term = deferredSearch.trim().toLowerCase();
    const visible = term
        ? guilds.filter(
              (g) =>
                  g.name.toLowerCase().includes(term) ||
                  (g.description ?? "").toLowerCase().includes(term),
          )
        : guilds;

    const refresh = () => {
        // Joined state is derived from this session's join responses only — the
        // list carries no membership flag — so clearing on refresh returns it
        // to unknown rather than leaving a stale "joined" on screen.
        setLoaded([]);
        setMoreError(null);
        // Back to page 1 being the authority on whether more exist.
        setLastHasMore(null);
    };

    return (
        <section className={style["community"]}>
            <div className={style["community-header"]}>
                <div>
                    <h1>Communities</h1>
                    <p>Find a community to join, or start your own.</p>
                </div>

                <div className={style["community-search"]}>
                    <input
                        type="text"
                        placeholder="Search loaded communities..."
                        value={search}
                        onChange={(e) => setSearch(e.target.value)}
                    />
                </div>

                <button
                    type="button"
                    className={style["button-primary"]}
                    onClick={() => setCreating(true)}
                >
                    <HiMiniPlus /> New community
                </button>
            </div>

            {error && (
                <p className={style["card-error"]}>
                    Could not load communities. {error.message}
                </p>
            )}

            <ul className={style["guild-grid"]}>
                {visible.map((guild) => (
                    <GuildCard key={guild.id} guild={guild} />
                ))}
            </ul>

            {isLoading && <p className={style["muted"]}>Loading communities...</p>}

            {!isLoading && visible.length === 0 && !error && (
                <p className={style["muted"]}>
                    {term
                        ? `No communities match "${search}".`
                        : "No communities yet."}
                </p>
            )}

            {hasMore && guilds.length > 0 && (
                <button
                    type="button"
                    className={`${style["button-ghost"]} ${style["load-more"]}`}
                    disabled={loadingMore}
                    onClick={loadMore}
                >
                    <FaAngleDown />
                    {loadingMore ? "Loading..." : "Load more"}
                </button>
            )}

            {moreError && (
                <p className={style["card-error"]}>{moreError}</p>
            )}

            <CreateGuildDialog
                open={creating}
                onOpenChange={setCreating}
                onCreated={refresh}
            />
        </section>
    );
}