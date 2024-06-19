import type { Config } from 'tailwindcss';

export default {
	darkMode: 'class',
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {
			colors: {
				primary: {
					light: '#333333',
					DEFAULT: '#000000',
					dark: '#000000'
				},
				secondary: {
					light: '#FFFFFF',
					DEFAULT: '#FFFFFF',
					dark: '#CCCCCC'
				},
				tertiary: {
					light: '#E0E0E0',
					DEFAULT: '#B0B0B0',
					dark: '#808080'
				},
				success: {
					light: '#A5D6A7',
					DEFAULT: '#4CAF50',
					dark: '#388E3C'
				},
				warning: {
					light: '#FFE082',
					DEFAULT: '#FFC107',
					dark: '#FFA000'
				},
				error: {
					light: '#EF9A9A',
					DEFAULT: '#F44336',
					dark: '#D32F2F'
				},
				surface: {
					light: '#F5F5F5',
					DEFAULT: '#EEEEEE',
					dark: '#E0E0E0'
				},
				text: {
					light: '#FFFFFF',
					DEFAULT: '#000000',
					dark: '#000000'
				},
				background: {
					light: '#FFFFFF',
					DEFAULT: '#F5F5F5',
					dark: '#121212' // Dark mode background color
				}
			}
		}
	},

	// eslint-disable-next-line @typescript-eslint/no-require-imports
	plugins: [require('@tailwindcss/typography')]
} as Config;
