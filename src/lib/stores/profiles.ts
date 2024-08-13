import type { Profile } from '$generated/Profile';
import Commands from '$lib/commands';
import { writable } from 'svelte/store';

function createStore() {
	let { set, subscribe } = writable<Profile[]>([]);

	const fetchAllMods = async () => {
		const tauri = await import('@tauri-apps/api');
		console.log('Fetching profiles and storing them');
		const fetched = await tauri.invoke<Profile[]>(Commands.FetchAllProfiles);
		set(fetched);
		return fetched;
	};

	return { subscribe, fetchAllMods };
}

export const ProfileStore = createStore();
export const findProfileById = (profiles: Profile[], id: string) => {
	return profiles.find((p) => p.id === id);
};

export const SelectedProfile = writable<Profile | undefined>();
