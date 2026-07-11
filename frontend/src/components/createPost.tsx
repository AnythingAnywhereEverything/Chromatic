"use client";

import { useEffect, useRef, useState } from "react";
import { Button, Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle, DialogTrigger, DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger, Field, Icon, Separator, Textarea } from "./ui/chromaticUI";
import { PiGif } from "react-icons/pi";
import { PiImage } from "react-icons/pi";
import { PiVideoCamera } from "react-icons/pi";
import { LuUndo2 } from "react-icons/lu";
import { LuRedo2 } from "react-icons/lu";
import { CiImageOn, CiPaperplane } from "react-icons/ci";
import cs from "@styles/components/createPost.module.scss"
import Form from "next/form";
import { useUndoRedo } from "@/hooks/useUndoRedo";
import React from "react";
import { getUser } from "@/api/user";
import { ImageValue, useImageUploader } from "@/hooks/useImageUploader";
import { HiOutlineDotsHorizontal } from "react-icons/hi";
import { AutoHeightTextarea } from "./ui/custom/textarea";
import { ContainerPreview, ContainerPreview2, ImageUploader2 } from "./ui/chromatic/image-uploader2";

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

const CreatePost: React.FC = () => {
    // const user = getUser();
    return (
        <Dialog>
            <DialogTrigger asChild>
                <Button>Hello</Button>
            </DialogTrigger>

            <DialogContent className={cs["createContainer"]}>
                <DialogTitle>Create your post</DialogTitle>
                <DialogDescription>
                    <PostFrom/>
                </DialogDescription>
            </DialogContent>
        </Dialog>
    )
}

interface CreatePostContent {
    userid : string
    content : string
    status : props
    media : string[]
}
const PostFrom:React.FC = () => {
    const [text, setText] = useState('');
    const [status, setStatus] = useState<PostVisibility>(PostVisibility.Everyone);
    const [imageValue, setImageValue] = useState<ImageValue[]>([])
    const inputRef = useRef(null);
    const { value, set, undo, redo, canUndo, canRedo} = useUndoRedo('');
    const uploader = useImageUploader({
       imageValue,
       onChange: setImageValue,
       max: 10
    });
    const capText = (text: string, limit = 2500) => text.slice(0, limit);

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
        <Form action={"#"}>
            <div className={cs["userfield"]}>
                <div className={cs["container"]}>
                    <div className={cs["profile"]}>
                        <img className={cs["avatar"]} src="https://placehold.co/200" alt="" />
                    </div>
                </div>
                <Field style={{paddingTop: "calc(var(--spacing) * 1)"}}>
                    <AutoHeightTextarea 
                        className={cs["textarea"]}
                        placeholder="What's your thought ?"
                        value={text}
                        ref={inputRef}
                        onChange={(e) => setText(capText(e.target.value))}
                    />
                    <ContainerPreview2
                        images={uploader.images}
                        onDelete={uploader.removeImage}
                    />
                    <PostStatus
                    visibility={status}
                    onChange={setStatus}
                    />
                </Field>
            </div>
            <Separator/>
            <Field orientation={'horizontal'}>
                <ImageUploader2
                    uploader={uploader}
                    >
                    <button>
                        <CiImageOn />
                    </button>
                </ImageUploader2>
            </Field>
        </Form>
    )
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


export default CreatePost;