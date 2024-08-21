<script
	lang="ts"
	context="module">
	const tauri = await import('@tauri-apps/api');
</script>

<script lang="ts">
	import FileSelectButton from '$lib/components/FileSelectButton.svelte';

	import { createCheckbox, createCollapsible, createLabel, melt } from '@melt-ui/svelte';
	import { cubicInOut } from 'svelte/easing';
	import { tweened } from 'svelte/motion';
	import { slide, fade } from 'svelte/transition';
	import type { OpenDialogOptions } from '@tauri-apps/api/dialog';
	import { getContext } from 'svelte';
	import { ToasterContext, type ToasterContextReturn } from '$lib/components/Toster.svelte';
	import Commands from '$lib/commands';
	import { goto, invalidate } from '$app/navigation';
	import type { PageData } from './$types';

	export let data: PageData;
	const { addToast } = getContext<ToasterContextReturn>(ToasterContext);

	const handleSubmit = async () => {
		if (info?.deployment_path === '') info.deployment_path = undefined;

		try {
			await tauri.invoke(Commands.UpdateModInfo, { modInfo: info });
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

		try {
			await tauri.invoke(Commands.DeleteModInfo, { modInfo: info });
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

	let chevronRotation = tweened(180, {
		duration: 200,
		easing: cubicInOut
	});

	const {
		elements: { root, input },
		helpers: { isChecked }
	} = createCheckbox({
		defaultChecked: info?.is_dll
	});

	const {
		elements: { root: collapsible, trigger, content },
		states: { open }
	} = createCollapsible({
		forceVisible: true
	});
	const {
		elements: { root: label }
	} = createLabel();

	$: if (info) info.is_dll = $isChecked;
	$: chevronRotation.set($open ? 0 : 180);
</script>

<!-- TODO: Make adding dll and folder mods separate using the tabs or select or something i dunno -->
<!-- TODO: Info Tooltips -->
<!-- TODO: ADD confirmation dialog to delete something -_- -->
{#if info}
	<form
		on:submit|preventDefault={handleSubmit}
		class="flex flex-col w-auto mt-20 justify-center ml-14 space-y-4">
		<h1 class="font-bold text-lg select-none">Edit mod</h1>
		<label
			use:melt={$label}
			class="labels">
			<span>Name</span>
			<input
				bind:value={info.name}
				placeholder="[Cool name]" />
		</label>
		<label
			use:melt={$label}
			class="labels">
			<span>Path</span>
			<div class="w-80 flex gap-2">
				<input
					class="fancy-input !w-auto flex-grow"
					bind:value={info.path}
					placeholder="[Path to mod]" />
				<FileSelectButton
					bind:value={info.path}
					tooltipDirection="left"
					{options}>
					<p
						slot="tooltip"
						class="text-pretty w-60 select-none">
						If you want to select a DLL mod make sure to tick the Is DLL toggle before you click
					</p>
				</FileSelectButton>
			</div>
		</label>
		<div class="flex justify-between mr-[105px]">
			<label
				class="tracking-tight text-sm font-bold select-none items-center flex"
				for="dll">
				<span>Is DLL</span>
			</label>
			<button
				id="dll"
				class="rounded-lg bg-neutral-100 size-6 text-black items-center flex justify-center"
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
		<div use:melt={$collapsible}>
			<div class="flex justify-end group">
				<button
					use:melt={$trigger}
					class="flex flex-row justify-end items-center mr-28 space-x-2 mb-1.5"
					type="button">
					<span
						class="text-royal-indigo-200 underline underline-offset-4 select-none group-hover:text-royal-indigo-300 transition-colors">
						Advanced Options
					</span>
					<svg
						class="text-royal-indigo-200 group-hover:text-royal-indigo-300 transition-colors"
						xmlns="http://www.w3.org/2000/svg"
						width="1em"
						height="1em"
						style="transform: rotate({$chevronRotation}deg);"
						viewBox="0 0 24 24">
						<path
							fill="none"
							stroke="currentColor"
							stroke-linecap="round"
							stroke-linejoin="round"
							stroke-width="2"
							d="m17 14l-5-5l-5 5" />
					</svg>
				</button>
			</div>

			{#if $open}
				<section
					transition:slide
					class="mt-2"
					use:melt={$content}>
					<label
						use:melt={$label}
						class="labels text-sm">
						<span>Deployment Path</span>
						<div class="flex w-80 gap-2">
							<input
								class="fancy-input !w-auto flex-grow"
								bind:value={info.deployment_path}
								placeholder="Leave empty for default" />
							<FileSelectButton
								bind:value={info.deployment_path}
								options={{ directory: true, title: 'Select deployment location' }} />
						</div>
					</label>
				</section>
			{/if}
		</div>
		<div class="flex flex-row space-x-3 justify-end mr-28">
			<button
				type="submit"
				class="outline outline-1 outline-green-400 text-base tracking-wide w-24 rounded hover:outline-green-700 transition-all duration-150 active:outline-royal-indigo-400">
				Save
			</button>
			<button
				on:click={handleDelete}
				type="button"
				class="outline outline-1 outline-rose-500 text-center tracking-wide w-24 rounded hover:outline-rose-700 transition-all duration-150 active:outline-royal-400">
				Delete
			</button>
		</div>
	</form>
{:else}
	<span>Failed to fetch mod information! Something is wrong with this id: {data.id}</span>
{/if}

<style lang="postcss">
	.fancy-input {
		@apply shadow-inner shadow-zinc-500 rounded text-black w-80 px-2 py-1 tracking-tight text-sm focus:outline-none bg-zinc-50;
	}
	.labels {
		display: grid;
		grid-template-columns: 120px auto 1fr;
		@apply tracking-tight text-sm items-center;
		& > span {
			font-weight: bold;
		}
		& > input {
			@apply fancy-input;
		}
	}
</style>
