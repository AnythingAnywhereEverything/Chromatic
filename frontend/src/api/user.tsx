import { fetchWithAuth, getCacheUserId, getToken } from "@/handler/token_handler";

interface UserResponse {
  id: string;
  username: string;
  display_name: string;
  profile_picture_url: string;
  email: string;
  email_verified:boolean;
  active: boolean;
  created_at: string;
}

export interface SessionResponse{
  id: string,
  created_at: string,
  agent: string,
} 
interface PasswordData{
  current_password: string;
  new_password: string;
  confirm_password: string;
}
export async function getUser(): Promise<UserResponse> {
  const token = getToken();
  const userId = getCacheUserId();
  if (!token) throw new Error("No token found");

  const res = await fetchWithAuth(`/api/v2/users/${userId}`); // proxied to backend via nginx
  if (!res.ok) throw new Error("Failed to get user data");
  const data = await res.json();

  return data;
}

export const updateDisplayName = async (display_name: string) => {
  const token = getToken();
  const userId = getCacheUserId();
  if (!token) throw new Error("No token found");

  const res = await fetchWithAuth(`/api/v2/users/${userId}/display_name`, {
    method: "PATCH",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ display_name }),
  });
  
  if (!res.ok) throw new Error("Failed to update display name");
  return res.json();
};

export const updateUsername = async (username: string) => {
  const token = getToken();
  const userId = getCacheUserId();
  if (!token) throw new Error("No token found");

  const res = await fetchWithAuth(`/api/v2/users/${userId}/username`, {
    method: "PATCH",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ username }),
  });

  const data = await res.json();

  if (!res.ok) {
    const errorMessage =
      data?.errors?.[0]?.message || "Failed to update Username";
    throw new Error(errorMessage);
  }

  return data;
};

export const updateProfilePicture = async (file: File) => {
  const token = getToken();
  const userId = getCacheUserId();
  if (!token) throw new Error("No token found");

  const formData = new FormData();
  formData.append("file", file); // key must match backend field name

  const res = await fetchWithAuth(`/api/v2/users/${userId}/profile_image`, {
    method: "PATCH",
    body: formData,
  });
  const data = await res.json();

  if (!res.ok) {
    const errorMessage =
      data?.errors?.[0]?.message || "Failed to update profile image";
    throw new Error(errorMessage);
  }

  return data;
};

export const requestMailVerification = async () => {
  const token = getToken();
  if (!token) throw new Error("No token found");

  const res = await fetchWithAuth(`/api/v2/auth/email`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
  })

  const data = await res.json();

  if (!res.ok) {
    const errorMessage =
      data?.errors?.[0]?.message || "Failed to request";
    throw new Error(errorMessage);
  }

  return data?.email_token
}

export const verifyEmail = async (email_token:string) => {
  const res = await fetch(`/api/v2/auth/email`, {
    method: "PATCH",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ email_token }),
  })

  return res.ok;
}

export const getSessions = async (): Promise<SessionResponse[]> =>{
  const user_id = getCacheUserId();
  const res = await fetchWithAuth( `/api/v2/users/${user_id}/session`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
    },
  })
  const data = await res.json()
  return data;
}

export const deleteSelectSession = async (session_id : string) => {
  const user_id = getCacheUserId();
  const res = await fetchWithAuth(`/api/v2/users/${user_id}/session`,{
    method: "DELETE",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({session_id, user_id}),
  })
  return res.ok
}

export const updatePassword = async (data: PasswordData) => {
    const user_id = getCacheUserId();
    const res = await fetchWithAuth( `/api/v2/users/${user_id}/password`, {
        method: "PATCH",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(data)
    })
    if (!res.ok) throw new Error("Change password failed")
    return res.ok
}