"use client";

import { useEffect, useState } from "react";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { HiArrowLeft, HiHashtag } from "react-icons/hi2";
import { PiPaperPlaneFill } from "react-icons/pi";
import { UserIdAvatar } from "@/app/_components/ui/chromatic/initialAvatar";
import { formatSocialMediaDate } from "@/app/_components/ui/chromatic/post/helpers/dateFormater";
import { GuildChannel, mockChannels, mockMessages } from "./mockChannels";
import style from "../community.module.scss";

export default function GuildViewBody({ guildId }: { guildId: string }) {
    const router = useRouter();

    const [channels] = useState<GuildChannel[]>(() => mockChannels(guildId));
    const [selectedId, setSelectedId] = useState<string>(channels[0]?.id ?? "");

    const selected =
        channels.find((channel) => channel.id === selectedId) ?? channels[0];

    useEffect(() => {
        const onPopState = () => setSelectedId(channels[0]?.id ?? "");
        window.addEventListener("popstate", onPopState);
        return () => window.removeEventListener("popstate", onPopState);
    }, [channels]);

    const selectChannel = (channel: GuildChannel) => {
        setSelectedId(channel.id);
        window.history.pushState(
            { guildId, channelId: channel.id },
            "",
            `/community/${guildId}`,
        );
    };

    const messages = selected ? mockMessages(selected) : [];

    return (
        <section className={style["guild-view"]}>
            <aside className={style["channel-rail"]}>
                <div className={style["channel-rail-header"]}>
                    <UserIdAvatar userId={guildId} name="C" size={36} />
                    <div>
                        <strong>Community</strong>
                        <span>Guild {guildId}</span>
                    </div>
                </div>

                <button
                    type="button"
                    className={style["channel-back"]}
                    onClick={() => router.push("/community")}
                >
                    <HiArrowLeft /> All communities
                </button>

                <h3 className={style["channel-list-heading"]}>Text channels</h3>

                <ul className={style["channel-list"]}>
                    {channels.map((channel) => (
                        <li key={channel.id}>
                            <button
                                type="button"
                                className={`${style["channel-row"]} ${
                                    channel.id === selected?.id
                                        ? style["channel-active"]
                                        : ""
                                }`}
                                onClick={() => selectChannel(channel)}
                            >
                                <HiHashtag />
                                <span>{channel.name}</span>
                            </button>
                        </li>
                    ))}
                </ul>
            </aside>

            <div className={style["channel-pane"]}>
                <div className={style["channel-pane-header"]}>
                    <h2>
                        <HiHashtag /> {selected?.name}
                    </h2>
                </div>

                <p className={style["mock-notice"]}>
                    Channel contents are placeholders — there is no channel
                    endpoint yet, so nothing here is saved or sent.
                </p>

                <div className={style["channel-messages"]}>
                    {messages.length === 0 && (
                        <p className={style["muted"]}>
                            Nothing here yet.
                        </p>
                    )}

                    {messages.map((message) => (
                        <div
                            key={message.id}
                            className={style["mock-message"]}
                        >
                            <UserIdAvatar
                                userId={message.author_id}
                                name={message.author_name}
                                size={36}
                            />
                            <div className={style["mock-message-body"]}>
                                <span className={style["mock-message-author"]}>
                                    {message.author_name}
                                    <span
                                        className={style["mock-message-time"]}
                                    >
                                        {" "}
                                        {formatSocialMediaDate(
                                            message.created_at,
                                        )}
                                    </span>
                                </span>
                                <p className={style["mock-message-content"]}>
                                    {message.content}
                                </p>
                            </div>
                        </div>
                    ))}
                </div>

                <form
                    className={style["mock-compose"]}
                    onSubmit={(e) => e.preventDefault()}
                >
                    <textarea
                        placeholder="Messaging is not available yet"
                        disabled
                        rows={1}
                    />
                    <button
                        type="submit"
                        className={style["button-primary"]}
                        disabled
                    >
                        <PiPaperPlaneFill />
                    </button>
                </form>
            </div>
        </section>
    );
}