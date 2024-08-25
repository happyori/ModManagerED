import type { PageLoad } from './$types';

export const load = (async ({ url }) => {
	const id = url.searchParams.get('id');
	if (id === null) {
		throw new Error('Failed to extract id from url');
	}
	const { ModInfoStore, findModInfoById } = await import('$lib/stores/mods');
	const mods = await ModInfoStore.fetchAllMods();
	const mod = findModInfoById(mods, id);
	return { mod, id };
}) satisfies PageLoad;
