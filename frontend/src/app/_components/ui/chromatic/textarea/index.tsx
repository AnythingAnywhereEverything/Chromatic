import React from "react";
import { FaQuoteLeft, FaQuoteRight } from "react-icons/fa6";
import style from "./textarea.module.scss";
interface EditableTextAreaProps {
    value: string;
    maxChars: number;
    placeholder: string;
    isOwner?: boolean;
    onChange?: (value: string) => void;
    minHeight?: string;
    className?: string;
    showQuoteIcons?: boolean;
}

const EditableTextArea = ({
    value,
    maxChars,
    placeholder,
    isOwner,
    onChange,
    minHeight,
    className,
    showQuoteIcons = false,
}: EditableTextAreaProps) => {
    const [isEditing, setIsEditing] = React.useState(false);
    const [newValue, setNewValue] = React.useState(value);

    const textareaRef = React.useRef<HTMLTextAreaElement | null>(null);

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
        onChange?.(trimmedValue);
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
            {showQuoteIcons && 
            <div className={style["quote-icon-left"]}>
                <FaQuoteLeft />
            </div>
            }
            <div className={style["textarea-wrapper"]}>
                <textarea
                    ref={textareaRef}
                    autoComplete="off"
                    autoFocus
                    value={newValue}
                    onChange={handleInputChange}
                    onBlur={handleInputBlur}
                    maxLength={maxChars}
                    placeholder={placeholder}
                    className={`${style["textarea-input"]} ${className}`}
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
        <div className={style["quote-icon-right"]}>
                {showQuoteIcons && <FaQuoteRight />}
            </div>
        </>
    ) : (
        <div className={style["textarea-container"]}>
            {showQuoteIcons && 
            <div className={style["quote-icon-left"]}>
                <FaQuoteLeft />
            </div>
            }
                <span
                    className={ `${style["editable-textarea"]} ${className}` }
                    onClick={handleEditClick}
                >
                    {value || placeholder}
                </span>
            <div className={style["quote-icon-right"]}>
                {showQuoteIcons && <FaQuoteRight />}
            </div>
        </div>
    );
};

// ! main problem is you need to create container and wrapper by yourself.


export default EditableTextArea;