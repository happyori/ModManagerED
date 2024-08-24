import type { Profile } from '$generated/binding';
import { writable } from 'svelte/store';

const { rpc } = await import('$lib/utilities/rpc');

function createStore() {
	const { set, subscribe } = writable<Profile[]>([]);

	const fetchAllMods = async () => {
		console.log('Fetching profiles and storing them');
		const fetched = await rpc.api.profile.fetch_all();
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
