import type { PageLoad } from './$types';
import { ModInfoStore } from '$lib/stores/mods';

export const load = (async ({ depends }) => {
	depends('mods:data');
	const mods = await ModInfoStore.fetchAllMods();

	return { mods };
}) satisfies PageLoad;
