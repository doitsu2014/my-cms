/**
 * API Configuration
 *
 * This file centralizes all API endpoint configurations using runtime config.
 * All API calls should use these helper functions instead of hardcoded URLs.
 */

import { config } from './runtime-config';

/**
 * Get the REST API base URL from runtime config
 * Falls back to relative path if not configured
 */
export const getRestApiBaseUrl = (): string => {
  return config().restApiUrl || '';
};

/**
 * Get the GraphQL API URL from runtime config
 */
export const getGraphQLApiUrl = (): string => {
  return config().graphqlApiUrl || 'http://localhost:8989/graphql';
};

/**
 * Get the Media Upload API URL from runtime config
 */
export const getMediaUploadApiUrl = (): string => {
  const baseUrl = getRestApiBaseUrl();
  if (config().mediaUploadApiUrl) {
    return config().mediaUploadApiUrl!;
  }
  return baseUrl ? `${baseUrl.replace(/\/$/, '')}/media/images` : '/media/images';
};

/**
 * Helper function to construct full API URL
 * @param path - API endpoint path (e.g., '/admin/blogs' or 'admin/blogs')
 * @returns Full API URL
 */
export const getApiUrl = (path: string): string => {
  const baseUrl = getRestApiBaseUrl();
  const cleanPath = path.startsWith('/') ? path : `/${path}`;

  // If baseUrl is empty, return the path as-is (relative)
  if (!baseUrl) {
    return `/api${cleanPath}`;
  }

  // Remove trailing slash from baseUrl and ensure path starts with /
  return `${baseUrl.replace(/\/$/, '')}${cleanPath}`;
};

/**
 * Create headers with authentication token
 * @param token - JWT access token
 * @param additionalHeaders - Additional headers to include
 * @returns Headers object with Authorization
 */
export const createAuthHeaders = (
  token: string | null,
  additionalHeaders?: HeadersInit
): HeadersInit => {
  const headers: Record<string, string> = {
    ...(additionalHeaders as Record<string, string>),
  };

  if (token) {
    headers['Authorization'] = `Bearer ${token}`;
  }

  return headers;
};

/**
 * Authenticated fetch wrapper
 * Automatically includes Authorization header with Bearer token
 * Token auto-refresh is handled by Supabase client internally
 *
 * @param url - API endpoint URL
 * @param token - JWT access token
 * @param options - Fetch options (method, body, headers, etc.)
 * @returns Promise<Response>
 *
 * @example
 * const response = await authenticatedFetch(
 *   getApiUrl('/posts?categoryType=Blog'),
 *   token,
 *   { method: 'GET' }
 * );
 */
export const authenticatedFetch = async (
  url: string,
  token: string | null,
  options?: RequestInit,
  _keycloak?: unknown
): Promise<Response> => {
  const headers = createAuthHeaders(token, options?.headers);

  return fetch(url, {
    ...options,
    headers,
  });
};

/**
 * Get the Media Image URL from a path
 * @param path - The image path returned from upload API
 * @returns Full URL to access the image via imgproxy
 */
export const getMediaImageUrl = (path: string): string => {
  const baseUrl = getRestApiBaseUrl();
  const cleanPath = path.startsWith('/') ? path.slice(1) : path;
  return baseUrl ? `${baseUrl.replace(/\/$/, '')}/media/images/${cleanPath}` : `/media/images/${cleanPath}`;
};

/**
 * API Configuration object for easy access
 */
export const API_CONFIG = {
  REST_BASE: getRestApiBaseUrl(),
  GRAPHQL: getGraphQLApiUrl(),
  MEDIA_UPLOAD: getMediaUploadApiUrl(),
} as const;
