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

    const [portalRoot, setPortalRoot] = useState<HTMLDivElement | null>(null);

    return (
        <ThemeProvider enableSystem>
            <QueryClientProvider client={queryClient}>
                <GoogleOAuthProvider
                    clientId={process.env.NEXT_PUBLIC_GOOGLE_CLIENT_ID!}
                >
                    <PortalProvider container={portalRoot}>
                        {children}
                    </PortalProvider>
                    <div data-portal-root ref={setPortalRoot} />
                </GoogleOAuthProvider>
            </QueryClientProvider>
        </ThemeProvider>
    );
}
