// hooks/useUser.js
import { useQuery } from "@tanstack/react-query";
import { getUser } from "@/api/user";

export const useUser = () => {
    return useQuery({
        queryKey: ["user"],
        queryFn: async () => {
            return await getUser();
        },
        staleTime: 5 * 60 * 1000,
        retry: false,
    });
};