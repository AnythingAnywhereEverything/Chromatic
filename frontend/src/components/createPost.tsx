import { useEffect, useRef, useState } from "react";
import { Button, DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger, Field, Icon, Textarea } from "./ui/chormaticUI";
import { PiGif } from "react-icons/pi";
import { PiImage } from "react-icons/pi";
import { PiVideoCamera } from "react-icons/pi";
import { LuUndo2 } from "react-icons/lu";
import { LuRedo2 } from "react-icons/lu";
import { CiPaperplane } from "react-icons/ci";
import s from "@styles/components/postbox.module.scss";
import Form from "next/form";
import { useUndoRedo } from "@/hooks/useUndoRedo";
import React from "react";
import ImageUploader from "./ui/chormatic/image-uploader";

enum PostVisibility{
    Everyone = 1 << 0,
    FriendsOnly = 1 << 1,
    TaggedPeople = 1 << 2,
    NoOne = 1 << 3
}

interface PostImage {
    postId: string,
    image_url: string,
    position: number
}

type ImgPrepared = PostImage & { file: File | null }

const CreatePost: React.FC = () => {
    const { value, set, undo, redo, canUndo, canRedo} = useUndoRedo('');

    const [status, setStatus] = useState(PostVisibility.Everyone);
    const inputRef = useRef(null);
    const [imgPrepare, setImgPrepare] = useState<ImgPrepared[]>([])
    const images = [...imgPrepare]
    .sort((a, b) => a.position - b.position)
    .map(i => i.image_url !== "" ? i.image_url : (i.file ?? ""));

    const handleImageChange = (vals: (File | string)[]) => {
        setImgPrepare(prev => {
            return vals.map((v, index) => {
                if (typeof v === "string") {
                    const oldImg = prev.find(i => i.image_url === v);

                    if (oldImg) {
                        return {
                            ...oldImg,
                            position: index
                        };
                    }

                    return {
                        postId: "",
                        image_url: v,
                        position: index,
                        file: null
                    } as ImgPrepared;
                }

                return {
                    postId: "",
                    image_url: "",
                    position: index,
                    file: v
                } as ImgPrepared;
            });
        });
    };

    useEffect(() => {
        const handleKeyDown = (e: KeyboardEvent) => {
            if (inputRef.current && inputRef.current === document.activeElement) {
                if (e.ctrlKey && e.key.toLowerCase() === "z") {
                    e.preventDefault();
                    undo();
                }
                if (e.ctrlKey && e.key.toLowerCase() === "y") {
                    e.preventDefault();
                    redo();
                }
            }
        };

        window.addEventListener("keydown", handleKeyDown);
        return () => window.removeEventListener("keydown", handleKeyDown);
    }, [undo, redo, inputRef]);
    return (
        <Field className={s.createPostContainer}>
            <Form action={'#'}>
                <Field className={s.text}>
                    <Textarea 
                    placeholder="This is text box"
                    ref={inputRef}
                    value={value}
                    onChange={(e) => set(e.target.value)}
                    />
                </Field>
                <Field orientation={'horizontal'}>
                    <Field>
                        <div>
                            <PostStatus
                            visibility={status}
                            onChange={setStatus}
                            />
                        </div>

                    </Field>
                    <div style={{ display: "flex" }}>
                        <Button onClick={undo} disabled={!canUndo}><LuUndo2 /></Button>
                        <Button onClick={redo} disabled={!canRedo}><LuRedo2 /></Button>
                    </div>
                </Field>
                <Field orientation={'horizontal'}>
                    <Field orientation={'horizontal'}>
                        <PiImage/>
                        <ImageUploader
                        id="add-image"
                        min={0}
                        accept="image/jpeg, image/png"
                        value={images}
                            />
                        <PiGif/>
                        <PiVideoCamera/>
                    </Field>
                    <CiPaperplane/>
                </Field>
            </Form>
        </Field>
    ); 
}

type props = {
    visibility: PostVisibility;
    onChange: (value: PostVisibility) => void;
};

const PostStatus: React.FC<props> = ({visibility, onChange}) =>{
    const options = [
        PostVisibility.Everyone,
        PostVisibility.FriendsOnly,
        PostVisibility.TaggedPeople,
        PostVisibility.NoOne,
    ];

    function getStatusName(status: PostVisibility): string {
    switch (status) {
        case PostVisibility.Everyone:
            return "Everyone";
        case PostVisibility.FriendsOnly:
            return "Friends Only";
        case PostVisibility.TaggedPeople:
            return "People You Tag";
        case PostVisibility.NoOne:
            return "No One";
        default:
            return "Unknown";
    }
}
    return(
        <Field orientation={'horizontal'}>
            <p>Who can see this post :</p>

            <DropdownMenu>
                <DropdownMenuTrigger >
                        {getStatusName(visibility)}
                </DropdownMenuTrigger>

                <DropdownMenuContent>
                    {options.map(option => (
                        <DropdownMenuItem
                            key={option}
                            onClick={() => onChange(option)}
                        >
                            {getStatusName(option)}
                        </DropdownMenuItem>
                    ))}
                </DropdownMenuContent>
            </DropdownMenu>
        </Field>
    )
}

const imageHolder: React.FC = (image) => {

    return (
    <Field className={s.iamgeContainer}>
        
    </Field>
    );
}

export default React.memo(CreatePost);