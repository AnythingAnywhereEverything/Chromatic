"use client";

import { useEffect, useState, useRef } from "react";
import style from "@styles/layouts/authLayout.module.scss";
import { NextPageWithLayout } from "@/types/global";
import Link from "next/link";
import Form from "next/form";
import { useRouter } from "next/navigation";
import { useUser } from "@/hooks/useUser";
import { useAuthService } from "@/hooks/useAuthService";
import { FaEye, FaEyeSlash } from "react-icons/fa";

const SignInForm: NextPageWithLayout = () => {
    // Actions
    const [reveal, setReveal] = useState(false);
    const [error, setError] = useState<string | null>(null);

    // Form fields state
    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");
    const [isEditingUser, setEditingUser] = useState(false);
    const [isEditingPass, setEditingPass] = useState(false);

    const [isSubmitting, setIsSubmitting] = useState(false);
    const [isSubmitable, setIsSubmitable] = useState(false);

    // Input refs
    const usernameInputRef = useRef<HTMLInputElement | null>(null);
    const passwordInputRef = useRef<HTMLInputElement | null>(null);

    // Handlers

    const handleUsernameChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const value = e.target.value;
        setUsername(value);
    };

    const handlePasswordChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        setPassword(e.target.value);
    };

    const handleReveal = () => {
        setReveal((prev) => !prev);
    };

    const { data, isLoading } = useUser();
    const router = useRouter();

    useEffect(() => {
        setIsSubmitable(username.trim() !== "" && password.trim() !== "");
    }, [username, password]);

    useEffect(() => {
        if (!isLoading && data) {
            router.replace("/");
        }
    }, [isLoading, data, router]);

    const { login } = useAuthService();

    const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();

        const formData = new FormData(e.currentTarget);

        setIsSubmitting(true);

        try {
            await login({
                username_or_email: formData
                    .get("username")
                    ?.toString()
                    .trim()
                    .toLowerCase() as string,
                password: formData.get("password")?.toString().trim() as string,
            });

            setError(null);
            router.push("/");
        } catch (error: any) {
            console.error(error);
            setError(error.message);
        } finally {
            setIsSubmitting(false);
        }
    };

    return (
        <div className={style["form-container"]}>
            <img
                src="/asset/logo_no_border.svg"
                alt="Chromatic Logo"
                className={style["logo"]}
                width={80}
                height={80}
                draggable={false}
            />
            <h2 className={style["title"]}>Sign in to Chromatic</h2>
            <Form
                action={"#"}
                onSubmit={handleSubmit}
                className={style["form"]}
            >
                <fieldset className={style["fields"]}>
                    <div
                        className={style["wrapper"]}
                        onClick={() => {
                            usernameInputRef.current?.focus();
                        }}
                    >
                        <span
                            className={`${style["movable"]} ${
                                isEditingUser || username ? style["active"] : ""
                            }`}
                        >
                            Username or email address
                        </span>

                        <input
                            className={style["input-field"]}
                            aria-invalid={error ? "true" : "false"}
                            required
                            autoComplete="username"
                            name="username"
                            ref={usernameInputRef}
                            value={username}
                            onChange={handleUsernameChange}
                            onFocus={() => setEditingUser(true)}
                            onBlur={() => setEditingUser(false)}
                        />
                    </div>
                    <div
                        className={style["wrapper"]}
                        onClick={() => {
                            passwordInputRef.current?.focus();
                        }}
                    >
                        <span
                            className={`${style["movable"]} ${
                                isEditingPass || password ? style["active"] : ""
                            }`}
                        >
                            Password
                        </span>

                        <input
                            ref={passwordInputRef}
                            className={style["input-field"]}
                            aria-invalid={error ? "true" : "false"}
                            autoComplete="current-password"
                            required
                            type={reveal ? "text" : "password"}
                            name="password"
                            id="password"
                            value={password}
                            onChange={handlePasswordChange}
                            onFocus={() => setEditingPass(true)}
                            onBlur={() => setEditingPass(false)}
                        />
                        <button
                            className={style["password-toggle"]}
                            type="button"
                            onClick={handleReveal}
                        >
                            {reveal ? <FaEyeSlash /> : <FaEye />}
                        </button>
                    </div>
                </fieldset>
                {error && (
                    <span className={style["error-message"]}>{error}</span>
                )}
                <button
                    className={`${style["submit"]} ${isSubmitting ? style["submitting"] : ""}`}
                    type="submit"
                    disabled={!isSubmitable}
                >
                    {isSubmitting ? (
                        <img src="/asset/svgs/dot_loading.svg" alt="Saving" />
                    ) : (
                        "Sign In"
                    )}
                </button>
                <div className={style["auth-links"]}>
                    <div className={style["prompt"]}>
                        Dont have an account yet?{" "}
                        <Link href={"/auth/signup"}>Sign Up.</Link>
                    </div>
                    <div className={style["forgot-password"]}>
                        <Link href={"#"}>Forgot password?</Link>
                    </div>
                </div>
            </Form>

            <article className={style["terms-of-service"]}>
                By clicking Sign In, you agree to our{" "}
                <Link href={"/terms-of-service"}>Terms of Service</Link> and{" "}
                <Link href={"/privacy-policy"}>Privacy Policy</Link>. Your
                continued use of our services constitutes your acceptance of
                these terms.
            </article>
        </div>
    );
};

export default SignInForm;
