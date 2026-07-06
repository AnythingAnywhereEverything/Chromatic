"use client";

import { useState } from "react";
import { GoogleOAuthProvider } from "@react-oauth/google";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { LayerProvider } from "./_components/layer";

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
        <QueryClientProvider client={queryClient}>
            <GoogleOAuthProvider
                clientId={process.env.NEXT_PUBLIC_GOOGLE_CLIENT_ID!}
            >
                <LayerProvider>
                    {children}
                </LayerProvider>
            </GoogleOAuthProvider>
        </QueryClientProvider>
    );
}
