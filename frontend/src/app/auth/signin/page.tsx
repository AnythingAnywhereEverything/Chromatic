import style from "@styles/layouts/authLayout.module.scss";
import SignInForm from "./form";

export const metadata = {
    title: "Sign In",
    description: "Sign in to your account",
};

const SignIn = () => {
    return (
        <div className={style.authContainer}>
            <SignInForm />
        </div>
    );
};

export default SignIn;
