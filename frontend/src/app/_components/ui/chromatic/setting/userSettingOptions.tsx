import { PostAvatar } from "../post/header/avatar";
import style from "./scss/user-settingOptions.module.scss";
import { UserResponse } from "@/api/user";

interface options {
    user: UserResponse
    option: string
    onChangeOption: (option: string) => void;
    list: string[]
}

function UserSettingOption({user , option, list, onChangeOption}: options) {
    return (
        <section className={style["setting-options"]}>
            {/* //? if you want to add the View Avatar body here */}
            <div className={style["setting-profile"]}>
                <PostAvatar
                    userId={user.id}
                    username={user.username}
                    displayName={user.display_name}
                    avatar={user.avatar}
                    thumbhash={user.avatar_thumbhash}
                    width={48}
                    height={48}
                />
                {user.display_name ? (
                    <div className={style["profile"]}>
                        <span>{user.display_name}</span>
                        <span className={style["username"]}>{user.username}</span>
                    </div>
                ) : (
                    <span>{user.username}</span>
                )}
            </div>

            <div className={style["options"]}>
                {list.map((item) => (
                    <button 
                    key={item}
                    type="button" 
                    className={` ${style["option"]}
                    ${option === item ? style["active"] : ""}`}
                    onClick={() => {onChangeOption(item)}}>
                        <span>{item}</span>
                    </button>
                ))}
            </div>
        </section>
    );
}

export { UserSettingOption };
