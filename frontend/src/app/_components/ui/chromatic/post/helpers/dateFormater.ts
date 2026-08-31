const userTimeZone = Intl.DateTimeFormat().resolvedOptions().timeZone;

function parseUtcDate(dateInput: Date | string | number): Date {
    if (typeof dateInput === "string") {
        // * Rust chrono::DateTime<Utc> may arrive without the trailing Z.
        if (!/[zZ]|[+-]\d{2}:\d{2}$/.test(dateInput)) {
            return new Date(`${dateInput}Z`);
        }
    }

    return new Date(dateInput);
}

function formatSocialMediaDate(dateInput: Date | string | number): string {
    const date = parseUtcDate(dateInput);
    const now = new Date();

    const diffMs = now.getTime() - date.getTime();

    // * Future dates
    if (diffMs < 0) {
        return "Just now";
    }

    const diffMins = Math.floor(diffMs / 60000);
    const diffHours = Math.floor(diffMs / 3600000);
    const diffDays = Math.floor(diffMs / 86400000);

    // * More than 10 days: "1 Jan 26"
    if (diffDays > 10) {
        return date.toLocaleDateString("en-GB", {
            timeZone: userTimeZone,
            day: "numeric",
            month: "short",
            year: "2-digit",
        });
    }

    if (diffDays >= 1) {
        return `${diffDays} ${diffDays === 1 ? "day" : "days"} ago`;
    }

    if (diffHours >= 1) {
        return `${diffHours} ${diffHours === 1 ? "hour" : "hours"} ago`;
    }

    if (diffMins >= 1) {
        return `${diffMins} ${diffMins === 1 ? "min" : "mins"} ago`;
    }

    return "Just now";
}

function formatdatemonthyear(dateInput: Date | string | number): string {
    const date = parseUtcDate(dateInput);

    return date.toLocaleDateString("en-GB", {
        timeZone: userTimeZone,
        day: "numeric",
        month: "short",
        year: "2-digit",
    });
}

function formatFullDateWithExactTime(dateInput: Date | string | number): string {
    const date = parseUtcDate(dateInput);

    return date.toLocaleDateString("en-US", {
        timeZone: userTimeZone,
        weekday: "long",
        month: "long",
        day: "numeric",
        year: "numeric",
        hour: "numeric",
        minute: "2-digit",
    }).replace(" at ", " at ");
}

export {
    formatSocialMediaDate,
    formatdatemonthyear,
    formatFullDateWithExactTime,
};