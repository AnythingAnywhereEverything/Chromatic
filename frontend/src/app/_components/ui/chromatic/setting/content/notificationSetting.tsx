import { DialogClose } from "../../dialogue";
import style from "../scss/notification-setting.module.scss";
import { SettingContentProps } from "./accountSetting";

function NotificationSetting({ title, user }: SettingContentProps) {
    
    // ? I hope this will work as reusable
    const switchToggle = (value: boolean) => {
        return (
            <label className={style["notification-toggle"]}>
                <input type="checkbox" 
                    checked={value}
                    onChange={(e) => switchToggle(e.target.checked)}
                />
                <span className={style["slider"]}></span>
            </label>
        )
    }

    return (
        <div className={style["notification-setting"]}>
            <section>
                <span>{title}</span>
                <DialogClose>X</DialogClose>
            </section>
            <section className={style["notification"]}>
                <h2>Overview</h2>
                <div className={style["notification-overview"]}>
                    <span>Notify me when...</span>

                    <div className={style["notification-item"]}>
                        <span>Get friend request</span>
                        <label className={style["notification-toggle"]}>
                            <input type="checkbox" />
                            <span className={style["slider"]}></span>
                        </label>
                    </div>

                    <div className={style["notification-item"]}>
                        <span>Receive newsletter</span>
                        {switchToggle(false)}
                    </div>
                </div>
            </section>
        </div>
    );
}

export { NotificationSetting };