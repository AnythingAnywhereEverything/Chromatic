import { DialogPrimitive } from "../../base/dialogue";

import styles from "./style.module.scss";

const Dialog = ({
    ...props
}: React.ComponentProps<typeof DialogPrimitive.Root>) => {
    return <DialogPrimitive.Root data-component="dialog" overlayClassName={styles["dialog-overlay"]} {...props} />;
}

const DialogTrigger = ({
    ...props
}: React.ComponentProps<typeof DialogPrimitive.Trigger>) => {
    return (
        <DialogPrimitive.Trigger data-component="dialog-trigger" {...props} />
    );
}

const DialogContent = ({
    ...props
}: React.ComponentProps<typeof DialogPrimitive.Content>) => {
    return (
        <DialogPrimitive.Content data-component="dialog-content" {...props} />
    );
}

const DialogHeading = ({
    ...props
}: React.ComponentProps<typeof DialogPrimitive.Heading>) => {
    return (
        <DialogPrimitive.Heading data-component="dialog-heading" {...props} />
    );
}

const DialogDescription = ({
    ...props
}: React.ComponentProps<typeof DialogPrimitive.Description>) => {
    return (
        <DialogPrimitive.Description
            data-component="dialog-description"
            {...props}
        />
    );
}

const DialogClose = ({
    ...props
}: React.ComponentProps<typeof DialogPrimitive.Close>) => {
    return (
        <DialogPrimitive.Close data-component="dialog-close" {...props} />
    );
}

export {
    Dialog,
    DialogTrigger,
    DialogContent,
    DialogHeading,
    DialogDescription,
    DialogClose,
};