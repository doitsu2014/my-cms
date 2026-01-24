import React, { type ReactNode, useEffect, useRef } from 'react';
import { useAuth } from './AuthContext';

interface ProtectedRouteProps {
  children: ReactNode;
}

export const ProtectedRoute: React.FC<ProtectedRouteProps> = ({ children }) => {
  const { authenticated, loading, login } = useAuth();
  const loginTriggered = useRef(false);

  // Effect to trigger login only once when needed
  useEffect(() => {
    // Check if we're in the middle of processing an OAuth callback
    const hash = window.location.hash;
    const isProcessingCallback = hash && (
      hash.includes('state=') || 
      hash.includes('code=') || 
      hash.includes('session_state=')
    );

    // Check sessionStorage to see if we recently triggered a login
    const lastLoginAttempt = sessionStorage.getItem('keycloak_login_attempt');
    const now = Date.now();
    const recentLoginAttempt = lastLoginAttempt && (now - parseInt(lastLoginAttempt)) < 30000; // 30 seconds

    // If not authenticated, not loading, not processing callback, and haven't recently triggered login
    if (!authenticated && !loading && !isProcessingCallback && !recentLoginAttempt && !loginTriggered.current) {
      loginTriggered.current = true;
      // Store timestamp to prevent multiple login attempts
      sessionStorage.setItem('keycloak_login_attempt', now.toString());
      // Trigger login redirect
      login();
    }
  }, [authenticated, loading, login]);

  // Clean up sessionStorage flag when authenticated
  useEffect(() => {
    if (authenticated) {
      sessionStorage.removeItem('keycloak_login_attempt');
      loginTriggered.current = false;
    }
  }, [authenticated]);

  if (loading) {
    return (
      <div className="flex items-center justify-center min-h-screen">
        <div className="text-center">
          <div className="loading loading-spinner loading-lg"></div>
          <p className="mt-4 text-gray-600">Loading...</p>
        </div>
      </div>
    );
  }

  if (!authenticated) {
    // Check if we're in the middle of processing an OAuth callback
    const hash = window.location.hash;
    const isProcessingCallback = hash && (
      hash.includes('state=') || 
      hash.includes('code=') || 
      hash.includes('session_state=')
    );

    if (isProcessingCallback) {
      // Don't trigger login again, Keycloak is processing the callback
      return (
        <div className="flex items-center justify-center min-h-screen">
          <div className="text-center">
            <div className="loading loading-spinner loading-lg"></div>
            <p className="mt-4 text-gray-600">Completing authentication...</p>
          </div>
        </div>
      );
    }

    // Show redirecting message while login is being triggered
    return (
      <div className="flex items-center justify-center min-h-screen">
        <div className="text-center">
          <div className="loading loading-spinner loading-lg"></div>
          <p className="mt-4 text-gray-600">Redirecting to login...</p>
        </div>
      </div>
    );
  }

  return <>{children}</>;
};
