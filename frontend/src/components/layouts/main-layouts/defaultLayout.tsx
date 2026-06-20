import { LayoutProps } from "@/types/global";
import { Navbar } from "../navbarWraper";
import { Field } from "@components/ui/chormaticUI";
import s from "@styles/layouts/defaultlayout.module.scss"

const DefaultLayout:React.FC<LayoutProps> = ({children}) => {
    return (
        <Field orientation={'horizontal'} style={{alignItems: "stretch"}} className={s.layout}>
          <Navbar />
          <Field className={s.content}>
            <main className={s.main}>{children}</main>
            <footer className={s.footer}>Standard Footer</footer>
          </Field>
        </Field>
    );
};

export default DefaultLayout;