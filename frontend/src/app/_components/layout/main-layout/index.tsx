"use client";

import { useState, useEffect } from "react";
import { useWindowWidth } from "@lib/utils";
import MainDesktopLayout from "./desktop";
import MainMobileLayout from "./mobile";

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

    return isMobile ? (
        <MainMobileLayout>{children}</MainMobileLayout>
    ) : (
        <MainDesktopLayout>{children}</MainDesktopLayout>
    );
}