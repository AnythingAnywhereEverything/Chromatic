import { LayoutProps } from "@/types/global";
import { Navbar } from "../navbarWraper";
import { Field } from "@components/ui/chromaticUI";
import s from "@styles/layouts/defaultlayout.module.scss"
import RightLayout from "./rightLayout";

const DefaultLayout:React.FC<LayoutProps> = ({children}) => {
    return (
        <Field orientation={'horizontal'} style={{alignItems: "stretch"}} className={s.layout}>
          <Navbar />
            <div className={s.container}>
              <div className={s.content}>
                <main className={s.main}>
                  {children}
                  </main>
              </div>
                  <RightLayout/>
          </div>
        </Field>
    );
};

export default DefaultLayout;