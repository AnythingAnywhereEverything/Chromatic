import {
    createContext,
    Fragment,
    useContext,
    useMemo,
    useReducer,
    useRef,
    type ReactNode,
} from "react";

type LayerInstance = {
    id: symbol;
    node: ReactNode;
};

type LayerHandle = {
    update(node: ReactNode): void;
    close(): void;
};

type LayerAPI = {
    open(node: ReactNode): LayerHandle;
};

const LayerContext = createContext<LayerAPI | null>(null);

export function LayerProvider({ children }: { children: ReactNode }) {
    const registry = useRef(new Map<symbol, LayerInstance>());

    const rerender = useReducer(x => x + 1, 0)[1];

    const api = useMemo<LayerAPI>(() => {
        return {
            open(node) {
                const id = Symbol();

                registry.current.set(id, { id, node });
                rerender();

                return {
                    update(next) {
                        const layer = registry.current.get(id);
                        if (!layer) return;

                        layer.node = next;
                        rerender();
                    },
                    close() {
                        registry.current.delete(id);
                        rerender();
                    },
                };
            },
        };
    }, [rerender]); // * stable dependency prevents stale closure issues

    return (
        <LayerContext.Provider value={api}>
            {children}

            <div className="layerContainer">
                {[...registry.current.values()].map(layer => (
                    <Fragment key={layer.id.toString()}>
                        {layer.node}
                    </Fragment>
                ))}
            </div>
        </LayerContext.Provider>
    );
}

export function useLayer() {
    const layer = useContext(LayerContext);

    if (!layer) {
        throw new Error("Missing LayerProvider.");
    }

    return layer;
}
