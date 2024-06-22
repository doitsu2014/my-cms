import type { Config } from 'tailwindcss';
import colors from 'tailwindcss/colors';

const app_colors = {
	black: {
		DEFAULT: '#000000',
		50: '#E6E6E6',
		100: '#CCCCCC',
		200: '#999999',
		300: '#666666',
		400: '#333333',
		500: '#000000',
		600: '#000000',
		700: '#000000',
		800: '#000000',
		900: '#000000',
		950: '#000000'
	},
	rusty: {
		DEFAULT: '#DE3C4B',
		50: '#FCEEEF',
		100: '#F8D8DB',
		200: '#F2B0B7',
		300: '#EB8993',
		400: '#E4626F',
		500: '#DE3C4B',
		600: '#C02030',
		700: '#901824',
		800: '#601018',
		900: '#30080C',
		950: '#1A0407'
	},
	electric: {
		DEFAULT: '#87F5FB',
		50: '#F5FEFF',
		100: '#E6FDFE',
		200: '#CEFBFD',
		300: '#B5F9FD',
		400: '#A1F7FC',
		500: '#87F5FB',
		600: '#3FEFF9',
		700: '#07D3DE',
		800: '#058D94',
		900: '#02464A',
		950: '#012627'
	},
	silver: {
		DEFAULT: '#CEC3C1',
		50: '#FAF9F9',
		100: '#F6F4F4',
		200: '#EBE6E5',
		300: '#E2DBDA',
		400: '#D9D0CF',
		500: '#CEC3C1',
		600: '#AC9996',
		700: '#866E6A',
		800: '#584946',
		900: '#2E2524',
		950: '#171312'
	},
	cerulean: {
		DEFAULT: '#007EA7',
		50: '#DBF6FF',
		100: '#BDEEFF',
		200: '#75DDFF',
		300: '#33CCFF',
		400: '#00B0EB',
		500: '#007EA7',
		600: '#006385',
		700: '#004D66',
		800: '#003242',
		900: '#001B24',
		950: '#000B0F'
	}
};

export default {
	darkMode: 'class',
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {
			colors: {
				...app_colors,
				primary: app_colors.black,
				secondary: app_colors.electric,
				tertiary: app_colors.silver,
				rusty: app_colors.rusty,
				error: app_colors.rusty,
				warning: colors.yellow,
				success: colors.green,
				surface: colors.slate
			}
		}
	},

	// eslint-disable-next-line @typescript-eslint/no-require-imports
	plugins: [require('@tailwindcss/typography')]
} as Config;
