import { createTauRPCProxy, type GameInstance } from '$generated/binding';
import { writable } from 'svelte/store';

function createStore() {
	const { set: innerSet, subscribe } = writable<GameInstance>();

	const set = async (value: GameInstance) => {
		const rpc = await createTauRPCProxy();
		const output = await rpc.api.game.update(value);
		innerSet(output);
	};

	const tryFetchOrCreate = async () => {
		const rpc = await createTauRPCProxy();
		try {
			const instance = await rpc.api.game.fetch();
			innerSet(instance);
			return instance;
		} catch {
			const instance = await rpc.api.game.upsert({ mod_engine_path: null, path: null });
			innerSet(instance);
			return instance;
		}
	};

	const resetStore = async () => {
		const rpc = await createTauRPCProxy();
		const instance = await rpc.api.game.reset();
		innerSet(instance);
		return instance;
	};

	return { subscribe, tryFetchOrCreate, resetStore, set };
}

export const GameInstanceStore = createStore();
