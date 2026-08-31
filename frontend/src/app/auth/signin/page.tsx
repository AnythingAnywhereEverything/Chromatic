import { Image } from "@/app/_components/ui/chromatic/Image";
import SignInForm from "./form";
import style from "@styles/layouts/authLayout.module.scss";

export const metadata = {
  title: "Sign In",
  description: "Sign in to your account",
};

const SignIn = () => {
  return (
    <main className={style["auth-container"]}>
      <div className={style["container-left"]}>
        <div className={style["down-fall"]}>
            <div className={style["slogan"]}>
              <h2>Some slogan for social media learning style text thingy</h2>
            </div>
            <img
              className={style["prop"]}
              src={"/asset/image.png"}
            />
        </div>
      </div>
      <SignInForm />
    </main>
  );
};



export default SignIn;
