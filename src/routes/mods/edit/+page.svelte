<script lang="ts">
	import FileSelectButton from '$lib/components/FileSelectButton.svelte';
	import Collapse from '$lib/components/Collapse.svelte';

	import { createCheckbox, createLabel, melt } from '@melt-ui/svelte';
	import { fade } from 'svelte/transition';
	import type { OpenDialogOptions } from '@tauri-apps/api/dialog';
	import { getContext } from 'svelte';
	import { ToasterContext, type ToasterContextReturn } from '$lib/components/Toster.svelte';
	import { goto, invalidate } from '$app/navigation';
	import type { PageData } from './$types';
	import { createTauRPCProxy } from '$generated/binding';

	export let data: PageData;
	const { addToast } = getContext<ToasterContextReturn>(ToasterContext);

	const handleSubmit = async () => {
		if (info?.deployment_path === '') info.deployment_path = null;
		if (info === undefined) return;
		const rpc = await createTauRPCProxy();

		try {
			await rpc.api.mod.update(info);
			addToast({
				data: {
					title: 'Succefully update mod',
					type: 'success'
				}
			});
		} catch (e) {
			addToast({
				data: {
					title: 'Failed to update mod',
					content: `Error message: ${e}`,
					type: 'error'
				},
				closeDelay: 1500
			});
		}
	};

	const handleDelete = async () => {
		if (!info) return goto('/mods');
		const rpc = await createTauRPCProxy();

		try {
			await rpc.api.mod.delete(info);
			await invalidate('mods:data');
			await goto('/mods');
		} catch (error) {
			addToast({
				data: {
					title: 'Failed to delete mod',
					content: `Error message: ${error}`,
					type: 'error'
				},
				closeDelay: 1500
			});
		}
	};

	let info = data.mod;

	let options: OpenDialogOptions = {
		directory: false,
		filters: [{ name: 'DLL Mod', extensions: ['dll'] }],
		multiple: false,
		title: `Please select the mod folder`
	};

	$: options = {
		...options,
		directory: !info?.is_dll,
		title: `Please select the ${info?.is_dll ? 'DLL mod' : 'mod folder'}`
	};

	const {
		elements: { root, input },
		helpers: { isChecked }
	} = createCheckbox({
		defaultChecked: info?.is_dll
	});

	const {
		elements: { root: label }
	} = createLabel();

	$: if (info) info.is_dll = $isChecked;
</script>

<!-- TODO: Make adding dll and folder mods separate using the tabs or select or something i dunno -->
<!-- TODO: Info Tooltips -->
<!-- TODO: ADD confirmation dialog to delete something -_- -->
{#if info}
	<form
		on:submit|preventDefault={handleSubmit}
		class="ml-14 mt-20 w-fit flex flex-col justify-center space-y-4">
		<h1 class="select-none golden-text text-lg font-bold">Edit mod</h1>
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
						transition:fade={{ duration: 70 }}
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
				Save
			</button>
			<button
				on:click={handleDelete}
				type="button"
				class="w-24 rounded text-center tracking-wide outline-1 outline-rose-500 outline transition-all duration-150 active:outline-pallete-accent hover:outline-rose-700">
				Delete
			</button>
		</div>
	</form>
{:else}
	<span>Failed to fetch mod information! Something is wrong with this id: {data.id}</span>
{/if}

<style lang="scss">
	.labels {
		display: grid;
		grid-template-columns: 120px auto 1fr;
		--at-apply: tracking-tight text-sm items-center;
		span {
			font-weight: bold;
		}
	}
</style>
