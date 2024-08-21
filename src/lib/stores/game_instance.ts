import type { GameInstance } from '$generated/GameInstance';
import Commands from '$lib/commands';
import { writable } from 'svelte/store';

function createStore() {
	let { set: innerSet, subscribe } = writable<GameInstance>();

	const set = async (value: GameInstance) => {
		const tauri = await import('@tauri-apps/api');
		const output = await tauri.invoke<GameInstance>(Commands.UpdateGameInstance, {
			gameInstance: value
		});
		innerSet(output);
	};

	const tryFetchOrCreate = async () => {
		const tauri = await import('@tauri-apps/api');
		try {
			const instance = await tauri.invoke<GameInstance>(Commands.FetchTheGameInstance);
			innerSet(instance);
			return instance;
		} catch {
			const instance = await tauri.invoke<GameInstance>(Commands.UpsertGameInstance, {
				gameInstanceDataModel: {}
			});
			innerSet(instance);
			return instance;
		}
	};

	const resetStore = async () => {
		const tauri = await import('@tauri-apps/api');
		const instance = await tauri.invoke<GameInstance>(Commands.ResetGameInstance);
		innerSet(instance);
		return instance;
	};

	return { subscribe, tryFetchOrCreate, resetStore, set };
}

export const GameInstanceStore = createStore();