import { useQuery } from "@tanstack/react-query";
import { getCurrentProfile } from "@/api/user/profile";

export const useProfile = () => {
    return useQuery({
        queryKey: ["profile"],
        queryFn: async () => {
            return await getCurrentProfile();
        },
        staleTime: 5 * 60 * 1000,
        retry: false,
    });
};