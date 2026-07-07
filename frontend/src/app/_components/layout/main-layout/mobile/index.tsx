"use client";

import BottomBar from "./bottom-bar";
import style from "./style.module.scss";
import Topbar from "./top-bar";

export default function MainMobileLayout({
    children,
}: {
    children: React.ReactNode;
}) {
    return (
        <div className={style["main-layout"]}>
            <Topbar />
            <div className={style["main-container"]}>{children}</div>
            <BottomBar />
        </div>
    );
}