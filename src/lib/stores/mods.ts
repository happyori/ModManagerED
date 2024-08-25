import { createTauRPCProxy, type ModInfo } from '$generated/binding';
import { derived, writable } from 'svelte/store';

function createStore() {
	const { set, subscribe } = writable<ModInfo[]>([]);

	const fetchAllMods = async () => {
		const rpc = await createTauRPCProxy();
		const fetched = await rpc.api.mod.fetch_all();
		set(fetched);
		return fetched;
	};

	const fetchActiveMods = async (profileId: string) => {
		const rpc = await createTauRPCProxy();
		const active = await rpc.api.mod.manage.fetch_active(profileId);

		return active;
	};

	return { subscribe, fetchAllMods, fetchActiveMods };
}

export const ModInfoStore = createStore();
export const ExtraModStore = derived(ModInfoStore, ($mods) => $mods.filter((m) => m.is_dll));
export const MainModStore = derived(ModInfoStore, ($mods) => $mods.filter((m) => !m.is_dll));

export const findModInfoById = (store: ModInfo[], id: string) => {
	return store.find((val) => val.id === id);
};
