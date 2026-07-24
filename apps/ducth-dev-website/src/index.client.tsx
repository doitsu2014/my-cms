import React from 'react';
import { hydrateRoot } from 'react-dom/client';
import App from './App';
import { readBrowserConfig } from './config/read-browser-config';

readBrowserConfig();
document.documentElement.setAttribute('data-theme', 'emerald');
const rootEl = document.getElementById('root');
if (rootEl) {
  hydrateRoot(rootEl, <React.StrictMode><App /></React.StrictMode>);
}
