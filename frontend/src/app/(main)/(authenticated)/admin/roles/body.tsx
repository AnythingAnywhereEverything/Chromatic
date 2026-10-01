"use client";

import { useQuery, useQueryClient } from "@tanstack/react-query";
import { useDeferredValue, useState } from "react";
import { PiPlus } from "react-icons/pi";
import { getStaffRoles } from "@/api/admin/staffRoles";
import { AdminStaffRole } from "@/api/admin/types";
import CreateRoleDialog from "./CreateRoleDialog";
import PermissionChips from "./PermissionChips";
import RoleDialog from "./RoleDialog";
import style from "./role-manage.module.scss";

export default function AdminRolesBody() {
    const queryClient = useQueryClient();
    const [search, setSearch] = useState("");
    const deferredSearch = useDeferredValue(search);
    const [selectedRoleId, setSelectedRoleId] = useState<string | null>(null);
    const [creating, setCreating] = useState(false);

    const { data: page, isLoading } = useQuery({
        queryKey: ["admin-staff-roles", deferredSearch],
        queryFn: () => getStaffRoles({ q: deferredSearch || undefined }),
        staleTime: 60 * 1000,
        retry: false,
    });

    const roles: AdminStaffRole[] = page?.rows ?? [];

    /** The table row the dialog is open for, used only to flash it. */
    const refresh = () => {
        queryClient.invalidateQueries({ queryKey: ["admin-staff-roles"] });
    };

    return (
        <section className={style["role-management"]}>
            <h1>Staff Roles</h1>

            <div className={style["management-actions"]}>
                <div className={style["search-bar"]}>
                    <input
                        type="text"
                        placeholder="Search roles..."
                        value={search}
                        onChange={(e) => setSearch(e.target.value)}
                    />
                </div>

                <button
                    type="button"
                    className={style["button-primary"]}
                    onClick={() => setCreating(true)}
                >
                    <PiPlus /> New role
                </button>
            </div>

            <div className={style["role-list-header"]}>
                <span className={style["role-name"]}>Name</span>
                <span className={style["role-position"]}>Position</span>
                <span className={style["role-permissions"]}>Permissions</span>
            </div>

            <ul className={style["role-list"]}>
                {roles.map((role) => (
                    <li key={role.id}>
                        <button
                            type="button"
                            className={`${style["role-grid"]} ${
                                selectedRoleId === role.id
                                    ? style["role-selected"]
                                    : ""
                            }`}
                            onClick={() => setSelectedRoleId(role.id)}
                        >
                            <span className={style["role-name"]}>
                                {role.name}
                            </span>
                            <span className={style["role-position"]}>
                                {role.position}
                            </span>
                            <span className={style["role-permissions"]}>
                                <PermissionChips
                                    selected={role.permission_bitmask}
                                />
                            </span>
                        </button>
                    </li>
                ))}
            </ul>

            {isLoading && <p className={style["muted"]}>Loading roles...</p>}

            {!isLoading && roles.length === 0 && (
                <p className={style["muted"]}>
                    {search
                        ? `No roles match "${search}".`
                        : "No staff roles yet."}
                </p>
            )}

            <RoleDialog
                open={selectedRoleId !== null}
                roleId={selectedRoleId}
                onOpenChange={(open) => {
                    if (!open) setSelectedRoleId(null);
                }}
                onSaved={refresh}
                onDeleted={refresh}
            />

            <CreateRoleDialog
                open={creating}
                onOpenChange={setCreating}
                onCreated={refresh}
            />
        </section>
    );
}