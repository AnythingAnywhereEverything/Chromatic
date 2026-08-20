function formatSocialMediaDate(dateInput: Date | string | number): string {
    const date = new Date(dateInput);
    const now = new Date();

    const diffMs = now.getTime() - date.getTime();

    // Future dates
    if (diffMs < 0) {
        return "Just now";
    }

    const diffMins = Math.floor(diffMs / 60000);
    const diffHours = Math.floor(diffMs / 3600000);
    const diffDays = Math.floor(diffMs / 86400000);

    // * More than 10 days: "1 Jan 26"
    if (diffDays > 10) {
        return date.toLocaleDateString("en-GB", {
            day: "numeric",
            month: "short",
            year: "2-digit",
        });
    }

    // 1–10 days: "1 day", "5 days"
    if (diffDays >= 1) {
        return `${diffDays} ${diffDays === 1 ? "day" : "days"}`;
    }

    // 1–23 hours: "1 hour", "8 hours"
    if (diffHours >= 1) {
        return `${diffHours} ${diffHours === 1 ? "hour" : "hours"}`;
    }

    // 1–59 minutes: "1 min", "25 mins"
    if (diffMins >= 1) {
        return `${diffMins} ${diffMins === 1 ? "min" : "mins"}`;
    }

    return "Just now";
}

function formatdatemonthyear(dateInput: Date | string | number): string {
    const date = new Date(dateInput);
    return date.toLocaleDateString("en-GB", {
        day: "numeric",
        month: "short",
        year: "2-digit",
    });
}

export { formatSocialMediaDate, formatdatemonthyear };