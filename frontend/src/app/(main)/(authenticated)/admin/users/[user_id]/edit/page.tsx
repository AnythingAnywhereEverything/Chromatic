import AdminUserEditBody from "./body";

export default function AdminUserEdit({
    params,
}: {
    params: { userId: string };
}) {
    return <AdminUserEditBody userId={params.userId} />;
}
