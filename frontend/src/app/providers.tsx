"use client";

import { useRef, useState } from "react";
import { GoogleOAuthProvider } from "@react-oauth/google";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ThemeProvider } from "next-themes";
import { PortalProvider } from "./_components/portal";

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

    const portalRootRef = useRef<HTMLDivElement | null>(null);

    return (
        <ThemeProvider enableSystem>
            <QueryClientProvider client={queryClient}>
                <GoogleOAuthProvider
                    clientId={process.env.NEXT_PUBLIC_GOOGLE_CLIENT_ID!}
                >
                    <PortalProvider container={portalRootRef.current}>
                        {children}
                    </PortalProvider>
                    <div 
                        id="portal-root" 
                        ref={portalRootRef}
                        style={{ 
                            position: "fixed",
                            zIndex: 9999,
                            width: "100vw",
                            height: "100vh",
                            pointerEvents: "none",
                            top: 0,
                            left: 0,
                        }}
                    />
                </GoogleOAuthProvider>
            </QueryClientProvider>
        </ThemeProvider>
    );
}
