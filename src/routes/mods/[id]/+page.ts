import type { PageLoad } from './$types';

export const load = (async ({ params }) => {
	const { ModInfoStore, findModInfoById } = await import('$lib/stores/mods');
	const mods = await ModInfoStore.fetchAllMods();
	const mod = findModInfoById(mods, params.id);
	return { mod, ...params };
}) satisfies PageLoad;
