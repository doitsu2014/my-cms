import { renderToString } from 'react-dom/server';
import { StaticRouter } from 'react-router-dom';
import { ApolloProvider } from '@apollo/client';
import { getDataFromTree } from '@apollo/client/react/ssr';
import { buildGraphQLClient } from './infrastructure/graphql/graphql-client';
import AppContent from './AppContent';
import { resolveRuntimeConfig } from './config/validate-env';
import type { RuntimeConfig } from './config/runtime-config';
import './App.css';
import './i18n/i18n';

export default async function render(url: string): Promise<{ html: string; apolloState: object }> {
  const config = resolveRuntimeConfig(process.env);
  (globalThis as typeof globalThis & { __WEBSITE_RUNTIME_CONFIG__?: RuntimeConfig }).__WEBSITE_RUNTIME_CONFIG__ = config;
  const client = buildGraphQLClient();
  const app = (
    <ApolloProvider client={client}>
      <StaticRouter location={url}>
        <AppContent />
      </StaticRouter>
    </ApolloProvider>
  );
  await getDataFromTree(app);
  return { html: renderToString(app), apolloState: client.extract() };
}
