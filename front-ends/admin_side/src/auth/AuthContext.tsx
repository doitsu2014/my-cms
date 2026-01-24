import React, { createContext, useContext, useEffect, useState, type ReactNode } from 'react';
import keycloak from './keycloak';
import type Keycloak from 'keycloak-js';
import { config } from '../config/runtime-config';

interface AuthContextType {
  keycloak: Keycloak | null;
  authenticated: boolean;
  loading: boolean;
  token: string | null;
  userInfo: {
    name?: string;
    email?: string;
    username?: string;
    picture?: string;
  } | null;
  login: () => void;
  logout: () => void;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

interface AuthProviderProps {
  children: ReactNode;
}

// Track if Keycloak has been initialized to prevent multiple initializations
let keycloakInitialized = false;
let tokenRefreshInterval: NodeJS.Timeout | null = null;

export const AuthProvider: React.FC<AuthProviderProps> = ({ children }) => {
  const [authenticated, setAuthenticated] = useState(false);
  const [loading, setLoading] = useState(true);
  const [token, setToken] = useState<string | null>(null);
  const [userInfo, setUserInfo] = useState<AuthContextType['userInfo']>(null);
  const initRef = React.useRef(false);

  useEffect(() => {
    // Prevent double initialization (React Strict Mode in dev, or component re-mounting)
    if (initRef.current || keycloakInitialized) {
      return;
    }
    
    initRef.current = true;
    keycloakInitialized = true;
    
    // Initialize Keycloak with PKCE and custom scope
    // IMPORTANT: Include 'offline_access' to get refresh tokens
    const baseScope = config().keycloakScope || 'my-headless-cms-api-all email openid profile';
    const scope = baseScope.includes('offline_access') ? baseScope : `${baseScope} offline_access`;

    keycloak
      .init({
        onLoad: 'check-sso', // Check SSO silently
        pkceMethod: 'S256', // Use PKCE with SHA-256
        checkLoginIframe: false, // Disable iframe for better performance and compatibility
        scope: scope, // Include custom CMS API scope + offline_access for refresh tokens
        // Enable redirect mode for better handling of OAuth callbacks
        flow: 'standard',
        // Clean up URL after successful authentication
        enableLogging: false,
        // Enable token persistence in localStorage to survive page refreshes
        // This stores the token, refresh token, and ID token in browser storage
        // Critical for maintaining user session across page reloads
        token: localStorage.getItem('kc_token') || undefined,
        refreshToken: localStorage.getItem('kc_refreshToken') || undefined,
        idToken: localStorage.getItem('kc_idToken') || undefined,
      })
      .then((auth) => {
        setAuthenticated(auth);
        setLoading(false);

        if (auth && keycloak.token) {
          setToken(keycloak.token);

          // Persist tokens to localStorage for session continuity across page reloads
          if (keycloak.token) {
            localStorage.setItem('kc_token', keycloak.token);
          }
          if (keycloak.refreshToken) {
            localStorage.setItem('kc_refreshToken', keycloak.refreshToken);
          }
          if (keycloak.idToken) {
            localStorage.setItem('kc_idToken', keycloak.idToken);
          }

          // Clean up URL hash after successful authentication
          // Add a small delay to ensure Keycloak has finished processing
          setTimeout(() => {
            if (window.location.hash && (
              window.location.hash.includes('state=') ||
              window.location.hash.includes('code=') ||
              window.location.hash.includes('session_state=')
            )) {
              // Use history API to remove hash without triggering navigation
              window.history.replaceState(null, '', window.location.pathname + window.location.search);
            }
          }, 100);

          // Load user info from token claims
          const tokenParsed = keycloak.idTokenParsed as any;
          setUserInfo({
            name: tokenParsed?.name,
            email: tokenParsed?.email,
            username: tokenParsed?.preferred_username,
            picture: tokenParsed?.avatar,
          });

          // Setup token refresh (clear any existing interval first)
          if (tokenRefreshInterval) {
            clearInterval(tokenRefreshInterval);
          }

          tokenRefreshInterval = setInterval(() => {
            keycloak.updateToken(70).then((refreshed) => {
              if (refreshed && keycloak.token) {
                setToken(keycloak.token);

                // Update localStorage with refreshed tokens
                if (keycloak.token) {
                  localStorage.setItem('kc_token', keycloak.token);
                }
                if (keycloak.refreshToken) {
                  localStorage.setItem('kc_refreshToken', keycloak.refreshToken);
                }
                if (keycloak.idToken) {
                  localStorage.setItem('kc_idToken', keycloak.idToken);
                }

                console.log('Token refreshed');
              }
            }).catch(() => {
              console.error('Failed to refresh token');
              // Clear tokens from localStorage on refresh failure
              localStorage.removeItem('kc_token');
              localStorage.removeItem('kc_refreshToken');
              localStorage.removeItem('kc_idToken');
              keycloak.logout();
            });
          }, 60 * 1000); // Check every minute
        }
      })
      .catch((error) => {
        console.error('Keycloak initialization failed:', error);
        setLoading(false);
      });
    
    // Cleanup function
    return () => {
      if (tokenRefreshInterval) {
        clearInterval(tokenRefreshInterval);
        tokenRefreshInterval = null;
      }
    };
  }, []);

  const login = () => {
    // Ensure we request offline_access scope for refresh tokens
    const baseScope = config().keycloakScope || 'my-headless-cms-api-all email openid profile';
    const scope = baseScope.includes('offline_access') ? baseScope : `${baseScope} offline_access`;

    keycloak.login({
      scope: scope,
    });
  };

  const logout = () => {
    // Clear tokens from localStorage before logging out
    localStorage.removeItem('kc_token');
    localStorage.removeItem('kc_refreshToken');
    localStorage.removeItem('kc_idToken');

    keycloak.logout({
      redirectUri: window.location.origin,
    });
  };

  const value: AuthContextType = {
    keycloak,
    authenticated,
    loading,
    token,
    userInfo,
    login,
    logout,
  };

  if (loading) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <div className="text-center">
          <div className="loading loading-spinner loading-lg"></div>
          <p className="mt-4 text-gray-600">Authenticating...</p>
        </div>
      </div>
    );
  }

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
};

export const useAuth = (): AuthContextType => {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
};
