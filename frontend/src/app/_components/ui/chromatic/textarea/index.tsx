import React from "react";
import { FaQuoteLeft, FaQuoteRight } from "react-icons/fa6";
import style from "./textarea.module.scss";
type EditableTextAreaProps = React.TextareaHTMLAttributes<HTMLTextAreaElement> & {
    value: string;
    maxChars: number;
    isOwner?: boolean;
    onUpdateChange?: (value: string) => void;
    minHeight?: string;
    showQuoteIcons?: boolean;
    overwriteClassname?: string;
}

const EditableTextArea = ({
    value,
    maxChars,
    isOwner,
    onUpdateChange,
    minHeight,
    className,
    overwriteClassname,
    showQuoteIcons = false,
    ...props
}: EditableTextAreaProps) => {
    const [isEditing, setIsEditing] = React.useState(false);
    const [newValue, setNewValue] = React.useState(value);

    const textareaRef = React.useRef<HTMLTextAreaElement | null>(null);

    const wrapperClassName =
        overwriteClassname ?? style["textarea-wrapper"];
    
    const inputClassName =
        overwriteClassname ?? `${style["textarea-input"]} ${className ?? ""}`;

    const adjustHeight = () => {
        const textarea = textareaRef.current;

        if (textarea) {
            textarea.style.height = "auto";
            textarea.style.height = `${textarea.scrollHeight}px`;
        }
    };

    const handleEditClick = () => {
        setNewValue(value);
        setIsEditing(true);
    };

    const handleInputChange = (
        e: React.ChangeEvent<HTMLTextAreaElement>
    ) => {
        if (e.target.value.length <= maxChars) {
            setNewValue(e.target.value);
        }
    };

    const handleInputBlur = () => {
        const trimmedValue = newValue.trim();

        setNewValue(trimmedValue);
        onUpdateChange?.(trimmedValue);
        setIsEditing(false);
    };

    React.useEffect(() => {
        if (isEditing) {
            adjustHeight();
        }
    }, [isEditing, newValue]);

    if (!isOwner) {
        return (
            <span className={className}>
                {showQuoteIcons && <FaQuoteLeft />}
                {value}
                {showQuoteIcons && <FaQuoteRight />}
            </span>
        );
    }

    return isEditing ? (
        <>
            {showQuoteIcons && (
                <div className={style["quote-icon-left"]}>
                    <FaQuoteLeft />
                </div>
            )}

            <div className={wrapperClassName}>
                <textarea
                    {...props}
                    ref={textareaRef}
                    autoComplete="off"
                    value={newValue}
                    autoFocus
                    onChange={handleInputChange}
                    onBlur={handleInputBlur}
                    maxLength={maxChars}
                    className={inputClassName}
                    onKeyDown={(e) => {
                        if (e.key === "Enter") {
                            handleInputBlur();
                        }
                    }}
                    style={{
                        resize: "none",
                        overflowY: "hidden",
                        minHeight,
                        width: "100%",
                        fontSize: "var(--text-base)",
                        outline: "none",
                    }}
                />
            </div>

            {showQuoteIcons && (
                <div className={style["quote-icon-right"]}>
                    <FaQuoteRight />
                </div>
            )}
        </>
    ) : (
        <div className={style["textarea-container"]}>
            {showQuoteIcons && (
                <div className={style["quote-icon-left"]}>
                    <FaQuoteLeft />
                </div>
            )}

            <span
                className={
                    overwriteClassname
                        ? overwriteClassname
                        : `${style["editable-textarea"]} ${className ?? ""}`
                }
                onClick={handleEditClick}
            >
                {value || props.placeholder}
            </span>

            {showQuoteIcons && (
                <div className={style["quote-icon-right"]}>
                    <FaQuoteRight />
                </div>
            )}
        </div>
    );
};

// ! main problem is you need to create container and wrapper by yourself.


export default EditableTextArea;