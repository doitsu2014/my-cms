import type { RuntimeConfig } from './runtime-config';
import { readBrowserConfig } from './read-browser-config';

declare global {
  var __WEBSITE_RUNTIME_CONFIG__: RuntimeConfig | undefined;
}

const isBrowser = typeof document !== 'undefined';

/**
 * Resolve the runtime configuration for the current execution context.
 *
 * - **Server (SSR / Node):** returns the config injected by the SSR handler
 *   via `setRuntimeConfigForServer` (stored on `globalThis.__WEBSITE_RUNTIME_CONFIG__`).
 *   The function never reads `document` on the server, so it is safe for
 *   server-side rendering. Throws a clear error when called on the server
 *   without an injected config so accidental browser-only access is caught
 *   loudly instead of silently producing placeholder HTML.
 * - **Browser:** reads the JSON config from the `<script id="app-config">`
 *   element injected by the SSR handler.
 *
 * All `SITE_CONFIG` getters and the page thumbnail helpers should route
 * their access through this function so that SSR never touches `document`.
 */
export function getRuntimeConfig(): RuntimeConfig {
  if (isBrowser) {
    return readBrowserConfig();
  }
  const serverConfig = globalThis.__WEBSITE_RUNTIME_CONFIG__;
  if (!serverConfig) {
    throw new Error(
      'getRuntimeConfig: server runtime config is not set. The SSR handler must call setRuntimeConfigForServer() before rendering.',
    );
  }
  return serverConfig;
}

/**
 * Inject the server-side runtime configuration. Called once per request from
 * `index.server.tsx` after `resolveRuntimeConfig(process.env)` so the React
 * tree and any code reachable from `getRuntimeConfig()` see the validated
 * env-backed config without re-reading `document`.
 */
export function setRuntimeConfigForServer(config: RuntimeConfig): void {
  globalThis.__WEBSITE_RUNTIME_CONFIG__ = config;
}
