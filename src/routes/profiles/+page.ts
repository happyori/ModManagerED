import type { PageLoad } from './$types';

export const load = (async () => {
	const { ProfileStore } = await import('$lib/stores/profiles');
	const profiles = await ProfileStore.fetchAllMods();
	return { profiles };
}) satisfies PageLoad;
