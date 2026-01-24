import keycloak from '@/auth/keycloak';

/**
 * Build headers for API requests with Keycloak authentication
 * @param includeAuthorizedToken - Whether to include the Bearer token
 * @param isContentTypeJson - Whether to set Content-Type to application/json
 */
export const buildHeader = async (includeAuthorizedToken: boolean = true, isContentTypeJson: boolean = true) => {
  let header: any = {};

  // Add Keycloak Bearer token if authenticated
  if (includeAuthorizedToken && keycloak.token) {
    header['Authorization'] = `Bearer ${keycloak.token}`;
  }

  if (isContentTypeJson) {
    header["Content-Type"] = 'application/json';
  }

  return header;
};

/**
 * Get authentication headers for API requests
 * Uses Keycloak token from the authenticated session
 */
export const getAuthHeaders = () => {
  if (!keycloak.token) {
    console.warn('No authentication token available');
    return { 'Content-Type': 'application/json' };
  }

  return {
    'Authorization': `Bearer ${keycloak.token}`,
    'Content-Type': 'application/json',
  };
};

/**
 * Check if user is authenticated
 */
export const isAuthenticated = (): boolean => {
  return keycloak.authenticated || false;
};

/**
 * Get current user token
 */
export const getToken = (): string | undefined => {
  return keycloak.token;
};

/**
 * Refresh the authentication token
 */
export const refreshToken = async (): Promise<boolean> => {
  try {
    const refreshed = await keycloak.updateToken(70);
    return refreshed;
  } catch (error) {
    console.error('Failed to refresh token:', error);
    return false;
  }
};

