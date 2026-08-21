import { useState } from "react";
import style from "./displayname.module.scss";

interface DisplayNameProps {
    displayName: string;
    isOwner?: boolean;
    onChange?: (newDisplayName: string) => void;
}

const DisplayName = ({ displayName, isOwner, onChange }: DisplayNameProps) => {
    const [isEditing, setIsEditing] = useState(false);
    const [newDisplayName, setNewDisplayName] = useState(displayName);

    const handleEditClick = () => {
        setIsEditing(true);
    };

    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {        
        if (e.target.value.length <= 32) {
            setNewDisplayName(e.target.value);
        }
    };

    const handleInputBlur = () => {
        setNewDisplayName(newDisplayName.trim());
        console.log("DisplayName: onChange called with", newDisplayName.trim());
        if (onChange) {
            onChange(newDisplayName);
        }
        setIsEditing(false);
    };

    return !isOwner ? (
        <div className={style["display-name"]}>
            <span className={style["display-name-text"]}>{displayName}</span>
        </div>
    ) : (
        <div className={style["display-name"]}>
            {isEditing ? (
                <div className={style["display-name-input-wrapper"]}>
                    <input
                        id="display-name-input"
                        type="text"
                        autoComplete="off"
                        value={newDisplayName}
                        onChange={handleInputChange}
                        onBlur={handleInputBlur}
                        onKeyDown={(k) => {
                            if (k.key === "Enter") {
                                handleInputBlur();
                            }
                        }}
                        autoFocus
                        className={style["display-name-input"]}
                    />
                </div>
            ) : (
                <span
                    className={style["editable-text"]}
                    onClick={handleEditClick}
                >
                    {displayName}
                </span>
            )}
        </div>
    );
};

export default DisplayName;
