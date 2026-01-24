/// <reference types="@rsbuild/core/types" />

/**
 * Imports the SVG file as a React component.
 * @requires [@rsbuild/plugin-svgr](https://npmjs.com/package/@rsbuild/plugin-svgr)
 */
declare module '*.svg?react' {
  import type React from 'react';
  const ReactComponent: React.FunctionComponent<React.SVGProps<SVGSVGElement>>;
  export default ReactComponent;
}

/**
 * Environment Variables Type Definitions
 *
 * These environment variables are injected at build time by Rsbuild.
 * See .env.example for documentation on each variable.
 */
interface ImportMetaEnv {
  // Keycloak Configuration
  readonly PUBLIC_KEYCLOAK_URL: string;
  readonly PUBLIC_KEYCLOAK_REALM: string;
  readonly PUBLIC_KEYCLOAK_CLIENT_ID: string;
  readonly PUBLIC_KEYCLOAK_SCOPE: string;

  // Backend API Configuration (my-cms)
  readonly PUBLIC_GRAPHQL_API_URL: string;
  readonly PUBLIC_GRAPHQL_CACHE_API_URL?: string;
  readonly PUBLIC_REST_API_URL: string;
  readonly PUBLIC_MEDIA_UPLOAD_API_URL: string;

  // Optional Configuration
  readonly HOME_PAGE_CACHE_ENABLED?: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
