import React, { act, useState } from "react";
import {
  Dropdown,
  DropdownContent,
  DropdownItem,
  DropdownTrigger,
} from "../dropdown";
import style from "./style.module.scss";
import { MdArrowDropDown } from "react-icons/md";

export enum Visibility {
  Everyone = "everyone",
  Friend = "friend",
  Private = "private",
}
type Props = {
  title?: string;
  visibility: Visibility;
  onChange: (value: Visibility) => void;
};

const VISIBILITY_MAP: Record<Visibility, string> = {
  [Visibility.Everyone]: "Everyone",
  [Visibility.Friend]: "Friends",
  [Visibility.Private]: "Private",
};

const PostStatus: React.FC<Props> = ({ title, visibility, onChange }) => {
  const [statusShow, setStausShow] = useState(false);
  return (
    <Dropdown open={statusShow} onOpenChange={setStausShow}>
      <DropdownTrigger asChild>
        <div className={style["border"]}>
          <div
            className={style["status-trigger"]}
            onClick={() => setStausShow(!statusShow)}
          >
            {title !== "" && title ? title : ""}
            {VISIBILITY_MAP[visibility]}
          </div>
        </div>
      </DropdownTrigger>

      <DropdownContent className={"status-body"}>
        {(Object.keys(VISIBILITY_MAP) as Visibility[]).map((key) => (
          <DropdownItem
            className={style["status-item"]}
            key={key}
            onSelect={() => onChange(key)}
          >
            <input
              type="checkbox"
              onChange={() => onChange(key)}
              checked={visibility === key}
            />
            {VISIBILITY_MAP[key]}
          </DropdownItem>
        ))}
      </DropdownContent>
    </Dropdown>
  );
};

export { PostStatus };
