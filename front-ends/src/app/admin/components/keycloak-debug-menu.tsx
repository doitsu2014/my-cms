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

export default function KeycloakDebugMenu() {
  const { userInfo, authenticated, token, keycloak } = useAuth();
  const detailsRef = useRef<HTMLDetailsElement>(null);

  const userName = userInfo?.name || userInfo?.username || 'Admin User';
  const userEmail = userInfo?.email || 'admin@example.com';

  const closeDropdown = () => {
    if (detailsRef.current) {
      detailsRef.current.open = false;
    }
  };

  const handleCheckAuth = () => {
    console.log('=== Authentication Status ===');
    console.log('Authenticated:', authenticated);
    console.log('Keycloak authenticated:', keycloak?.authenticated);
    toast.info(`Authenticated: ${authenticated}`);
    closeDropdown();
  };

  const handleShowToken = () => {
    console.log('=== Token Information ===');
    console.log('Token:', token);
    console.log('Token parsed:', keycloak?.tokenParsed);
    console.log(
      'Token expires in (seconds):',
      keycloak?.tokenParsed?.exp
        ? keycloak.tokenParsed.exp - Math.floor(Date.now() / 1000)
        : 'N/A'
    );
    console.log('Refresh token available:', keycloak?.refreshToken ? 'YES' : 'NO');

    const hasRefreshToken = keycloak?.refreshToken ? 'YES' : 'NO';
    toast.info(
      token
        ? `Token: ${token.substring(0, 30)}... | Refresh: ${hasRefreshToken}`
        : 'No token available',
      { duration: 6000 }
    );
    closeDropdown();
  };

  const handleRefreshToken = async () => {
    try {
      console.log('=== Refreshing Token ===');
      const refreshed = await keycloak?.updateToken(70);
      console.log('Token refreshed:', refreshed);
      console.log('New token:', keycloak?.token);
      toast.success(
        refreshed ? 'Token refreshed successfully' : 'Token still valid (no refresh needed)'
      );
    } catch (error) {
      console.error('Token refresh failed:', error);
      toast.error('Token refresh failed! Check console for details.');
    }
    closeDropdown();
  };

  const handleShowUserInfo = () => {
    console.log('=== User Information ===');
    console.log('User Info:', userInfo);
    console.log('Keycloak subject:', keycloak?.subject);
    console.log('Keycloak realm access:', keycloak?.realmAccess);
    console.log('Keycloak resource access:', keycloak?.resourceAccess);
    toast.info(`User: ${userName} | Email: ${userEmail}`, { duration: 5000 });
    closeDropdown();
  };

  const handleCheckTokenValidity = () => {
    const isExpired = keycloak?.isTokenExpired();
    const timeSkew = keycloak?.timeSkew;
    console.log('=== Token Validity ===');
    console.log('Is token expired:', isExpired);
    console.log('Time skew:', timeSkew);
    console.log('Token parsed:', keycloak?.tokenParsed);
    toast.info(`Token expired: ${isExpired} | Time skew: ${timeSkew}s`);
    closeDropdown();
  };

  const handleShowAllKeycloakInfo = () => {
    console.log('=== Complete Keycloak State ===');
    console.log('Keycloak instance:', keycloak);
    console.log('Authenticated:', authenticated);
    console.log('Token:', token);
    console.log('User Info:', userInfo);
    console.log('Token Parsed:', keycloak?.tokenParsed);
    console.log('Refresh Token:', keycloak?.refreshToken);
    console.log('ID Token:', keycloak?.idToken);
    console.log('ID Token Parsed:', keycloak?.idTokenParsed);
    toast.info('Complete Keycloak state logged to console. Open DevTools to view.', {
      duration: 5000,
    });
    closeDropdown();
  };

  const handleLoadUserProfile = async () => {
    try {
      console.log('=== Loading User Profile ===');
      const profile = await keycloak?.loadUserProfile();
      console.log('User profile:', profile);
      toast.success(`Profile: ${profile?.username} (${profile?.email})`);
    } catch (error) {
      console.error('Failed to load user profile:', error);
      toast.error('Failed to load user profile!');
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
          <span>Keycloak Debug</span>
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
            Load Profile
          </button>
        </li>
        <div className="divider my-1"></div>
        <li>
          <button
            onClick={handleShowAllKeycloakInfo}
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
