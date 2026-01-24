import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';

// Set DaisyUI theme to emerald
document.documentElement.setAttribute('data-theme', 'emerald');

const rootEl = document.getElementById('root');
if (rootEl) {
  const root = ReactDOM.createRoot(rootEl);
  root.render(
    <React.StrictMode>
      <App />
    </React.StrictMode>,
  );
}
