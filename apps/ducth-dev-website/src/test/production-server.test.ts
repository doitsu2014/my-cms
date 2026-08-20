// @vitest-environment node

import { once } from 'node:events';
import { createServer, type Server } from 'node:http';
import fs from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawn, type ChildProcess } from 'node:child_process';
import { afterEach, describe, expect, it } from 'vitest';

const websiteRoot = fileURLToPath(new URL('../..', import.meta.url));
const processes: ChildProcess[] = [];

const wait = (milliseconds: number) =>
  new Promise((resolve) => setTimeout(resolve, milliseconds));

async function listen(server: Server): Promise<number> {
  server.listen(0, '127.0.0.1');
  await once(server, 'listening');
  const address = server.address();
  if (!address || typeof address === 'string') {
    throw new Error('Test server did not bind a TCP port');
  }
  return address.port;
}

function closeServer(server: Server): Promise<void> {
  return new Promise((resolve, reject) =>
    server.close((error) => (error ? reject(error) : resolve())),
  );
}

async function getAvailablePort(): Promise<number> {
  const server = createServer();
  const port = await listen(server);
  await closeServer(server);
  return port;
}

async function waitForResponse(
  url: string,
  init?: RequestInit,
): Promise<Response> {
  let lastError: unknown;
  for (let attempt = 0; attempt < 50; attempt += 1) {
    try {
      return await fetch(url, init);
    } catch (error) {
      lastError = error;
      await wait(50);
    }
  }
  throw lastError;
}

async function stopProcess(process: ChildProcess): Promise<void> {
  if (process.exitCode !== null) return;
  process.kill('SIGTERM');
  await Promise.race([
    once(process, 'exit'),
    wait(2_000).then(() => {
      if (process.exitCode === null) process.kill('SIGKILL');
    }),
  ]);
}

afterEach(async () => {
  await Promise.all(processes.splice(0).map(stopProcess));
});

function startProductionServer(
  port: number,
  graphqlApiUrl: string,
  options: { enabled?: boolean; otlpEndpoint?: string } = {},
): ChildProcess {
  const child = spawn(
    process.execPath,
    ['--import', './instrumentation.mjs', 'server.prod.mjs'],
    {
      cwd: websiteRoot,
      env: {
        ...process.env,
        NODE_ENV: 'production',
        WEBSITE_PORT: String(port),
        WEBSITE_SITE_NAME: 'Test website',
        WEBSITE_SITE_URL: 'https://website.test',
        WEBSITE_DEFAULT_TITLE: 'Test title',
        WEBSITE_DEFAULT_DESCRIPTION: 'Test description',
        WEBSITE_PUBLIC_GRAPHQL_API_URL: graphqlApiUrl,
        WEBSITE_PUBLIC_MEDIA_BASE_URL: 'https://media.test',
        ENABLED_OTLP_EXPORTER: options.enabled ? 'true' : 'false',
        OTEL_SERVICE_NAME: 'ducth-dev-website',
        OTEL_EXPORTER_OTLP_TRACES_ENDPOINT:
          options.otlpEndpoint || 'http://127.0.0.1:9/v1/traces',
        OTEL_TRACES_SAMPLER: 'always_on',
      },
      stdio: 'pipe',
    },
  );
  processes.push(child);
  return child;
}

