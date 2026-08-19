import getIdColor from "@lib/getIdColor";

const getInitials = (name: string) => {
    if (!name) return "?";
    const parts = name.trim().split(/\s+/);
    if (parts.length === 1) return parts[0].charAt(0).toUpperCase();
    return (
        parts[0].charAt(0) + parts[parts.length - 1].charAt(0)
    ).toUpperCase();
};

export const UserIdAvatar = ({
    userId,
    name,
    size = 48,
}: {
    userId: string;
    name: string;
    size?: number;
}) => {
    const initials = getInitials(name);
    const backgroundColor = getIdColor(userId);

    const avatarStyle = {
        width: `${size}px`,
        height: `${size}px`,
        backgroundColor: backgroundColor,
        color: "#ffffff",
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
        borderRadius: "50%",
        fontWeight: "600",
        fontSize: `${size * 0.4}px`, // Scales font size proportionally
        userSelect: "none" as const,
        fontFamily: "sans-serif",
    };

    return (
        <div style={avatarStyle} aria-label={name || `User ${userId}`}>
            {initials}
        </div>
    );
};