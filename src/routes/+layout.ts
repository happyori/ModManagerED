// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)

import { ProfileStore } from '$lib/stores/profiles';
import type { LayoutLoad } from './$types';

// See: https://beta.tauri.app/start/frontend/sveltekit/ for more info
export const prerender = true;
export const ssr = false;

export const load = (async ({ depends }) => {
	depends('profiles:data');
	const profiles = await ProfileStore.fetchAllMods();

	return { profiles };
}) satisfies LayoutLoad;