"use client";

import React, {
    createContext,
    useContext,
    useEffect,
    useState,
    useMemo,
    ReactNode,
} from "react";
import { createPortal } from "react-dom";

type PortalContainer = HTMLElement | null;

interface PortalProviderContextType {
    container: PortalContainer;
}

interface PortalProviderProps {
    children: ReactNode;
    /** An optional element to use as the portal container. Defaults to document.body */
    container?: PortalContainer;
}

const PortalContext = createContext<PortalProviderContextType | undefined>(
    undefined,
);

export const usePortalContainer = () => {
    const context = useContext(PortalContext);
    if (context === undefined) {
        throw new Error(
            "usePortalContainer must be used within a PortalProvider",
        );
    }
    return context.container;
};

export const PortalProvider = ({
    children,
    container,
}: PortalProviderProps) => {
    const [mounted, setMounted] = useState(false);
    const [internalContainer, setInternalContainer] =
        useState<HTMLElement | null>(null);

    useEffect(() => {
        setMounted(true);
        // If no container prop is provided, we use body,
        // but we wait until useEffect to ensure we are on the client.
        if (!container) {
            setInternalContainer(document.body);
        }
    }, [container]);

    if (!mounted) {
        return null;
    }

    // Memoize context to prevent unnecessary re-renders
    const value = useMemo(
        () => ({
            container: container || internalContainer,
        }),
        [container, internalContainer],
    );

    return (
        <PortalContext.Provider value={value}>
            {children}
        </PortalContext.Provider>
    );
};

interface PortalProps {
    children: ReactNode;
    /** Target element for the portal. If not provided, it uses the PortalProvider context or document.body */
    target?: HTMLElement | null;
    /** A wrapper element for the portaled content (Optional) */
    wrapperProps?: React.HTMLAttributes<HTMLDivElement>;
}

export const Portal = ({ children, target, wrapperProps }: PortalProps) => {
    const [mounted, setMounted] = useState(false);

    let contextContainer = null;
    try {
        contextContainer = usePortalContainer();
    } catch (error) {
        // If the hook throws, it means we're not inside a PortalProvider.
        // We'll just ignore and fallback to target or document.body.
    }

    useEffect(() => {
        setMounted(true);
    }, []);

    if (!mounted) return null;

    // Priority: target | Context container | Fallback to body
    const targetElement =
        target ||
        contextContainer ||
        (typeof document !== "undefined" ? document.body : null);

    console.log("Portal component using targetElement:", targetElement);

    if (!targetElement) return null;

    return createPortal(<div {...wrapperProps}>{children}</div>, targetElement);
};
