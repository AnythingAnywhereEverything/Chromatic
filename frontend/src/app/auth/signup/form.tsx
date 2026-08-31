"use client";

import { register } from "@/api/auth";
import { useUser } from "@/hooks/useUser";
import { useRouter } from "next/navigation";
import React, { useEffect, useState } from "react";
import Link from "next/link";
import { FaEye, FaEyeSlash } from "react-icons/fa";
import Form from "next/form";
import style from "@styles/layouts/authLayout.module.scss";

const validateEmail = (email: string) => {
    return /^[a-zA-Z0-9._%+\-]+@[a-zA-Z0-9.\-]+\.[a-zA-Z]{2,}$/.test(email);
};

const validateUsername = (username: string, skipMinLength = false) => {
    const trimmed = username.trim();
    if (!trimmed && !skipMinLength) return "Username cannot be empty.";
    if (!skipMinLength && trimmed.length < 3)
        return "Username must be at least 3 characters.";
    if (trimmed.length > 32) return "Username cannot exceed 32 characters.";
    return undefined;
};
const validatePassword = (password: string) => {
    if (!password) return "Password is required.";
    if (password.length < 8) return "Password must be at least 8 characters.";
    // passwiord must contain special characters
    if (!/[!@#$%^&*(),.?":{}|<>]/.test(password))
        return "Password must contain at least one special character.";
    // password must contain at least one uppercase letter
    if (!/[A-Z]/.test(password))
        return "Password must contain at least one uppercase letter.";
    // password must contain at least one lowercase letter
    if (!/[a-z]/.test(password))
        return "Password must contain at least one lowercase letter.";
    return undefined;
};

const validateConfirmPassword = (password: string, confirmPassword: string) => {
    if (!confirmPassword) return "Please confirm your password.";
    if (confirmPassword !== password) return "Password does not match.";
    return undefined;
};

const SignUpForm = () => {
    // State for managing form input visibility and edit status
    const [revealPassword, setRevealPassword] = useState(false);
    const [revealConfirm, setRevealConfirm] = useState(false);

    const [submitting, setSubmitting] = useState(false);
    const [isSubmitable, setIsSubmitable] = useState(false);

    const { data, isLoading } = useUser();
    const router = useRouter();

    useEffect(() => {
        if (!isLoading && data) {
            router.replace("/");
        }
    }, [isLoading, data, router]);

    type Errors = Partial<{
        username: string;
        email: string;
        password: string;
        confirmPassword: string;
    }>;

    const [editing, setEditing] = useState({
        username: false,
        email: false,
        password: false,
        confirmPassword: false,
    });

    const [values, setValues] = useState({
        username: "",
        email: "",
        password: "",
        confirmPassword: "",
    });

    const [errors, setErrors] = useState<Errors>({});

    useEffect(() => {
        setIsSubmitable(
            !errors.username &&
                !errors.email &&
                !errors.password &&
                !errors.confirmPassword &&
                values.username.trim() !== "" &&
                values.email.trim() !== "" &&
                values.password.trim() !== "" &&
                values.confirmPassword.trim() !== "",
        );
    }, [errors, values]);

    const handleEdit = (field: keyof typeof editing, value: boolean) => {
        // validate error everytime the field turns false
        if (!value) {
            const nextErrors: Errors = { ...errors };
            if (field === "username") {
                const error = validateUsername(values.username);
                if (error) {
                    nextErrors.username = error;
                } else {
                    delete nextErrors.username;
                }
            }
            if (field === "email") {
                const error = validateEmail(values.email)
                    ? undefined
                    : "Please enter a valid email address.";
                if (error) {
                    nextErrors.email = error;
                } else {
                    delete nextErrors.email;
                }
            }
            if (field === "password") {
                const error = validatePassword(values.password);
                if (error) {
                    nextErrors.password = error;
                } else {
                    delete nextErrors.password;
                }
            }
            if (field === "confirmPassword") {
                const error = validateConfirmPassword(
                    values.password,
                    values.confirmPassword,
                );
                if (error) {
                    nextErrors.confirmPassword = error;
                } else {
                    delete nextErrors.confirmPassword;
                }
            }
            setErrors(nextErrors);
        }

        setEditing((prev) => ({
            ...prev,
            [field]: value,
        }));
    };

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { name, value } = e.target;

        if (name === "username" && value.length > 32) {
            return;
        }

        setValues((prev) => ({
            ...prev,
            [name]: value,
        }));

        if (errors[name as keyof Errors]) {
            setErrors((prev) => ({
                ...prev,
                [name]: undefined,
            }));
        }
    };

    const validate = () => {
        const nextErrors: Errors = {};

        if (!values.username.trim()) {
            nextErrors.username = "Username is required.";
        }

        if (!values.email.trim()) {
            nextErrors.email = "Email is required.";
        } else if (!validateEmail(values.email)) {
            nextErrors.email = "Please enter a valid email address.";
        }

        if (!values.password) {
            nextErrors.password = "Password is required.";
        } else if (values.password.length < 8) {
            nextErrors.password = "Password must be at least 8 characters.";
        }

        if (!values.confirmPassword) {
            nextErrors.confirmPassword = "Please confirm your password.";
        } else if (values.confirmPassword !== values.password) {
            nextErrors.confirmPassword = "Passwords do not match.";
        }

        setErrors(nextErrors);

        return Object.keys(nextErrors).length === 0;
    };

    const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();

        if (!validate()) return;

        register(values)
            .then(() => {
                const path = "/auth/signin";
                const searchParams = new URLSearchParams(window.location.search);
                const redirectTo = searchParams.get("rt") || "/";

                router.push(`${path}?rt=${encodeURIComponent(redirectTo)}`);
            })
            .catch((error) => {
                console.error("Registration error:", error);
            });
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
            <h2>Sign up to Chromatic</h2>
            <Form
                action={"#"}
                onSubmit={handleSubmit}
                className={style["form"]}
            >
                <section className={style["fields"]}>
                    <section className={style["field"]}>
                        <div className={style["wrapper"]}>
                            <span
                                className={`${style["movable"]} ${
                                    editing.username || values.username
                                        ? style["active"]
                                        : ""
                                }`}
                            >
                                Username
                            </span>
                            <input
                                className={style["input-field"]}
                                aria-invalid={
                                    errors.username ? "true" : "false"
                                }
                                required
                                name="username"
                                id="username"
                                value={values.username}
                                onChange={handleChange}
                                onFocus={() => handleEdit("username", true)}
                                onBlur={() => handleEdit("username", false)}
                            />
                        </div>

                        {errors.username && (
                            <span className={style["error-message"]}>
                                {errors.username}
                            </span>
                        )}
                    </section>

                    <section className={style["field"]}>
                        <div className={style["wrapper"]}>
                            <span
                                className={`${style["movable"]} ${
                                    editing.email || values.email
                                        ? style["active"]
                                        : ""
                                }`}
                            >
                                Email
                            </span>
                            <input
                                className={style["input-field"]}
                                aria-invalid={errors.email ? "true" : "false"}
                                required
                                type="email"
                                name="email"
                                id="email"
                                value={values.email}
                                onChange={handleChange}
                                onFocus={() => handleEdit("email", true)}
                                onBlur={() => handleEdit("email", false)}
                            />
                        </div>

                        {errors.email && (
                            <span className={style["error-message"]}>
                                {errors.email}
                            </span>
                        )}
                    </section>

                    <section className={style["field"]}>
                        <div className={style["wrapper"]}>
                            <span
                                className={`${style["movable"]} ${
                                    editing.password || values.password
                                        ? style["active"]
                                        : ""
                                }`}
                            >
                                Password
                            </span>
                            <input
                                className={style["input-field"]}
                                aria-invalid={
                                    errors.password ? "true" : "false"
                                }
                                autoComplete="new-password"
                                required
                                type={revealPassword ? "text" : "password"}
                                name="password"
                                id="password"
                                value={values.password}
                                onChange={handleChange}
                                onFocus={() => handleEdit("password", true)}
                                onBlur={() => handleEdit("password", false)}
                            />

                            <button
                                className={style["password-toggle"]}
                                type="button"
                                onClick={() =>
                                    setRevealPassword((prev) => !prev)
                                }
                            >
                                {revealPassword ? <FaEyeSlash /> : <FaEye />}
                            </button>
                        </div>

                        {errors.password && (
                            <span className={style["error-message"]}>
                                {errors.password}
                            </span>
                        )}
                    </section>

                    <section className={style["field"]}>
                        <div className={style["wrapper"]}>
                            <span
                                className={`${style["movable"]} ${
                                    editing.confirmPassword ||
                                    values.confirmPassword
                                        ? style["active"]
                                        : ""
                                }`}
                            >
                                Confirm Password
                            </span>
                            <input
                                className={style["input-field"]}
                                aria-invalid={
                                    errors.confirmPassword ? "true" : "false"
                                }
                                autoComplete="new-password"
                                required
                                type={revealConfirm ? "text" : "password"}
                                name="confirmPassword"
                                id="confirmPassword"
                                value={values.confirmPassword}
                                onChange={handleChange}
                                onFocus={() =>
                                    handleEdit("confirmPassword", true)
                                }
                                onBlur={() =>
                                    handleEdit("confirmPassword", false)
                                }
                            />

                            <button
                                className={style["password-toggle"]}
                                type="button"
                                onClick={() =>
                                    setRevealConfirm((prev) => !prev)
                                }
                            >
                                {revealConfirm ? <FaEyeSlash /> : <FaEye />}
                            </button>
                        </div>

                        {errors.confirmPassword && (
                            <span className={style["error-message"]}>
                                {errors.confirmPassword}
                            </span>
                        )}
                    </section>
                </section>

                <button
                    className={style["submit"]}
                    type="submit"
                    disabled={!isSubmitable || submitting}
                >
                    Sign Up
                </button>

                <p className={style["prompt"]}>
                    Already have an account?{" "}
                    <Link href={"/auth/signin"}>Sign In.</Link>
                </p>
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

export default SignUpForm;
