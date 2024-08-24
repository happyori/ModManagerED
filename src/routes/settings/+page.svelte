<script lang="ts">
	import { createLabel, melt } from '@melt-ui/svelte';
	import type { PageData } from './$types';
	import { GameInstanceStore } from '$lib/stores/game_instance';
	import { getContext } from 'svelte';
	import { ToasterContext, type ToasterContextReturn } from '$lib/components/Toster.svelte';
	import FileSelectButton from '$lib/components/FileSelectButton.svelte';
	import type { OpenDialogOptions } from '@tauri-apps/api/dialog';

	const { addSuccessfulToast, addFailedToast } = getContext<ToasterContextReturn>(ToasterContext);
	export let data: PageData;
	let instance = data.instance;

	const handleSubmit = async () => {
		try {
			await GameInstanceStore.set(instance);
			addSuccessfulToast('Settings saved!');
		} catch (error) {
			addFailedToast('Settings failed to save!');
			console.error(error);
		}
	};

	const handleReset = async () => {
		try {
			instance = await GameInstanceStore.resetStore();
			addSuccessfulToast('Successfuly reset the instance');
		} catch (e) {
			addFailedToast('Failed to reset instance!');
			console.error(e);
		}
	};

	const {
		elements: { root: label }
	} = createLabel();

	const selectionOptions: OpenDialogOptions = {
		defaultPath: instance.path,
		directory: true,
		title: 'Select the Elden Ring Game folder (where the exe lives)'
	};

	$: console.log(instance);
</script>

<form
	on:submit|preventDefault={handleSubmit}
	class="h-[80%] w-fit flex flex-col items-start justify-center gap-y-4">
	<h1 class="select-none text-xl font-bold">Settings</h1>
	<div class="flex flex-row items-center gap-x-2">
		<label
			for="path"
			use:melt={$label}>
			Path
		</label>
		<input
			id="path"
			class="w-80 text-ellipsis rounded bg-zinc-50 px-2 py-1.5 text-sm text-black tracking-tight shadow-inner shadow-zinc-500 focus:outline-none"
			placeholder="Path to ELDEN RING"
			bind:value={instance.path} />
		<FileSelectButton
			bind:value={instance.path}
			options={selectionOptions}>
			<p slot="tooltip">Select ELDEN RING/Game folder (where the executable lives)</p>
		</FileSelectButton>
	</div>
	<div class="w-full flex flex-row justify-end gap-4">
		<button
			type="submit"
			class="active:outline-royal-indigo-400 w-20 rounded text-base tracking-wide outline-1 outline-green-400 outline transition-all duration-150 hover:outline-green-700">
			Save
		</button>
		<button
			type="button"
			on:click={handleReset}
			class="active:outline-royal-400 w-20 rounded text-center tracking-wide outline-1 outline-rose-500 outline transition-all duration-150 hover:outline-rose-700">
			Reset
		</button>
	</div>
</form>
