"use client";

import { useEffect, useState } from "react";
import style from "@styles/layouts/authLayout.module.scss";
import { NextPageWithLayout } from "@/types/global";
import {
    Field,
    FieldLabel,
    FieldDescription,
    FieldGroup,
    FieldSet,
    Input,
    InputGroup,
    InputGroupAddon,
    Button,
    InputGroupInput,
    Icon,
    FieldError,
    FieldSeparator,
} from "@components/ui/chormaticUI";
import Link from "next/link";
import Form from "next/form";
import { useRouter } from "next/navigation";
import GoogleAuthButton from "@components/ui/GoogleLoginBtn";
import { useUser } from "@/hooks/useUser";
import { useAuthService } from "@/hooks/useAuthService";

const SignInForm: NextPageWithLayout = () => {
    const [reveal, setReveal] = useState(false);
    const [error, setError] = useState<string | null>(null);

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
            <FieldSet>
                <h2>Sign In</h2>
                <GoogleAuthButton />
                <FieldSeparator>or</FieldSeparator>
                <Form action={"#"} onSubmit={handleSubmit}>
                    <FieldGroup>
                        <Field>
                            <FieldLabel htmlFor="username">
                                Username or Email
                            </FieldLabel>
                            <Input
                                aria-invalid={error ? "true" : "false"}
                                required
                                name="username"
                                id="username"
                                placeholder="example@gmail.com"
                            />
                        </Field>
                        <Field>
                            <FieldLabel htmlFor="password">Password</FieldLabel>
                            <InputGroup>
                                <InputGroupAddon align="inline-end">
                                    <Button
                                        type="button"
                                        variant={"ghost"}
                                        size={"icon-xs"}
                                        onClick={() => setReveal(!reveal)}
                                    >
                                        <Icon value={reveal ? "" : ""} />
                                    </Button>
                                </InputGroupAddon>
                                <InputGroupInput
                                    aria-invalid={error ? "true" : "false"}
                                    autoComplete="password"
                                    required
                                    type={reveal ? "text" : "password"}
                                    name="password"
                                    id="password"
                                    placeholder="• • • • • • • •"
                                />
                            </InputGroup>
                        </Field>
                        <Field>
                            <Button type="submit">Sign In</Button>
                            {error && <FieldError>{error}</FieldError>}
                            <FieldDescription>
                                <Link href={"#"}>Forgot password?</Link>
                            </FieldDescription>
                        </Field>
                        <FieldDescription>
                            Dont have an account yet?{" "}
                            <Link href={"/auth/signup"}>Sign Up.</Link>
                        </FieldDescription>
                    </FieldGroup>
                </Form>
            </FieldSet>
        </div>
    );
};

export default SignInForm;
