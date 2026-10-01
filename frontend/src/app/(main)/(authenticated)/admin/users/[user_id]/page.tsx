import AdminUserBody from "./body";

export default async function AdminUserPage({
    params,
}: {
    params: Promise<{ user_id: string }>;
}) {
    const resolvedParams = await params;

    return <AdminUserBody params={resolvedParams} />;
}