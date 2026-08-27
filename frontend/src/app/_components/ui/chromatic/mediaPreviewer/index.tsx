// based media previwer

import {
    useClick,
    useDismiss,
    useFloating,
    useInteractions,
    useMergeRefs,
    useRole,
} from "@floating-ui/react";
import React from "react";

interface MediaPreviewerProps {
    initialOpen: boolean;
    open?: boolean;
    onOpenChange?: (open: boolean) => void;
    outsidePress?: boolean;
    overlayClassName?: string;
}

type MediaTypes = "image" | "video";

function useMediaPreviewer({
    initialOpen = false,
    open: controlledOpen,
    onOpenChange: setControlledOpen,
    overlayClassName,
    outsidePress = true,
}: MediaPreviewerProps) {
    const [uncontrolledOpen, setUncontrolledOpen] = React.useState(initialOpen);

    const open = controlledOpen ?? uncontrolledOpen;
    const setOpen = setControlledOpen ?? setUncontrolledOpen;

    const handleOpenChange = React.useCallback(
        (nextOpen: boolean) => {
            if (nextOpen) {
                setOpen(true);
                return;
            }
            setOpen(false);
        },
        [setOpen],
    );

    const data = useFloating({
        open,
        onOpenChange: handleOpenChange,
    });

    const context = data.context;

    const click = useClick(context, {
        enabled: controlledOpen == null,
    });

    const dismiss = useDismiss(context, {
        outsidePressEvent: "mousedown",
        outsidePress,
    });

    const role = useRole(context);

    const interactions = useInteractions([click, dismiss, role]);

    return React.useMemo(
        () => ({
            open,
            setOpen,
            handleOpenChange,
            overlayClassName,
            ...interactions,
            ...data,
        }),
        [open, setOpen, handleOpenChange, overlayClassName, interactions, data],
    );
}

type ContextType = ReturnType<typeof useMediaPreviewer>;

const useMediaPreviewerContext = () => {
    const context = React.useContext(MediaPreviewerContext);
    if (!context) {
        throw new Error(
            "useMediaPreviewerContext must be used within a MediaPreviewerProvider",
        );
    }
    return context;
};

const MediaPreviewerContext = React.createContext<ContextType | undefined>(
    undefined,
);

function MediaPreviewer({
    children,
    ...IoOptions
}: {
    children: React.ReactNode;
} & MediaPreviewerProps) {
    return (
        <MediaPreviewerContext.Provider value={useMediaPreviewer(IoOptions)}>
            {children}
        </MediaPreviewerContext.Provider>
    );
}

interface MediaPreviewerTriggerProps {
    children: React.ReactNode;
    asChild?: boolean;
}

const MediaPreviewerTrigger = React.forwardRef<
    HTMLElement,
    React.HTMLProps<HTMLElement> & MediaPreviewerTriggerProps
>(function MediaPreviewerTrigger(
    { children, asChild = false, ...props },
    propRef,
) {
    const context = useMediaPreviewerContext();
    const childrenRef = (children as any).ref;
    const ref = useMergeRefs([context.refs.setReference, propRef, childrenRef]);

    if (asChild && React.isValidElement(children)) {
        return React.cloneElement(
            children,
            context.getReferenceProps({
                ref,
                ...props,
                ...(children.props as Record<string, unknown>),
                "data-state": context.open ? "open" : "closed",
            } as Record<string, unknown>),
        );
    }
    return (
        <button type="button" {...context.getReferenceProps({ ref, ...props })}>
            {children}
        </button>
    );
});

// load full image
interface MediaPreviewerContentProps {
    children: React.ReactNode;
    src: string;
    type: MediaTypes;
    className?: string;
}

const MediaPreviewerContent = React.forwardRef<
    HTMLDivElement,
    React.HTMLProps<HTMLDivElement> & MediaPreviewerContentProps
>(function MediaPreviewerContent({ children, ...props }, propRef) {
    const context = useMediaPreviewerContext();
    const ref = useMergeRefs([context.refs.setFloating, propRef]);
    return (
        <div {...context.getFloatingProps({ ref, ...props })}>
            {children}
        </div>
    );
});

export { useMediaPreviewerContext, MediaPreviewer, useMediaPreviewer };
