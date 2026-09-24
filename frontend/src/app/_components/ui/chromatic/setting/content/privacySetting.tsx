import { useEffect, useState } from "react";
import style from "../scss/privacy-setting.module.scss";
import { SettingContentProps } from "./accountSetting";
import { getUserSettingType, updateUserSettingType, UserSettingResponse } from "@/api/user";
import {
    PendingFollow,
    acceptFollowRequest,
    getPendingFollowRequests,
    rejectFollowRequest,
} from "@/api/user/follow";
import {
    Dropdown,
    DropdownContent,
    DropdownItem,
    DropdownTrigger,
} from "../../dropdown";
import { IoIosArrowDown } from "react-icons/io";
import { PostAvatar } from "../../post/header/avatar";

const PROFILE_VISIBILITY_OPTIONS = ["everyone", "private"];
const WHO_CAN_FOLLOW_OPTIONS = ["everyone", "request"];

const PROFILE_VISIBILITY_LABELS: Record<string, string> = {
    everyone: "Everyone",
    private: "Private",
};

const WHO_CAN_FOLLOW_LABELS: Record<string, string> = {
    everyone: "Everyone",
    request: "Request",
};

function PrivacySettingContent({ user }: SettingContentProps) {
    const [privacySetting, setPrivacySetting] = useState<UserSettingResponse | null>(null);
    const [profileVisibility, setProfileVisibility] = useState("everyone");
    const [whoCanFollowMe, setWhoCanFollowMe] = useState("everyone");
    const [pendingRequests, setPendingRequests] = useState<PendingFollow[]>([]);

    const [loading, setLoading] = useState(true);
    const [saving, setSaving] = useState(false);
    const [busyId, setBusyId] = useState<string | null>(null);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        const load = async () => {
            try {
                const [privacy, requests] = await Promise.all([
                    getUserSettingType("privacy"),
                    getPendingFollowRequests(),
                ]);

                if (privacy) {
                    setPrivacySetting(privacy);
                    setProfileVisibility(privacy.setting_value?.profile_visibility ?? "everyone");
                    setWhoCanFollowMe(privacy.setting_value?.who_can_follow_me ?? "everyone");
                }
                if (requests) {
                    setPendingRequests(requests);
                }
            } catch (e) {
                setError(e instanceof Error ? e.message : "Failed to load privacy settings");
            } finally {
                setLoading(false);
            }
        };

        load();
    }, []);

    const persist = async (next: { profile_visibility: string; who_can_follow_me: string }) => {
        setSaving(true);
        setError(null);
        try {
            const updated = await updateUserSettingType("privacy", next);
            if (updated) setPrivacySetting(updated);
            setProfileVisibility(next.profile_visibility);
            setWhoCanFollowMe(next.who_can_follow_me);
        } catch (e) {
            setError(e instanceof Error ? e.message : "Failed to save setting");
        } finally {
            setSaving(false);
        }
    };

    const handleWhoCanFollow = (value: string) => {
        setWhoCanFollowMe(value);
        persist({ profile_visibility: profileVisibility, who_can_follow_me: value });
    };

    const handleProfileVisibility = (value: string) => {
        setProfileVisibility(value);
        persist({ profile_visibility: value, who_can_follow_me: whoCanFollowMe });
    };

    const handleAccept = async (followerId: string) => {
        setBusyId(followerId);
        setError(null);
        try {
            await acceptFollowRequest(followerId);
            setPendingRequests((prev) => prev.filter((r) => r.follower_id !== followerId));
        } catch (e) {
            setError(e instanceof Error ? e.message : "Failed to accept request");
        } finally {
            setBusyId(null);
        }
    };

    const handleReject = async (followerId: string) => {
        setBusyId(followerId);
        setError(null);
        try {
            await rejectFollowRequest(followerId);
            setPendingRequests((prev) => prev.filter((r) => r.follower_id !== followerId));
        } catch (e) {
            setError(e instanceof Error ? e.message : "Failed to reject request");
        } finally {
            setBusyId(null);
        }
    };

    if (loading) {
        return <div className={style["loading"]}>Loading...</div>;
    }

    return (
        <div className={style["privacy-setting"]}>
            {error && <div className={style["empty"]}>{error}</div>}

            <section className={style["section"]}>
                <h2>Profile</h2>

                <div className={style["item"]}>
                    <span>Who can follow me</span>
                    <Dropdown>
                        <DropdownTrigger asChild>
                            <button
                                type="button"
                                className={style["dropdown-trigger"]}
                                disabled={saving}
                            >
                                <span>{WHO_CAN_FOLLOW_LABELS[whoCanFollowMe] ?? whoCanFollowMe}</span>
                                <IoIosArrowDown />
                            </button>
                        </DropdownTrigger>
                        <DropdownContent className={style["dropdown-content"]}>
                            {WHO_CAN_FOLLOW_OPTIONS.map((option) => (
                                <DropdownItem key={option} className={style["dropdown-item"]}>
                                    <button
                                        type="button"
                                        className={`${style["dropdown-item-button"]} ${
                                            whoCanFollowMe === option ? style["active"] : ""
                                        }`}
                                        disabled={saving}
                                        onClick={() => handleWhoCanFollow(option)}
                                    >
                                        {WHO_CAN_FOLLOW_LABELS[option]}
                                    </button>
                                </DropdownItem>
                            ))}
                        </DropdownContent>
                    </Dropdown>
                </div>

                <div className={style["item"]}>
                    <span>Profile visibility</span>
                    <Dropdown>
                        <DropdownTrigger asChild>
                            <button
                                type="button"
                                className={style["dropdown-trigger"]}
                                disabled={saving}
                            >
                                <span>{PROFILE_VISIBILITY_LABELS[profileVisibility] ?? profileVisibility}</span>
                                <IoIosArrowDown />
                            </button>
                        </DropdownTrigger>
                        <DropdownContent className={style["dropdown-content"]}>
                            {PROFILE_VISIBILITY_OPTIONS.map((option) => (
                                <DropdownItem key={option} className={style["dropdown-item"]}>
                                    <button
                                        type="button"
                                        className={`${style["dropdown-item-button"]} ${
                                            profileVisibility === option ? style["active"] : ""
                                        }`}
                                        disabled={saving}
                                        onClick={() => handleProfileVisibility(option)}
                                    >
                                        {PROFILE_VISIBILITY_LABELS[option]}
                                    </button>
                                </DropdownItem>
                            ))}
                        </DropdownContent>
                    </Dropdown>
                </div>
            </section>

            <section className={style["section"]}>
                <h2>Pending follow requests</h2>

                {pendingRequests.length === 0 ? (
                    <div className={style["empty"]}>No pending requests.</div>
                ) : (
                    <div className={style["request-list"]}>
                        {pendingRequests.map((request) => (
                            <div key={request.follower_id} className={style["request-item"]}>
                                <div className={style["request-user"]}>
                                    <PostAvatar
                                        userId={request.follower_id}
                                        username={request.username}
                                        displayName={request.display_name ?? request.username}
                                        avatar={request.avatar}
                                        thumbhash={request.avatar_thumbhash}
                                        width={32}
                                        height={32}
                                    />
                                    <div className={style["request-name"]}>
                                        <span>{request.display_name ?? request.username}</span>
                                        <span className={style["username"]}>@{request.username}</span>
                                    </div>
                                </div>

                                <div className={style["request-actions"]}>
                                    <button
                                        type="button"
                                        className={`${style["button"]} ${style["accept"]}`}
                                        disabled={busyId === request.follower_id}
                                        onClick={() => handleAccept(request.follower_id)}
                                    >
                                        Accept
                                    </button>
                                    <button
                                        type="button"
                                        className={style["button"]}
                                        disabled={busyId === request.follower_id}
                                        onClick={() => handleReject(request.follower_id)}
                                    >
                                        Reject
                                    </button>
                                </div>
                            </div>
                        ))}
                    </div>
                )}
            </section>
        </div>
    );
}

export { PrivacySettingContent };
