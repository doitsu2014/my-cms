import { createClient } from "@supabase/supabase-js";
import { config } from "../config/runtime-config";

let supabaseClient: ReturnType<typeof createClient> | null = null;

export function getSupabaseClient() {
  if (supabaseClient) return supabaseClient;

  const cfg = config();
  supabaseClient = createClient(cfg.supabaseUrl, cfg.supabaseAnonKey, {
    auth: {
      autoRefreshToken: true,
      persistSession: true,
      detectSessionInUrl: true,
    },
  });
  return supabaseClient;
}
