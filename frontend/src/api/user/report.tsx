import { fetchWithAuth } from "@/handler/token_handler";

export type ReportTargetType = "user" | "post" | "comment";

export const REPORT_TYPES = [
    "spam",
    "harassment",
    "hate_speech",
    "nsfw",
    "violence",
    "impersonation",
    "misinformation",
    "other",
] as const;

export type ReportType = (typeof REPORT_TYPES)[number];

export const MAX_DESCRIPTION_CHARS = 1000;

export interface ReportRequest {
    targetId: string;
    targetType: ReportTargetType;
    reportType: ReportType;
    description?: string | null;
}

export const reportUser = async ({
    targetId,
    targetType,
    reportType,
    description,
}: ReportRequest): Promise<void> => {
    const res = await fetchWithAuth("v2/users/report", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({
            target_id: targetId,
            target_type: targetType,
            report_type: reportType,
            description: description?.trim() ? description.trim() : null,
        }), 
    });

    if (!res.ok) {
        const data = await res.json().catch(() => null);
        const errorMessage =
            data?.errors?.[0]?.message || "Failed to submit report";
        throw new Error(errorMessage);
    }
};