"use client";

import { useState, useEffect } from "react";
import { useWindowWidth } from "@lib/utils";
import { DesktopLayout } from "./desktop";
import { MobileLayout } from "./mobile";

import mStyle from "./mobile/style.module.scss";
import dStyle from "./desktop/style.module.scss";
import gStyle from "./guard/style.module.scss";
import { useUser } from "@/hooks/useUser";
import GuardTopBar from "./guard";

export default function MainLayout({ children }: { children: React.ReactNode }) {
    const [isMounted, setIsMounted] = useState(false);
    const width = useWindowWidth();

    useEffect(() => {
        setIsMounted(true);
    }, []);
    
    let userData = useUser();

    if (!isMounted) {
        return null; 
    }

    const isAuthenticated = userData?.error ? false : true;

    console.log("isAuthenticated", isAuthenticated);
    console.log("userData", userData);

    const isMobile = width !== null && width <= 768;

    const style = isAuthenticated ? (isMobile ? mStyle : dStyle) : gStyle;

    // * Perserve the children while switching between mobile and desktop layouts, so that the state of the children is not lost
    return (
        <div className={style["main-layout"]}>
            {!isAuthenticated && (
                <GuardTopBar />
            )}
            {isAuthenticated && (isMobile ? <MobileLayout.Topbar /> : <DesktopLayout.Sidebar />)}
            <div className={style["main-container"]}>{children}</div>
            {isAuthenticated && isMobile && <MobileLayout.BottomBar />}
        </div>
    )
}