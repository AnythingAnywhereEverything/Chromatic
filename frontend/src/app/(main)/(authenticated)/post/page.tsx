import style from "./style.module.scss"
import AuthenticatedLayout from "../layout";
import PostGroup from "./postGroup";
import PostRightLayout from "./rightside";
export const metadata = {
    title: "Post Page", // Let next js handle title and description for SEO purposes
    description: "This is the post page",
};

export default async function PostPage() {
    return (
    <AuthenticatedLayout>
        <div className={style["postLayout"]}>
            <PostGroup />
            <PostRightLayout/>
        </div>
    </AuthenticatedLayout>
    );
}