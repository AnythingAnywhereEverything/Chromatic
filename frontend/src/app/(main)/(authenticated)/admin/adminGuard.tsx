"use client";

import { useQuery } from "@tanstack/react-query";
import { useRouter } from "next/navigation";
import { useEffect } from "react";
import { getAdminStats } from "@/api/admin/stats";
import { AdminApiError } from "@/api/admin/types";
import AdminNavigator from "./adminNavigator";
import style from "./admin.module.scss";

export const AdminGuard = ({ children }: { children: React.ReactNode }) => {
    const router = useRouter();

    const { data, isLoading, error } = useQuery({
        queryKey: ["admin-gate"],
        queryFn: getAdminStats,
        staleTime: 60 * 1000,
        retry: false,
    });

    const noSession = error instanceof AdminApiError && error.status === 401;

    useEffect(() => {
        if (noSession) {
            router.push("/auth/signin");
        }
    }, [noSession, router]);

    if (isLoading) {
        return null;
    }

    if (!data) {
        return (
            <section className={style["access-denied"]}>
                <h1>Access denied</h1>
                <p>Only administrators can view this panel.</p>
            </section>
        );
    }

    return (
        <article className={style["admin-body"]}>
            <AdminNavigator />
            {children}
        </article>
    );
};