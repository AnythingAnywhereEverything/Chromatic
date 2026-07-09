/**
 * Objective: Create a better tooltip component that is more flexible and customizable than the current one.
 *
 * Features:
 * - Customizable position (top, bottom, left, right)
 * - Customizable offset
 * - Delay before showing and hiding the tooltip
 * - Support for rich content (HTML, React components)
 * - Accessible (ARIA attributes)
 * - Class names for styling
 * - Auto positioning to avoid clipping
 * - Component-based API for better integration with React
 * - Optionally has Arrow Anchor
 * - Tooltip content will be moved to a portal to avoid clipping issues
 * - Improve positioning to allow alignment of the tooltip relative to the trigger element (e.g., center, start, end)
 *
 * Usage:
 * <Tooltip align="center" position="top" offset={10} delay={300}>
 *      <TooltipContent>
 *          <div>Tooltip content goes here</div>
 *          <TooltipArrow />
 *      </TooltipContent>
 *      <TooltipTrigger>
 *          <TooltipTriggerAnchor>
 *              <div>Anchored</div>
 *          </TooltipTriggerAnchor>
 *          <div>
 *              This is a tooltip trigger
 *          </div>
 *      </TooltipTrigger>
 * </Tooltip>
 */

import React from "react";
import { Positions } from "./types";
import { Portal } from "../../portal";
import { getTooltipPosition } from "./utils";
import style from "./style.module.scss";

type TooltipContext = {
    content: React.ReactNode | null;
    setContent: (content: React.ReactNode | null) => void;
    anchor: React.RefObject<HTMLElement | null> | null;
    setAnchor: (anchor: React.RefObject<HTMLElement | null> | null) => void;
    options: {
        offset: number;
        delay: number;
        closeDelay: number;
        position: Positions;
        disableHoverableContent: boolean;
    };
};

const TooltipContext = React.createContext<TooltipContext | null>(null);

interface TooltipProps {
    children: React.ReactNode;
    align?: "start" | "center" | "end";
    offset?: number;
    delay?: number;
    closeDelay?: number;
    disableHoverableContent?: boolean;
    position?: Positions;
}

const Tooltip: React.FC<TooltipProps> = (props) => {
    const {
        children,
        offset = 8,
        delay = 0,
        closeDelay = 300,
        disableHoverableContent = false,
        align = "center",
        position = "top",
    } = props;
    const [content, setContent] = React.useState<React.ReactNode | null>(null);
    const [anchor, setAnchor] = React.useState<React.RefObject<HTMLElement | null> | null>(null);

    const contextValue = React.useMemo(
        () => ({
            content,
            setContent,
            anchor,
            setAnchor,
            options: { offset, delay, closeDelay, position, disableHoverableContent },
        }),
        [content, offset, delay, closeDelay, position, disableHoverableContent],
    );

    return (
        <TooltipContext.Provider value={contextValue}>
            {children}
        </TooltipContext.Provider>
    );
};

const useProviderContext = () => {
    const context = React.useContext(TooltipContext);
    if (!context) {
        throw new Error("TooltipTrigger must be used within a Tooltip");
    }
    return context;
};

interface TooltipAnchorProps extends React.HTMLAttributes<HTMLDivElement> {
    children: React.ReactNode;
    asChild?: boolean;
}

const TooltipAnchor: React.FC<TooltipAnchorProps> = ({
    children,
    asChild = false,
    ...props
}) => {
    const context = useProviderContext();
    const { setAnchor } = context;

    const anchorRef = React.useRef<HTMLElement | null>(null);

    React.useEffect(() => {
        setAnchor(anchorRef);

        return () => setAnchor(null);
    }, [setAnchor]);

    return asChild ? (
        React.cloneElement(children as React.ReactElement<any>, {
            ...props,
            ref: (node: HTMLElement) => {
                anchorRef.current = node;

                const childRef = (children as any).ref;
                if (typeof childRef === "function") childRef(node);
                else if (childRef) childRef.current = node;
            },
        })
    ) : (
        <div ref={anchorRef as React.RefObject<HTMLDivElement>} {...props}>
            {children}
        </div>
    );
};

interface TooltipArrowProps {
    children?: React.ReactNode;
    anchorRef?: React.RefObject<HTMLElement>;
}

