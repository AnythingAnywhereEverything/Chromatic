// app/layout.tsx
import "@styles/global.scss";
import Providers from "./providers";
import { Metadata } from "next";

export const metadata: Metadata = {
    title: "Webapp",
    description: "Web Application",
    openGraph: {
        title: "Webapp",
        description: "Web Application",
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
        <html lang="en">
            <body>
                <Providers>{children}</Providers>
            </body>
        </html>
    );
}
