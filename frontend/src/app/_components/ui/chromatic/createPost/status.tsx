import React, { act, useState } from "react";
import { Dropdown, DropdownContent, DropdownItem, DropdownTrigger } from "../dropdown";
import style from "./style.module.scss"
import { MdArrowDropDown } from "react-icons/md";

export enum PostVisibility {
  Everyone = "EVERYONE",
  FriendsOnly = "FRIENDS_ONLY",
  NoOne = "NO_ONE"
}

type Props = {
  title?: string;
  visibility: PostVisibility;
  onChange: (value: PostVisibility) => void;
};

const VISIBILITY_MAP: Record<PostVisibility, string> = {
  [PostVisibility.Everyone]: 'Everyone',
  [PostVisibility.FriendsOnly]: 'Friends Only',
  [PostVisibility.NoOne]: 'Private',
};

const PostStatus: React.FC<Props> = ({title, visibility, onChange }) => {
  const [statusShow,setStausShow] = useState(false)
  return (
    <Dropdown 
    open={statusShow}
    onOpenChange={setStausShow}>
      <DropdownTrigger asChild
        onClick={() => setStausShow(!statusShow)}
      >
        <div
        className={style["status-trigger"]}
        style={{width: "fit-content", display: "flex"}}
        >
          {title !== "" && title ? title : ""}
          {VISIBILITY_MAP[visibility]}
          <div style={{alignItems: "center"}}>
           <MdArrowDropDown
              style={{ 
                rotate: statusShow ? "180deg" : "0deg", 
                transition: "rotate 0.2s ease-in-out" // Makes it spin smoothly
              }} 
            />
          </div>
        </div>

      </DropdownTrigger>

      <DropdownContent 
      className={"status-body"}
      >
        {(Object.keys(VISIBILITY_MAP) as PostVisibility[]).map((key) => (
          <DropdownItem
          className={style["status-item"]}
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
