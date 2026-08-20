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

    // Commented  for frontend only development purposes, to allow access to authenticated pages without login
    // useEffect(() => {
    //     if (!isLoading && !user) {
    //         router.replace("/");
    //     }
    // }, [user, isLoading, router]);

    // if (!user) {
    //     return (
    //         <div>
    //             <p>Loading application...</p>
    //         </div>
    //     );
    // }

    return (
        <main style={{ display: "flex", flexDirection: "column", width: "100%" }}>
            {children}
        </main>
    );
}
