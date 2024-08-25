<script lang="ts">
	import FileSelectButton from '$lib/components/FileSelectButton.svelte';

	import { createCheckbox, createLabel, melt } from '@melt-ui/svelte';
	import { fade } from 'svelte/transition';
	import type { OpenDialogOptions } from '@tauri-apps/api/dialog';
	import { getContext } from 'svelte';
	import { ToasterContext, type ToasterContextReturn } from '$lib/components/Toster.svelte';
	import { goto } from '$app/navigation';
	import { createTauRPCProxy, type ModInfoDataModel } from '$generated/binding';
	import Collapse from '$lib/components/Collapse.svelte';

	const { addToast } = getContext<ToasterContextReturn>(ToasterContext);

	const handleSubmit = async () => {
		if (info.deployment_path === '') info.deployment_path = null;
		const rpc = await createTauRPCProxy();

		try {
			await rpc.api.mod.create(info);
			await goto('/mods');
		} catch (e) {
			addToast({
				data: {
					title: 'Failed to create mod',
					content: `Error message: ${e}`,
					type: 'error'
				},
				closeDelay: 1500
			});
		}
	};

	let info: ModInfoDataModel = {
		path: '',
		name: '',
		deployment_path: null,
		is_dll: false
	};

	let options: OpenDialogOptions = {
		directory: !info.is_dll,
		filters: [{ name: 'DLL Mod', extensions: ['dll'] }],
		multiple: false,
		title: `Please select the ${info.is_dll ? 'DLL mod' : 'mod folder'}`
	};

	$: options = {
		...options,
		directory: !info.is_dll,
		title: `Please select the ${info.is_dll ? 'DLL mod' : 'mod folder'}`
	};

	const {
		elements: { root, input },
		helpers: { isChecked }
	} = createCheckbox();

	const {
		elements: { root: label }
	} = createLabel();

	$: info.is_dll = $isChecked;
</script>

<!-- TODO: Make adding dll and folder mods separate using the tabs or select or something i dunno -->
<!-- TODO: Info Tooltips -->
<form
	on:submit|preventDefault={handleSubmit}
	class="ml-14 mt-20 w-fit flex flex-col justify-center space-y-4">
	<h1 class="select-none golden-text text-lg font-bold">Add mod</h1>
	<label
		use:melt={$label}
		class="labels">
		<span>Name</span>
		<input
			class="fancy-input"
			bind:value={info.name}
			placeholder="[Cool name]" />
	</label>
	<label
		use:melt={$label}
		class="labels">
		<span>Path</span>
		<div class="w-80 flex gap-2">
			<input
				class="fancy-input flex-grow !w-auto"
				bind:value={info.path}
				placeholder="[Path to mod]" />
			<FileSelectButton
				bind:value={info.path}
				tooltipDirection="left"
				{options}>
				<p
					slot="tooltip"
					class="w-60 select-none text-pretty">
					If you want to select a DLL mod make sure to tick the Is DLL toggle before you click
				</p>
			</FileSelectButton>
		</div>
	</label>
	<div class="flex justify-between">
		<label
			class="flex select-none items-center text-sm font-bold tracking-tight"
			for="dll">
			<span>Is DLL</span>
		</label>
		<button
			id="dll"
			class="size-6 flex items-center justify-center rounded-lg bg-neutral-100 text-black"
			use:melt={$root}
			type="button">
			{#if $isChecked}
				<svg
					transition:fade={{ duration: 90 }}
					xmlns="http://www.w3.org/2000/svg"
					width="1em"
					height="1em"
					viewBox="0 0 24 24">
					<path
						fill="none"
						stroke="currentColor"
						stroke-linecap="round"
						stroke-linejoin="round"
						stroke-width="2"
						d="M5 12h7m7 0h-7m0 0V5m0 7v7" />
				</svg>
			{/if}
			<input use:melt={$input} />
		</button>
	</div>
	<Collapse title="Advanced Options">
		<label
			use:melt={$label}
			class="labels text-sm">
			<span>Deployment Path</span>
			<div class="w-80 flex gap-2">
				<input
					class="fancy-input flex-grow !w-auto"
					bind:value={info.deployment_path}
					placeholder="Leave empty for default" />
				<FileSelectButton
					bind:value={info.deployment_path}
					options={{ directory: true, title: 'Select deployment location' }} />
			</div>
		</label>
	</Collapse>

	<div class="flex flex-row justify-end space-x-3">
		<button
			type="submit"
			class="w-24 rounded text-base tracking-wide outline-1 outline-green-400 outline transition-all duration-150 active:outline-pallete-accent hover:outline-green-700">
			Add
		</button>
		<a
			href="/mods"
			class="w-24 rounded text-center tracking-wide outline-1 outline-rose-500 outline transition-all duration-150 active:outline-pallete-accent hover:outline-rose-700">
			Cancel
		</a>
	</div>
</form>

<style lang="scss">
	.fancy-input {
		--at-apply: 'shadow-inner shadow-zinc-500 rounded text-black w-80 px-2 py-1 tracking-tight text-sm focus:outline-none bg-zinc-50';
	}
	.labels {
		display: grid;
		grid-template-columns: 120px auto 1fr;
		--at-apply: tracking-tight text-sm items-center;
		span {
			font-weight: bold;
		}
	}
</style>