const TooltipArrow: React.FC<TooltipArrowProps> = ({ children }) => {
    return <div className={style["tooltip-arrow"]}>{children}</div>;
};

interface TooltipContentProps {
    children: React.ReactNode;
}

const TooltipContent: React.FC<TooltipContentProps> = ({ children }) => {
    const context = useProviderContext();
    const { setContent } = context;

    React.useEffect(() => {
        setContent(children);
        return () => setContent(null);
    }, [children, setContent]);

    return null;
};

interface TooltipTriggerProps extends React.HTMLAttributes<HTMLDivElement> {
    children: React.ReactNode;
    asChild?: boolean;
}

const TooltipTrigger: React.FC<TooltipTriggerProps> = (props) => {
    const context = useProviderContext();

    // Destructure context and props
    const { content, options, anchor } = context;
    const { children, asChild = false, ...rest } = props;
    const { offset, position, closeDelay, delay, disableHoverableContent } = options;

    const compId = React.useId();
    
    // References
    const triggerRef = React.useRef<HTMLDivElement>(null);
    const closeDelayTimerRef = React.useRef<NodeJS.Timeout | null>(null);

    const [isVisible, setIsVisible] = React.useState(false);
    const [isFullyClosed, setIsFullyClosed] = React.useState(true);
    const [coords, setCoords] = React.useState({
        top: 0,
        left: 0,
        position: position,
    });

    const updatePosition = React.useCallback(() => {

        const targetElement = anchor?.current ?? triggerRef.current;
        console.log("Updating tooltip position for target:", targetElement);
        if (!targetElement) return;
        const {
            top,
            left,
            position: pos,
        } = getTooltipPosition(targetElement, position, offset);
        setCoords({ top, left, position: pos });
    }, [position, offset, anchor]);

    const renderTooltip = () => {
        return (
            <div
                className={`${style["tooltip-wrapper"]} ${style[coords.position]} ${disableHoverableContent ? style["disable-hoverable-content"] : ""}`}
                style={{ 
                    top: `${coords.top}px`, 
                    left: `${coords.left}px`,
                    "--tooltip-offset": `${offset}px`,
                } as React.CSSProperties}
                onMouseEnter={handleMouseEnter}
                onMouseLeave={handleMouseLeave}
            >
                <div
                    className={`${style["tooltip-content"]} ${isVisible ? style["visible"] : style["hidden"]}`}
                >
                    {content}
                    <div className={style["tooltip-arrow"]} />
                </div>
            </div>
        );
    };

    const handleMouseEnter = () => {
        if (closeDelayTimerRef.current) clearTimeout(closeDelayTimerRef.current);
        updatePosition();
        setIsFullyClosed(false);
        setIsVisible(true);
        window.addEventListener("scroll", updatePosition, true);
    };

    const handleMouseLeave = () => {
        if (closeDelayTimerRef.current) clearTimeout(closeDelayTimerRef.current);
        setIsVisible(false);
        closeDelayTimerRef.current = setTimeout(() => {
            setIsFullyClosed(true);
            window.removeEventListener("scroll", updatePosition, true);
        }, closeDelay);
    };

    return asChild ? (
        <>
            {React.cloneElement(children as React.ReactElement, {
                ...(React.isValidElement(children) ? (children.props as any) : {}),
                ref: (node: HTMLElement) => {
                    (
                        triggerRef as React.RefObject<HTMLElement | null>
                    ).current = node;
                    const { ref } = children as any;
                    if (typeof ref === "function") ref(node);
                    else if (ref) ref.current = node;
                },
                onMouseEnter: handleMouseEnter,
                onMouseLeave: handleMouseLeave,
            })}
            {!isFullyClosed && <TooltipPortal>{renderTooltip()}</TooltipPortal>}
        </>
    ) : (
        <div
            ref={triggerRef}
            onMouseEnter={handleMouseEnter}
            onMouseLeave={handleMouseLeave}
            {...rest}
        >
            {children}
            {!isFullyClosed && <TooltipPortal>{renderTooltip()}</TooltipPortal>}
        </div>
    );
};

const TooltipPortal: React.FC<{ children: React.ReactNode }> = ({ children }) => {
    return <Portal>{children}</Portal>;
};

export { Tooltip, TooltipContent, TooltipTrigger, TooltipAnchor, TooltipArrow };
