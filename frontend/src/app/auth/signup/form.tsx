"use client";

import { register } from "@/api/auth";
import { useUser } from "@/hooks/useUser";
import field from "@styles/ui/chromatic/field.module.scss";
import style from "@styles/layouts/authLayout.module.scss";
import { useRouter } from "next/navigation";
import React, { useEffect, useState } from "react";
import { FieldError, FieldSeparator } from "@components/ui/chromaticUI";
import { Form } from "@base-ui/react";
import Link from "next/link";
import { FaRegEye, FaRegEyeSlash } from "react-icons/fa";

const SignUpForm = () => {
    const [revealPassword, setRevealPassword] = useState(false);
    const [revealConfirm, setRevealConfirm] = useState(false);

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

    const [values, setValues] = useState({
        username: "",
        email: "",
        password: "",
        confirmPassword: "",
    });

    const [errors, setErrors] = useState<Errors>({});

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { name, value } = e.target;

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
        } else if (
            !/^[a-zA-Z0-9._%+\-]+@[a-zA-Z0-9.\-]+\.[a-zA-Z]{2,}$/.test(
                values.email,
            )
        ) {
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
            .then((response) => {
                console.log(response);
                router.push("/auth/signin");
            })
            .catch((error) => {
                console.error("Registration error:", error);
            });
    };

    return (
        <div className={style.form}>
            <section className={field.fieldSet}>
                <h2>Sign Up</h2>

                <FieldSeparator>or</FieldSeparator>

                <Form action={"#"} onSubmit={handleSubmit}>
                    <section className={field.fieldGroup}>
                        <section>
                            <label htmlFor="username">Username</label>

                            <div
                                style={{
                                    marginTop: "calc(var(--spacing) * 2)",
                                }}
                                className={style["wrapper"]}
                            >
                                <input
                                    className={style["input-field"]}
                                    aria-invalid={errors.username ? "true" : "false"}
                                    required
                                    name="username"
                                    id="username"
                                    placeholder="webapp_user"
                                    value={values.username}
                                    onChange={handleChange}
                                />
                            </div>

                            {errors.username && (
                                <FieldError>{errors.username}</FieldError>
                            )}
                        </section>

                        <section>
                            <label htmlFor="email">Email</label>

                            <div
                                style={{
                                    marginTop: "calc(var(--spacing) * 2)",
                                }}
                                className={style["wrapper"]}
                            >
                                <input
                                    className={style["input-field"]}
                                    aria-invalid={errors.email ? "true" : "false"}
                                    required
                                    type="email"
                                    name="email"
                                    id="email"
                                    placeholder="example@gmail.com"
                                    value={values.email}
                                    onChange={handleChange}
                                />
                            </div>

                            {errors.email && (
                                <FieldError>{errors.email}</FieldError>
                            )}
                        </section>

                        <section>
                            <label htmlFor="password">Password</label>

                            <div
                                style={{
                                    marginTop: "calc(var(--spacing) * 2)",
                                }}
                                className={style["wrapper"]}
                            >
                                <input
                                    className={style["input-field"]}
                                    aria-invalid={errors.password ? "true" : "false"}
                                    autoComplete="new-password"
                                    required
                                    type={revealPassword ? "text" : "password"}
                                    name="password"
                                    id="password"
                                    placeholder="• • • • • • • •"
                                    value={values.password}
                                    onChange={handleChange}
                                />

                                <button
                                    className={style["password-toggle"]}
                                    type="button"
                                    onClick={() =>
                                        setRevealPassword((prev) => !prev)
                                    }
                                >
                                    {revealPassword ? (
                                        <FaRegEyeSlash />
                                    ) : (
                                        <FaRegEye />
                                    )}
                                </button>
                            </div>

                            {errors.password && (
                                <FieldError>{errors.password}</FieldError>
                            )}
                        </section>

                        <section>
                            <label htmlFor="confirmPassword">
                                Confirm Password
                            </label>

                            <div
                                style={{
                                    marginTop: "calc(var(--spacing) * 2)",
                                }}
                                className={style["wrapper"]}
                            >
                                <input
                                    className={style["input-field"]}
                                    aria-invalid={
                                        errors.confirmPassword
                                            ? "true"
                                            : "false"
                                    }
                                    autoComplete="new-password"
                                    required
                                    type={revealConfirm ? "text" : "password"}
                                    name="confirmPassword"
                                    id="confirmPassword"
                                    placeholder="• • • • • • • •"
                                    value={values.confirmPassword}
                                    onChange={handleChange}
                                />

                                <button
                                    className={style["password-toggle"]}
                                    type="button"
                                    onClick={() =>
                                        setRevealConfirm((prev) => !prev)
                                    }
                                >
                                    {revealConfirm ? (
                                        <FaRegEyeSlash />
                                    ) : (
                                        <FaRegEye />
                                    )}
                                </button>
                            </div>

                            {errors.confirmPassword && (
                                <FieldError>
                                    {errors.confirmPassword}
                                </FieldError>
                            )}
                        </section>

                        <section className={style["btn-field"]}>
                            <button
                                className={style["submit"]}
                                type="submit"
                            >
                                Sign Up
                            </button>
                        </section>

                        <p className={field["fieldDescription"]}>
                            Already have an account?{" "}
                            <Link href={"/auth/signin"}>Sign In.</Link>
                        </p>
                    </section>
                </Form>
            </section>
        </div>
    );
};

export default SignUpForm;