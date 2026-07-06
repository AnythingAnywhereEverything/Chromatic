export const metadata = {
    title: "User's Profile Name", // Let next js handle title and description for SEO purposes
    description: "This is the profile page",
};

export default function ProfilePage({
    children,
}: {
    children: React.ReactNode;
}) {
    // Strictly import and use the components hete
    // due to it being a SSR page, and not a client component. This is to avoid hydration errors.
    return <div>{children}</div>;
}