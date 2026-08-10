import { useEffect, useRef, useState } from "react";
import style from "./style.module.scss"
import { LuThumbsUp } from "react-icons/lu";
import { GoComment } from "react-icons/go";
import { IoMdShare } from "react-icons/io";
import { IoBookmarkOutline } from "react-icons/io5";
interface PostProps {
    id: string;
    ownerId: string;
    ownerName: string;
    content:string
    attachment?: string[]
    like: number;
    comment: string[];
    bookmark: boolean
}

interface commenter {
    userId: string;
    username: string
    like: string
    comment: string[]
}
const Post:React.FC<PostProps> = ({
    id,
    ownerId,
    ownerName,
    content,
    like,
    comment,
    attachment}) => {
    
    const [open,setOpen] = useState(false);
    const [showReadMoreButton, setShowReadMoreButton] = useState(false)
    const ref = useRef<HTMLSpanElement | null>(null);

    useEffect(() => {
        if (ref.current) {
            console.log(ref.current.scrollHeight, ref.current.clientHeight)
            setShowReadMoreButton(
                ref.current.scrollHeight !== ref.current.clientHeight
            )
        }
    },[])
    return (
        <section className={style["container"]}
        key={id}>
            <div className={style["header"]}>
                <section className={style["profile"]}>
                    <div className={style["avatar"]}>
                        <img src="https://placehold.co/400" alt="" />
                    </div>
                    <div className={style["username"]}>
                        <p>{
                            ownerName ?? ownerName ? ownerName: "Username"
                        }</p>
                    </div>
                </section>
                <div className={style["option"]}>
                    ...
                </div>
            </div>

            <div className={style["main-container"]}>
                <div className={style["text-container"]}>
                    <span 
                    className={`style["content"] ${!open ? style["is-collapsed"]: ""}`} 
                    ref={ref}
                    >
                        {content}
                    </span>
                    <div>

                    {showReadMoreButton && (
                        <button
                        type="button"
                        onClick={() => setOpen(!open)}
                        className={style["read-more-btn"]}
                        >
                            {open ? "Show less" : "Read more"}
                        </button>
                    )}
                    </div>
                </div>

                <div className={style["subject-tag"]}>
                    <div>

                    </div>
                </div>                
            </div>

            <div className={style["bottom-container"]}>
                <section className={style["interaction"]}>
                    <div>
                        <button type="button">
                            <LuThumbsUp/>
                        </button>
                        {like}
                    </div>
                    <div>
                        <GoComment/>
                        {comment?.length || 0}
                    </div>
                </section>

                <section className={style["interaction"]}>
                    <button type="button">
                        <IoMdShare/>
                    </button>
                    <button type="button">
                        <IoBookmarkOutline/>
                    </button>
                </section>
            </div>
        </section>
    )
}

export {Post}