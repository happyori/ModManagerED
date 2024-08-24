import { createTauRPCProxy, type ModInfo } from '$generated/binding';
import { derived, writable } from 'svelte/store';

function createStore() {
	let { set, subscribe } = writable<ModInfo[]>([]);

	const fetchAllMods = async () => {
		const rpc = await createTauRPCProxy();
		const fetched = await rpc.api.mod.fetch_all();
		set(fetched);
		return fetched;
	};

	return { subscribe, fetchAllMods };
}

export const ModInfoStore = createStore();
export const ExtraModStore = derived(ModInfoStore, ($mods) => $mods.filter((m) => m.is_dll));
export const MainModStore = derived(ModInfoStore, ($mods) => $mods.filter((m) => !m.is_dll));

export const findModInfoById = (store: ModInfo[], id: string) => {
	return store.find((val) => val.id === id);
};
