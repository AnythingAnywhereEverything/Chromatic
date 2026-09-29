"use client";

import { ping } from "@/api/ping";
import { getAdminStats } from "@/api/admin/stats";
import { useEffect, useState } from "react";
import { useQuery } from "@tanstack/react-query";
import style from "../admin.module.scss";
import { AdminAuditEntry } from "@/api/admin/types";
import { getAdminAudit } from "@/api/admin/audit";

export default function AdminOverviewBody() {
    const [rustHealth, setRustHealth] = useState<boolean>(false);
    const [auditLog, setAuditLog] = useState<Array<AdminAuditEntry>>([]);

    const { data: stats } = useQuery({
        queryKey: ["admin-stats"],
        queryFn: getAdminStats,
        retry: false,
    });

    useEffect(() => {
        const fetchAuditLog = async () => {
            try {
                setAuditLog(await getAdminAudit());
            } catch (error) {
                console.error("Failed to fetch audit log:", error);
            }
        };

        fetchAuditLog();
    }, []);

    useEffect(() => {
        const checkHealth = async () => {
            try {
                await ping();
                setRustHealth(true);
            } catch {
                setRustHealth(false);
            }
        };

        checkHealth();

        const interval = setInterval(checkHealth, 15000);

        return () => clearInterval(interval);
    }, []);

    const tiles = [
        { label: "Total users", value: stats?.total_users },
        { label: "Active", value: stats?.active_users },
        { label: "Suspended", value: stats?.suspended_users },
        { label: "Superusers", value: stats?.superusers },
        { label: "Deleted", value: stats?.deleted_users },
    ];

    return (
        <section className={style["overview"]}>
            <h1>Overview</h1>

            <section className={style["stats-grid"]}>
                {tiles.map((tile) => (
                    <div key={tile.label} className={style["stat-tile"]}>
                        <span>{tile.label}</span>
                        <strong>{tile.value ?? "—"}</strong>
                    </div>
                ))}
            </section>

            <section className={style["health-section"]}>
                <h2>System Health</h2>
                <p>Rust service: {rustHealth ? "OK" : "DOWN"}</p>
            </section>

            <section className={style["audit-log-section"]}>
                <h2>Audit Log</h2>
                <ul className={style["audit-log-list"]}>
                    <li className={style["audit-log-header"]}>
                        <span>Action</span>
                        <span>Target User</span>
                        <span>Performed By</span>
                        <span>Created At</span>
                    </li>
                    {auditLog.map((audit) => (
                        <li key={audit.id}>
                            <span>{audit.action}</span>
                        </li>
                    ))}
                </ul>
            </section>
        </section>
    );
}
