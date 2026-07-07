// app/layout.tsx
import "@styles/global.scss";
import Providers from "./providers";
import { Metadata } from "next";

export const metadata: Metadata = {
    title: "Chromatic",
    description: "Chromatic is a social media platform that allows users to share and discover knowledge.",
    openGraph: {
        title: "Chromatic",
        description: "Chromatic is a social media platform that allows users to share and discover knowledge.",
        url: "https://www.chromatic.com",
        siteName: "Chromatic",
        images: [
            {
                url: "https://www.chromatic.com/og-image.png",
                width: 800,
                height: 600,
            },
        ],
        locale: "en_US",
        type: "website",
    },
};

export default function RootLayout({
    children,
}: {
    children: React.ReactNode;
}) {
    return (
        <html lang="en" suppressHydrationWarning>
            <body>
                <Providers>{children}</Providers>
            </body>
        </html>
    );
}
