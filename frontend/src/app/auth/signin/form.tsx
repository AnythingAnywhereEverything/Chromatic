"use client";

import { useEffect, useState } from "react";
import style from "@styles/layouts/authLayout.module.scss";
import field from "@styles/ui/chromatic/field.module.scss";
import { NextPageWithLayout } from "@/types/global";
import {
    FieldError,
    FieldSeparator,
} from "@components/ui/chromaticUI";
import Link from "next/link";
import Form from "next/form";
import { useRouter } from "next/navigation";
import { useUser } from "@/hooks/useUser";
import { useAuthService } from "@/hooks/useAuthService";
import { FaRegEye, FaRegEyeSlash } from "react-icons/fa";

const SignInForm: NextPageWithLayout = () => {
    const [reveal, setReveal] = useState(false);
    const [error, setError] = useState<string | null>(null);

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
        <div className={style.form}>
            <section className={`${field.fieldSet}`}>
                <h2>Sign In</h2>
                <FieldSeparator>or</FieldSeparator>
                <Form action={"#"} onSubmit={handleSubmit}>
                    <section className={`${field.fieldGroup}`}>
                        <section>
                            <label 
                             htmlFor="username">Username or Email</label>
                            <div
                            style={{marginTop: "calc(var(--spacing) * 2)"}} 
                            className={style["wrapper"]}>
                                <input
                                    className={style["inputField"]}
                                    aria-invalid={error ? "true" : "false"}
                                    required
                                    name="username"
                                    id="username"
                                    placeholder="example@gmail.com"
                                />
                                </div>
                        </section>
                        <section>
                            <label htmlFor="password">Password</label>
                                <div 
                                style={{marginTop: "calc(var(--spacing) * 2)"}} 
                                className={style["wrapper"]}>
                                <input
                                    className={style["inputField"]}
                                    aria-invalid={error ? "true" : "false"}
                                    autoComplete="password"
                                    required
                                    type={reveal ? "text" : "password"}
                                    name="password"
                                    id="password"
                                    placeholder="• • • • • • • •"
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
                            {error && <FieldError>{error}</FieldError>}
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
    );
};

export default SignInForm;
