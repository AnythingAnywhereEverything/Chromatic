"use client";

import { useState, useEffect } from "react";
import { useWindowWidth } from "@lib/utils";
import { DesktopLayout } from "./desktop";
import { MobileLayout } from "./mobile";

import mStyle from "./mobile/style.module.scss";
import dStyle from "./desktop/style.module.scss";

export default function MainLayout({ children }: { children: React.ReactNode }) {
    const [isMounted, setIsMounted] = useState(false);
    const width = useWindowWidth();

    useEffect(() => {
        setIsMounted(true);
    }, []);

    if (!isMounted) {
        return null; 
    }

    const isMobile = width !== null && width <= 768;

    const style = isMobile ? mStyle : dStyle;

    // * Perserve the children while switching between mobile and desktop layouts, so that the state of the children is not lost
    return (
        <div className={style["main-layout"]}>
            {isMobile ? <MobileLayout.Topbar /> : <DesktopLayout.Sidebar />}
            <div className={style["main-container"]}>{children}</div>
            {isMobile && <MobileLayout.BottomBar />}
        </div>
    )
}