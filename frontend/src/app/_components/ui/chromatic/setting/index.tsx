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
import { AccountSettingContent, SettingContentProps } from "./content/accountSetting";
import { useUser } from "@/hooks/useUser";
import { UserResponse } from "@/api/user";
import { NotificationSetting } from "./content/notificationSetting";


function UserSetting() {
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
        notification: {
            title: "Notification",
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
    // ? not quite a good idea
    const CurrentSetting = SettingMap[settingContent]?.component;
    const currentSetting = SettingMap[settingContent];

    useEffect(() => {
        if (currentUser && currentUser.data) {
            setUser(currentUser.data);
        }
    }, [currentUser, currentUser.data]);

    useEffect(() => {
        if (!SettingMap[settingContent]) {
            setSettingContent("account");
        }
    }, [settingContent]);

    if (!user) {
        return;
    }
    return (
        <Dialog outsidePress={false}>
            <DialogTrigger asChild>
                <button type="button">
                    <Tooltip>
                        <TooltipTrigger asChild>
                            <IoSettingsSharp />
                        </TooltipTrigger>
                        <TooltipContent>User setting</TooltipContent>
                    </Tooltip>
                </button>
            </DialogTrigger>

            <DialogContent className={style["setting-container"]}>
                <UserSettingOption
                    option={settingContent}
                    user={user}
                    onChangeOption={setSettingContent}
                    list={Object.keys(SettingMap)}
                />
                <section className={style["content"]}>
                    <CurrentSetting title={currentSetting.title} user={user} />
                </section>
            </DialogContent>
        </Dialog>
    );
}

export { UserSetting };
