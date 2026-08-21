import { useMutation, useQueryClient } from "@tanstack/react-query";
import {
  updateDisplayName,
  updateUsername,
} from "@/api/user";
import {updateUserProfile } from "@/api/user/profile";

export const useUserService = () => {
  const queryClient = useQueryClient();

  const handleUserSuccess = (updatedUser: any) => {
    queryClient.setQueryData(["user"], updatedUser);
  };

  const handleProfileSuccess = (updatedProfile: any) => {
    queryClient.setQueryData(["profile"], updatedProfile);
    queryClient.invalidateQueries({ queryKey: ["user"] });
  };

  const displayNameMutation = useMutation({
    mutationFn: updateDisplayName,
    onSuccess: handleUserSuccess,
  });

  const usernameMutation = useMutation({
    mutationFn: updateUsername,
    onSuccess: handleUserSuccess,
  });

  const profileMutation = useMutation({
    mutationFn: updateUserProfile,
    onSuccess: handleProfileSuccess,
  });

  return {
    updateDisplayName: displayNameMutation,
    updateUsername: usernameMutation,
    updateUserProfile: profileMutation,
  };
};