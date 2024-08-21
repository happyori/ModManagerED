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
	class="flex flex-col justify-center items-start gap-y-4 h-[80%] w-fit">
	<h1 class="text-xl font-bold select-none">Settings</h1>
	<div class="flex flex-row items-center gap-x-2">
		<label
			for="path"
			use:melt={$label}>
			Path
		</label>
		<input
			id="path"
			class="shadow-inner shadow-zinc-500 rounded text-black w-80 px-2 py-1.5 text-ellipsis tracking-tight text-sm focus:outline-none bg-zinc-50"
			placeholder="Path to ELDEN RING"
			bind:value={instance.path} />
		<FileSelectButton
			bind:value={instance.path}
			options={selectionOptions}>
			<p slot="tooltip">Select ELDEN RING/Game folder (where the executable lives)</p>
		</FileSelectButton>
	</div>
	<div class="flex flex-row gap-4 justify-end w-full">
		<button
			type="submit"
			class="outline outline-1 outline-green-400 text-base tracking-wide w-20 rounded hover:outline-green-700 transition-all duration-150 active:outline-royal-indigo-400">
			Save
		</button>
		<button
			type="button"
			on:click={handleReset}
			class="outline outline-1 outline-rose-500 text-center tracking-wide w-20 rounded hover:outline-rose-700 transition-all duration-150 active:outline-royal-400">
			Reset
		</button>
	</div>
</form>