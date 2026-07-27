import React from 'react';
import { createRoot, hydrateRoot } from 'react-dom/client';
import App from './App';
import { readBrowserConfig } from './config/read-browser-config';
import { getRouteLanguage } from './lib/i18n/getRouteLanguage';

const isThemePreference = (value: unknown): value is 'light' | 'dark' =>
  value === 'light' || value === 'dark';

readBrowserConfig();
document.documentElement.lang = getRouteLanguage(window.location.pathname);
try {
  const stored = window.localStorage.getItem('ducth-theme');
  if (isThemePreference(stored)) {
    document.documentElement.dataset.theme = stored;
  }
} catch {
  /* storage unavailable; ignore */
}

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
