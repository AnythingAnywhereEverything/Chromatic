import Link from "next/link";
import { Button,
     Dialog,
      DialogContent,
       DialogDescription, 
       DialogHeader, 
       DialogTitle, 
       DialogTrigger, 
       DropdownMenu,
    DropdownMenuContent, 
    DropdownMenuItem, 
    DropdownMenuTrigger, 
    Field, 
    Label } from "./chormaticUI";
import { HiOutlineDotsHorizontal } from "react-icons/hi";
import { FiThumbsUp } from "react-icons/fi";
import { RxLoop } from "react-icons/rx";
import { BsChat } from "react-icons/bs";
import { FaRegBookmark } from "react-icons/fa";
import { PiShareFat } from "react-icons/pi";
import s from "@styles/components/postbox.module.scss"
import { useEffect, useState } from "react";
import dialog from "@styles/ui/Chormatic/dialogue.module.scss"
import Form from "next/form";

const CustomOverlay = ({ state }: { state: boolean }) => (
    <div
        data-state={state ? "open" : "close"}
        className={dialog.overlay}
    />
);
// * Post Header

type TopPostProps = {
    ownerid : string;
    profile: string;
    username : string;
    currentUser:string;
};

const TopPost: React.FC<TopPostProps> = ({ownerid, profile, username, currentUser}) => {
    // * Temp, I'm lazy..
    const tempCurrentUser = "1"

    return (
        <Field orientation={'horizontal'}>
            <div className={s.profile}>
                <Link  href ='#'>
                    <img src="https://placehold.co/50" alt="" />
                </Link>
            </div>
            <Field>
                <div>
                    {/* id */}
                    <Link style={{textDecoration: 'none', color: 'var(--text-neutral)'}} href='#'>Username</Link>
                </div>
            </Field>
            <div>
                <DropdownMenu>
                    <DropdownMenuTrigger asChild>
                        <HiOutlineDotsHorizontal/>
                    </DropdownMenuTrigger>


                    {ownerid != tempCurrentUser ? (
                        <DropdownMenuContent>
                            <DropdownMenuItem>
                                <Button>Not interest</Button>
                            </DropdownMenuItem>
                            <DropdownMenuItem>
                                <Button>Hide this user for 30 days</Button>
                            </DropdownMenuItem>
                            <DropdownMenuItem>
                                <Button>Report</Button>
                            </DropdownMenuItem>
                        </DropdownMenuContent>
                    ) : (
                        <DropdownMenuContent>
                            <DropdownMenuItem asChild>
                                <EditPostDialog/>
                            </DropdownMenuItem>
                            <DropdownMenuItem>
                                <Button>Delete post</Button>
                            </DropdownMenuItem>
                        </DropdownMenuContent>
                    )}
                </DropdownMenu>
            </div>
        </Field>
    )
}

type BottomProp = {
    like :number;
    comments? : string[];
    repost : number;
}
const BottomPost: React.FC<BottomProp> = ({like, comments, repost}) => {
    const formatNumber = (num: number): string => {
        return new Intl.NumberFormat('en-US', {
            notation: 'compact',
            compactDisplay: 'short',
            maximumFractionDigits: 1,
        }).format(num);
    };
    
    const commentCount = formatNumber(comments?.length ?? 0);
    
    return (
        <Field orientation={'horizontal'}>
            <Field orientation={'horizontal'}>
                <div>
                    <Field orientation={'horizontal'}>
                        <Button id="like">
                            <FiThumbsUp/>
                        </Button>
                        <p>{formatNumber(like)}</p>
                    </Field>
                </div>
                <div>
                    <Field orientation={'horizontal'}>
                        <Button id="repost">
                            <RxLoop/>
                        </Button>
                        <p>{formatNumber(repost)}</p>
                    </Field>
                </div>
                <div>
                    <Field orientation={'horizontal'}>
                        <Button id="comment">
                            <BsChat/>
                        </Button>
                        {commentCount}
                    </Field>
                </div>
            </Field>

            <div>
                <Field orientation={'horizontal'}>
                    <Button>
                        <FaRegBookmark/>
                    </Button>

                    <Button>
                        <PiShareFat/>
                    </Button>
                </Field>
            </div>
        </Field>
    )
}


type editProps = {
    id: string;
    text: string;
    image : string[];
    status : number;
}

const EditPostDialog: React.FC = () =>{
    const [open, setOpen] = useState(false);

    return (
        <Dialog
            open={open}
            onOpenChange={setOpen}
            modal={true}
        >
            <DialogTrigger asChild>
                <Button variant={'default'}>
                    Edit Post
                </Button>
            </DialogTrigger>

            {open && <CustomOverlay state={open} />}

            <DialogContent>
                <DialogHeader>
                    <DialogTitle>Edit your Post</DialogTitle>
                </DialogHeader>

                <Form action={'#'}>
                    <Field>
                        </Field>                    
                </Form>
            </DialogContent>
        </Dialog>
    )
}

export  {
    TopPost,
    BottomPost
}