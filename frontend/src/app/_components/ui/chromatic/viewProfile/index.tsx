import {
    getPublicUserProfile,
    PublicUserProfileResponse,
    updateUserProfile,
} from "@/api/user/profile";
import { useProfile } from "@/hooks/useProfile";
import React, { useEffect, useState } from "react";
import {
    Dialog,
    DialogClose,
    DialogContent,
    DialogTrigger,
} from "@/app/_components/ui/chromatic/dialogue";
import style from "./viewProfile.module.scss";
import ViewProfileBody, { ProfilePayload } from "./body";
import { Portal } from "@/app/_components/portal";
import { useUser } from "@/hooks/useUser";
import { useUserService } from "@/hooks/useUserService";
import { getUserFeed, mediaPostProps } from "@/api/post/getFeed";

interface ViewProfileOptions {
    username: string;
    containerRef?: React.RefObject<HTMLDivElement>;
    open?: boolean;
    onOpenSettings?: () => void;
}

function useViewProfile({
    username,
    containerRef,
    open,
    onOpenSettings,
}: ViewProfileOptions) {
    const [isOpen, setIsOpen] = useState(open ?? false);
    const [profile, setProfile] = useState<PublicUserProfileResponse | null>(
        null,
    );
    const [isOwner, setIsOwner] = useState(false);
    const [recentPost, setRecentPost] = React.useState<mediaPostProps[]>([]);

    // use user service to get the current user id
    const cUser = useProfile();

    useEffect(() => {
        if (!isOpen) return;

        const fetchProfile = async () => {
            try {
                const data = await getPublicUserProfile(username);

                if (!data) {
                    console.error("Profile not found for username:", username);
                    return;
                }

                setProfile(data);
                setIsOwner(cUser?.data?.id === data.id);
            } catch (error) {
                console.error("Error fetching profile:", error);
            }
        };

        fetchProfile();
    }, [username, isOpen, cUser?.data?.id]);

    // If is owner use profile from user service
    useEffect(() => {
        if (!isOwner || !cUser?.data || isOpen) {
            return;
        }

        setProfile(cUser.data);
    }, [isOwner, isOpen, cUser?.data]);

    const openProfile = () => {
        setIsOpen(true);
    };

    const closeProfile = () => {
        setIsOpen(false);
    };

    useEffect(() => {
        if (open !== undefined) {
            setIsOpen(open);
        }
    }, [open]);

    // React.useEffect(() => {
    //   async function fetchFeed() {
    //     const res = await getUserFeed();
    //     setRecentPost(res);
    //   }
    //   fetchFeed();
    // }, []);

    return React.useMemo(
        () => ({
            username,
            containerRef,
            isOpen,
            openProfile,
            closeProfile,
            onOpenSettings,
            profile,
            isOwner,
            setProfile,
            recentPost,
        }),
        [
            isOpen,
            onOpenSettings,
            username,
            containerRef,
            profile,
            isOwner,
            setProfile,
            recentPost,
        ],
    );
}

type ViewProfileContextType = ReturnType<typeof useViewProfile>;

const ViewProfileContext = React.createContext<ViewProfileContextType | null>(
    null,
);

