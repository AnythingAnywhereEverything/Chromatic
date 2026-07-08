export type Positions = "top" | "bottom" | "left" | "right";

export interface TooltipOptions {
    offset?: number;
    delay?: number;
    closeDelay?: number;
    disableHover?: boolean;
    position?: Positions;
    anchor?: HTMLElement | null; // New: specific element to align arrow to
}

export interface TooltipPosition {
    top: number;
    left: number;
    position: Positions;
}