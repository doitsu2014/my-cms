import { useRef } from 'react';
import { toast } from 'sonner';
import {
  Bug,
  Key,
  RefreshCw,
  User,
  Clock,
  Database,
  UserCircle,
  ChevronDown,
} from 'lucide-react';
import { useAuth } from '@/auth/AuthContext';
import { getSupabaseClient } from '@/auth/supabase';

export default function SupabaseDebugMenu() {
  const { userInfo, authenticated, token, session } = useAuth();
  const detailsRef = useRef<HTMLDetailsElement>(null);

  const userName = userInfo?.name || userInfo?.username || 'Admin User';
  const userEmail = userInfo?.email || 'admin@example.com';

  const closeDropdown = () => {
    if (detailsRef.current) {
      detailsRef.current.open = false;
    }
  };

  const handleCheckAuth = () => {
    const supabase = getSupabaseClient();
    console.log('=== Authentication Status ===');
    console.log('Authenticated:', authenticated);
    supabase.auth.getSession().then(({ data }) => {
      console.log('Session:', data.session);
      toast.info(`Authenticated: ${authenticated} | Session: ${!!data.session}`);
    });
    closeDropdown();
  };

  const handleShowToken = () => {
    console.log('=== Token Information ===');
    console.log('Token:', token);
    if (session) {
      console.log('Token expires at:', session.expires_at
        ? new Date(session.expires_at * 1000).toISOString()
        : 'N/A');
      console.log('Refresh token available:', session.refresh_token ? 'YES' : 'NO');
    }
    toast.info(
      token
        ? `Token: ${token.substring(0, 30)}...`
        : 'No token available',
      { duration: 6000 }
    );
    closeDropdown();
  };

  const handleRefreshToken = async () => {
    try {
      console.log('=== Refreshing Token ===');
      const { data, error } = await getSupabaseClient().auth.refreshSession();
      console.log('Token refreshed:', !error);
      console.log('New session:', data.session);
      toast.success(
        error ? 'Token refresh failed' : 'Token refreshed successfully'
      );
    } catch (e) {
      console.error('Token refresh failed:', e);
      toast.error('Token refresh failed! Check console for details.');
    }
    closeDropdown();
  };

  const handleShowUserInfo = () => {
    console.log('=== User Information ===');
    console.log('User Info:', userInfo);
    console.log('Session user:', session?.user);
    toast.info(`User: ${userName} | Email: ${userEmail}`, { duration: 5000 });
    closeDropdown();
  };

  const handleCheckTokenValidity = () => {
    const supabase = getSupabaseClient();
    supabase.auth.getSession().then(({ data: { session: s } }) => {
      const isExpired = s?.expires_at ? s.expires_at < Math.floor(Date.now() / 1000) : null;
      console.log('=== Token Validity ===');
      console.log('Is token expired:', isExpired);
      console.log('Expires at:', s?.expires_at
        ? new Date(s.expires_at * 1000).toISOString()
        : 'N/A');
      toast.info(`Token expired: ${isExpired ?? 'unknown'}`);
    });
    closeDropdown();
  };

  const handleShowAllSupabaseInfo = () => {
    const supabase = getSupabaseClient();
    console.log('=== Complete Supabase Auth State ===');
    console.log('Session:', session);
    console.log('Authenticated:', authenticated);
    console.log('Token:', token);
    console.log('User Info:', userInfo);
    supabase.auth.getSession().then(({ data: { session: s } }) => {
      console.log('Live session:', s);
      console.log('User:', s?.user);
    });
    toast.info('Complete auth state logged to console. Open DevTools to view.', {
      duration: 5000,
    });
    closeDropdown();
  };

  const handleLoadUserProfile = async () => {
    try {
      console.log('=== Loading User Profile ===');
      const { data } = await getSupabaseClient().auth.getUser();
      console.log('User:', data.user);
      toast.success(`User: ${data.user?.email ?? 'unknown'}`);
    } catch (error) {
      console.error('Failed to load user:', error);
      toast.error('Failed to load user!');
    }
    closeDropdown();
  };

  return (
    <details ref={detailsRef} className="dropdown dropdown-end">
      <summary className="btn btn-ghost btn-sm gap-1 list-none">
        <Bug className="w-4 h-4 text-warning" />
        <span className="hidden sm:inline text-xs">Debug</span>
        <ChevronDown className="w-3 h-3" />
      </summary>
      <ul className="dropdown-content z-[1] menu p-2 shadow-lg bg-base-100 rounded-box w-56 border border-base-300">
        <li className="menu-title">
          <span>Supabase Debug</span>
        </li>
        <li>
          <button onClick={handleCheckAuth} className="flex items-center gap-2">
            <Key className="w-4 h-4" />
            Check Auth
          </button>
        </li>
        <li>
          <button onClick={handleShowToken} className="flex items-center gap-2">
            <Database className="w-4 h-4" />
            Show Token
          </button>
        </li>
        <li>
          <button onClick={handleRefreshToken} className="flex items-center gap-2">
            <RefreshCw className="w-4 h-4" />
            Refresh Token
          </button>
        </li>
        <li>
          <button onClick={handleShowUserInfo} className="flex items-center gap-2">
            <User className="w-4 h-4" />
            User Info
          </button>
        </li>
        <li>
          <button onClick={handleCheckTokenValidity} className="flex items-center gap-2">
            <Clock className="w-4 h-4" />
            Check Validity
          </button>
        </li>
        <li>
          <button onClick={handleLoadUserProfile} className="flex items-center gap-2">
            <UserCircle className="w-4 h-4" />
            Load User
          </button>
        </li>
        <div className="divider my-1"></div>
        <li>
          <button
            onClick={handleShowAllSupabaseInfo}
            className="flex items-center gap-2 text-accent"
          >
            <Database className="w-4 h-4" />
            Full State (Console)
          </button>
        </li>
      </ul>
    </details>
  );
}
