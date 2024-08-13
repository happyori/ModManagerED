import { type ModInfo } from '$generated/ModInfo';
import Commands from '$lib/commands';
import { derived, writable } from 'svelte/store';

function createStore() {
	let { set, subscribe } = writable<ModInfo[]>([]);

	const fetchAllMods = async () => {
		const tauri = await import('@tauri-apps/api');
		console.log('Fetching mods and storing them');
		const fetched = await tauri.invoke<ModInfo[]>(Commands.FetchAllMods);
		console.log(fetched);
		set(fetched);
		return fetched;
	};

	return { subscribe, fetchAllMods };
}

export const ModInfoStore = createStore();
export const ExtraModStore = derived(ModInfoStore, ($mods) => $mods.filter((m) => m.is_dll));
export const MainModStore = derived(ModInfoStore, ($mods) => $mods.filter((m) => !m.is_dll));
