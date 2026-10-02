"use client";

import { useState } from "react";
import Link from "next/link";
import { HiOutlineCheck } from "react-icons/hi2";
import { Guild, GuildApiError } from "@/api/guild/types";
import { deleteGuild, joinGuild } from "@/api/guild/guilds";
import { formatLargeNumber } from "@/lib/utils";
import { UserIdAvatar } from "@/app/_components/ui/chromatic/initialAvatar";
import { formatdatemonthyear } from "@/app/_components/ui/chromatic/post/helpers/dateFormater";
import style from "./community.module.scss";

export default function GuildCard({ guild }: { guild: Guild }) {
    const [joined, setJoined] = useState(false);
    const [members, setMembers] = useState(guild.total_members);
    const [joining, setJoining] = useState(false);
    const [error, setError] = useState<string | null>(null);

    const join = async () => {
        setJoining(true);
        setError(null);
        try {
            const updated = await joinGuild(guild.id);
            setMembers(updated.total_members);
            setJoined(true);
        } catch (err) {
            if (err instanceof GuildApiError && err.code === "already_member") {
                setJoined(true);
                return;
            }
            setError(err instanceof Error ? err.message : "Could not join.");
        } finally {
            setJoining(false);
        }
    };

    const handleDeleteGuild = async () => {
        try {
            const response = await deleteGuild(guild.id);
        } catch (err) {
            console.error(err);
        }
    };

    return (
        <li className={style["guild-card"]}>
            <div className={style["guild-card-head"]}>
                    <UserIdAvatar
                        userId={guild.id}
                        name={guild.name}
                        size={44}
                    />
                    <Link
                        href={`/community/${guild.id}`}
                        className={style["guild-card-title"]}
                    >
                        <strong>{guild.name}</strong>
                        <span className={style["guild-card-owner"]}>
                            {formatLargeNumber(members)}{" "}
                            {members === 1 ? "member" : "members"}
                        </span>
                    </Link>

                <button
                    style={{ marginLeft: "auto" }}
                    type="button"
                    className={style["button-delete"]}
                    onClick={handleDeleteGuild}
                >
                    X
                </button>
            </div>

            {guild.description && (
                <p className={style["guild-card-description"]}>
                    {guild.description}
                </p>
            )}

            <div className={style["guild-card-footer"]}>
                <span className={style["guild-card-created"]}>
                    Created {formatdatemonthyear(guild.created_at)}
                </span>

                {joined ? (
                    <span className={style["button-joined"]}>
                        <HiOutlineCheck /> Joined
                    </span>
                ) : (
                    <button
                        type="button"
                        className={style["button-primary"]}
                        onClick={join}
                        disabled={joining}
                    >
                        {joining ? "Joining..." : "Join"}
                    </button>
                )}
            </div>

            {error && <p className={style["card-error"]}>{error}</p>}
        </li>
    );
}
