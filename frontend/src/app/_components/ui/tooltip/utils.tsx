import type { Positions } from './types';

function getTooltipPosition(anchor: HTMLElement, position: Positions, offset: number) {
    const rect = anchor.getBoundingClientRect();
    
    // get the current scroll position to adjust the tooltip's position
    const scrollY = window.scrollY || window.pageYOffset;
    const scrollX = window.scrollX || window.pageXOffset;

    // get window dimensions
    const windowWidth = window.innerWidth;
    const windowHeight = window.innerHeight;

    // get element width and height
    const elementWidth = rect.width;
    const elementHeight = rect.height;

    let top = 0;
    let left = 0;
    let pos = position;

    const getAbsolutePosition = (position: Positions) => {
        switch (position) {
            case "top":
                top = rect.top + scrollY - offset;
                left = rect.left + scrollX + rect.width / 2;
                break;
            case "bottom":
                top = rect.bottom + scrollY + offset;
                left = rect.left + scrollX + rect.width / 2;
                break;
            case "left":
                top = rect.top + scrollY + rect.height / 2;
                left = rect.left + scrollX - offset;
                break;
            case "right":
                top = rect.top + scrollY + rect.height / 2;
                left = rect.right + scrollX + offset;
                break;
        }
    }
    getAbsolutePosition(position);

    // Ensure the tooltip doesn't go off-screen
    // First guard, switch to most reliable position if the tooltip is out of bounds
    const isOutOfBounds = (top < 0 || top + rect.height > windowHeight || left < 0 || left + rect.width > windowWidth);
    if (isOutOfBounds) {
        const positions: Positions[] = ["top", "bottom", "left", "right"];
        for (const p of positions) {
            getAbsolutePosition(p);
            // Check due to different offsets for top/left vs bottom/right, we need to check both conditions
            if (p === 'top' || p === 'left') {
                if (top - elementHeight >= 0 || left - elementWidth >= 0 ) {
                    pos = p;
                    break;
                } else {
                    if (top + elementHeight <= windowHeight || left + elementWidth <= windowWidth) {
                        pos = p;
                        break;
                    }
                }
            }
        }
    }

    if (top < 0) {
        top = 0;
    } else if (top + elementHeight > windowHeight + elementHeight) {
        top = windowHeight - elementHeight;
    }
    if (left < 0) {
        left = 0;
    } else if (left + elementWidth > windowWidth) {
        left = windowWidth - elementWidth;
    }
    return { top, left, position: pos };
}
export { getTooltipPosition };