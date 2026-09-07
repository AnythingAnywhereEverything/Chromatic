import { ComponentType } from "react";
import style from "../scss/account-setting.module.scss";
import { UserResponse } from "@/api/user";

export interface SettingContentProps {
    title: string;
    user: UserResponse;
}

function AccountSettingContent({ title, user }: SettingContentProps) {
    if (!user) {
        return <div>Loading...</div>;
    }
    return (
        <div className={style["account-setting"]}>
            <span>{title}</span>
            <section className={style["profile"]}>
                <h2>Account info</h2>
                <div className={style["username"]}>
                    <span>Username</span>
                    <div className={style["editor"]}>
                        <span>{user.username}</span>
                        <button type="button" className={style["edit-button"]}>
                            Edit
                        </button>
                    </div>
                </div>

                <div className={style["email"]}>
                    <span>Email</span>
                    <div className={style["editor"]}>
                        <span>{user.email}</span>
                        <button type="button" className={style["edit-button"]}>
                            Edit
                        </button>
                    </div>
                </div>

                <div className={style["phone"]}>
                    <span>Phone number</span>
                    <div className={style["editor"]}>
                        <span>********12</span>
                        <button type="button" className={style["edit-button"]}>
                            Edit
                        </button>
                    </div>
                </div>
            </section>
            <section className={style["password-security"]}>
                <h2>Password & Security</h2>
                <div className={style["password"]}>
                    <span>Password</span>
                    <div className={style["editor"]}></div>
                    <button type="button" className={style["edit-button"]}>
                        Edit
                    </button>
                </div>

                <div className={style["device"]}>
                    <span>Device</span>
                    <div className={style["editor"]}></div>
                    <button type="button" className={style["edit-button"]}>
                        Edit
                    </button>
                </div>
            </section>
            <section className={style["account-deactivation"]}>
                <div className={style["deactivation"]}>
                    <span>Account Deactivation</span>
                    <div className={style["editor"]}>
                        <button type="button" className={style["edit-button"]}>
                            Deactivate Account
                        </button>
                    </div>
                </div>
                
                <div className={style["delete"]}>
                    <span>Delete Account</span>
                    <div className={style["editor"]}>
                        <button type="button" className={style["edit-button"]}>
                            Delete Account
                        </button>
                    </div>
                </div>
            </section>
        </div>
    );
}

export { AccountSettingContent };
