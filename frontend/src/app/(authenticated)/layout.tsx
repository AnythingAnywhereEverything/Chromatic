"use client";

import { useUser } from "@/hooks/useUser";
import { useRouter } from "next/navigation";
import { useEffect } from "react";

export default function AuthenticatedLayout({
    children,
}: {
    children: React.ReactNode;
}) {
    const { data: user, isLoading } = useUser();
    const router = useRouter();

    // Kick out unauthenticated users instantly from sub-pages
    useEffect(() => {
        if (!isLoading && !user) {
            router.replace("/");
        }
    }, [user, isLoading, router]);

    if (isLoading || !user) {
        return (
            <div>
                <p>Loading application...</p>
            </div>
        );
    }

    return (
        <main className="page-body">
            {children}
        </main>
    );
}
