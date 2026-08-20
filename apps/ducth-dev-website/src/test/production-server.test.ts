// @vitest-environment node

import { once } from 'node:events';
import { createServer, type Server } from 'node:http';
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

async function waitForResponse(url: string): Promise<Response> {
  let lastError: unknown;
  for (let attempt = 0; attempt < 50; attempt += 1) {
    try {
      return await fetch(url);
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
): ChildProcess {
  const child = spawn(process.execPath, ['server.prod.mjs'], {
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
    },
    stdio: 'pipe',
  });
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
});
