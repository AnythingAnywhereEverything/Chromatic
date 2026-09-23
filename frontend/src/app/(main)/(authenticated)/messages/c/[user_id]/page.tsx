import MessageContainer from "../../body";

export default async function Page({
    params,
}: {
    params: Promise<{ user_id: string }>;
}) {
    const { user_id } = await params;

    return <MessageContainer userId={user_id} />;
}