import { useState } from "react";
import { Dialog, DialogContent, DialogDescription, DialogHeading, DialogTrigger } from "../dialogue";
import style from "./style.module.scss";

type TagsProps = {
    art: "Art";
    english: "English";
    science: "Science";
    
}
const CreateCommunityBtn:React.FC = () => {
    const [name, setName] = useState("");
    const [tag, setTag] = useState<TagsProps[]>([]);
    return (
        <Dialog >
            <DialogTrigger asChild>
                <button 
                type="button">
                    Create Community
                </button>
            </DialogTrigger>

            <DialogContent className={style["container"]}>
                <DialogHeading>Create Community</DialogHeading>
                <DialogDescription>

                    <section className={style["tag-container"]}>
                        <span>
                            audience tags
                        </span>
                        <div className={style["tag-selection"]}>

                        </div>
                    </section>
                </DialogDescription>
            </DialogContent>
        </Dialog>
    )
};

export {CreateCommunityBtn}