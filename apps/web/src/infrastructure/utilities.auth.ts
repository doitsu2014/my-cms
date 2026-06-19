import { getSupabaseClient } from "../auth/supabase";

export async function getAuthorizationHeader(): Promise<{
  Authorization: string;
} | null> {
  const { data } = await getSupabaseClient().auth.getSession();
  const token = data.session?.access_token;
  if (!token) return null;
  return { Authorization: `Bearer ${token}` };
}

export const getAuthHeaders = () => {
  const supabase = getSupabaseClient();

  return supabase.auth.getSession().then(({ data }) => {
    const token = data.session?.access_token;
    if (!token) {
      console.warn("No authentication token available");
      return { "Content-Type": "application/json" };
    }
    return {
      Authorization: `Bearer ${token}`,
      "Content-Type": "application/json",
    };
  });
};

export const buildHeader = async (
  includeAuthorizedToken: boolean = true,
  isContentTypeJson: boolean = true
) => {
  const header: Record<string, string> = {};

  if (includeAuthorizedToken) {
    const { data } = await getSupabaseClient().auth.getSession();
    const token = data.session?.access_token;
    if (token) {
      header["Authorization"] = `Bearer ${token}`;
    }
  }

  if (isContentTypeJson) {
    header["Content-Type"] = "application/json";
  }

  return header;
};

export const isAuthenticated = async (): Promise<boolean> => {
  const { data } = await getSupabaseClient().auth.getSession();
  return !!data.session;
};

export const getToken = async (): Promise<string | undefined> => {
  const { data } = await getSupabaseClient().auth.getSession();
  return data.session?.access_token;
};

export const refreshToken = async (): Promise<boolean> => {
  try {
    const { data } = await getSupabaseClient().auth.refreshSession();
    return !!data.session;
  } catch (error) {
    console.error("Failed to refresh token:", error);
    return false;
  }
};
