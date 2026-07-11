"use client";

import * as React from "react";
import {
    useFloating,
    autoUpdate,
    offset,
    flip,
    shift,
    useHover,
    useFocus,
    useDismiss,
    useRole,
    useInteractions,
    useDelayGroup,
    useMergeRefs,
    useTransitionStyles,
    safePolygon,
    arrow,
    FloatingArrow,
    useTransitionStatus,
} from "@floating-ui/react";
import type {
    FloatingArrowProps,
    Middleware,
    Placement,
} from "@floating-ui/react";
import { Portal } from "../../../portal";

interface TooltipOptions {
    initialOpen?: boolean;
    placement?: Placement;
    open?: boolean;
    onOpenChange?: (open: boolean) => void;
    allowHovering?: boolean;
}

const anchorArrow = ({
    anchorRef,
    arrowRef,
}: {
    anchorRef: React.RefObject<HTMLElement | null>;
    arrowRef: React.RefObject<HTMLElement | null>;
}): Middleware => ({
    name: "anchorArrow",

    async fn(state) {
        const anchor = anchorRef.current;
        if (!anchor) {
            return {};
        }

        const anchorRect = anchor.getBoundingClientRect();
        const floatingRect = state.elements.floating.getBoundingClientRect();

        let x: number | undefined;
        let y: number | undefined;

        const arrowWidth = arrowRef.current?.clientWidth ?? 0;
        const arrowHeight = arrowRef.current?.clientHeight ?? 0;

        switch (state.placement.split("-")[0]) {
            case "top":
            case "bottom":
                x =
                    anchorRect.left +
                    anchorRect.width / 2 -
                    floatingRect.left -
                    arrowWidth / 2;
                break;

            case "left":
            case "right":
                y =
                    anchorRect.top +
                    anchorRect.height / 2 -
                    floatingRect.top -
                    arrowHeight / 2;
                break;
        }
        
        return {
            data: {
                x,
                y,
            },
        };

    },
});

export function useTooltip({
    initialOpen = false,
    placement = "top",
    open: controlledOpen,
    onOpenChange: setControlledOpen,
    allowHovering = false,
}: TooltipOptions = {}) {
    const [uncontrolledOpen, setUncontrolledOpen] = React.useState(initialOpen);
    const open = controlledOpen ?? uncontrolledOpen;
    const setOpen = setControlledOpen ?? setUncontrolledOpen;

    const { context: delayContext } = useFloating();
    const { delay } = useDelayGroup(delayContext);

    const arrowRef = React.useRef(null);
    const arrowAnchorRef = React.useRef<HTMLElement | null>(null);

    const data = useFloating({
        placement,
        open,
        onOpenChange: setOpen,
        whileElementsMounted: autoUpdate,
        middleware: [
            offset(5),
            flip(),
            shift({
                mainAxis: true,
                padding: 8,
            }),
            arrow({ element: arrowRef }),
            anchorArrow({ anchorRef: arrowAnchorRef, arrowRef }),
        ],
    });

    const context = data.context;

    const hover = useHover(context, {
        move: false,
        enabled: controlledOpen == null,
        delay,
        handleClose: allowHovering ? safePolygon() : undefined,
    });
    const focus = useFocus(context, {
        enabled: controlledOpen == null,
    });
    const dismiss = useDismiss(context);
    const role = useRole(context, { role: "tooltip" });

    const interactions = useInteractions([hover, focus, dismiss, role]);

    return React.useMemo(
        () => ({
            open,
            setOpen,
            arrowAnchorRef,
            ...interactions,
            ...data,
            getArrowProps: () => ({
                ref: arrowRef,
                context: data.context,
            }),
        }),
        [open, setOpen, interactions, data],
    );
}

type ContextType = ReturnType<typeof useTooltip> | null;

const TooltipContext = React.createContext<ContextType>(null);

export const useTooltipState = () => {
    const context = React.useContext(TooltipContext);

    if (context == null) {
        throw new Error("Tooltip components must be wrapped in <Tooltip />");
    }

    return context;
};

type TooltipArrowProps = Omit<FloatingArrowProps, "ref" | "context">;

export const TooltipArrowAnchor = React.forwardRef<
    HTMLElement,
    React.HTMLProps<HTMLElement>
>(function TooltipArrowAnchor(props, propRef) {
    const state = useTooltipState();

    const ref = useMergeRefs([state.arrowAnchorRef, propRef]);

    return <div ref={ref} {...props} />;
});

export function TooltipArrow(props: TooltipArrowProps) {
    const state = useTooltipState();

    const middlewareData = state.middlewareData["anchorArrow"];

    let style: React.CSSProperties = {};

    if (middlewareData) {
        if (middlewareData.x != null) {
            style.left = `${middlewareData.x}px`;
        }
        if (middlewareData.y != null) {
            style.top = `${middlewareData.y}px`;
        }
    }

    return (
        <FloatingArrow
            {...state.getArrowProps()}
            {...props}
            style={{
                ...props.style,
                ...style,
            }}
        />
    );
}

export function Tooltip({
    children,
    ...options
}: { children: React.ReactNode } & TooltipOptions) {
    // This can accept any props as options, e.g. `placement`,
    // or other positioning options.
    const tooltip = useTooltip(options);
    return (
        <TooltipContext.Provider value={tooltip}>
            {children}
        </TooltipContext.Provider>
    );
}

export const TooltipTrigger = React.forwardRef<
    HTMLElement,
    React.HTMLProps<HTMLElement> & { asChild?: boolean }
>(function TooltipTrigger({ children, asChild = false, ...props }, propRef) {
    const state = useTooltipState();

    const childrenRef = (children as any).ref;
    const ref = useMergeRefs([state.refs.setReference, propRef, childrenRef]);

    if (asChild && React.isValidElement(children)) {
        return React.cloneElement(
            children,
            state.getReferenceProps({
                ref,
                ...props,
                ...(children.props as Record<string, unknown>),
                "data-state": state.open ? "open" : "closed",
            } as Record<string, unknown>),
        );
    }

    return (
        <button
            ref={ref}
            data-state={state.open ? "open" : "closed"}
            {...state.getReferenceProps(props)}
        >
            {children}
        </button>
    );
});

export const TooltipContent = React.forwardRef<
    HTMLDivElement,
    React.HTMLProps<HTMLDivElement>
>(function TooltipContent(
    { children, style, className, ...props },
    propRef,
) {
    const state = useTooltipState();

    const ref = useMergeRefs([state.refs.setFloating, propRef]);

    useDelayGroup(state.context, { id: state.context.floatingId });

    const { isMounted, status } = useTransitionStatus(state.context);

    if (!isMounted) return null;

    return (
        <Portal>
            <div
                ref={ref}
                style={state.floatingStyles}
                {...state.getFloatingProps()}
            >
                <div
                    className={className}
                    data-status={status}
                    data-side={state.placement.split("-")[0]}
                    style={style}
                    {...props}
                >
                    {children}
                </div>
            </div>
        </Portal>
    );
});

const TooltipPrimitive = {
    Provider: Tooltip,
    Root: Tooltip,
    Trigger: TooltipTrigger,
    Content: TooltipContent,
    Anchor: TooltipArrowAnchor,
    Arrow: TooltipArrow,
};

export { TooltipPrimitive };