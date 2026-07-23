import {
  ApolloClient,
  InMemoryCache,
  createHttpLink,
  type NormalizedCacheObject,
} from '@apollo/client';
import { readBrowserConfig } from '../../config/read-browser-config';
import type { RuntimeConfig } from '../../config/runtime-config';

// Check if running in browser
const isBrowser = typeof window !== 'undefined';

/**
 * Resolve the GraphQL API URL from the runtime configuration.
 *
 * Server-side: uses the global `__WEBSITE_RUNTIME_CONFIG__` injected by the
 * SSR handler. Browser-side: reads `window.__APP_CONFIG__` via
 * `readBrowserConfig()`.
 */
function resolveGraphqlApiUrl(): string {
  if (isBrowser) {
    return readBrowserConfig().graphqlApiUrl;
  }
  // Server-side: the SSR handler provides the config via a global variable
  // (set up in index.server.tsx).
  const globalAny = globalThis as unknown as {
    __WEBSITE_RUNTIME_CONFIG__?: RuntimeConfig;
  };
  if (!globalAny.__WEBSITE_RUNTIME_CONFIG__) {
    throw new Error('graphql-client (server): __WEBSITE_RUNTIME_CONFIG__ is not set');
  }
  return globalAny.__WEBSITE_RUNTIME_CONFIG__.graphqlApiUrl;
}

/**
 * Build an Apollo GraphQL client for the my-cms backend.
 *
 * - On the server: creates a fresh client for each request. The client reads
 *   the GraphQL URL from the runtime config.
 * - On the client: restores the cache from SSR data if `initialState` is
 *   provided. The client reads the GraphQL URL from `window.__APP_CONFIG__`.
 *
 * The HTTP link URI is resolved via a function so the URL is materialised
 * lazily — after the SSR-injected `<script id="app-config">` is in the DOM
 * on the client, and after the per-request global is set on the server.
 */
export function buildGraphQLClient(initialState?: NormalizedCacheObject) {
  const cache = new InMemoryCache();

  // Restore cache from SSR data (client-side only)
  if (isBrowser && initialState) {
    cache.restore(initialState);
  }

  const httpLink = createHttpLink({
    uri: resolveGraphqlApiUrl,
  });

  return new ApolloClient({
    link: httpLink,
    cache,
    ssrMode: !isBrowser,
    defaultOptions: {
      watchQuery: {
        fetchPolicy: isBrowser ? 'cache-first' : 'network-only',
      },
      query: {
        fetchPolicy: isBrowser ? 'cache-first' : 'network-only',
      },
    },
  });
}

// Client-side: Restore from window.__APOLLO_STATE__ if available
const initialState = isBrowser
  ? (window as unknown as { __APOLLO_STATE__?: NormalizedCacheObject }).__APOLLO_STATE__
  : undefined;
export const graphqlClient = buildGraphQLClient(initialState);
