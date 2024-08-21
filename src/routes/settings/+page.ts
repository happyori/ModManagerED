import { GameInstanceStore } from '$lib/stores/game_instance';
import type { PageLoad } from './$types';

export const load = (async () => {
	const instance = await GameInstanceStore.tryFetchOrCreate();
	return { instance };
}) satisfies PageLoad;
