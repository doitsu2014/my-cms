import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { gql } from '@apollo/client';
import { buildGraphQLClient } from './graphql-client';
import type { RuntimeConfig } from '../../config/runtime-config';

const SAMPLE_CONFIG: RuntimeConfig = {
  siteName: 'Test',
  siteUrl: 'https://test.test',
  avatarUrl: 'https://test.test/a.png',
  defaultTitle: 'Test',
  defaultDescription: 'Test',
  defaultLocale: 'en',
  graphqlApiUrl: 'https://example.test/graphql',
  graphqlCacheApiUrl: 'https://example.test/graphql',
  mediaBaseUrl: 'https://api.test/media',
  port: '3001',
};

const SAMPLE_DOC = gql`
  query Sample {
    __typename
  }
`;

describe('buildGraphQLClient', () => {
  beforeEach(() => {
    // Inject the <script id="app-config"> tag into the document head so
    // readBrowserConfig() can read it.
    document.head.innerHTML = '';
    const script = document.createElement('script');
    script.id = 'app-config';
    script.type = 'application/json';
    script.textContent = JSON.stringify(SAMPLE_CONFIG);
    document.head.appendChild(script);
  });

  afterEach(() => {
    document.head.innerHTML = '';
    vi.restoreAllMocks();
  });

  it('issues Apollo queries to the URL provided in window.__APP_CONFIG__.graphqlApiUrl', async () => {
    const observed: string[] = [];

    // Spy on globalThis.fetch so HttpLink's outbound request is captured
    // before jsdom's fetch implementation is involved. Return a valid
    // GraphQL-shaped payload so Apollo's response parser does not throw.
    const fetchSpy = vi
      .spyOn(globalThis, 'fetch')
      .mockImplementation(async (input) => {
        const url =
          typeof input === 'string'
            ? input
            : input instanceof URL
              ? input.toString()
              : (input as Request).url;
        observed.push(url);
        return new Response(JSON.stringify({ data: { __typename: 'Query' } }), {
          status: 200,
          headers: { 'content-type': 'application/json' },
        });
      });

    const client = buildGraphQLClient();
    await client.query({ query: SAMPLE_DOC, fetchPolicy: 'network-only' });

    expect(observed).toEqual(['https://example.test/graphql']);

    fetchSpy.mockRestore();
  });

  it('does not issue queries to any other origin', async () => {
    const otherOrigins: string[] = [];

    const fetchSpy = vi
      .spyOn(globalThis, 'fetch')
      .mockImplementation(async (input) => {
        const url =
          typeof input === 'string'
            ? input
            : input instanceof URL
              ? input.toString()
              : (input as Request).url;
        try {
          const parsed = new URL(url);
          if (parsed.origin !== 'https://example.test') {
            otherOrigins.push(parsed.origin);
          }
        } catch {
          // ignore — non-absolute URIs are fine for the test
        }
        return new Response(JSON.stringify({ data: { __typename: 'Query' } }), {
          status: 200,
          headers: { 'content-type': 'application/json' },
        });
      });

    const client = buildGraphQLClient();
    await client.query({ query: SAMPLE_DOC });

    expect(otherOrigins).toEqual([]);

    fetchSpy.mockRestore();
  });
});
