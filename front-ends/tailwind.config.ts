import type { Config } from 'tailwindcss';

/**
 * Tailwind CSS 4 Configuration
 *
 * Note: With Tailwind CSS v4, plugins (including DaisyUI) are now configured
 * via @plugin directive in CSS files (see src/App.css).
 *
 * This config file is kept minimal for compatibility but most configuration
 * is now done in CSS files using the new @import and @plugin syntax.
 */
const config: Config = {
  content: [
    './src/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  theme: {
    extend: {},
  },
  // DaisyUI and other plugins are configured in App.css using @plugin directive
  plugins: [],
};

export default config;
