"use client";

import { useState, useEffect } from "react";

import { useIsMobile } from "@lib/utils";

import { DesktopLayout } from "./desktop";
import { MobileLayout } from "./mobile";

import mStyle from "./mobile/style.module.scss";
import dStyle from "./desktop/style.module.scss";
import gStyle from "./guard/style.module.scss";

import { useUser } from "@/hooks/useUser";
import GuardTopBar from "./guard";

export default function MainLayout({
    children,
}: {
    children: React.ReactNode;
}) {
    const [isMounted, setIsMounted] = useState(false);

    const isMobile = useIsMobile();
    const userData = useUser();

    useEffect(() => {
        setIsMounted(true);
    }, []);

    if (!isMounted || isMobile === null) {
        return null;
    }

    const isAuthenticated = !userData?.error;

    const style = isAuthenticated
        ? isMobile
            ? mStyle
            : dStyle
        : gStyle;

    return (
        <div className={style["main-layout"]}>
            {!isAuthenticated && <GuardTopBar />}

            {isAuthenticated &&
                (isMobile ? (
                    <MobileLayout.Topbar />
                ) : (
                    <DesktopLayout.Sidebar />
                ))}

            <div className={style["main-container"]}>
                {children}
            </div>

            {isAuthenticated && isMobile && <MobileLayout.BottomBar />}
        </div>
    );
}