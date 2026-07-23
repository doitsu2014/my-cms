import type { RuntimeConfig } from './runtime-config';

/**
 * Validate a required non-URL environment variable.
 *
 * - Throws an Error whose message includes the variable name when the value is
 *   missing (undefined or empty string).
 * - Returns the trimmed value when the string is non-empty.
 *
 * Used for required env vars that are not URLs (e.g. WEBSITE_SITE_NAME,
 * WEBSITE_DEFAULT_TITLE, WEBSITE_DEFAULT_DESCRIPTION).
 */
export function parseRequiredString(name: string, raw: string | undefined): string {
  if (raw === undefined || raw === '') {
    throw new Error(`${name} is required`);
  }
  return raw.trim();
}

/**
 * Validate and parse a required environment variable that must be a valid URL.
 *
 * - Throws an Error whose message includes the variable name when the value is
 *   missing (undefined or empty string).
 * - Throws an Error whose message includes the variable name and the first 64
 *   characters of the offending value when the URL is malformed.
 * - Returns the trimmed value when the URL is valid.
 *
 * @param name  The env variable name (e.g. "WEBSITE_PUBLIC_GRAPHQL_API_URL").
 * @param raw   The raw env value (typically `process.env[name]`).
 */
export function parseRequiredEnv(name: string, raw: string | undefined): string {
  if (raw === undefined || raw === '') {
    throw new Error(`${name} is required`);
  }

  const trimmed = raw.trim();

  try {
    // new URL() throws TypeError on invalid URLs
    new URL(trimmed);
  } catch {
    const truncated = trimmed.slice(0, 64);
    throw new Error(`${name} is not a valid URL: ${truncated}`);
  }

  return trimmed;
}

/**
 * Resolve the validated runtime configuration from `process.env`.
 *
 * Required variables (must be valid URLs / non-empty strings):
 * - WEBSITE_PUBLIC_GRAPHQL_API_URL (URL)
 * - WEBSITE_PUBLIC_MEDIA_BASE_URL (URL)
 * - WEBSITE_SITE_URL (URL)
 * - WEBSITE_SITE_NAME (string)
 * - WEBSITE_DEFAULT_TITLE (string)
 * - WEBSITE_DEFAULT_DESCRIPTION (string)
 *
 * Optional variables (with documented defaults):
 * - WEBSITE_PUBLIC_GRAPHQL_CACHE_API_URL → defaults to WEBSITE_PUBLIC_GRAPHQL_API_URL
 * - WEBSITE_AVATAR_URL → no default; when unset, the returned RuntimeConfig omits avatarUrl
 * - WEBSITE_DEFAULT_LOCALE → defaults to "en"
 * - WEBSITE_PORT → defaults to "3001"
 *
 * Throws on the first missing or invalid required value.
 */
export function resolveRuntimeConfig(env: NodeJS.ProcessEnv): RuntimeConfig {
  const graphqlApiUrl = parseRequiredEnv(
    'WEBSITE_PUBLIC_GRAPHQL_API_URL',
    env.WEBSITE_PUBLIC_GRAPHQL_API_URL,
  );
  const mediaBaseUrl = parseRequiredEnv(
    'WEBSITE_PUBLIC_MEDIA_BASE_URL',
    env.WEBSITE_PUBLIC_MEDIA_BASE_URL,
  );
  const siteUrl = parseRequiredEnv('WEBSITE_SITE_URL', env.WEBSITE_SITE_URL);
  const siteName = parseRequiredString('WEBSITE_SITE_NAME', env.WEBSITE_SITE_NAME);
  const defaultTitle = parseRequiredString(
    'WEBSITE_DEFAULT_TITLE',
    env.WEBSITE_DEFAULT_TITLE,
  );
  const defaultDescription = parseRequiredString(
    'WEBSITE_DEFAULT_DESCRIPTION',
    env.WEBSITE_DEFAULT_DESCRIPTION,
  );

  // Optional: WEBSITE_PUBLIC_GRAPHQL_CACHE_API_URL falls back to WEBSITE_PUBLIC_GRAPHQL_API_URL
  const graphqlCacheApiUrl = env.WEBSITE_PUBLIC_GRAPHQL_CACHE_API_URL
    ? env.WEBSITE_PUBLIC_GRAPHQL_CACHE_API_URL.trim()
    : graphqlApiUrl;

  // Optional: WEBSITE_AVATAR_URL returns undefined when unset
  const avatarUrl = env.WEBSITE_AVATAR_URL ? env.WEBSITE_AVATAR_URL.trim() : undefined;

  const defaultLocale = env.WEBSITE_DEFAULT_LOCALE
    ? env.WEBSITE_DEFAULT_LOCALE.trim()
    : 'en';

  const port = env.WEBSITE_PORT ? env.WEBSITE_PORT.trim() : '3001';

  return {
    siteName,
    siteUrl,
    avatarUrl,
    defaultTitle,
    defaultDescription,
    defaultLocale,
    graphqlApiUrl,
    graphqlCacheApiUrl,
    mediaBaseUrl,
    port,
  };
}
