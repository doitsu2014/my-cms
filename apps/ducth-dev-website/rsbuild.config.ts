import { defineConfig } from '@rsbuild/core';
import { pluginReact } from '@rsbuild/plugin-react';

export default defineConfig({
  html: { template: './index.html' },
  server: { port: 3001 },
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
});