describe('production website server', () => {
  it('serves /healthz without invoking the SSR GraphQL dependency path', async () => {
    let graphqlRequests = 0;
    const graphqlServer = createServer((_request, response) => {
      graphqlRequests += 1;
      response.writeHead(200, { 'content-type': 'application/json' });
      response.end(JSON.stringify({ data: {} }));
    });
    const graphqlPort = await listen(graphqlServer);
    const websitePort = await getAvailablePort();

    try {
      startProductionServer(
        websitePort,
        `http://127.0.0.1:${graphqlPort}/posts/graphql/immutable`,
      );

      const response = await waitForResponse(
        `http://127.0.0.1:${websitePort}/healthz`,
      );

      expect(response.status).toBe(200);
      expect(graphqlRequests).toBe(0);
    } finally {
      await closeServer(graphqlServer);
    }
  });

  it('keeps the /en reader route SSR-backed', async () => {
    let graphqlRequests = 0;
    const graphqlServer = createServer((_request, response) => {
      graphqlRequests += 1;
      response.writeHead(200, { 'content-type': 'application/json' });
      response.end(JSON.stringify({ data: {} }));
    });
    const graphqlPort = await listen(graphqlServer);
    const websitePort = await getAvailablePort();

    try {
      startProductionServer(
        websitePort,
        `http://127.0.0.1:${graphqlPort}/posts/graphql/immutable`,
      );

      const response = await waitForResponse(
        `http://127.0.0.1:${websitePort}/en`,
      );

      expect(response.status).toBe(200);
      expect(graphqlRequests).toBeGreaterThan(0);
    } finally {
      await closeServer(graphqlServer);
    }
  });

  it('propagates a traceparent from an enabled SSR request to GraphQL', async () => {
    let graphQlTraceparent;
    const graphqlServer = createServer((_request, response) => {
      graphQlTraceparent = _request.headers.traceparent;
      response.writeHead(200, { 'content-type': 'application/json' });
      response.end(JSON.stringify({ data: {} }));
    });
    const graphqlPort = await listen(graphqlServer);
    const websitePort = await getAvailablePort();
    try {
      startProductionServer(
        websitePort,
        `http://127.0.0.1:${graphqlPort}/posts/graphql/immutable`,
        { enabled: true },
      );
      const response = await waitForResponse(
        `http://127.0.0.1:${websitePort}/en`,
      );
      expect(response.status).toBe(200);
      expect(graphQlTraceparent).toMatch(/^00-[\da-f]{32}-[\da-f]{16}-0[01]$/);
    } finally {
      await closeServer(graphqlServer);
    }
  });

  it('continues a valid incoming trace context for downstream GraphQL', async () => {
    let graphQlTraceparent;
    const graphqlServer = createServer((_request, response) => {
      graphQlTraceparent = _request.headers.traceparent;
      response.writeHead(200, { 'content-type': 'application/json' });
      response.end(JSON.stringify({ data: {} }));
    });
    const graphqlPort = await listen(graphqlServer);
    const websitePort = await getAvailablePort();
    const incoming = '00-0123456789abcdef0123456789abcdef-0123456789abcdef-01';
    try {
      startProductionServer(
        websitePort,
        `http://127.0.0.1:${graphqlPort}/posts/graphql/immutable`,
        { enabled: true },
      );
      const response = await waitForResponse(
        `http://127.0.0.1:${websitePort}/en`,
        { headers: { traceparent: incoming } },
      );
      expect(response.status).toBe(200);
      expect(graphQlTraceparent).toMatch(
        /^00-0123456789abcdef0123456789abcdef-[\da-f]{16}-01$/,
      );
      expect(graphQlTraceparent).not.toBe(incoming);
    } finally {
      await closeServer(graphqlServer);
    }
  });

  it('exports raw visitor client attributes on an enabled SSR trace', async () => {
    let telemetryPayload = Buffer.alloc(0);
    const graphqlServer = createServer((_request, response) => {
      response.writeHead(200, { 'content-type': 'application/json' });
      response.end(JSON.stringify({ data: {} }));
    });
    const otlpServer = createServer((request, response) => {
      const chunks: Buffer[] = [];
      request.on('data', (chunk: Buffer) => chunks.push(chunk));
      request.on('end', () => {
        telemetryPayload = Buffer.concat(chunks);
        response.writeHead(200);
        response.end();
      });
    });
    const graphqlPort = await listen(graphqlServer);
    const otlpPort = await listen(otlpServer);
    const websitePort = await getAvailablePort();
    try {
      const website = startProductionServer(
        websitePort,
        `http://127.0.0.1:${graphqlPort}/posts/graphql/immutable`,
        {
          enabled: true,
          otlpEndpoint: `http://127.0.0.1:${otlpPort}/v1/traces`,
        },
      );
      const response = await waitForResponse(`http://127.0.0.1:${websitePort}/en`, {
        headers: {
          'x-forwarded-for': '203.0.113.9, 198.51.100.10',
          'user-agent':
            'Mozilla/5.0 (iPhone; CPU iPhone OS 18_1 like Mac OS X) AppleWebKit/605.1.15 Version/18.1 Mobile/15E148 Safari/604.1',
        },
      });
      expect(response.status).toBe(200);
      await stopProcess(website);

      const exported = telemetryPayload.toString('utf8');
      expect(exported).toContain('203.0.113.9');
      expect(exported).toContain('user_agent.original');
      expect(exported).toContain('user_agent.browser.name');
      expect(exported).toContain('Safari');
      expect(exported).toContain('device.type');
      expect(exported).toContain('mobile');
      expect(exported).not.toContain('198.51.100.10');
    } finally {
      await closeServer(graphqlServer);
      await closeServer(otlpServer);
    }
  });

  it('keeps enabled health checks dependency-free and leaves the browser bundle uninstrumented', async () => {
    let graphqlRequests = 0;
    let telemetryRequests = 0;
    const graphqlServer = createServer((_request, response) => {
      graphqlRequests += 1;
      response.writeHead(200, { 'content-type': 'application/json' });
      response.end(JSON.stringify({ data: {} }));
    });
    const otlpServer = createServer((_request, response) => {
      telemetryRequests += 1;
      response.writeHead(200);
      response.end();
    });
    const graphqlPort = await listen(graphqlServer);
    const otlpPort = await listen(otlpServer);
    const websitePort = await getAvailablePort();
    try {
      startProductionServer(
        websitePort,
        `http://127.0.0.1:${graphqlPort}/posts/graphql/immutable`,
        {
          enabled: true,
          otlpEndpoint: `http://127.0.0.1:${otlpPort}/v1/traces`,
        },
      );
      const response = await waitForResponse(
        `http://127.0.0.1:${websitePort}/healthz`,
      );
      expect(response.status).toBe(200);
      expect(graphqlRequests).toBe(0);
      const clientFiles = await fs.readdir(
        path.join(websiteRoot, 'dist/client/static/js'),
      );
      const clientBundle = (
        await Promise.all(
          clientFiles.map((file) =>
            fs.readFile(
              path.join(websiteRoot, 'dist/client/static/js', file),
              'utf8',
            ),
          ),
        )
      ).join('\n');
      expect(clientBundle.toLowerCase()).not.toContain('opentelemetry');
      expect(clientBundle.toLowerCase()).not.toContain('instrumentation.mjs');
      expect(telemetryRequests).toBe(0);
    } finally {
      await closeServer(graphqlServer);
      await closeServer(otlpServer);
    }
  });
});
