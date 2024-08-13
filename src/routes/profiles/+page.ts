import type { PageLoad } from './$types';
import { ProfileStore } from '$lib/stores/profiles';

export const load = (async () => {
	const profiles = await ProfileStore.fetchAllMods();
	return { profiles };
}) satisfies PageLoad;
