"use client";

import { useState } from "react";
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogHeading,
} from "@/app/_components/ui/chromatic/dialogue";
import { AdminApiError } from "@/api/admin/types";
import { createStaffRole } from "@/api/admin/staffRoles";
import PermissionChips from "./PermissionChips";
import { knownPermissions, RoleFormState, togglePermission } from "./roleForm";
import style from "./role-manage.module.scss";

const EMPTY: RoleFormState = {
    name: "",
    description: "",
    position: 0,
    permission_bitmask: [],
};

export default function CreateRoleDialog({
    open,
    onOpenChange,
    onCreated,
}: {
    open: boolean;
    onOpenChange: (open: boolean) => void;
    onCreated: () => void;
}) {
    const [form, setForm] = useState<RoleFormState>(EMPTY);
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

    const submit = async () => {
        const name = form.name.trim();
        if (!name) {
            setError("A role needs a name.");
            setErrorField("name");
            return;
        }

        setSaving(true);
        setError(null);
        setErrorField(null);

        try {
            await createStaffRole({
                name,
                description: form.description.trim() || undefined,
                position: form.position,
                permission_bitmask: knownPermissions(form.permission_bitmask),
            });
            onCreated();
            handleOpenChange(false);
        } catch (err) {
            const apiError = err as AdminApiError;
            if (
                apiError.code === "staff_role_name_taken" ||
                apiError.code === "staff_role_invalid_name"
            ) {
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
                <DialogHeading className={style["dialog-heading"]}>
                    New staff role
                </DialogHeading>
                <DialogDescription className={style["dialog-description"]}>
                    Permissions are stored and reviewable, but nothing enforces
                    them yet — the panel is still gated on superuser alone.
                </DialogDescription>

                <div className={style["dialog-body"]}>
                    <label className={style["field"]}>
                        <span>Name</span>
                        <input
                            type="text"
                            value={form.name}
                            autoFocus
                            placeholder="Moderator"
                            onChange={(e) =>
                                setForm({ ...form, name: e.target.value })
                            }
                        />
                        {errorField === "name" && error && (
                            <em className={style["field-error"]}>{error}</em>
                        )}
                    </label>

                    <label className={style["field"]}>
                        <span>Description</span>
                        <textarea
                            value={form.description}
                            rows={2}
                            placeholder="What this role is for"
                            onChange={(e) =>
                                setForm({
                                    ...form,
                                    description: e.target.value,
                                })
                            }
                        />
                    </label>

                    <label className={style["field"]}>
                        <span>Position</span>
                        <input
                            type="number"
                            value={form.position}
                            onChange={(e) =>
                                setForm({
                                    ...form,
                                    position: Number(e.target.value),
                                })
                            }
                        />
                        <em className={style["field-hint"]}>
                            Lower sorts first.
                        </em>
                    </label>

                    <div className={style["field"]}>
                        <span>Permissions</span>
                        <PermissionChips
                            selected={form.permission_bitmask}
                            onToggle={(bit) =>
                                setForm((current) => ({
                                    ...current,
                                    permission_bitmask: togglePermission(
                                        current.permission_bitmask,
                                        bit,
                                    ),
                                }))
                            }
                        />
                    </div>

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
                        disabled={saving}
                    >
                        {saving ? "Creating..." : "Create role"}
                    </button>
                </div>
            </DialogContent>
        </Dialog>
    );
}