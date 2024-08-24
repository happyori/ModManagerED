import { defineConfig, presetIcons, presetTypography, presetUno, presetWebFonts } from 'unocss';

export default defineConfig({
	shortcuts: [
		// ...
	],
	theme: {
		colors: {
			// ...
		}
	},
	presets: [
		presetUno(),
		presetIcons(),
		presetTypography(),
		presetWebFonts({
			provider: 'fontshare',
			fonts: {
				sans: { name: 'Roboto', provider: 'bunny' },
				sato: 'Satoshi'
			}
		})
	]
});
