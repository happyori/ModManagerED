import { findModInfoById, ModInfoStore } from '$lib/stores/mods';
import type { PageLoad } from './$types';

export const load = (async ({ params }) => {
	let mods = await ModInfoStore.fetchAllMods();
	let mod = findModInfoById(mods, params.id);
	return { mod, ...params };
}) satisfies PageLoad;
