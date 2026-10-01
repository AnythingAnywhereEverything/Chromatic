import GuildViewBody from "./body";

export default async function GuildViewPage({
    params,
}: {
    params: Promise<{ guild_id: string }>;
}) {
    const { guild_id } = await params;

    return <GuildViewBody guildId={guild_id} />;
}