"use client";
import { useEffect, useState } from "react";
import style from "./user-edit.module.scss";
import { AdminUserDetail } from "@/api/admin/types";
import { getAdminUser, suspendUser, activateUser } from "@/api/admin/users";
import { formatFullDateWithExactTime } from "@/app/_components/ui/chromatic/post/helpers/dateFormater";
import { PostAvatar } from "@/app/_components/ui/chromatic/post/header/avatar";
export default function AdminUserBody({
    params,
}: {
    params: { user_id: string };
}) {
    const [user, setUser] = useState<AdminUserDetail>();
    const [loading, setLoading] = useState(false);
    useEffect(() => {
        if (!params.user_id) {
            return;
        }

        const fetchUser = async () => {
            setLoading(true);
            const result = await getAdminUser(params.user_id);

            if (result) {
                setUser(result);
            }
            setLoading(false);
        };

        fetchUser();
    }, [params.user_id]);

    const handleSuspend = async () => {
        if (!user) return;

        setLoading(true);

        const result = await suspendUser(user.user.id);

        if (result) {
            setUser({
                ...user,
                user: {
                    ...user.user,
                    is_active: false,
                },
            });
        }

        setLoading(false);
    };

    const handleActivate = async () => {
        if (!user) return;

        setLoading(true);

        const result = await activateUser(user.user.id);

        if (result) {
            setUser({
                ...user,
                user: {
                    ...user.user,
                    is_active: true,
                },
            });
        }

        setLoading(false);
    };
    return (
        <section className={style["admin-user-body"]}>
            {loading ? (
                <p>Loading...</p>
            ) : (
                user && (
                    <section className={style["user-body-content"]}>
                        <h1>Edit User</h1>
                        <PostAvatar
                            userId={user.user.id}
                            username={user.user.username}
                            displayName={user.user.display_name ?? ""}
                            avatar={user.user.avatar ?? null}
                            thumbhash={user.user.avatar_thumbhash ?? null}
                            width={40}
                            height={40}
                        />
                        User: {user.user.username}
                        {/* Currently can't edit user details except for deactivation and deletion */}
                        <div className={style["user-body"]}>
                            {/* Editable will be in future */}
                            <div className={style["user-account"]}>
                                <h3>Account</h3>
                                <p>Username: {user.user.username}</p>
                                <p>Email: {user.user.email}</p>
                                <p>
                                    Display Name:{" "}
                                    {user.user.display_name
                                        ? user.user.display_name
                                        : ""}
                                </p>
                            </div>

                            <div className={style["user-permissions"]}>
                                <h3>Permissions</h3>

                                {/* Anything work */}
                                <div className={style["user-status-container"]}>
                                    <p className={style["admin-user-status"]}>
                                        Status:{" "}
                                        {user.user.is_active
                                            ? "Active"
                                            : "Inactive"}
                                    </p>

                                    <div
                                        className={
                                            style[
                                                `status-${user.user.is_active ? "active" : "inactive"}`
                                            ]
                                        }
                                    />
                                </div>
                                {/* Superuser checkbox, read-only for now */}
                                <label>
                                    <input
                                        type="checkbox"
                                        checked={user.user.is_superuser}
                                        readOnly
                                    />
                                    Superuser : further implementation needed
                                </label>
                                {/* Staff roles will be listed here in the future */}
                                <div className={style["user-staff-roles"]}>
                                    Staff roles: further implementation needed
                                </div>
                            </div>

                            <div className={style["user-statistic"]}>
                                <div className={style["user-followers"]}>
                                    <h3>Followers</h3>
                                    <p>{user.user.followers_count || 0}</p>
                                </div>
                                <div className={style["user-following"]}>
                                    <h3>Following</h3>
                                    <p>{user.user.following_count || 0}</p>
                                </div>
                                <div className={style["user-posts"]}>
                                    <h3>Posts</h3>
                                    <p>{user.user.posts_count || 0}</p>
                                </div>
                            </div>

                            <div className={style["user-info"]}>
                                <h3>Account Information</h3>

                                <p>ID: {user.user.id}</p>
                                <p>
                                    Email verified: further implementation
                                    needed
                                </p>
                                <p>
                                    Created at:{" "}
                                    {formatFullDateWithExactTime(
                                        user.user.created_at,
                                    )}
                                </p>
                                <p>
                                    Updated at:{" "}
                                    {formatFullDateWithExactTime(
                                        user.user.updated_at,
                                    )}
                                </p>
                                <p>
                                    Delete at:{" "}
                                    {user.user.deleted_at
                                        ? formatFullDateWithExactTime(
                                              user.user.deleted_at,
                                          )
                                        : "Not deleted"}
                                </p>
                            </div>
                        </div>
                        <div className={style["admin-actions"]}>
                            <button
                                className={`${style[`${user.user.is_active ? "suspend" : "activate"}`]}`}
                                type="button"
                                onClick={
                                    user.user.is_active
                                        ? handleSuspend
                                        : handleActivate
                                }
                                disabled={loading}
                            >
                                {user.user.is_active
                                    ? "Deactivate User"
                                    : "Activate User"}
                            </button>
                            <button type="button"
                            className={style["delete"]}
                            >Delete User</button>
                        </div>
                    </section>
                )
            )}
        </section>
    );
}
