import type { Config } from 'tailwindcss';

export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],

	theme: {
		extend: {
			colors: {
				'royal-indigo': {
					50: '#F7F5FE',
					100: '#D2C1F8',
					200: '#AC8DF2',
					300: '#8659ED',
					400: '#632EDC',
					500: '#5628C1',
					600: '#4A22A5',
					700: '#3D1C89',
					800: '#31176D',
					900: '#241151',
					950: '#180B35',
				},
			}
		}
	},
	// @ts-ignore
	plugins: [require('@tailwindcss/typography')]
} as Config;