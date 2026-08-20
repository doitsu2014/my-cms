// @vitest-environment node

import { afterEach, describe, expect, it } from 'vitest';
import { getServerFetch, withServerSpan } from './server-telemetry';

// @ts-expect-error The preload-only ESM seam intentionally has no browser-facing TypeScript entry.
const { boundedRoute, readWebsiteTelemetryConfig } = await import('../telemetry-config.mjs');

afterEach(() => {
  delete globalThis.__WEBSITE_OTEL_BRIDGE__;
});

describe('website telemetry configuration', () => {
  it.each([undefined, '', 'false', 'TRUE', '1'])(
    'fails open when enabled flag is %j',
    (enabled) => {
      expect(
        readWebsiteTelemetryConfig({ ENABLED_OTLP_EXPORTER: enabled }),
      ).toEqual({ enabled: false });
    },
  );

  it('rejects invalid exporter configuration without exposing values', () => {
    expect(
      readWebsiteTelemetryConfig({
        ENABLED_OTLP_EXPORTER: 'true',
        OTEL_EXPORTER_OTLP_TRACES_ENDPOINT: 'file:///secret-token',
        OTEL_SERVICE_NAME: 'secret-service',
      }),
    ).toEqual({ enabled: false, invalid: true });
  });

  it('accepts valid server-only configuration with the website identity', () => {
    expect(
      readWebsiteTelemetryConfig({
        ENABLED_OTLP_EXPORTER: 'true',
        OTEL_EXPORTER_OTLP_TRACES_ENDPOINT: 'http://jaeger:4318/v1/traces',
        OTEL_TRACES_SAMPLER: 'always_on',
      }),
    ).toEqual({
      enabled: true,
      endpoint: 'http://jaeger:4318/v1/traces',
      sampler: 'always_on',
      serviceName: 'ducth-dev-website',
    });
  });

  it('bounds route attributes and removes query values', () => {
    expect(boundedRoute('/en?authorization=secret')).toBe('/en');
    expect(boundedRoute(`${'/'.repeat(300)}secret`)).toHaveLength(256);
  });
});

describe('server telemetry boundary', () => {
  it('is a no-op when the Node preload is absent or disabled', async () => {
    let called = false;
    await withServerSpan(
      { method: 'GET', route: '/en', protocol: '1.1', headers: {} },
      async (span) => {
        called = true;
        span.setStatus(200);
      },
    );
    expect(called).toBe(true);
  });

  it('delegates only the explicit SSR and fetch boundaries to the Node bridge', async () => {
    const seen: string[] = [];
    globalThis.__WEBSITE_OTEL_BRIDGE__ = {
      runServerSpan: async (input, callback) => {
        seen.push(`${input.method} ${input.route}`);
        return callback({
          setStatus: () => undefined,
          setErrorCategory: () => undefined,
        });
      },
      tracedFetch: async (input) => {
        seen.push(String(input));
        return new Response('{}', { status: 200 });
      },
    };
    await withServerSpan(
      {
        method: 'GET',
        route: '/en',
        protocol: '1.1',
        headers: { cookie: 'secret' },
      },
      async () => undefined,
    );
    await getServerFetch()(
      'https://api.test/posts/graphql/immutable?token=secret',
      { method: 'POST', body: 'query secret' },
    );
    expect(seen).toEqual([
      'GET /en',
      'https://api.test/posts/graphql/immutable?token=secret',
    ]);
  });
});
