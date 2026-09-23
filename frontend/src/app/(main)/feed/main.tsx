"use client";

import style from "./style.module.scss";
import PostGroup from "./feedLayout";
import PostRightLayout from "./rightside";

export const PostPage: React.FC = () => {
    return (
        <div className={style["post-layout"]}>
            <div className={style["post-left-layout"]}>
                <PostGroup />
            </div>
        </div>
    );
};
