import type { PageLoad } from './$types';
import { ModInfoStore } from '$lib/stores/mods';

export const load = (async () => {
	const mods = await ModInfoStore.fetchAllMods();

	return { mods };
}) satisfies PageLoad;
