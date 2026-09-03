import { Portal } from "@/app/_components/portal";
import EmojiPicker, { Categories, EmojiStyle, Theme } from "emoji-picker-react";
import React from "react";
import {
    autoUpdate,
    useFloating,
    offset,
    Placement,
    flip,
    shift,
    useInteractions,
    useDismiss,
} from "@floating-ui/react";

interface EPickerProps {
    children?: React.ReactNode;
    placement?: Placement;
    initialOpen?: boolean;
    controlledOpen?: boolean;
    setControlledOpen?: (open: boolean) => void;
    onEmojiClick?: (emojiObject: any, event: MouseEvent) => void;
}

import style from "./epicker.module.scss";
import { FaBowlFood, FaClockRotateLeft } from "react-icons/fa6";
import { FaCubes, FaLeaf, FaPlane, FaRegSmileWink } from "react-icons/fa";
import { IoFlagSharp, IoGameController } from "react-icons/io5";
import { MdEmojiSymbols } from "react-icons/md";

const EPicker = ({
    children,
    placement = "bottom-start",
    initialOpen = false,
    controlledOpen,
    setControlledOpen,
    onEmojiClick,
}: EPickerProps) => {
    const [uncontrolledOpen, setUncontrolledOpen] = React.useState(initialOpen);

    const open = controlledOpen ?? uncontrolledOpen;
    const setOpen = setControlledOpen ?? setUncontrolledOpen;

    const { refs, floatingStyles, context } = useFloating({
        placement,
        open,
        onOpenChange: setOpen,
        whileElementsMounted: autoUpdate,
        middleware: [
            offset(8),
            flip(),
            shift({
                mainAxis: true,
                padding: 8,
            }),
        ],
    });

    // add dismiss on outside click

    const dismiss = useDismiss(context, {
        outsidePress: true,
    });

    const { getReferenceProps, getFloatingProps } = useInteractions([dismiss]);

    return (
        <div>
            <button
                ref={refs.setReference}
                {...getReferenceProps({
                    onClick: () => setOpen(!open),
                })}
                onClick={() => setOpen(!open)}
            >
                {children}
            </button>

            {open && (
                <Portal>
                    <div
                        ref={refs.setFloating}
                        {...getFloatingProps()}
                        style={{
                            ...floatingStyles,
                            pointerEvents: "auto",
                        }}
                    >
                        {/* Force lighttheme for web dynamic theming, this is troublesome */}
                        <EmojiPicker
                            className={style["emoji-picker"]}
                            theme={Theme.LIGHT}
                            onEmojiClick={onEmojiClick}
                            emojiStyle={EmojiStyle.NATIVE}
                            categoryIcons={{
                                [Categories.SUGGESTED]: <FaClockRotateLeft />,
                                [Categories.SMILEYS_PEOPLE]: <FaRegSmileWink />
,
                                [Categories.ANIMALS_NATURE]: (
                                    <FaLeaf />
                                ),
                                [Categories.FOOD_DRINK]: <FaBowlFood />,
                                [Categories.TRAVEL_PLACES]: <FaPlane />,
                                [Categories.ACTIVITIES]: <IoGameController />,
                                [Categories.OBJECTS]: <FaCubes />,
                                [Categories.SYMBOLS]: <MdEmojiSymbols />,
                                [Categories.FLAGS]: <IoFlagSharp />,
                            }}
                        />
                    </div>
                </Portal>
            )}
        </div>
    );
};

export default EPicker;
