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
    const [description, setDescription] = useState("");
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
                        <div className={style["community-name"]}>
                            <h3>Community Name</h3>
                            <input 
                            type="text"
                            value={name}
                            placeholder="Community name"
                            onChange={(e) => setName(e.target.value)}
                            />
                        </div>

                        <div className={style["community-name"]}>
                            <h3>Description</h3>
                            <input 
                            type="text"
                            value={description}
                            onChange={(e) => setDescription(e.target.value)}
                            placeholder="Description"
                            />
                        </div>

                        <button 
                        className={style["create-btn"]}>
                            Create your community
                        </button>
                    </section>
                </DialogDescription>
            </DialogContent>
        </Dialog>
    )
};

export {CreateCommunityBtn}