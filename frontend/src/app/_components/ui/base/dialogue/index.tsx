import * as React from "react";
import {
    useFloating,
    useClick,
    useDismiss,
    useRole,
    useInteractions,
    useMergeRefs,
    FloatingFocusManager,
    FloatingOverlay,
    useId,
    useTransitionStatus,
} from "@floating-ui/react";
import { Portal } from "@/app/_components/portal";

interface DialogOptions {
    initialOpen?: boolean;
    open?: boolean;
    onOpenChange?: (open: boolean) => void;
    onClose?: () => void | boolean | Promise<void | boolean>;
    overlayClassName?: string;
    outsidePress?: boolean;
    portalContainer?: HTMLElement | null;
    portalAsChild?: boolean;
    interactable?: boolean;
}

function useDialog({
    initialOpen = false,
    open: controlledOpen,
    onOpenChange: setControlledOpen,
    onClose,
    overlayClassName,
    outsidePress = true,
    portalContainer,
    portalAsChild = false,
    interactable = true,
}: DialogOptions = {}) {
    const [uncontrolledOpen, setUncontrolledOpen] = React.useState(initialOpen);

    const [labelId, setLabelId] = React.useState<string | undefined>();

    const [descriptionId, setDescriptionId] = React.useState<
        string | undefined
    >();

    const open = controlledOpen ?? uncontrolledOpen;
    const setOpen = setControlledOpen ?? setUncontrolledOpen;
    const close = React.useCallback(async () => {
        if (!onClose) {
            setOpen(false);
            return;
        }

        const result = await onClose();

        // * Returning false prevents the dialog from closing.
        if (result === false) {
            return;
        }

        setOpen(false);
    }, [onClose, setOpen]);

    const handleOpenChange = React.useCallback(
        (nextOpen: boolean) => {
            if (nextOpen) {
                setOpen(true);
                return;
            }
            void close();
        },
        [setOpen, close],
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
            close,
            ...interactions,
            ...data,
            labelId,
            descriptionId,
            setLabelId,
            setDescriptionId,
            overlayClassName,
            portalContainer,
            portalAsChild,
            interactable,
        }),
        [
            open,
            setOpen,
            close,
            interactions,
            data,
            labelId,
            descriptionId,
            overlayClassName,
            portalContainer,
            portalAsChild,
            interactable
        ],
    );
}

type ContextType =
    | (ReturnType<typeof useDialog> & {
          setLabelId: React.Dispatch<React.SetStateAction<string | undefined>>;
          setDescriptionId: React.Dispatch<
              React.SetStateAction<string | undefined>
          >;
      })
    | null;

const DialogContext = React.createContext<ContextType>(null);

const useDialogContext = () => {
    const context = React.useContext(DialogContext);

    if (context == null) {
        throw new Error("Dialog components must be wrapped in <Dialog />");
    }

    return context;
};

function Dialog({
    children,
    ...options
}: {
    children: React.ReactNode;
} & DialogOptions) {
    const dialog = useDialog(options);
    return (
        <DialogContext.Provider value={dialog}>
            {children}
        </DialogContext.Provider>
    );
}

interface DialogTriggerProps {
    children: React.ReactNode;
    asChild?: boolean;
}

const DialogTrigger = React.forwardRef<
    HTMLElement,
    React.HTMLProps<HTMLElement> & DialogTriggerProps
>(function DialogTrigger({ children, asChild = false, ...props }, propRef) {
    const context = useDialogContext();
    const childrenRef = (children as any).ref;
    const ref = useMergeRefs([context.refs.setReference, propRef, childrenRef]);

    // `asChild` allows the user to pass any element as the anchor
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
        <button
            ref={ref}
            // The user can style the trigger based on the state
            data-state={context.open ? "open" : "closed"}
            {...context.getReferenceProps(props)}
        >
            {children}
        </button>
    );
});

const DialogContent = React.forwardRef<
    HTMLDivElement,
    React.HTMLProps<HTMLDivElement>
>(function DialogContent(props, propRef) {
    const { context: floatingContext, ...context } = useDialogContext();
    const ref = useMergeRefs([context.refs.setFloating, propRef]);

    const { isMounted, status } = useTransitionStatus(floatingContext);

    if (!isMounted) return null;

    return (
        <Portal
            container={context.portalContainer}
            asChild={context.portalAsChild}
        >
            <FloatingOverlay
                data-status={status}
                className={context.overlayClassName}
                lockScroll
            >
                <FloatingFocusManager context={floatingContext}>
                    <div
                        ref={ref}
                        data-status={status}
                        aria-labelledby={context.labelId}
                        aria-describedby={context.descriptionId}
                        data-interact={context.interactable ? "true" : "false"}
                        {...context.getFloatingProps(props)}
                    >
                        {props.children}
                    </div>
                </FloatingFocusManager>
            </FloatingOverlay>
        </Portal>
    );
});

const DialogHeading = React.forwardRef<
    HTMLHeadingElement,
    React.HTMLProps<HTMLHeadingElement>
>(function DialogHeading({ children, ...props }, ref) {
    const { setLabelId } = useDialogContext();
    const id = useId();

    // Only sets `aria-labelledby` on the Dialog root element
    // if this component is mounted inside it.
    React.useLayoutEffect(() => {
        setLabelId(id);
        return () => setLabelId(undefined);
    }, [id, setLabelId]);

    return (
        <h2 {...props} ref={ref} id={id}>
            {children}
        </h2>
    );
});

const DialogDescription = React.forwardRef<
    HTMLParagraphElement,
    React.HTMLProps<HTMLParagraphElement>
>(function DialogDescription({ children, ...props }, ref) {
    const { setDescriptionId } = useDialogContext();
    const id = useId();

    // Only sets `aria-describedby` on the Dialog root element
    // if this component is mounted inside it.
    React.useLayoutEffect(() => {
        setDescriptionId(id);
        return () => setDescriptionId(undefined);
    }, [id, setDescriptionId]);

    return (
        <p {...props} ref={ref} id={id}>
            {children}
        </p>
    );
});

const DialogClose = React.forwardRef<
    HTMLButtonElement,
    React.ButtonHTMLAttributes<HTMLButtonElement>
>(function DialogClose({ onClick, ...props }, ref) {
    const { close } = useDialogContext();
    return (
        <button
            type="button"
            {...props}
            ref={ref}
            onClick={(event) => {
                onClick?.(event);
                if (!event.defaultPrevented) {
                    void close();
                }
            }}
        />
    );
});

export const DialogPrimitive = {
    Root: Dialog,
    Trigger: DialogTrigger,
    Content: DialogContent,
    Heading: DialogHeading,
    Description: DialogDescription,
    Close: DialogClose,
};
