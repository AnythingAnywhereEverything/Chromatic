import { useMutation, useQueryClient } from "@tanstack/react-query";
import {
  updateDisplayName,
  updateUsername,
  updateProfilePicture
} from "@/api/user";

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
    mutationFn: updateProfilePicture,
    onSuccess: handleSuccess,
  });

  return {
    updateDisplayName: displayNameMutation,
    updateUsername: usernameMutation,
    updateProfilePicture: profileMutation,
  };
};