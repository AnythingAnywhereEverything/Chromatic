"use client";

import { IoSettingsSharp } from "react-icons/io5";
import {
    Dialog,
    DialogClose,
    DialogContent,
    DialogHeading,
    DialogTrigger,
} from "../dialogue";
import { Tooltip, TooltipContent, TooltipTrigger } from "../tooltip";
import style from "./scss/setting.module.scss";
import { ComponentType, ReactNode, useEffect, useState } from "react";
import { UserSettingOption } from "./userSettingOptions";
import {
    AccountSettingContent,
    SettingContentProps,
} from "./content/accountSetting";
import { useUser } from "@/hooks/useUser";
import { UserResponse, UserSettingResponse } from "@/api/user";
import { NotificationSetting } from "./content/notificationSetting";

function UserSetting({
    open,
    onOpenChange,
}: {
    children?: ReactNode;
    open: boolean;
    onOpenChange: (open: boolean) => void;
}) {
    const SettingMap: Record<
        string,
        {
            title: string;
            component: ComponentType<SettingContentProps>;
        }
    > = {
        account: {
            title: "Account",
            component: AccountSettingContent,
        },
        notifications: {
            title: "Notifications",
            component: NotificationSetting,
        },
        privacy: {
            title: "Privacy",
            component: () => <div>Privacy Setting</div>,
        },
        logout: {
            title: "Logout",
            component: () => <div>Logout</div>,
        },
    };

    const [user, setUser] = useState<UserResponse | null>(null);
    const [settingContent, setSettingContent] = useState("account");
    const currentUser = useUser();

    const CurrentSetting = SettingMap[settingContent]?.component;
    const currentSetting = SettingMap[settingContent];

    useEffect(() => {
        if (currentUser?.data) {
            setUser(currentUser.data);
        }
    }, [currentUser]);

    useEffect(() => {
        if (!SettingMap[settingContent]) {
            setSettingContent("account");
        }
    }, [settingContent]);

    if (!user) {
        return null;
    }

    return (
        <Dialog open={open} onOpenChange={onOpenChange}>
            <DialogContent className={style["setting-container"]}>
                <UserSettingOption
                    option={settingContent}
                    user={user}
                    onChangeOption={setSettingContent}
                    list={Object.keys(SettingMap)}
                />

                <section className={style["content"]}>
                    <div className={style["header"]}>
                        <span>{currentSetting.title}</span>
                        <DialogClose>X</DialogClose>
                    </div>
                    <div className={style["content-body"]}>
                        <CurrentSetting user={user} />
                    </div>
                </section>
            </DialogContent>
        </Dialog>
    );
}

export { UserSetting };
