// Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)

import type { LayoutLoad } from './$types';
// See: https://beta.tauri.app/start/frontend/sveltekit/ for more info
export const prerender = true;
export const ssr = false;

export const load = (async ({ depends, untrack }) => {
	const { ProfileStore } = await import('$lib/stores/profiles');
	const { getCurrent } = await import('@tauri-apps/api/window');
	depends('profiles:data');
	const profiles = await ProfileStore.fetchAllMods();

	untrack(() => {
		getCurrent().setDecorations(true);
	});

	return { profiles };
}) satisfies LayoutLoad;
