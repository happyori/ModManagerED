import { defineConfig, presetIcons, presetTypography, presetUno, presetWebFonts } from 'unocss';

export default defineConfig({
	shortcuts: {
		'golden-text':
			'bg-gradient-from-yellow-500 to-amber-600 to-60% shape-br font-chill bg-gradient-linear bg-clip-text text-transparent font-bold',
		'fancy-input':
			'shadow-inner shadow-zinc-500 rounded text-black w-80 px-2 py-1 tracking-tight text-sm focus:outline-none bg-zinc-50'
	},
	theme: {
		colors: {
			pallete: {
				bg: '#252422',
				text: '#fffcf2',
				neutral: '#ccc5b9',
				accent: '#e2711d',
				complement: '#b841d6',
				darker: '#403d39'
			},
			accents: {
				'0': '#cc5803',
				'1': '#e2711d',
				'2': '#ff9505',
				'3': '#ffb627',
				'4': '#ffc971'
			}
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
				sato: 'Satoshi',
				chill: 'Chillax',
				default: 'Nunito'
			}
		})
	],
	safelist: [...'red green orange blue'.split(' ').map((col) => `bg-${col}-500`)]
});
