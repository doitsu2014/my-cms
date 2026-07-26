import type { Config } from 'tailwindcss';

type DaisyThemeConfig = {
  themes: Array<Record<string, Record<string, string>>>;
};

const config: Config & { daisyui: DaisyThemeConfig } = {
  content: ['./src/**/*.{js,ts,jsx,tsx,mdx}'],
  theme: { extend: {} },
  plugins: [],
  daisyui: {
    themes: [
      {
        'ink-tide': {
          primary: '#b13a25',
          'primary-content': '#fbe6da',
          'base-100': '#ece1cb',
          'base-200': '#f6efde',
          neutral: '#0a112a',
          'neutral-content': '#f6efde',
        },
      },
    ],
  },
};

export default config;
