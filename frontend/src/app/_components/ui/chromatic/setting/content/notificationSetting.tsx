import { useEffect, useState } from "react";
import style from "../scss/notification-setting.module.scss";
import { SettingContentProps } from "./accountSetting";
import { getUserSettingType, updateUserSettingType, UserSettingResponse } from "@/api/user";
import {
    Dropdown,
    DropdownContent,
    DropdownItem,
    DropdownTrigger,
} from "../../dropdown";
import { IoIosArrowDown } from "react-icons/io";

let initialNotificationCache: UserSettingResponse | null = null;

const NOTIFICATION_ENUM_TYPES = {
    EVERYONE: "everyone",
    FOLLOWERS: "follower",
    NONE: "none",
};

type NotificationEnumType =
    (typeof NOTIFICATION_ENUM_TYPES)[keyof typeof NOTIFICATION_ENUM_TYPES];

const NOTIFICATION_ENUM_LABELS: Record<NotificationEnumType, string> = {
    everyone: "Everyone",
    follower: "Followers",
    none: "None",
};



function NotificationSetting({ user }: SettingContentProps) {
    const [notificationSetting, setNotificationSetting] =
        useState<UserSettingResponse | null>(null);

    const [notificationState, setNotificationState] = useState({
        followRequest: false,
        messageRequest: false,
        newMessage: false,
        commentLike: false,
    });

    const [newPostNotification, setNewPostNotification] =
        useState<NotificationEnumType>(NOTIFICATION_ENUM_TYPES.FOLLOWERS);

    const [loading, setLoading] = useState(true);
    const [saving, setSaving] = useState(false);

    useEffect(() => {
        if (initialNotificationCache) {
            setNotificationSetting(initialNotificationCache);
            setLoading(false);
            return;
        }

        const fetchNotificationSettings = async () => {
            try {
                const res = await getUserSettingType("notification");

                if (res) {
                    initialNotificationCache = res;
                    setNotificationSetting(res);
                }
            } catch (e) {
                console.error("Failed to load notification settings", e);
            } finally {
                setLoading(false);
            }
        };

        fetchNotificationSettings();
    }, []);

    useEffect(() => {
        const setting = notificationSetting?.setting_value;

        if (!setting) {
            return;
        }

        setNotificationState((prev) => ({
            ...prev,
            followRequest: setting.follow?.follow_request === 1,
            messageRequest: setting.message?.message_requests === 1,
            newMessage: setting.message?.new_message === 1,
            commentLike: setting.post?.comment_likes === 1,
        }));
        setNewPostNotification(
            setting.post?.new_post ?? NOTIFICATION_ENUM_TYPES.FOLLOWERS,
        );
    }, [notificationSetting]);

    const persist = async (next: Record<string, any>) => {
        setSaving(true);
        try {
            const updated = await updateUserSettingType("notification", next);
            initialNotificationCache = updated;
            setNotificationSetting(updated);
        } catch (e) {
            console.error("Failed to save notification settings", e);
        } finally {
            setSaving(false);
        }
    };

    const updateToggle = (
        key: keyof typeof notificationState,
        value: boolean,
    ) => {
        const base = notificationSetting?.setting_value ?? {};

        let next: Record<string, any>;

        if (key === "followRequest") {
            next = {
                ...base,
                follow: { ...base.follow, follow_request: value ? 1 : 0 },
            };
        } else if (key === "messageRequest") {
            next = {
                ...base,
                message: { ...base.message, message_requests: value ? 1 : 0 },
            };
        } else if (key === "newMessage") {
            next = {
                ...base,
                message: { ...base.message, new_message: value ? 1 : 0 },
            };
        } else {
            next = {
                ...base,
                post: { ...base.post, comment_likes: value ? 1 : 0 },
            };
        }

        setNotificationState((prev) => ({ ...prev, [key]: value }));
        persist(next);
    };

    const updateNewPost = (value: NotificationEnumType) => {
        const base = notificationSetting?.setting_value ?? {};
        const next = {
            ...base,
            post: { ...base.post, new_post: value },
        };
        setNewPostNotification(value);
        persist(next);
    };

    // I want to make a loop loading component but I got holy vibe code and
    // I don't understand it, so I'll use this way
    if (loading) {
        return <div>Loading...</div>;
    }

    return (
        <div className={style["notification-setting"]}>
            <header className={style["header"]}>
                <h2>Notification Settings</h2>
            </header>

            {/* Follow Notifications */}
            <section className={style["group"]}>
                <h3 className={style["group-title"]}>Follow</h3>
                <div className={style["row"]}>
                    <span className={style["row-label"]}>Follow request</span>
                    <label className={style["notification-toggle"]}>
                        <input
                            type="checkbox"
                            checked={notificationState.followRequest}
                            disabled={saving}
                            onChange={(e) =>
                                updateToggle("followRequest", e.target.checked)
                            }
                        />
                        <span className={style["slider"]}></span>
                    </label>
                </div>
            </section>

            {/* Message Notifications */}
            <section className={style["group"]}>
                <h3 className={style["group-title"]}>Messages</h3>
                <div className={style["row"]}>
                    <span className={style["row-label"]}>Message request</span>
                    <label className={style["notification-toggle"]}>
                        <input
                            type="checkbox"
                            checked={notificationState.messageRequest}
                            disabled={saving}
                            onChange={(e) =>
                                updateToggle("messageRequest", e.target.checked)
                            }
                        />
                        <span className={style["slider"]}></span>
                    </label>
                </div>

                <div className={style["row"]}>
                    <span className={style["row-label"]}>New message</span>
                    <label className={style["notification-toggle"]}>
                        <input
                            type="checkbox"
                            checked={notificationState.newMessage}
                            disabled={saving}
                            onChange={(e) =>
                                updateToggle("newMessage", e.target.checked)
                            }
                        />
                        <span className={style["slider"]}></span>
                    </label>
                </div>
            </section>

            {/* Post Notifications */}
            {/* I feel like, I miss things here  */}
            <section className={style["group"]}>
                <h3 className={style["group-title"]}>Posts</h3>
                <div className={style["row"]}>
                    <span className={style["row-label"]}>Comment likes</span>
                    <label className={style["notification-toggle"]}>
                        <input
                            type="checkbox"
                            checked={notificationState.commentLike}
                            disabled={saving}
                            onChange={(e) =>
                                updateToggle("commentLike", e.target.checked)
                            }
                        />
                        <span className={style["slider"]}></span>
                    </label>
                </div>

                <div className={style["row"]}>
                    <span className={style["row-label"]}>New post</span>
                    <Dropdown>
                        <DropdownTrigger asChild>
                            <button
                                type="button"
                                className={style["dropdown-trigger"]}
                                disabled={saving}
                            >
                                <span>{NOTIFICATION_ENUM_LABELS[newPostNotification]}</span>
                                <IoIosArrowDown />
                            </button>
                        </DropdownTrigger>
                        <DropdownContent className={style["dropdown-content"]}>
                            {Object.values(NOTIFICATION_ENUM_TYPES).map((option) => (
                                <DropdownItem key={option} className={style["dropdown-item"]}>
                                    <button
                                        type="button"
                                        className={`${style["dropdown-item-button"]} ${
                                            newPostNotification === option ? style["active"] : ""
                                        }`}
                                        disabled={saving}
                                        onClick={() => updateNewPost(option)}
                                    >
                                        {NOTIFICATION_ENUM_LABELS[option]}
                                    </button>
                                </DropdownItem>
                            ))}
                        </DropdownContent>
                    </Dropdown>
                </div>
            </section>
        </div>
    );
}

export { NotificationSetting };
