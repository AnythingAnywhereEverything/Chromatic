import ExploreBody from "./body";

export const metadata = {
    title: "Explore Page", // Let next js handle title and description for SEO purposes
    description: "This is the explore page",
};

export default function ExplorePage({
    children,
}: {
    children: React.ReactNode;
}) {
    // Strictly import and use the components hete
    // due to it being a SSR page, and not a client component. This is to avoid hydration errors.
    return <ExploreBody />;
}