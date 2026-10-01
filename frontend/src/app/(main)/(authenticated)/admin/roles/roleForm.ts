import { STAFF_PERMISSION_BITS } from "@/api/admin/types";

/** The editable fields of a role, as the dialogs hold them. */
export interface RoleFormState {
    name: string;
    description: string;
    position: number;
    permission_bitmask: number[];
}

export function togglePermission(
    mask: number[],
    bit: number,
): number[] {
    const next = mask.includes(bit)
        ? mask.filter((value) => value !== bit)
        : [...mask, bit];

    return [...next].sort((a, b) => a - b);
}

/**
 * Whether anything actually changed, used to disable Save and to decide what to
 * put in the PATCH body.
 *
 * The mask comparison is order-insensitive for the reason above. `position` is
 * a number in the form and an `i32` from the server, so it is compared as a
 * number rather than by string.
 */
export function roleFormChanges(
    form: RoleFormState,
    original: RoleFormState,
): Partial<RoleFormState> {
    const changes: Partial<RoleFormState> = {};

    if (form.name !== original.name) changes.name = form.name;
    if (form.description !== original.description) {
        changes.description = form.description;
    }
    if (form.position !== original.position) {
        changes.position = form.position;
    }
    if (!sameMask(form.permission_bitmask, original.permission_bitmask)) {
        changes.permission_bitmask = form.permission_bitmask;
    }

    return changes;
}

/** Order-insensitive mask equality. */
function sameMask(a: number[], b: number[]): boolean {
    if (a.length !== b.length) return false;
    const left = [...a].sort((x, y) => x - y);
    const right = [...b].sort((x, y) => x - y);
    return left.every((value, index) => value === right[index]);
}

/**
 * Drops codepoints the panel does not know about before sending.
 *
 * Only ever used on create, where the form starts from `[]` and every value came
 * from a chip, so this is belt-and-braces. It exists because the backend accepts
 * an arbitrary `i64` codepoint and silently stores it — an unknown value would
 * round-trip forever and never render as anything.
 */
export function knownPermissions(mask: number[]): number[] {
    return mask
        .filter((bit) => STAFF_PERMISSION_BITS.includes(bit))
        .sort((a, b) => a - b);
}