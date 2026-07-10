import { FloatingDelayGroup } from "@floating-ui/react";
import { TooltipPrimitive } from "../../base/tooltip";

import styles from "./style.module.scss";

const TooltipContent = ({
    ...props
}: React.ComponentProps<typeof TooltipPrimitive.Content>) => {
    return (
        <TooltipPrimitive.Content
            className={styles["content"]}
            data-component="tooltip-content"
            {...props}
        />
    );
};

interface TooltipProps extends React.ComponentProps<
    typeof TooltipPrimitive.Root
> {
    openDelayDuration?: number;
    closeDelayDuration?: number;
}

const Tooltip = ({
    openDelayDuration,
    closeDelayDuration,
    ...props
}: TooltipProps) => {
    if (
        (openDelayDuration !== undefined && openDelayDuration > 0) ||
        (openDelayDuration === undefined &&
            closeDelayDuration !== undefined &&
            closeDelayDuration > 0)
    ) {
        return (
            <FloatingDelayGroup delay={{ open: openDelayDuration, close: closeDelayDuration }}>
                <TooltipPrimitive.Root data-component="tooltip" {...props} />
            </FloatingDelayGroup>
        );
    }

    return <TooltipPrimitive.Root data-component="tooltip" {...props} />;
};

const TooltipTrigger = ({
    ...props
}: React.ComponentProps<typeof TooltipPrimitive.Trigger>) => {
    return (
        <TooltipPrimitive.Trigger data-component="tooltip-trigger" {...props} />
    );
};

const TooltipArrow = ({
    ...props
}: React.ComponentProps<typeof TooltipPrimitive.Arrow>) => {
    return <TooltipPrimitive.Arrow data-component="tooltip-arrow" {...props} />;
};

const TooltipAnchor = ({
    ...props
}: React.ComponentProps<typeof TooltipPrimitive.Anchor>) => {
    return <TooltipPrimitive.Anchor data-component="tooltip-anchor" {...props} />;
};

export { Tooltip, TooltipTrigger, TooltipContent, TooltipArrow, TooltipAnchor };
