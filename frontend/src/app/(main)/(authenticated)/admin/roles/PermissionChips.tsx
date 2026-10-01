    
"use client";

import style from "./role-manage.module.scss";
import { STAFF_PERMISSIONS } from "@/api/admin/types";

/**
 * The six staff permissions as toggle chips.
 *
 * Shared by the create and edit dialogs, while also supporting a read-only
 * summary in the role table.
 *
 * Read-only mode deliberately uses <span> instead of <button>. The role row
 * itself is a <button>, so nested buttons would produce invalid HTML and a
 * hydration error.
 */
export default function PermissionChips({
    selected,
    onToggle,
}: {
    /** Current codepoints. Order is irrelevant; it gets normalised on toggle. */
    selected: number[];

    /** Absent renders the chips as read-only badges. */
    onToggle?: (bit: number) => void;
}) {
    const readOnly = !onToggle;

    return (
        <div className={style["permission-chips"]}>
            {STAFF_PERMISSIONS.map(({ bit, label, description }) => {
                const isOn = selected.includes(bit);

                if (readOnly) {
                    return (
                        <span
                            key={bit}
                            title={description}
                            aria-label={description}
                            className={`${style["permission-chip"]} ${
                                isOn ? style["permission-chip-on"] : ""
                            }`}
                        >
                            {label}
                        </span>
                    );
                }

                return (
                    <button
                        key={bit}
                        type="button"
                        title={description}
                        aria-pressed={isOn}
                        className={`${style["permission-chip"]} ${
                            isOn ? style["permission-chip-on"] : ""
                        }`}
                        onClick={() => onToggle(bit)}
                    >
                        {label}
                    </button>
                );
            })}
        </div>
    );
}
