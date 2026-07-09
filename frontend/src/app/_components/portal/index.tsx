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
    const value = useMemo(
        () => ({
            container:
                container ??
                (typeof document !== "undefined" ? document.body : null),
        }),
        [container],
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

export const Portal = ({
    children,
    target,
    wrapperProps,
}: PortalProps) => {
    const contextContainer = usePortalContainer();
    const [mounted, setMounted] = useState(false);

    useEffect(() => {
        setMounted(true);
    }, []);

    if (!mounted) return null;

    const targetElement =
        target ??
        contextContainer ??
        document.body;

    return createPortal(
        <div {...wrapperProps}>{children}</div>,
        targetElement,
    );
};