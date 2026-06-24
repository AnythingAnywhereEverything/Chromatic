import { Field } from "@components/ui/chormaticUI"
import s from "@styles/layouts/defaultlayout.module.scss"
const RightLayout:React.FC = () => {
    return (
        <Field style={{alignItems: 'stretch', height: '100%'}}>
            <Field className={s.rightLayout}>
                Hello world
            </Field>
            <footer className={s.footer}>Standard Footer</footer>
        </Field>
    )
}

export default RightLayout;