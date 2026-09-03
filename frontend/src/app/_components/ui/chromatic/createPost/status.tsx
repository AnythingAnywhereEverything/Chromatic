import React, { useState } from "react";
import {
    Dropdown,
    DropdownContent,
    DropdownItem,
    DropdownTrigger,
} from "../dropdown";
import style from "./style.module.scss";
import { MdArrowDropDown } from "react-icons/md";
import { IoIosArrowDown } from "react-icons/io";

export enum Visibility {
    Everyone = "everyone",
    Friend = "friend",
    Private = "private",
}
type Props = {
    title?: string; //? what is title in this
    visibility: Visibility;
    onChange: (value: Visibility) => void;
};

const VISIBILITY_MAP: Record<Visibility, string> = {
    [Visibility.Everyone]: "Everyone",
    [Visibility.Friend]: "Friends",
    [Visibility.Private]: "Private",
};

const PostStatus: React.FC<Props> = ({ visibility, onChange }) => {
    const [statusShow, setStausShow] = useState(false);
    return (
        <Dropdown open={statusShow} onOpenChange={setStausShow}>
            <DropdownTrigger asChild>
                <button
                    className={style["visibility-trigger"]}
                    onClick={() => setStausShow(!statusShow)}
                >
                    <span>
                        {VISIBILITY_MAP[visibility]} <IoIosArrowDown />
                    </span>
                </button>
            </DropdownTrigger>

            <DropdownContent className={style["visibility-body"]}>
                {(Object.keys(VISIBILITY_MAP) as Visibility[]).map((key) => (
                    <DropdownItem
                        className={`${style["visibility-item"]} ${visibility === key ? style["active"] : ""}`}
                        key={key}
                        onSelect={() => onChange(key)}
                    >
                        {VISIBILITY_MAP[key]}
                    </DropdownItem>
                ))}
            </DropdownContent>
        </Dropdown>
    );
};

export { PostStatus };
