import { preprocessMeltUI, sequence } from '@melt-ui/pp';
// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://beta.tauri.app/start/frontend/sveltekit/ for more info
import adapter from '@sveltejs/adapter-static';
import { sveltePreprocess } from 'svelte-preprocess'

/** @type {import('@sveltejs/kit').Config}*/
const config = {
	preprocess: sequence([sveltePreprocess(), preprocessMeltUI()]),
	kit: {
		adapter: adapter(),
		alias: {
			$generated: 'src/generated/*'
		}
	},
	vitePlugin: {
		inspector: {
			showToggleButton: 'always'
		}
	}
};
export default config;
