"use client";

import style from "./comment.module.scss";
import React, { useState } from "react";
import UserComment from "@/app/_components/ui/chromatic/userComment/userComment";
import CreateComment from "@/app/_components/ui/chromatic/userComment/createComment";
import { commentProps, getCommentsOnPost } from "@/api/post/comments";
import { UserResponse } from "@/api/user";
import { useUser } from "@/hooks/useUser";
type CommentSectionProps = {
    postId: string;
};

export const CommentSection = ({ postId }: CommentSectionProps) => {
    const [comments, setComments] = useState<commentProps[]>([]);
    const [user, setUser] = useState<UserResponse | null>(null);
    const currentUser = useUser();

    React.useEffect(() => {
        if (currentUser?.data) {
            setUser(currentUser.data);
        }
    }, [currentUser]);

    React.useEffect(() => {
        async function getComments() {
            const res = await getCommentsOnPost(postId);
            console.log("This is all comments", res);
            setComments(res);
        }
        getComments();
    }, [postId]);

    return (
        <div className={style["comment-layout"]}>
            {user && <CreateComment postId={postId} author={user} />}

            <div className={style["comments-container"]}>
                {comments.map((comment) => (
                    <UserComment key={comment.id} {...comment} />
                ))}
            </div>
        </div>
    );
};

export default CommentSection;
