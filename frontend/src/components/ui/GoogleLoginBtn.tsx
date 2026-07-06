 'use client';

import { useGoogleLogin } from "@react-oauth/google";
import { Button, Icon } from "./chormaticUI";
import { useRouter } from "next/navigation";
import { useQueryClient } from "@tanstack/react-query";
import { setCacheUserId, setToken } from "@/handler/token_handler";

import { IoLogoGoogle } from "react-icons/io5";

export default function GoogleAuthButton() {
  const router = useRouter();
  const queryClient = useQueryClient();

  const login = useGoogleLogin({
    onSuccess: async (tokenResponse) => {
      const res = await fetch("/api/v2/auth/oauth", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          provider: "google",
          access_token: tokenResponse.access_token,
        }),
      });

      if (!res.ok) {
        console.error("OAuth registration failed");
        return;
      }

      const data = await res.json();

      setToken(data.token);
      setCacheUserId(data.user_id);

      await queryClient.invalidateQueries({ queryKey: ["user"] });

      router.push("/");
    },
  });

  return (
    <Button variant={"outline"} onClick={() => login()}>
      <IoLogoGoogle style={{ marginRight: "0.5rem" }} />
      Continue with Google
    </Button>
  );
}