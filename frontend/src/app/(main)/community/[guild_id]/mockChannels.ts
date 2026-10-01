
export interface GuildChannel {
    id: string;
    guild_id: string;
    name: string;
    channel_type: "text";
    created_at: string;
}


export interface GuildChannelMessage {
    id: string;
    author_id: string;
    author_name: string;
    content: string;
    created_at: string;
}


function minutesAgo(minutes: number): string {
    return new Date(Date.now() - minutes * 60_000).toISOString();
}

export function mockChannels(guildId: string): GuildChannel[] {
    return [
        {
            id: `${guildId}-general`,
            guild_id: guildId,
            name: "general",
            channel_type: "text",
            created_at: minutesAgo(60 * 24 * 7),
        },
        {
            id: `${guildId}-announcements`,
            guild_id: guildId,
            name: "announcements",
            channel_type: "text",
            created_at: minutesAgo(60 * 24 * 7),
        },
        {
            id: `${guildId}-help`,
            guild_id: guildId,
            name: "help",
            channel_type: "text",
            created_at: minutesAgo(60 * 24 * 5),
        },
        {
            id: `${guildId}-resources`,
            guild_id: guildId,
            name: "resources",
            channel_type: "text",
            created_at: minutesAgo(60 * 24 * 3),
        },
    ];
}

export function mockMessages(channel: GuildChannel): GuildChannelMessage[] {
    if (channel.name !== "general") {
        return [];
    }

    return [
        {
            id: `${channel.id}-m1`,
            author_id: `${channel.guild_id}1`,
            author_name: "Ada Lovelace",
            content: "Welcome! Introduce yourself and say what you are hoping to get out of this community.",
            created_at: minutesAgo(320),
        },
        {
            id: `${channel.id}-m2`,
            author_id: `${channel.guild_id}2`,
            author_name: "Alan Turing",
            content: "Glad to be here. Is there a reading list somewhere?",
            created_at: minutesAgo(295),
        },
        {
            id: `${channel.id}-m3`,
            author_id: `${channel.guild_id}1`,
            author_name: "Ada Lovelace",
            content: "Pinned it in #resources for now.",
            created_at: minutesAgo(290),
        },
        {
            id: `${channel.id}-m4`,
            author_id: `${channel.guild_id}3`,
            author_name: "Grace Hopper",
            content: "Thanks — found it. The compilers track looks like a good starting point.",
            created_at: minutesAgo(120),
        },
    ];
}