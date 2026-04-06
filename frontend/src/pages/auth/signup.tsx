import { useUser } from "@/hooks/useUser";
import { NextPageWithLayout } from "@/types/global";
import { useRouter } from "next/router";
import React, { useState } from "react";

const SignIn: NextPageWithLayout = () => {
    const [revealPassword, setRevealPassword] = useState(false);
    const [revealConfirm, setRevealConfirm] = useState(false);

    const { data, isLoading } = useUser();
    const router = useRouter();

}