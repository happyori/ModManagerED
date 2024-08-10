import { tauri } from '@tauri-apps/api';
import type { PageLoad } from './$types';
import Commands from '$lib/commands';
import type { Profile } from '$generated/Profile';
import { ProfileStore } from '$lib/stores/profiles';

export const load = (async () => {
	const profiles = await tauri.invoke<Profile[]>(Commands.FetchAllProfiles);
	ProfileStore.set(profiles);
	return { profiles };
}) satisfies PageLoad;
