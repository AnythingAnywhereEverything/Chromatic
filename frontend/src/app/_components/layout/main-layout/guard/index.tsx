
import Link from "next/dist/client/link";
import style from "./style.module.scss";

const GuardTopBar = () => {
    return (
        <nav className={style["guard-topbar"]}>
            {/* Title & signin sign up button */}
            <h1>Chromatic</h1>
            <div className={style["auth-buttons"]}>
                <Link className={style["signin-button"]} href="/auth/signin">Sign In</Link>
                <Link className={style["signup-button"]} href="/auth/signup">Sign Up</Link>
            </div>
        </nav>
    );
};
export default GuardTopBar;
