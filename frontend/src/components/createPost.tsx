import { useEffect, useRef, useState } from "react";
import { Button, DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger, Field, Icon, Textarea } from "./ui/chromaticUI";
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

interface media_post {
    userid: string,
    content: string,
    
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

    const handleAddFiles = (files: File[]) => {
        setImgPrepare(prev => [
            ...prev,
            ...files.map((file, i) => ({
                image_url: "",
                position: prev.length + i,
                file,
            } as ImgPrepared))
        ])
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
                <ImageUploader
                images={images}
                onChange={handleAddFiles}
                />
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

type ImageUploaderProps = {
    images: (string | File)[]
    onChange: (files: File[]) => void
    max? : number
}

const ImageUploader: React.FC<ImageUploaderProps> = ({ 
    images, 
    onChange, 
    max = 5 }) => {
    const inputRef = useRef<HTMLInputElement>(null)

    // * revoke object urls on unmount to avoid memory leaks
    const previews = images.map(img =>
        typeof img === 'string' ? img : URL.createObjectURL(img)
    )

    useEffect(() => {
        return () => {
            previews.forEach((url, i) => {
                if (typeof images[i] !== 'string') URL.revokeObjectURL(url)
            })
        }
    }, [images])

    const handleFiles = (e: React.ChangeEvent<HTMLInputElement>) => {
        const files = Array.from(e.target.files ?? [])
        const remaining = max - images.length
        // * silently drop extras past the limit, adjust if you want a warning instead
        if (files.length && remaining > 0) onChange(files.slice(0, remaining))
        e.target.value = ''
    }

    return (
        <Field className={s.imageContainer}>
            <input
                ref={inputRef}
                type="file"
                accept="image/*"
                multiple
                hidden
                onChange={handleFiles}
                disabled={images.length >= max}
            />
            <button type="button" onClick={() => inputRef.current?.click()} disabled={images.length >= max}>
                Add image ({images.length}/{max})
            </button>
            {previews.map((url, i) => (
                <img key={i} src={url} alt="" />
            ))}
        </Field>
    )
}

export default React.memo(CreatePost);