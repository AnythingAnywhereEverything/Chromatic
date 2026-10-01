"use client";

import { useQuery, useQueryClient } from "@tanstack/react-query";
import { useEffect, useState } from "react";
import {
    Dialog,
    DialogClose,
    DialogContent,
    DialogDescription,
    DialogHeading,
} from "@/app/_components/ui/chromatic/dialogue";
import { AlertDialogue } from "@/app/_components/ui/chromatic/confirmation";
import { formatdatemonthyear } from "@/app/_components/ui/chromatic/post/helpers/dateFormater";
import {
    deleteStaffRole,
    getStaffRole,
    getStaffRoleMembers,
    updateStaffRole,
} from "@/api/admin/staffRoles";
import { AdminApiError, AdminStaffRoleMember } from "@/api/admin/types";
import PermissionChips from "./PermissionChips";
import { RoleFormState, roleFormChanges, togglePermission } from "./roleForm";
import style from "./role-manage.module.scss";

export default function RoleDialog({
    open,
    roleId,
    onOpenChange,
    onSaved,
    onDeleted,
}: {
    open: boolean;
    roleId: string | null;
    onOpenChange: (open: boolean) => void;
    onSaved: () => void;
    onDeleted: () => void;
}) {
    const queryClient = useQueryClient();
    const [form, setForm] = useState<RoleFormState | null>(null);
    const [saving, setSaving] = useState(false);
    const [deleting, setDeleting] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [errorField, setErrorField] = useState<"name" | null>(null);
    const [extraPages, setExtraPages] = useState<
        { rows: AdminStaffRoleMember[]; has_more: boolean }[]
    >([]);

    const enabled = open && roleId !== null;

    const detailQuery = useQuery({
        queryKey: ["admin-staff-role", roleId],
        queryFn: () => getStaffRole(roleId!),
        enabled,
        retry: false,
    });

    const membersQuery = useQuery({
        queryKey: ["admin-staff-role-members", roleId],
        queryFn: () => getStaffRoleMembers(roleId!),
        enabled,
        retry: false,
    });

    const detail = detailQuery.data;
    const role = detail?.role;
    const [seededFor, setSeededFor] = useState<string | null>(null);
    useEffect(() => {
        if (!role || !roleId || seededFor === roleId) return;
        setSeededFor(roleId);
        setForm(toFormState(role));
    }, [role, roleId, seededFor]);

    useEffect(() => {
        if (open) return;
        setForm(null);
        setSeededFor(null);
        setError(null);
        setErrorField(null);
        setExtraPages([]);
    }, [open]);

    if (!open || !roleId) return null;

    const isLoading = detailQuery.isLoading;

    const members = [
        ...(membersQuery.data?.rows ?? []),
        ...extraPages.flatMap((page) => page.rows),
    ];
    const lastMemberId = members[members.length - 1]?.user_id;
    const hasMoreMembers =
        extraPages.length > 0
            ? extraPages[extraPages.length - 1].has_more
            : (membersQuery.data?.has_more ?? false);

    const loadMoreMembers = async () => {
        try {
            const page = await getStaffRoleMembers(roleId, {
                before_user_id: lastMemberId,
            });
            setExtraPages((current) => [...current, page]);
        } catch (err) {
            setError((err as AdminApiError).message);
        }
    };

    const save = async () => {
        if (!form) return;

        const name = form.name.trim();
        if (!name) {
            setError("A role needs a name.");
            setErrorField("name");
            return;
        }

        const original = role ? toFormState(role) : form;
        const next = { ...form, name };
        const changes = roleFormChanges(next, original);
        const description = form.description.trim();
        if (description !== original.description.trim()) {
            changes.description = description;
        }

        if (Object.keys(changes).length === 0) return;

        setSaving(true);
        setError(null);
        setErrorField(null);

        try {
            const updated = await updateStaffRole(roleId, changes);
            setForm(toFormState(updated));
            queryClient.invalidateQueries({ queryKey: ["admin-staff-roles"] });
            onSaved();
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

    const remove = async () => {
        setDeleting(true);
        setError(null);
        try {
            await deleteStaffRole(roleId);
            onDeleted();
            onOpenChange(false);
        } catch (err) {
            setError((err as AdminApiError).message);
        } finally {
            setDeleting(false);
        }
    };

    return (
        <Dialog open={open} onOpenChange={onOpenChange}>
            <DialogContent className={style["dialog-content"]}>
                <DialogHeading className={style["dialog-heading"]}>
                    {isLoading ? "Loading role..." : form?.name || "Role"}
                </DialogHeading>
                <DialogDescription className={style["dialog-description"]}>
                    {detail
                        ? `Held by ${detail.member_count} ${
                              detail.member_count === 1 ? "person" : "people"
                          }. Permissions are descriptive only — nothing enforces them yet.`
                        : " "}
                </DialogDescription>

                <div className={style["dialog-body"]}>
                    {isLoading || !form ? (
                        <div className={style["skeleton"]} />
                    ) : (
                        <>
                            <label className={style["field"]}>
                                <span>Name</span>
                                <input
                                    type="text"
                                    value={form.name}
                                    onChange={(e) =>
                                        setForm({ ...form, name: e.target.value })
                                    }
                                />
                                {errorField === "name" && error && (
                                    <em className={style["field-error"]}>
                                        {error}
                                    </em>
                                )}
                            </label>

                            <label className={style["field"]}>
                                <span>Description</span>
                                <textarea
                                    value={form.description}
                                    rows={2}
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
                                        setForm((current) =>
                                            current
                                                ? {
                                                      ...current,
                                                      permission_bitmask:
                                                          togglePermission(
                                                              current.permission_bitmask,
                                                              bit,
                                                          ),
                                                  }
                                                : current,
                                        )
                                    }
                                />
                            </div>
                        </>
                    )}

                    <div className={style["members"]}>
                        <h3 className={style["members-heading"]}>
                            Members
                            {detail && <span>{detail.member_count}</span>}
                        </h3>

                        {membersQuery.isLoading ? (
                            <p className={style["muted"]}>Loading...</p>
                        ) : members.length === 0 ? (
                            <p className={style["muted"]}>
                                Nobody holds this role yet.
                            </p>
                        ) : (
                            <>
                                <ul className={style["member-list"]}>
                                    {members.map((member) => (
                                        <li
                                            key={member.user_id}
                                            className={style["member"]}
                                        >
                                            <span className={style["member-name"]}>
                                                {member.display_name ||
                                                    member.username}
                                            </span>
                                            <span className={style["muted"]}>
                                                {formatdatemonthyear(
                                                    member.assigned_at,
                                                )}
                                            </span>
                                        </li>
                                    ))}
                                </ul>

                                {hasMoreMembers && (
                                    <button
                                        type="button"
                                        className={style["button-ghost"]}
                                        onClick={loadMoreMembers}
                                    >
                                        Load more
                                    </button>
                                )}
                            </>
                        )}

                        <p className={style["muted"]}>
                            Roles are assigned from a user's dialog on the Users
                            page.
                        </p>
                    </div>

                    {error && !errorField && (
                        <em className={style["form-error"]}>{error}</em>
                    )}
                </div>

                <div className={style["dialog-actions"]}>
                    <AlertDialogue
                        type="destructive"
                        title={`Delete ${role?.name ?? "this role"}?`}
                        message={
                            detail
                                ? `This permanently deletes the role and removes it from ${
                                      detail.member_count
                                  } ${
                                      detail.member_count === 1
                                          ? "person"
                                          : "people"
                                  }. This cannot be undone.`
                                : "This permanently deletes the role and every assignment to it. This cannot be undone."
                        }
                        triggerName={deleting ? "Deleting..." : "Delete"}
                        confirmText="Delete role"
                        hasButton
                        onConfirm={remove}
                    />

                    <div className={style["dialog-actions-right"]}>
                        <DialogClose className={style["button-ghost"]}>
                            Cancel
                        </DialogClose>
                        <button
                            type="button"
                            className={style["button-primary"]}
                            onClick={save}
                            disabled={saving || isLoading || !form}
                        >
                            {saving ? "Saving..." : "Save"}
                        </button>
                    </div>
                </div>
            </DialogContent>
        </Dialog>
    );
}

/**
 * Role row to form state.
 *
 * `description` is normalised to `""` rather than left undefined so the textarea
 * is controlled even when there is none, and so both sides of the dirty check
 * compare two strings instead of a string and an absent key.
 */
function toFormState(role: {
    name: string;
    description?: string;
    position: number;
    permission_bitmask: number[];
}): RoleFormState {
    return {
        name: role.name,
        description: role.description ?? "",
        position: role.position,
        permission_bitmask: [...role.permission_bitmask].sort((a, b) => a - b),
    };
}