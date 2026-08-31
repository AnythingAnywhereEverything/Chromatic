import style from "@styles/layouts/authLayout.module.scss";
import SignUpForm from "./form";

export const metadata = {
    title: "Sign Up",
    description: "Sign up for your account",
};

const SignUp = () => {
    return (
        <div className={style['auth-container']}>
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
            <SignUpForm />
        </div>
    );
};

export default SignUp;
