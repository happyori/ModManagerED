import type { PageLoad } from './$types';

export const load = (async () => {
	const { GameInstanceStore } = await import('$lib/stores/game_instance');
	const instance = await GameInstanceStore.tryFetchOrCreate();
	return { instance };
}) satisfies PageLoad;
