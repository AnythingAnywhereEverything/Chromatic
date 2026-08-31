import style from "@styles/layouts/authLayout.module.scss";
import SignInForm from "./form";

export const metadata = {
    title: "Sign In | Chromatic",
    description: "Sign in to your account",
};

const SignIn = () => {
    return (
        <main className={style["auth-container"]}>
            <SignInForm />
        </main>
    );
};

export default SignIn;
