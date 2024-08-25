<script lang="ts">
	import { createTooltip, melt } from '@melt-ui/svelte';
	import type { OpenDialogOptions } from '@tauri-apps/api/dialog';
	import { fade } from 'svelte/transition';

	const handleOpenPath = async () => {
		const { open } = await import('@tauri-apps/api/dialog');

		const path = await open(options);

		if (typeof path === 'string') {
			value = path;
		} else {
			console.log('Canceled');
		}
	};

	export let options: OpenDialogOptions = {};
	export let value: string | undefined | null;
	export let tooltipDirection: 'top' | 'bottom' | 'left' | 'right' = 'bottom';

	const {
		elements: { arrow, content, trigger },
		states: { open }
	} = createTooltip({
		positioning: { placement: tooltipDirection },
		closeOnPointerDown: true,
		openDelay: 0,
		closeDelay: 20,
		forceVisible: true
	});
</script>

<button
	type="button"
	use:melt={$trigger}
	on:click={handleOpenPath}
	class="rounded bg-neutral-100 px-4 py-1 text-pallete-accent font-bold tracking-wide shadow-inner shadow-zinc-500 transition-colors hover:bg-neutral-300 hover:text-neutral-900">
	Select
</button>

{#if $open}
	<div
		transition:fade={{ duration: 200 }}
		use:melt={$content}
		class="rounded-lg bg-neutral-100 px-4 py-2 text-sm shadow-black shadow-lg">
		<span use:melt={$arrow} />
		<slot name="tooltip">
			<p>Use this to select a file/folder</p>
		</slot>
	</div>
{/if}
