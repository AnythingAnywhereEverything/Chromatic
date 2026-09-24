import { TagRow } from "@/api/tags/tags";
import {
    Dropdown,
    DropdownTrigger,
    DropdownContent,
    DropdownItem,
} from "../dropdown";
import style from "./tags.module.scss";
interface CreatePostTagsProps {
    tags: TagRow[];
    selectedTag: TagRow | null;
    onChangeTag: (tag: TagRow) => void;
}

function PostTags({
    tags,
    selectedTag,
    onChangeTag,
}: CreatePostTagsProps) {
    return (
        <Dropdown>
            <DropdownTrigger>
                {selectedTag ? `#${selectedTag.tag_name}` : "Select a tag"}
            </DropdownTrigger>
            <DropdownContent className={style["dropdown-content"]}>
                {tags.map((tag) => (
                    <button
                        type="button"
                        key={tag.id}
                        className={`${style["post-tag"]} ${selectedTag?.id === tag.id ? style["selected"] : ""}`}
                        onClick={() => onChangeTag(tag)}
                    >
                        #{tag.tag_name}
                    </button>
                ))}
            </DropdownContent>
        </Dropdown>
    );
}

export default PostTags;
