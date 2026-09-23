import { useEffect, useState } from "react";
import { DialogClose, DialogTrigger } from "../../dialogue";
import style from "../scss/notification-setting.module.scss";
import { SettingContentProps } from "./accountSetting";
import { getUserSettingType, UserSettingResponse } from "@/api/user";
import { Dropdown, DropdownContent, DropdownItem, DropdownTrigger } from "../../dropdown";

let initialNotificationCache: UserSettingResponse | null = null;

const NOTIFICATION_ENUM_TYPES = {
    EVERYONE: "everyone",
    FOLLOWERS: "follower",
    NONE: "none",
};

type NotificationEnumType =
    (typeof NOTIFICATION_ENUM_TYPES)[keyof typeof NOTIFICATION_ENUM_TYPES];



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

    useEffect(() => {
        if (initialNotificationCache) {
            setNotificationSetting(initialNotificationCache);
            return;
        }

        const fetchNotificationSettings = async () => {
            const res = await getUserSettingType("notification");

            if (res) {
                initialNotificationCache = res;
                setNotificationSetting(res);
                console.log(res);
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
            newPost: setting.post?.new_post === 1,
        }));
        setNewPostNotification(
            setting.post?.new_post ?? NOTIFICATION_ENUM_TYPES.FOLLOWERS,
        );
    }, [notificationSetting]);

    // I want to make a loop loading component but I got holy vibe code and
    // I don't understand it, so I'll use this way
    return (
        <div className={style["notification-setting"]}>
            <section className={style["notification"]}>
                <h2>Notification Settings</h2>

                {/* Follow Notifications */}
                <div className={style["notification-item"]}>
                    <span className={style["notification-label"]}>Follow</span>
                    <div className={style["notification-editor"]}>
                        <span>Follow request</span>
                        <label className={style["notification-toggle"]}>
                            <input
                                type="checkbox"
                                checked={notificationState.followRequest}
                                onChange={(e) =>
                                    setNotificationState((prev) => ({
                                        ...prev,
                                        followRequest: e.target.checked,
                                    }))
                                }
                            />
                            <span className={style["slider"]}></span>
                        </label>
                    </div>

                    {/* Message Notifications */}
                    <div className={style["notification-item"]}>
                        <span>Messages</span>
                        <div className={style["notification-editor"]}>
                            <span>Message request</span>
                            <label className={style["notification-toggle"]}>
                                <input
                                    type="checkbox"
                                    checked={notificationState.messageRequest}
                                    onChange={(e) =>
                                        setNotificationState((prev) => ({
                                            ...prev,
                                            messageRequest: e.target.checked,
                                        }))
                                    }
                                />
                                <span className={style["slider"]}></span>
                            </label>
                        </div>

                        <div className={style["notification-editor"]}>
                            <span>New message</span>
                            <label className={style["notification-toggle"]}>
                                <input
                                    type="checkbox"
                                    checked={notificationState.newMessage}
                                    onChange={(e) =>
                                        setNotificationState((prev) => ({
                                            ...prev,
                                            newMessage: e.target.checked,
                                        }))
                                    }
                                />
                                <span className={style["slider"]}></span>
                            </label>
                        </div>
                    </div>

                    {/* Post Notifications */}
                    {/* I feel like, I miss things here  */}
                    <div className={style["notification-item"]}>
                        <span>Posts</span>
                        <div className={style["notification-editor"]}>
                            <span>Comment likes</span>
                            <label className={style["notification-toggle"]}>
                                <input
                                    type="checkbox"
                                    checked={notificationState.commentLike}
                                    onChange={(e) =>
                                        setNotificationState((prev) => ({
                                            ...prev,
                                            commentLike: e.target.checked,
                                        }))
                                    }
                                />
                                <span className={style["slider"]}></span>
                            </label>
                        </div>

                        <div className={style["notification-editor"]}>
                            <span>New post</span>
                            <Dropdown>
                               <DropdownTrigger>What</DropdownTrigger>
                            </Dropdown>
                        </div>
                    </div>
                </div>
            </section>
        </div>
    );
}

export { NotificationSetting };
