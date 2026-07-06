"use client";

import style from "./style.module.scss";
import Sidebar from "./sidebar";

export default function MainDesktopLayout({
    children,
}: {
    children: React.ReactNode;
}) {
    return (
        <div className={style["main-layout"]}>
            <Sidebar />
            <div className={style["main-container"]}>{children}</div>
        </div>
    );
}