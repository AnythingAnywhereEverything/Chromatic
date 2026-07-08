"use client";

import React from "react";
import ReactDOM from "react-dom";
type PortalElement = HTMLDivElement;

interface PortalProps extends React.HTMLAttributes<PortalElement> {
    container?: Element | DocumentFragment | null;
}

type PortalProviderContextType = {
    container: Element | DocumentFragment | null;
};

const PortalProviderContext = React.createContext<PortalProviderContextType | null>(null);

export const usePortalProviderContext = () => {
    const context = React.useContext(PortalProviderContext);
    if (!context) {
        throw new Error("usePortalProviderContext must be used within a PortalProvider");
    }
    return context;
};

// use to get the portal container from the context, or fallback to document.body
export const usePortalContainer = () => {
    const context = usePortalProviderContext();
    return context.container || (typeof document !== "undefined" ? document.body : null);
};

// Generate dic of given div within provider
export const PortalProvider: React.FC<PortalProps> = ({ children, ...props }) => {
    const [container, setContainer] = React.useState<Element | DocumentFragment | null>(null);

    const containerRef = React.useRef<HTMLDivElement>(null);

    React.useEffect(() => {
        if (containerRef.current) {
            setContainer(containerRef.current);
        }
    }, []);

    return (
        <PortalProviderContext.Provider value={{ container }}>
            {children}
            <div ref={containerRef} {...props}></div>
        </PortalProviderContext.Provider>
    );
};

const Portal = React.forwardRef<PortalElement, PortalProps>((props, ref) => {
    const { container: containerProp, ...portalProps } = props;
    const [isMounted, setIsMounted] = React.useState(false);

    React.useEffect(() => {
        setIsMounted(true);
        return () => setIsMounted(false);
    }, []);
    const container = containerProp || (isMounted && globalThis?.document?.body);

    return container ?
        ReactDOM.createPortal(<div ref={ref} {...portalProps} />, container) :
        null;
});

export { Portal };