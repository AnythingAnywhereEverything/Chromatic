
import AuthenticatedLayout from "../layout";
import PostGroup from "./postGroup";
export const metadata = {
    title: "Post Page", // Let next js handle title and description for SEO purposes
    description: "This is the post page",
};

export default async function PostPage() {
    return <AuthenticatedLayout>
        <PostGroup />
    </AuthenticatedLayout>;
}