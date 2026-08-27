import style from "@styles/layouts/authLayout.module.scss";
import SignUpForm from "./form";

export const metadata = {
    title: "Sign Up",
    description: "Sign up for your account",
};

const SignUp = () => {
    return (
        <div className={style['auth-container']}>
            <SignUpForm />
        </div>
    );
};

export default SignUp;
