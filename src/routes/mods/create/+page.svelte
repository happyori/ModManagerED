<script lang="ts">
	import FileSelectButton from '$lib/components/FileSelectButton.svelte';

	import { createCheckbox, createCollapsible, createLabel, melt } from '@melt-ui/svelte';
	import { cubicInOut } from 'svelte/easing';
	import { tweened } from 'svelte/motion';
	import { slide, fade } from 'svelte/transition';
	import type { OpenDialogOptions } from '@tauri-apps/api/dialog';
	import { getContext } from 'svelte';
	import { ToasterContext, type ToasterContextReturn } from '$lib/components/Toster.svelte';
	import { goto } from '$app/navigation';
	import { createTauRPCProxy, type ModInfoDataModel } from '$generated/binding';

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

	let chevronRotation = tweened(180, {
		duration: 200,
		easing: cubicInOut
	});

	const {
		elements: { root, input },
		helpers: { isChecked }
	} = createCheckbox();

	const {
		elements: { root: collapsible, trigger, content },
		states: { open }
	} = createCollapsible({
		forceVisible: true
	});
	const {
		elements: { root: label }
	} = createLabel();

	$: info.is_dll = $isChecked;
	$: chevronRotation.set($open ? 0 : 180);
</script>

<!-- TODO: Make adding dll and folder mods separate using the tabs or select or something i dunno -->
<!-- TODO: Info Tooltips -->
<form
	on:submit|preventDefault={handleSubmit}
	class="ml-14 mt-20 w-auto flex flex-col justify-center space-y-4">
	<h1 class="select-none text-lg font-bold">Add mod</h1>
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
	<div class="mr-[105px] flex justify-between">
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
	<div use:melt={$collapsible}>
		<div class="group flex justify-end">
			<button
				use:melt={$trigger}
				class="mb-1.5 mr-28 flex flex-row items-center justify-end space-x-2"
				type="button">
				<span
					class="text-royal-indigo-200 group-hover:text-royal-indigo-300 select-none underline underline-offset-4 transition-colors">
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
			</section>
		{/if}
	</div>
	<div class="mr-28 flex flex-row justify-end space-x-3">
		<button
			type="submit"
			class="active:outline-royal-indigo-400 w-24 rounded text-base tracking-wide outline-1 outline-green-400 outline transition-all duration-150 hover:outline-green-700">
			Add
		</button>
		<a
			href="/mods"
			class="active:outline-royal-400 w-24 rounded text-center tracking-wide outline-1 outline-rose-500 outline transition-all duration-150 hover:outline-rose-700">
			Cancel
		</a>
	</div>
</form>

<style lang="postcss">
	.fancy-input {
		@apply 'shadow-inner shadow-zinc-500 rounded text-black w-80 px-2 py-1 tracking-tight text-sm focus:outline-none bg-zinc-50';
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
