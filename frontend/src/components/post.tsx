import { Field } from "./ui/chormaticUI";
import s from "@styles/components/postbox.module.scss"
type PostProp = {
    text?: string,
    images?: string[]
}
const Post: React.FC<PostProp> = (
    text,
    images = []
    ) => {

    const visibleImages = images.slice(0, 4);
    const count = images.length;
    return (
        <Field className={s.PostContainer}>
            {/* Profiles and action button */}
            <Field orientation={'horizontal'}>
                
            </Field>

            
        </Field>
    );
};

export default Post;