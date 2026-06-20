import { useState } from "react";
import { Button, DropdownMenu, DropdownMenuContent, DropdownMenuTrigger, Field, Icon, Textarea } from "./ui/chormaticUI";
import { PiGif } from "react-icons/pi";
import { PiImage } from "react-icons/pi";
import { PiVideoCamera } from "react-icons/pi";
import { LuUndo2 } from "react-icons/lu";
import { LuRedo2 } from "react-icons/lu";
import { CiPaperplane } from "react-icons/ci";
import s from "@styles/components/postbox.module.scss";

const CreatePost: React.FC = () => {
    const [text,setText] = useState('');


    return (
        <Field className={s.createPostContainer}>
            <Field className={s.text}>
                <Textarea placeholder="This is text box">


                </Textarea>
            </Field>
            <Field orientation={'horizontal'}>
                <Field>
                    <div>
                        <DropdownMenu>
                            <DropdownMenuTrigger>Everyone can see</DropdownMenuTrigger>
                            <DropdownMenuContent>Only you can see</DropdownMenuContent>
                        </DropdownMenu>
                    </div>

                </Field>
                <div style={{display: "flex"}}>
                    <Button><LuUndo2/></Button>
                    <Button><LuRedo2/></Button>
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
        </Field>
    ); 
}

const imageHolder: React.FC = (image) => {
    return (
    <>
    </>
    );
}

export default CreatePost;