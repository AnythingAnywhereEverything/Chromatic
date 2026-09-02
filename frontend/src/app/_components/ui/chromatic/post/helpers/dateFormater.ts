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

    if (diffDays >= 1) {
        return formatdatemonthyear(date);
    }

    if (diffHours >= 1) {
        return `${diffHours}h`;
    }

    if (diffMins >= 1) {
        return `${diffMins}m`;
    }

    return "Just now";
}

function formatdatemonthyear(dateInput: Date | string | number): string {
    const date = parseUtcDate(dateInput);
    const dateYear = Number(
        new Intl.DateTimeFormat("en-US", {
            timeZone: userTimeZone,
            year: "numeric",
        }).format(date)
    );
    const currentYear = Number(
        new Intl.DateTimeFormat("en-US", {
            timeZone: userTimeZone,
            year: "numeric",
        }).format(new Date())
    );
    return date.toLocaleDateString("en-US", {
        timeZone: userTimeZone,
        day: "numeric",
        month: "short",
        // * Only show year when the date is from a different year.
        ...(dateYear !== currentYear ? { year: "2-digit" } : {}),
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