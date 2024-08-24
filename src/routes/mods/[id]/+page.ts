import type { PageLoad } from './$types';

export const load = (async ({ params }) => {
	const { ModInfoStore, findModInfoById } = await import('$lib/stores/mods');
	let mods = await ModInfoStore.fetchAllMods();
	let mod = findModInfoById(mods, params.id);
	return { mod, ...params };
}) satisfies PageLoad;
