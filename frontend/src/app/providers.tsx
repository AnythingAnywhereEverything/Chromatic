"use client";

import { useState } from "react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ThemeProvider } from "next-themes";
import { PortalProvider } from "./_components/portal";
import { RealtimeProvider } from "./realtime";

export default function Providers({ children }: { children: React.ReactNode }) {
    const [queryClient] = useState(
        () =>
            new QueryClient({
                defaultOptions: {
                    queries: {
                        staleTime: 60 * 1000, // 1 minute
                    },
                },
            }),
    );

    const [portalRoot, setPortalRoot] = useState<HTMLDivElement | null>(null);

    return (
        <ThemeProvider enableSystem>
            <QueryClientProvider client={queryClient}>
                <PortalProvider container={portalRoot}>
                    <RealtimeProvider>{children}</RealtimeProvider>
                </PortalProvider>
                <div
                    data-portal-root
                    ref={setPortalRoot}
                    style={{
                        position: "fixed",
                        top: 0,
                        left: 0,
                        width: "100%",
                        height: "100%",
                        pointerEvents: "none",
                        zIndex: 9999,
                        // overflow: "hidden",
                    }}
                />
            </QueryClientProvider>
        </ThemeProvider>
    );
}
