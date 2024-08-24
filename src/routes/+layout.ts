// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)

import type { LayoutLoad } from './$types';
// See: https://beta.tauri.app/start/frontend/sveltekit/ for more info
export const prerender = true;
export const ssr = false;

export const load = (async ({ depends }) => {
	const { ProfileStore } = await import('$lib/stores/profiles');
	depends('profiles:data');
	const profiles = await ProfileStore.fetchAllMods();
	console.log(profiles);

	return { profiles };
}) satisfies LayoutLoad;
