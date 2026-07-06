"use client";

import { useUser } from "@/hooks/useUser";
import MainLayout from "../_components/layout/main-layout";

export default function AuthShell({
    children,
}: {
    children: React.ReactNode;
}) {

    return <MainLayout>{children}</MainLayout>;
}