"use client";

import { useEffect, useState } from "react";
import style from "@styles/layouts/authLayout.module.scss";
import field from "@styles/ui/chromatic/field.module.scss";
import { NextPageWithLayout } from "@/types/global";
import Link from "next/link";
import Form from "next/form";
import { useRouter } from "next/navigation";
import { useUser } from "@/hooks/useUser";
import { useAuthService } from "@/hooks/useAuthService";
import { FaRegEye, FaRegEyeSlash } from "react-icons/fa";

// todo: add anim on this page
// todo: re css on focused post page
// todo: complete create post

const SignInForm: NextPageWithLayout = () => {
    const [reveal, setReveal] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");
    const [isEditingUser, setEditingUser] = useState(false);
    
    const [isEditingPass, setEditingPass] = useState(false);
    
    const handleReveal = () => {
        setReveal((prev) => !prev);
    };
    
    const { data, isLoading } = useUser();
    const router = useRouter();

    useEffect(() => {
        if (!isLoading && data) {
            router.replace("/");
        }
    }, [isLoading, data, router]);

    const { login } = useAuthService();


    const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();

        try {
            await login({
                username_or_email: (
                    e.currentTarget.elements.namedItem(
                        "username",
                    ) as HTMLInputElement
                ).value,
                password: (
                    e.currentTarget.elements.namedItem(
                        "password",
                    ) as HTMLInputElement
                ).value,
            });

            setError(null);
            router.push("/");
        } catch (error: any) {
            setError(error.message);
        }
    };

    return (
        <div className={style["container-right"]}>
            <div className={style.form}>
            <section className={`${field.fieldSet} ${style['sign-in']}`}>
                <h2>Sign In</h2>
                <Form action={"#"} onSubmit={handleSubmit}>
                    <section className={`${field.fieldGroup}`}>
                        <section className={style["field"]}>
                            <label htmlFor="username"></label>
                            <div
                                style={{ marginTop: "calc(var(--spacing) * 2)" }}
                                className={style["wrapper"]}
                            >
                                <span
                                    className={`${style["example-movable-user"]} ${
                                        isEditingUser || username ? style.active : ""
                                    }`}
                                >
                                    Username
                                </span>
                                
                                <input
                                    className={style["input-field"]}
                                    aria-invalid={error ? "true" : "false"}
                                    required
                                    name="username"
                                    id="username"
                                    value={username}
                                    onChange={(e) => setUsername(e.target.value)}
                                    onFocus={() => setEditingUser(true)}
                                    onBlur={() => setEditingUser(false)}
                                />
                            </div>
                           <div
                                style={{ marginTop: "calc(var(--spacing) * 2)" }}
                                className={style["wrapper"]}
                                >
                                <span
                                    className={`${style["example-movable-password"]} ${
                                        isEditingPass || password ? style.active : ""
                                    }`}
                                    >
                                    Password
                                </span>
                                
                                <input
                                    className={style["input-field"]}
                                    aria-invalid={error ? "true" : "false"}
                                    autoComplete="password"
                                    required
                                    type={reveal ? "text" : "password"}
                                    name="password"
                                    id="password"
                                    value={password}
                                    onChange={(e) => setPassword(e.target.value)}
                                    onFocus={() => setEditingPass(true)}
                                    onBlur={() => setEditingPass(false)}
                                    />

                                <button
                                    className={style["password-toggle"]}
                                    type="button"
                                    onClick={handleReveal}
                                    >
                                    {reveal ? <FaRegEyeSlash /> : <FaRegEye />}
                                </button>
                            </div>
                        </section>
                        <section className={style["btn-field"]}>
                            <button className={style["submit"]} type="submit">Sign In</button>
                            <p className={field["fieldDescription"]}>
                                <Link href={"#"}>Forgot password?</Link>
                            </p>
                        </section>
                        <p className={field["fieldDescription"]}>
                            Dont have an account yet?{" "}
                            <Link href={"/auth/signup"}>Sign Up.</Link>
                        </p>
                    </section>
                </Form>
            </section>
            </div>
        </div>
    );
};

export default SignInForm;
