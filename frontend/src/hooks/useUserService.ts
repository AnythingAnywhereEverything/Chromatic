import { useMutation, useQueryClient } from "@tanstack/react-query";
import {
  updateDisplayName,
  updateUsername,
} from "@/api/user";
import { updateUserAvatar, updateUserBanner } from "@/api/user/profile";

export const useUserService = () => {
  const queryClient = useQueryClient();

  const handleSuccess = (updatedUser: any) => {
    queryClient.setQueryData(["user"], updatedUser);
  };

  const displayNameMutation = useMutation({
    mutationFn: updateDisplayName,
    onSuccess: handleSuccess,
  });

  const usernameMutation = useMutation({
    mutationFn: updateUsername,
    onSuccess: handleSuccess,
  });

  const profileMutation = useMutation({
    mutationFn: updateUserAvatar,
    onSuccess: handleSuccess,
  });

  const bannerMutation = useMutation({
    mutationFn: updateUserBanner,
    onSuccess: handleSuccess,
  });

  return {
    updateDisplayName: displayNameMutation,
    updateUsername: usernameMutation,
    updateUserAvatar: profileMutation,
    updateUserBanner: bannerMutation,
  };
};