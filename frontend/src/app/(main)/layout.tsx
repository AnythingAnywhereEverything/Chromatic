"use client";

import MainLayout from "../_components/layout/main-layout";

export default function AuthShell({ children }: { children: React.ReactNode }) {
    return (
        <MainLayout>
            <main
                style={{
                    display: "flex",
                    flexDirection: "column",
                    width: "100%",
                }}
            >
                {children}
            </main>
        </MainLayout>
    );
}
