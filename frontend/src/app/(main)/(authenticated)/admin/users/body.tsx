"use client";
import {
    Dropdown,
    DropdownContent,
    DropdownTrigger,
    DropdownItem,
} from "@/app/_components/ui/chromatic/dropdown";
import { useQuery } from "@tanstack/react-query";
import { useDeferredValue, useState } from "react";
import { useRouter } from "next/navigation";
import style from "./user-manage.module.scss";
import { getAdminUsers } from "@/api/admin/users";
import { AdminUser } from "@/api/admin/types";
import { formatdatemonthyear } from "@/app/_components/ui/chromatic/post/helpers/dateFormater";
import { PostAvatar } from "@/app/_components/ui/chromatic/post/header/avatar";
import { PiExport, PiPlus } from "react-icons/pi";
import { HiOutlineTrash } from "react-icons/hi2";
import { FiEdit } from "react-icons/fi";

export default function AdminUsersBody() {
    const [selectedUsers, setSelectedUsers] = useState<string[]>([]);
    const [search, setSearch] = useState("");
    const deferredSearch = useDeferredValue(search);
    const router = useRouter();


    const { data: page } = useQuery({
        queryKey: ["admin-users", deferredSearch],
        queryFn: () => getAdminUsers({ q: deferredSearch || undefined }),
        staleTime: 60 * 1000,
        retry: false,
    });

    const users: AdminUser[] = page?.rows ?? [];

    const allSelected =
        users.length > 0 && selectedUsers.length === users.length;
    const toggleSelectAll = () => {
        setSelectedUsers(allSelected ? [] : users.map((user) => user.id));
    };

    const toggleUser = (id: string) => {
        setSelectedUsers((current) =>
            current.includes(id)
                ? current.filter((userId) => userId !== id)
                : [...current, id],
        );
    };

    const handleEditUser = (id: string) => {
        router.push(`/admin/users/${id}`); 
    };

    return (
        <section className={style["user-management"]}>
            <h1>Admin Users</h1>

            <div className={style["management-actions"]}>
                <div className={style["filters"]}>
                    <div className={style["search-bar"]}>
                        <input
                            type="text"
                            placeholder="Search users..."
                            value={search}
                            onChange={(e) => setSearch(e.target.value)}
                        />
                    </div>

                    {/* further impl: role / status / date filters */}
                    <Dropdown>
                        <DropdownTrigger
                            className={style["role-dropdown-trigger"]}
                        >
                            <span>Role</span>
                        </DropdownTrigger>

                        <DropdownContent>
                            <DropdownItem>Admin</DropdownItem>
                            <DropdownItem>Moderator</DropdownItem>
                            <DropdownItem>Staff</DropdownItem>
                            <DropdownItem>User</DropdownItem>
                        </DropdownContent>
                    </Dropdown>

                    <Dropdown>
                        <DropdownTrigger>
                            <span>Status</span>
                        </DropdownTrigger>

                        <DropdownContent
                            className={style["status-dropdown-content"]}
                        >
                            <DropdownItem>Active</DropdownItem>
                            <DropdownItem>Inactive</DropdownItem>
                            <DropdownItem>Deleted</DropdownItem>
                        </DropdownContent>
                    </Dropdown>

                    <Dropdown>
                        <DropdownTrigger>
                            <span>Date</span>
                        </DropdownTrigger>

                        <DropdownContent
                            className={style["date-dropdown-content"]}
                        >
                            <DropdownItem>Today</DropdownItem>
                            <DropdownItem>This Week</DropdownItem>
                            <DropdownItem>This Month</DropdownItem>
                            <DropdownItem>This Year</DropdownItem>
                        </DropdownContent>
                    </Dropdown>
                </div>

                <div className={style["create-export"]}>
                    <button className={style["export-users"]}>
                        <PiExport /> Export
                    </button>
                    <button className={style["create-user"]}>
                        <PiPlus /> Create
                    </button>
                </div>
            </div>

            <div className={style["user-list-header"]}>
                <li className={style["user-select-all"]}>
                    <label>
                        <input
                            type="checkbox"
                            checked={allSelected}
                            onChange={toggleSelectAll}
                        />
                    </label>
                </li>

                <li className={style["user-username"]}>
                    <span>Username</span>
                </li>
                <li className={style["user-email"]}>
                    <span>Email</span>
                </li>
                <li className={style["user-status"]}>
                    <span>Status</span>
                </li>
                <li className={style["user-role"]}>
                    <span>Role</span>
                </li>
                <li className={style["user-joined"]}>
                    <span>Joined</span>
                </li>
                <li className={style["user-actions"]}>
                    <span>Actions</span>
                </li>
            </div>
            <ul className={style["user-list"]}>
                {users.map((user) => (
                    <li key={user.id} className={style["user"]}>
                        <label className={style["user-grid"]}>
                            <input
                                type="checkbox"
                                checked={selectedUsers.includes(user.id)}
                                onChange={() => toggleUser(user.id)}
                            />

                            <span className={style["user-username"]}>
                                <PostAvatar
                                    userId={user.id}
                                    username={user.username}
                                    displayName={user.display_name ?? ""}
                                    avatar={user.avatar ?? null}
                                    thumbhash={user.avatar_thumbhash ?? null}
                                    width={32}
                                    height={32}
                                />
                                {user.username}
                            </span>
                            <span className={style["user-email"]}>
                                {user.email}
                            </span>
                            <span className={style["user-status"]}>
                                {user.is_active ? "Active" : "Inactive"}
                            </span>

                            <span className={style["user-role"]}>
                                {user.is_superuser ? "Admin" : "User"}
                            </span>
                            <span className={style["user-joined"]}>
                                {formatdatemonthyear(user.created_at)}
                            </span>
                            <div className={style["user-actions"]}>
                                <button
                                    className={style["user-action-edit"]}
                                    type="button"
                                    onClick={() => handleEditUser(user.id)}
                                >
                                    <FiEdit />
                                </button>
                                <button
                                    className={style["user-action-delete"]}
                                    type="button"
                                >
                                    <HiOutlineTrash />
                                </button>
                            </div>
                        </label>
                    </li>
                ))}
            </ul>
            {users.length === 0 && <p>No users found.</p>}
        </section>
    );
}
