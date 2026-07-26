import React from 'react';
import { createRoot, hydrateRoot } from 'react-dom/client';
import App from './App';
import { readBrowserConfig } from './config/read-browser-config';
import { getRouteLanguage } from './lib/i18n/getRouteLanguage';

readBrowserConfig();
document.documentElement.lang = getRouteLanguage(window.location.pathname);
document.documentElement.dataset.theme = 'ink-tide';

const rootEl = document.getElementById('root');
if (rootEl) {
  const tree = (
    <React.StrictMode>
      <App />
    </React.StrictMode>
  );
  if (process.env.NODE_ENV === 'production') {
    hydrateRoot(rootEl, tree);
  } else {
    createRoot(rootEl).render(tree);
  }
}