const ViewProfile = ({
    children,
    ...options
}: { children: React.ReactNode } & ViewProfileOptions): React.ReactNode => {
    const contextValue = useViewProfile(options);

    const [payload, setPayload] = useState<ProfilePayload>({});
    const [hasChanges, setHasChanges] = useState(false);
    const [isSaving, setIsSaving] = useState(false);

    useEffect(() => {
        const hasChanges = Object.keys(payload).length > 0;

        setHasChanges(hasChanges);
    }, [payload]);

    const handleChange = (newPayload: ProfilePayload) => {
        setPayload((prevPayload) => {
            const updatedPayload = { ...prevPayload, ...newPayload };
            Object.keys(updatedPayload).forEach((key) => {
                if (updatedPayload[key as keyof ProfilePayload] === undefined) {
                    delete updatedPayload[key as keyof ProfilePayload];
                }
            });
            return updatedPayload;
        });
    };

    const profileService = useUserService();
    const handleSave = async () => {
        if (isSaving) return;
        setIsSaving(true);
        const formData = new FormData();
        if (payload.display_name !== undefined) {
            formData.append("display_name", payload.display_name);
        }
        if (payload.uploaded_avatar) {
            formData.append("uploaded_avatar", payload.uploaded_avatar);
        }
        if (payload.remove_avatar !== undefined) {
            formData.append("remove_avatar", "true");
        }
        if (payload.uploaded_banner) {
            formData.append("uploaded_banner", payload.uploaded_banner);
        }
        if (payload.remove_banner !== undefined) {
            formData.append("remove_banner", "true");
        }
        if (payload.quote !== undefined) {
            formData.append("quote", payload.quote);
        }

        const data = await profileService.updateUserProfile.mutateAsync(formData);
        setIsSaving(false);
        contextValue.setProfile(data);
        setPayload({});
    };

    return (
        <ViewProfileContext.Provider value={contextValue}>
            <Dialog
                overlayClassName={style["root-overlay"]}
                onClose={() => {
                    if (hasChanges) {
                        return false;
                    }
                    contextValue.closeProfile();
                    return true;
                }}
                interactable={!isSaving}
            >
                <DialogTrigger
                    asChild
                    onClick={() => {
                        contextValue.openProfile();
                    }}
                >
                    {children}
                </DialogTrigger>

                {contextValue.profile && (
                    <DialogContent className={style["root-content"]}>
                        <ViewProfileBody
                            profile={contextValue.profile}
                            is_owner={contextValue.isOwner}
                            payload={payload}
                            onChange={(newPayload) => {
                                console.log(
                                    "ViewProfile: onChange",
                                    newPayload,
                                );
                                handleChange(newPayload);
                            }}
                        />
                    </DialogContent>
                )}
            </Dialog>
            { contextValue.isOwner && 
                <Portal>
                    <div className={style["unsaved-changes-wrapper"]}>
                        <div
                            className={style["unsaved-changes"]}
                            data-state={
                                contextValue.isOpen && hasChanges
                                    ? "open"
                                    : "closed"
                            }
                        >
                            <span className={style["unsaved-changes-text"]}>
                                You have unsaved changes.
                            </span>
                            <button
                                className={style["unsaved-cancel-button"]}
                                onClick={() => {
                                    setPayload({});
                                }}
                                disabled={isSaving}
                            >
                                Cancel
                            </button>
                            <button
                                className={style["unsaved-save-button"]}
                                onClick={handleSave}
                                disabled={isSaving}
                            >
                                {isSaving ? (
                                    <svg
                                        xmlns="http://www.w3.org/2000/svg"
                                        viewBox="0 0 200 200"
                                    >
                                        <circle
                                            fill="var(--text-neutral)"
                                            stroke="var(--text-neutral)"
                                            strokeWidth="15"
                                            r="15"
                                            cx="40"
                                            cy="100"
                                        >
                                            <animate
                                                attributeName="opacity"
                                                calcMode="spline"
                                                dur="2"
                                                values="1;0;1;"
                                                keySplines=".5 0 .5 1;.5 0 .5 1"
                                                repeatCount="indefinite"
                                                begin="-.4"
                                            ></animate>
                                        </circle>
                                        <circle
                                            fill="var(--text-neutral)"
                                            stroke="var(--text-neutral)"
                                            strokeWidth="15"
                                            r="15"
                                            cx="100"
                                            cy="100"
                                        >
                                            <animate
                                                attributeName="opacity"
                                                calcMode="spline"
                                                dur="2"
                                                values="1;0;1;"
                                                keySplines=".5 0 .5 1;.5 0 .5 1"
                                                repeatCount="indefinite"
                                                begin="-.2"
                                            ></animate>
                                        </circle>
                                        <circle
                                            fill="var(--text-neutral)"
                                            stroke="var(--text-neutral)"
                                            strokeWidth="15"
                                            r="15"
                                            cx="160"
                                            cy="100"
                                        >
                                            <animate
                                                attributeName="opacity"
                                                calcMode="spline"
                                                dur="2"
                                                values="1;0;1;"
                                                keySplines=".5 0 .5 1;.5 0 .5 1"
                                                repeatCount="indefinite"
                                                begin="0"
                                            ></animate>
                                        </circle>
                                    </svg>
                                ) : (
                                    "Save"
                                )}
                            </button>
                        </div>
                    </div>
                </Portal>
            }
        </ViewProfileContext.Provider>
    );
};

export { ViewProfile };
