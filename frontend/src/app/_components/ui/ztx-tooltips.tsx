import { ReactNode, useRef } from "react";
import { useLayer } from "../layer";

// ! This file is experimental!

export function Tooltip({
	children,
	content,
    position = "auto"
}: {
	children: ReactNode;
	content: ReactNode;
    position?: "auto" | "top" | "bottom" | "left" | "right";
}) {
	const layer = useLayer();
	const ref = useRef<HTMLDivElement>(null);
	const handleRef = useRef<{ close: () => void } | null>(null);

	const open = () => {
		if (!ref.current) return;

		const { top, left } = getTooltipPosition(ref.current, position);

		handleRef.current = layer.open(
			<div
				style={{
					position: "absolute",
					top: top,
					left: left,
					transform: "translate(-50%, -100%)",
					backgroundColor: "black",
					color: "white",
					padding: "5px 10px",
					borderRadius: "4px",
					zIndex: 1000,
				}}
			>
				{content}
			</div>
		);
	};

	const close = () => {
		handleRef.current?.close();
		handleRef.current = null;
	};

	return (
		<div
			ref={ref}
			onMouseEnter={open}
			onMouseLeave={close}
		>
			{children}
		</div>
	);
}

function getTooltipPosition(element: HTMLElement, position: "auto" | "top" | "bottom" | "left" | "right" = "auto") {
	const rect = element.getBoundingClientRect();

    // Get the current scroll position to adjust the tooltip's position accordingly
    const scrollTop = window.pageYOffset || document.documentElement.scrollTop;
    const scrollLeft = window.pageXOffset || document.documentElement.scrollLeft;

    const getAutoPosition = () => {
        const viewportHeight = window.innerHeight;
        const viewportWidth = window.innerWidth;

        const spaceAbove = rect.top;
        const spaceBelow = viewportHeight - rect.bottom;
        const spaceLeft = rect.left;
        const spaceRight = viewportWidth - rect.right;

        if (spaceAbove >= 40) {
            return "top";
        } else if (spaceBelow >= 40) {
            return "bottom";
        } else if (spaceLeft >= 40) {
            return "left";
        } else if (spaceRight >= 40) {
            return "right";
        } else {
            // Default to top if no sufficient space
            return "top";
        }
    }

    if (position === "auto") {
        position = getAutoPosition();
    }

    switch (position) {
        case "top":
            return { top: rect.top + scrollTop, left: rect.left + rect.width / 2 + scrollLeft };
        case "bottom":
            return { top: rect.bottom + scrollTop, left: rect.left + rect.width / 2 + scrollLeft };
        case "left":
            return { top: rect.top + rect.height / 2 + scrollTop, left: rect.left + scrollLeft };
        case "right":
            return { top: rect.top + rect.height / 2 + scrollTop, left: rect.right + scrollLeft };
    }
}