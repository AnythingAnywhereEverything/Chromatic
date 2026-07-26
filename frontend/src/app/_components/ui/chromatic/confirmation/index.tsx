import style from "./style.module.scss"
import {  Dialog, DialogContent, DialogHeading, DialogTrigger,DialogDescription, DialogClose } from "../dialogue";
import { IoInformationCircleOutline
    ,IoWarningOutline
    ,IoTrashOutline
    ,IoCheckmarkOutline
} from "react-icons/io5";
import { useState } from "react";
interface BaseAlertProps {
    title: string;
    message: string;
    type?: 'info' | 'confirm' | 'warning' | 'error' | 'destructive';
    confirmText?: string;
    cancelText?: string;
    triggerName: string;
    setDuration?: number;
}

type TypeCheckProps =
    | { typeCheck: true; checkTextValue: string }
    | { typeCheck?: false; checkTextValue?: never };

type ButtonProps =
    | { hasButton: true; onConfirm: () => void }
    | { hasButton?: false; onConfirm?: never , onCancel?: never};

type AlertDialogProps = BaseAlertProps & TypeCheckProps & ButtonProps;

const getTimerClass = (type?: AlertDialogProps["type"]) =>
    `${style["timer"]} ${style[`timer-${type}`] ?? ""}`;
const getButtonClass = (type?: AlertDialogProps["type"]) =>
    `${style[`button-${type}`] ?? ""}`;
function isMatching () {
    return 0;
}
function iconForTitle (type?: AlertDialogProps["type"]) {
    switch (type) {
        case "info":
            return <IoInformationCircleOutline/>;
        case "warning":
            return <IoWarningOutline/>;
        case "destructive":
            return <IoTrashOutline/>;
        case "confirm":
            return <IoCheckmarkOutline/>;
        default:
            break;
    }
}
const AlertDialogue:React.FC<AlertDialogProps> = ({
    title, 
    message, 
    type, 
    confirmText,
    cancelText,
    hasButton,
    onConfirm,
    triggerName,
    typeCheck,
    checkTextValue
}) => {
    
    return(
     <Dialog outsidePress={!hasButton}>
        <DialogTrigger>{triggerName}</DialogTrigger>
            <DialogContent
                className={`${style["container"]}`}
            >
            <div className={style["dialog-header"]}>
                <DialogHeading className={style["dialog-heading"]}>
                    {iconForTitle(type)} {title}
                </DialogHeading>
            </div>
            <div className={getTimerClass(type)}></div>
            <div className={style["dialog-description"]}>
                <DialogDescription className={style["description"]}>{message}</DialogDescription>
            {hasButton && (
                <>
                    {typeCheck && (
                        <div className={style["typecheck-box"]}>
                            <span>Please type "{checkTextValue}" to proceed your action</span>
                            <input type="text" className={style["input-box"]} placeholder="Insert your text here" />
                        </div>
                    )}

                    <div className={style["actions"]}>
                        <DialogClose>{cancelText ?? "Cancel"}</DialogClose>
                        <DialogClose className={getButtonClass(type)} onClick={() => 0}>{confirmText ?? "Confirm"}</DialogClose>
                    </div>
                </>
            )}
            </div>
        </DialogContent>
     </Dialog>   
    )
}

export {
    AlertDialogue,
}