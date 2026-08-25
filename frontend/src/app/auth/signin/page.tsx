import SignInForm from "./form";
import style from "./style.module.scss";

export const metadata = {
    title: "Sign In",
    description: "Sign in to your account",
};

const SignIn = () => {
    return (
        <main className={style.authContainer}>
            <h2>Chromatic</h2>
            <SignInForm />
        </main>
    );
};

export default SignIn;
