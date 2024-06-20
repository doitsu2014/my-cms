import type { Config } from 'tailwindcss';

export default {
	darkMode: 'class',
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {
			colors: {
				primary: {
					light: '#3b82f6', // Light mode primary color
					dark: '#2563eb' // Dark mode primary color
				},
				secondary: {
					light: '#fbbf24',
					dark: '#f59e0b'
				},
				tertiary: {
					light: '#10b981',
					dark: '#059669'
				},
				success: {
					light: '#34d399',
					dark: '#10b981'
				},
				warning: {
					light: '#f97316',
					dark: '#ea580c'
				},
				error: {
					light: '#ef4444',
					dark: '#dc2626'
				},
				surface: {
					light: '#ffffff',
					dark: '#1f2937'
				}
			}
		}
	},

	// eslint-disable-next-line @typescript-eslint/no-require-imports
	plugins: [require('@tailwindcss/typography')]
} as Config;
