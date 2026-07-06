"use client";

import { useState } from "react";
import { GoogleOAuthProvider } from "@react-oauth/google";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { LayerProvider } from "./_components/layer";
import { ThemeProvider } from "next-themes";

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
                    <LayerProvider>
                        {children}
                    </LayerProvider>
                </GoogleOAuthProvider>
            </QueryClientProvider>
        </ThemeProvider>
    );
}
