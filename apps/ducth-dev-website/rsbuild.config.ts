import fs from 'node:fs';
import path from 'node:path';
import { defineConfig } from '@rsbuild/core';
import { pluginReact } from '@rsbuild/plugin-react';
import { resolveRuntimeConfig } from './src/config/validate-env';

function loadDotEnv(envPath: string): Record<string, string> {
  if (!fs.existsSync(envPath)) return {};
  const raw = fs.readFileSync(envPath, 'utf8');
  const parsed: Record<string, string> = {};
  for (const line of raw.split(/\r?\n/)) {
    const match = line.match(/^\s*([A-Z0-9_]+)\s*=\s*(.*?)\s*$/);
    if (!match) continue;
    const [, key, rawValue] = match;
    const value = rawValue.replace(/^['"]|['"]$/g, '');
    parsed[key] = value;
  }
  return parsed;
}

const fileEnv = loadDotEnv(path.resolve(process.cwd(), '.env'));
const mergedEnv: NodeJS.ProcessEnv = { ...process.env, ...fileEnv };

let devConfig: ReturnType<typeof resolveRuntimeConfig> | null = null;
try {
  devConfig = resolveRuntimeConfig(mergedEnv);
} catch (err) {
  console.warn(
    '[rsbuild dev] runtime config not available:',
    err instanceof Error ? err.message : err,
  );
}

const escapeJsonForScript = (value: unknown): string =>
  JSON.stringify(value)
    .replace(/</g, '\\u003c')
    .replace(/>/g, '\\u003e')
    .replace(/&/g, '\\u0026')
    .replace(/'/g, '\\u0027')
    .replace(/\u2028/g, '\\u2028')
    .replace(/\u2029/g, '\\u2029');

const appConfigTags = devConfig
  ? [
      {
        tag: 'script',
        attrs: { id: 'app-config', type: 'application/json' },
        children: escapeJsonForScript(devConfig),
        head: true,
      },
    ]
  : [];

export default defineConfig({
  html: { template: './index.html', tags: appConfigTags },
  server: { port: 3001, historyApiFallback: true },
  resolve: { alias: { '@': './src' } },
  plugins: [pluginReact()],
  environments: {
    web: {
      source: { entry: { index: './src/index.client.tsx' } },
      output: { target: 'web', distPath: { root: 'dist/client' } },
    },
    node: {
      source: { entry: { index: './src/index.server.tsx' } },
      output: { target: 'node', distPath: { root: 'dist/server' }, filename: { js: '[name].mjs' } },
      tools: { rspack: { output: { library: { type: 'module' } }, experiments: { outputModule: true } } },
    },
  },
  setupMiddlewares: (middlewares) => {
    middlewares.unshift((req, res, next) => {
      if (req.path === '/') {
        res.statusCode = 302;
        res.setHeader('Location', '/en');
        res.setHeader('Content-Length', '0');
        res.end();
        return;
      }
      next();
    });
  },
});
