"use client";

import { useState } from "react";
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

    return (
        <ThemeProvider enableSystem>
            <QueryClientProvider client={queryClient}>
                <GoogleOAuthProvider
                    clientId={process.env.NEXT_PUBLIC_GOOGLE_CLIENT_ID!}
                >
                    <PortalProvider 
                        className="portalProvider"
                        style={{ 
                            position: "fixed",
                            top: 0,
                            left: 0,
                            width: "100vw",
                            height: "100vh",
                            overflow: "hidden",
                            pointerEvents: "none",
                            zIndex: 9999,
                        }}
                    >
                        {children}
                    </PortalProvider>
                </GoogleOAuthProvider>
            </QueryClientProvider>
        </ThemeProvider>
    );
}
