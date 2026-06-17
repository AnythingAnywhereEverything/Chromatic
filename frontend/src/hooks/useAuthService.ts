import { login, logout } from "@/api/auth";
import { clearCacheUserId, clearToken, setCacheUserId, setToken } from "@/handler/token_handler";
import { useMutation, useQueryClient } from "@tanstack/react-query";

export const useAuthService = () => {
    const queryClient = useQueryClient();

    const loginMut = useMutation({
        mutationFn: login,
        onSuccess: (data) => {
            setToken(data.token);
            setCacheUserId(data.user_id);
            queryClient.invalidateQueries({ queryKey: ["user"] });
        },
    });

    const logoutMut = useMutation({
        mutationFn: logout,
        onSuccess: () => {
            clearToken();
            clearCacheUserId();
            queryClient.removeQueries({ queryKey: ["user"] });
        },
    });

    return {
        login: loginMut.mutateAsync,
        logout: logoutMut.mutateAsync,
    };
};