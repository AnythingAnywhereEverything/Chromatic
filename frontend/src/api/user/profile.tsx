import { fetchWithAuth, fetchWithOptionAuth } from "@/handler/token_handler";

export interface PublicUserProfileResponse {
  id: string;
  username: string;
  display_name: string;
  avatar: string | null;
  avatar_thumbhash: string | null;
  banner: string | null;
  banner_thumbhash: string | null;
  bio: string | null;
  is_blocked: boolean;
  is_follower: boolean;
  is_following: boolean;
  followers_count: number;
  following_count: number;
  active: boolean;
  created_at: string;
}

interface UserResponse {
  id: string;
  username: string;
  display_name: string;
  avatar: string | null;
  avatar_thumbhash: string | null;
  banner: string | null;
  banner_thumbhash: string | null;
  bio: string | null;
  email: string;
  email_verified:boolean;
  active: boolean;
  created_at: string;
}


export async function getPublicUserProfile(username: string): Promise<PublicUserProfileResponse | null> {
  const res = await fetchWithOptionAuth(`v2/users/profile/${username}`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
    },
  });
  if (!res.ok) return null;
  const data = await res.json();
  return data;
}

export async function getCurrentProfile(): Promise<PublicUserProfileResponse | null> {
  const res = await fetchWithAuth(`v2/users/me/profile`);
  if (!res.ok) return null;
  const data = await res.json();
  return data;
}

export async function updateUserProfile(formData: FormData): Promise<PublicUserProfileResponse | null> {

  const res = await fetchWithAuth(`v2/users/me/profile`, {
    method: "PATCH",
    body: formData,
  });

  if (!res.ok) return null;
  const data = await res.json();
  return data;
}