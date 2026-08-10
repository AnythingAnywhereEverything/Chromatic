"use client"

import { useRef, useState } from "react";
import { Dialog, DialogClose, DialogContent, DialogTrigger } from "../dialogue";
import { ImageValue, useImageUploader } from "@/hooks/useImageUploader";
import { Form } from "@base-ui/react";
import style from "./style.module.scss"
import { AutoHeightTextarea } from "@components/ui/custom/textarea";
import { ContainerPreview2, ImageUploader2 } from "@components/ui/chromatic/image-uploader2";
import { PostStatus, PostVisibility } from "./status";
import { CiImageOn } from "react-icons/ci";
import { Icon } from "@components/ui/chromaticUI";
import { MdOutlineEmojiEmotions } from "react-icons/md";
interface CreatePostProps {
    ownerId: string;
    ownerName: string;
    ownerPfp: string;
}

const CreatePost:React.FC<CreatePostProps> = (
    {ownerId,
    ownerName,
    ownerPfp,
}) => {
    if (!ownerId) {return 0;}

    const [text ,setText] = useState("");
    const [imageValue, setImageValue] = useState<ImageValue[]>([])
    const inputRef = useRef(null);
    const uploader = useImageUploader({
        imageValue,
        onChange: setImageValue,
        max: 10
    })
    const capText = (text: string, limit = 2500) => text.slice(0, limit);
    const [status, setStatus] = useState<PostVisibility>(PostVisibility.Everyone);

    return(
        <Dialog outsidePress = {false}>
            <DialogTrigger 
            className={style["active-button"]}
            >
                Yesh
            </DialogTrigger>

            <DialogContent className={style["container"]}>
                <Form action={"#"}>
                    <section className={style["top"]}>
                         <div className={style["create-title"]}>
                             <div className={style["title"]}>
                                 <h1>
                                 +
                                 </h1>
                                 <h1>Create Post</h1>
                             </div>

                             <DialogClose>X</DialogClose>
                         </div>

                         <div>
                             <div className={style["profile"]}>
                                 <div className={style["user"]}>
                                     <div className={style["avatar"]}>
                                         <img src="https://placehold.co/400" alt="" />
                                     </div>
                                     <p>Username</p>

                                     <div>
                                         <PostStatus
                                             visibility={status}
                                             onChange={(value) => setStatus(value)}
                                             />
                                     </div>
                                 </div>
                             </div>
                         </div>
                    </section>

                    <section className={style["mid"]}>
                        <div>
                            <AutoHeightTextarea
                                className={style["textarea"]}
                                value={text}
                                ref={inputRef}
                                onChange={(e) => setText(capText(e.target.value))}
                                placeholder="Insert you texts here."
                            />
                        </div>

                        <div className={style["image-container"]}>
                            <h4>Images</h4>
                            <div className={style["images"]}>

                            </div>
                        </div>

                        <div className={style["video-container"]}>
                            <h4>Video</h4>
                            <div className={style["videos"]}>

                            </div>
                        </div>

                        {/* //todo: tags selection */}
                        <div className={style["tag-container"]}>
                            <h4>Subject tag</h4>
                            <div className={style["tag"]}>

                            </div>
                        </div>
                        
                    </section>

                    <section className={style["bottom"]}>
                        <div className={style["icon-container"]}>
                            <div style={{height: "24px", width: "24px"}}>
                                <CiImageOn style={{height: "100%", width: "100%"}}/>
                                <input 
                                type="file" 
                                hidden
                                />
                            </div>

                            <div style={{height: "24px", width: "24px"}}>
                                <MdOutlineEmojiEmotions  style={{height: "100%", width: "100%"}}/>
                            </div>
                        </div>

                        <div className={style["button-container"]}>
                            <div>
                                <button
                                className={style["cancel"]}
                                type="button">
                                    Cancel
                                </button>
                            </div>

                            <div>
                                <button 
                                className={style["submit"]} 
                                type="button">
                                    Submit
                                </button>
                            </div>
                        </div>
                    </section>
                </Form>
            </DialogContent>
        </Dialog>
    )
}

export {CreatePost}