import express from 'express';
import fs from 'node:fs/promises';
import path from 'node:path';
import crypto from 'node:crypto';

const requiredUrl = (name) => {
  const value = process.env[name]?.trim();
  if (!value) throw new Error(`${name} is required`);
  try { new URL(value); } catch { throw new Error(`${name} is invalid: ${value.slice(0, 64)}`); }
  return value;
};
const requiredText = (name) => {
  const value = process.env[name]?.trim();
  if (!value) throw new Error(`${name} is required`);
  return value;
};
const CONFIG = {
  siteName: requiredText('WEBSITE_SITE_NAME'),
  siteUrl: requiredUrl('WEBSITE_SITE_URL'),
  avatarUrl: process.env.WEBSITE_AVATAR_URL?.trim() || undefined,
  defaultTitle: requiredText('WEBSITE_DEFAULT_TITLE'),
  defaultDescription: requiredText('WEBSITE_DEFAULT_DESCRIPTION'),
  defaultLocale: process.env.WEBSITE_DEFAULT_LOCALE || 'en',
  graphqlApiUrl: requiredUrl('WEBSITE_PUBLIC_GRAPHQL_API_URL'),
  graphqlCacheApiUrl: process.env.WEBSITE_PUBLIC_GRAPHQL_CACHE_API_URL?.trim() || requiredUrl('WEBSITE_PUBLIC_GRAPHQL_API_URL'),
  mediaBaseUrl: requiredUrl('WEBSITE_PUBLIC_MEDIA_BASE_URL'),
  port: process.env.WEBSITE_PORT || process.env.PORT || '3001',
};
const escapeJsonForScript = (value) => JSON.stringify(value)
  .replace(/</g, '\\u003c').replace(/>/g, '\\u003e').replace(/&/g, '\\u0026')
  .replace(/'/g, '\\u0027').replace(/\u2028/g, '\\u2028').replace(/\u2029/g, '\\u2029');
const app = express();
const root = process.cwd();
const templatePath = path.join(root, 'dist/client/index.html');
const render = (await import('./dist/server/index.mjs')).default;
app.use('/static', express.static(path.join(root, 'dist/client/static')));
app.get('/{*path}', async (req, res) => {
  const correlationId = crypto.randomUUID();
  try {
    const template = await fs.readFile(templatePath, 'utf8');
    const rendered = await render(req.path);
    const state = `<script id="app-config" type="application/json">${escapeJsonForScript(CONFIG)}</script>`;
    const html = template.replace('<!--app-content-->', rendered.html).replace('</head>', `${state}</head>`)
      .replace('</body>', `<script>window.__APOLLO_STATE__=${escapeJsonForScript(rendered.apolloState)}</script></body>`);
    if (req.path === '/') {
      res.redirect(302, '/en');
    } else {
      res.status(200).send(html);
    }
  } catch {
    console.error(JSON.stringify({ method: req.method, path: req.path, correlationId }));
    res.status(500).send('<!doctype html><html><body><h1>Server Error</h1></body></html>');
  }
});
app.listen(Number(CONFIG.port));
