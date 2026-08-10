import { Field } from "./ui/chromaticUI";
import s from "@styles/components/postbox.module.scss"
import { getUser } from "@/api/user";
import { profile } from "console";
import {BottomPost, TopPost } from "./ui/postComp";

export type PostProp = {
    postid: string;
    ownerid: string;
    ownerName: string;
    profileImg: string;
    text?: string;
    images?: string[];
    like: number;
    comments?: string[];
    repost: number;
}


const Post: React.FC<PostProp> = (
    {postid, 
    ownerid, 
    ownerName, 
    profileImg, 
    text, 
    images = [], 
    like,
    comments = [],
    repost}) => {
    // const currentuser = getUser()
    const tempCurrentUser = "1"
    const visibleImages = images.slice(0, 4);
    const count = images.length;
    const galleryClass = `gallery${Math.min(count, 4)}`;

    return (
         <article className={s.postContainer} key={postid}>
            <TopPost 
                ownerid={ownerid}
                username={ownerName}
                profile={profileImg}
                currentUser={tempCurrentUser}
            />
            <Field className={s.postMid}>
                {!!count && (
                    <div className={`${s.gallery} ${s[galleryClass]}`}>
                        {visibleImages.map((src, index) => (
                            <div
                            key={`${src}-${index}`}
                            className={s.item}
                            >
                                <img src={src} alt="" />
                                {count > 4 && index === 3 && (
                                    <div className={s.overlay}>
                                        +{count - 4}
                                    </div>
                                )}
                            </div>
                        ))}
                    </div>
                )}
                {text && <Field style={{padding: "calc(var(--spacing) * 2)"}}>
                        <p className={s.textcontent}>{text}</p>
                    </Field>}
            </Field>
            <BottomPost
            like={like}
            comments={comments}
            repost={repost}
            />
        </article>
    );
};
