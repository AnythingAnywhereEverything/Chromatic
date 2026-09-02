import Link from "next/link";
import style from "./style.module.scss";

const GuardTopBar = () => {
    return (
        <nav className={style["guard-topbar"]}>
            <div className={style["top-bar-logo"]}>
                <img src="/asset/logo_no_border.svg" alt="Chromatic Logo" />
                <h1>Chromatic</h1>
            </div>
            <div className={style["auth-buttons"]}>
                <Link className={style["signin-button"]} href="/auth/signin">
                    Sign In
                </Link>
                <Link className={style["signup-button"]} href="/auth/signup">
                    Sign Up
                </Link>
            </div>
        </nav>
    );
};
export default GuardTopBar;
