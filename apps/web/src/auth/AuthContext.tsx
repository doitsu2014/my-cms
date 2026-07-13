import React, { createContext, useContext, useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import type { Session, User } from "@supabase/supabase-js";
import { getSupabaseClient } from "./supabase";

interface AuthContextType {
  user: User | null;
  session: Session | null;
  isLoading: boolean;
  getAccessToken: () => Promise<string | null>;
  signOut: () => Promise<void>;

  authenticated: boolean;
  loading: boolean;
  token: string | null;
  userInfo: {
    id?: string;
    name?: string;
    email?: string;
    username?: string;
    picture?: string;
  } | null;
  login: () => void;
  logout: () => Promise<void>;
}

const AuthContext = createContext<AuthContextType | null>(null);

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const navigate = useNavigate();
  const [user, setUser] = useState<User | null>(null);
  const [session, setSession] = useState<Session | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [token, setToken] = useState<string | null>(null);

  useEffect(() => {
    const supabase = getSupabaseClient();

    supabase.auth.getSession().then(({ data: { session } }) => {
      setSession(session);
      setUser(session?.user ?? null);
      setToken(session?.access_token ?? null);
      setIsLoading(false);
    });

    const { data: authListener } = supabase.auth.onAuthStateChange(
      (_event, session) => {
        setSession(session);
        setUser(session?.user ?? null);
        setToken(session?.access_token ?? null);
      }
    );

    return () => authListener.subscription.unsubscribe();
  }, []);

  const getAccessToken = async () => {
    const { data } = await getSupabaseClient().auth.getSession();
    return data.session?.access_token ?? null;
  };

  const signOut = async () => {
    await getSupabaseClient().auth.signOut();
  };

  const authenticated = !!user;

  const userInfo: AuthContextType["userInfo"] = user
    ? {
        id: user.id,
        name: user.user_metadata?.name ?? user.user_metadata?.full_name ?? undefined,
        email: user.email,
        username: user.user_metadata?.preferred_username ?? user.user_metadata?.username ?? undefined,
        picture: user.user_metadata?.avatar_url ?? user.user_metadata?.picture ?? undefined,
      }
    : null;

  const login = () => {
    navigate("/admin/login");
  };

  const logout = async () => {
    await signOut();
  };

  if (isLoading) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <div className="text-center">
          <div className="loading loading-spinner loading-lg"></div>
          <p className="mt-4 text-gray-600">Authenticating...</p>
        </div>
      </div>
    );
  }

  return (
    <AuthContext.Provider
      value={{
        user,
        session,
        isLoading,
        getAccessToken,
        signOut,
        authenticated,
        loading: isLoading,
        token,
        userInfo,
        login,
        logout,
      }}
    >
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const ctx = useContext(AuthContext);
  if (!ctx) throw new Error("useAuth must be used within AuthProvider");
  return ctx;
}
