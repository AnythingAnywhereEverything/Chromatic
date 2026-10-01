import { useEffect, useState } from "react";

import {
    MAX_DESCRIPTION_CHARS,
    reportUser,
    ReportTargetType,
    ReportType,
    REPORT_TYPES,
} from "@/api/user/report";
import {
    Dialog,
    DialogClose,
    DialogContent,
    DialogDescription,
    DialogHeading,
} from "../dialogue";

import style from "./report.module.scss";

const REPORT_TYPE_LABELS: Record<ReportType, string> = {
    spam: "Spam",
    harassment: "Harassment or bullying",
    hate_speech: "Hate speech",
    nsfw: "Not safe for work",
    violence: "Violence or threats",
    impersonation: "Impersonation",
    misinformation: "Misinformation",
    other: "Something else",
};

interface ReportDialogueProps {
    targetId: string;
    targetType: ReportTargetType;
    /**
     * What follows "Report" in the heading -- "@username", "post", "comment".
     * The dialog is shared by all three target types, so the noun comes from the
     * call site rather than being derived here.
     */
    label: string;
    open: boolean;
    onOpenChange: (open: boolean) => void;
}

/**
 * The dialog is controlled rather than using DialogTrigger because it lives
 * next to a Dropdown: DropdownContent unmounts when the menu closes, which would
 * take the dialog root with it before it could ever render. The trigger is the
 * menu item, which flips `open` to true as the menu closes.
 */
const ReportDialogue = ({
    targetId,
    targetType,
    label,
    open,
    onOpenChange,
}: ReportDialogueProps) => {
    const [reportType, setReportType] = useState<ReportType>("spam");
    const [description, setDescription] = useState("");
    const [error, setError] = useState<string | null>(null);
    const [submitting, setSubmitting] = useState(false);
    const [submitted, setSubmitted] = useState(false);

    // The component stays mounted while only DialogContent unmounts, so the form
    // has to be cleared when it opens rather than when it closes.
    useEffect(() => {
        if (open) {
            setReportType("spam");
            setDescription("");
            setError(null);
            setSubmitting(false);
            setSubmitted(false);
        }
    }, [open]);

    const handleSubmit = async () => {
        setSubmitting(true);
        setError(null);
        try {
            await reportUser({
                targetId,
                targetType,
                reportType,
                description,
            });
            setSubmitted(true);
        } catch (err) {
            setError(
                err instanceof Error ? err.message : "Failed to submit report",
            );
        } finally {
            setSubmitting(false);
        }
    };

    return (
        <Dialog
            open={open}
            onOpenChange={onOpenChange}
            outsidePress={!submitting}
            onClose={() => !submitting}
        >
            <DialogContent className={style["container"]}>
                <DialogHeading className={style["heading"]}>
                    {submitted ? "Report submitted" : `Report ${label}`}
                </DialogHeading>

                {submitted ? (
                    <>
                        <DialogDescription className={style["description"]}>
                            Thanks for letting us know. Our moderators will take
                            a look.
                        </DialogDescription>
                        <div className={style["actions"]}>
                            <DialogClose className={style["submit"]}>
                                Done
                            </DialogClose>
                        </div>
                    </>
                ) : (
                    <>
                        <DialogDescription className={style["description"]}>
                            Tell us what is wrong with this account. Your report
                            is private.
                        </DialogDescription>

                        <label className={style["field"]}>
                            <span>Reason</span>
                            <select
                                value={reportType}
                                onChange={(event) =>
                                    setReportType(
                                        event.target.value as ReportType,
                                    )
                                }
                                disabled={submitting}
                            >
                                {REPORT_TYPES.map((type) => (
                                    <option key={type} value={type}>
                                        {REPORT_TYPE_LABELS[type]}
                                    </option>
                                ))}
                            </select>
                        </label>

                        <label className={style["field"]}>
                            <span>Details (optional)</span>
                            <textarea
                                value={description}
                                onChange={(event) =>
                                    setDescription(event.target.value)
                                }
                                maxLength={MAX_DESCRIPTION_CHARS}
                                rows={4}
                                placeholder="Add anything that would help us understand."
                                disabled={submitting}
                            />
                            <span className={style["counter"]}>
                                {description.length}/{MAX_DESCRIPTION_CHARS}
                            </span>
                        </label>

                        {error && <p className={style["error"]}>{error}</p>}

                        <div className={style["actions"]}>
                            <DialogClose
                                className={style["cancel"]}
                                disabled={submitting}
                            >
                                Cancel
                            </DialogClose>
                            <button
                                type="button"
                                className={style["submit"]}
                                onClick={handleSubmit}
                                disabled={submitting}
                            >
                                {submitting ? "Submitting..." : "Submit report"}
                            </button>
                        </div>
                    </>
                )}
            </DialogContent>
        </Dialog>
    );
};

export { ReportDialogue };
