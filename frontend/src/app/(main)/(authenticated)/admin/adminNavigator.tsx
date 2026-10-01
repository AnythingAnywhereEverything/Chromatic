"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import style from "./admin-nav.module.scss";

const ADMIN_NAV_ITEMS = [
    { label: "Overview", href: "/admin/overview" },
    { label: "Users", href: "/admin/users" },
    { label: "Roles", href: "/admin/roles" },
];

export default function AdminNavigator() {
    const pathname = usePathname();

    const isActive = (href: string) =>
        pathname === href || pathname.startsWith(`${href}/`);

    return (
        <nav className={style["admin-nav"]}>
            <h2>Chromatic</h2>
            {ADMIN_NAV_ITEMS.map(({ label, href }) => {
                const active = isActive(href);
                return (
                    <Link
                        key={href}
                        href={href}
                        className={`${style["admin-nav-item"]} ${
                            active ? style["active"] : ""
                        }`}
                        >
                        {label}
                    </Link>
                );
            })}
        </nav>
    );
}
