import type { PageLoad } from './$types';

export const load = (async ({ depends }) => {
	const { ModInfoStore } = await import('$lib/stores/mods');
	depends('mods:data');
	const mods = await ModInfoStore.fetchAllMods();

	return { mods };
}) satisfies PageLoad;
