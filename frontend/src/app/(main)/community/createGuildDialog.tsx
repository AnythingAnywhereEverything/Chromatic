"use client";

import { useState } from "react";
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogHeading,
} from "@/app/_components/ui/chromatic/dialogue";
import { createGuild } from "@/api/guild/guilds";
import { GuildApiError } from "@/api/guild/types";
import style from "./community.module.scss";

const EMPTY = { name: "", description: "" };
const MAX_NAME_CHARS = 128;

export default function CreateGuildDialog({
    open,
    onOpenChange,
    onCreated,
}: {
    open: boolean;
    onOpenChange: (open: boolean) => void;

    onCreated: () => void;
}) {
    const [form, setForm] = useState(EMPTY);
    const [saving, setSaving] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [errorField, setErrorField] = useState<"name" | null>(null);

    const handleOpenChange = (next: boolean) => {
        if (!next) {
            setForm(EMPTY);
            setError(null);
            setErrorField(null);
        }
        onOpenChange(next);
    };

    const nameTooLong = [...form.name.trim()].length > MAX_NAME_CHARS;

    const submit = async () => {
        const name = form.name.trim();
        if (!name) {
            setError("A community needs a name.");
            setErrorField("name");
            return;
        }

        setSaving(true);
        setError(null);
        setErrorField(null);

        try {
            await createGuild({ name, description: form.description.trim() });
            onCreated();
            handleOpenChange(false);
        } catch (err) {
            const apiError = err as GuildApiError;
            if (apiError.code === "guild_invalid_name") {
                setErrorField("name");
            }
            setError(apiError.message);
        } finally {
            setSaving(false);
        }
    };

    return (
        <Dialog open={open} onOpenChange={handleOpenChange}>
            <DialogContent className={style["dialog-content"]}>
                <DialogHeading>New community</DialogHeading>
                <DialogDescription className={style["dialog-description"]}>
                    You will be the owner. A #general channel is created with it.
                </DialogDescription>

                <div className={style["dialog-body"]}>
                    <label className={style["field"]}>
                        <span>Name</span>
                        <input
                            type="text"
                            value={form.name}
                            autoFocus
                            placeholder="Study group, club, project..."
                            onChange={(e) =>
                                setForm({ ...form, name: e.target.value })
                            }
                        />
                        {errorField === "name" && error && (
                            <em className={style["field-error"]}>{error}</em>
                        )}
                        {!errorField && nameTooLong && (
                            <em className={style["field-error"]}>
                                Keep the name under {MAX_NAME_CHARS} characters.
                            </em>
                        )}
                    </label>

                    <label className={style["field"]}>
                        <span>Description</span>
                        <textarea
                            value={form.description}
                            rows={3}
                            placeholder="What this community is for"
                            onChange={(e) =>
                                setForm({
                                    ...form,
                                    description: e.target.value,
                                })
                            }
                        />
                        <em className={style["field-hint"]}>Optional.</em>
                    </label>

                    {error && !errorField && (
                        <em className={style["form-error"]}>{error}</em>
                    )}
                </div>

                <div className={style["dialog-actions"]}>
                    <button
                        type="button"
                        className={style["button-ghost"]}
                        onClick={() => handleOpenChange(false)}
                        disabled={saving}
                    >
                        Cancel
                    </button>
                    <button
                        type="button"
                        className={style["button-primary"]}
                        onClick={submit}
                        disabled={saving || nameTooLong}
                    >
                        {saving ? "Creating..." : "Create community"}
                    </button>
                </div>
            </DialogContent>
        </Dialog>
    );
}