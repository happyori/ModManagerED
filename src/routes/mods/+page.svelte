<script lang="ts">
	import { goto } from '$app/navigation';
	import ActionButton from '$lib/components/ActionButton.svelte';
	import { createTabs, melt } from '@melt-ui/svelte';
	import { cubicInOut } from 'svelte/easing';
	import { crossfade } from 'svelte/transition';
	import ModContent from '$lib/components/ModContent.svelte';
	import { ExtraModStore, MainModStore } from '$lib/stores/mods';

	const tabs = [
		{ id: 'mods', title: 'Main Mods' },
		{ id: 'extra', title: 'Extra Mods' }
	];

	const {
		elements: { root, list, content, trigger },
		states: { value }
	} = createTabs({
		defaultValue: 'mods',
		orientation: 'horizontal'
	});

	const [send, receive] = crossfade({
		duration: 250,
		easing: cubicInOut
	});

	const handleCreate = () => goto('/mods/create');
</script>

<div
	use:melt={$root}
	class="flex max-w-full h-[80%] mx-2 mt-3 flex-col overflow-hidden rounded-xl shadow-lg data-[orientation=vertical]:flex-row">
	<div
		use:melt={$list}
		class="flex shrink-0 overflow-x-auto bg-neutral-100 data-[orientation=vertical]:flex-col data-[orientation=vertical]:border-r"
		aria-label="Choose mod type">
		{#each tabs as tab (tab.id)}
			<button
				use:melt={$trigger(tab.id)}
				class="trigger relative">
				{tab.title}
				{#if $value === tab.id}
					<div
						in:send={{ key: 'trigger' }}
						out:receive={{ key: 'trigger' }}
						class="absolute bottom-1 left-1/2 h-1 w-6 -translate-x-1/2 rounded-full bg-royal-indigo-500" />
				{/if}
			</button>
		{/each}
	</div>
	<div
		use:melt={$content('mods')}
		aria-label="Main Mods"
		class="w-full h-full bg-neutral-50">
		<ModContent modStore={MainModStore} />
	</div>
	<div
		use:melt={$content('extra')}
		aria-label="Extra Mods"
		class="w-full h-full bg-neutral-50">
		<ModContent modStore={ExtraModStore} />
	</div>
</div>
<ActionButton on:click={handleCreate}>+</ActionButton>

<style lang="postcss">
	.trigger {
		display: flex;
		align-items: center;
		justify-content: center;

		cursor: default;
		user-select: none;

		border-radius: 0;
		background-color: theme(colors.neutral.100);

		color: theme(colors.neutral.900);
		font-weight: 500;
		line-height: 1;

		flex: 1;
		height: theme(spacing.12);
		padding-inline: theme(spacing.2);

		&:focus {
			position: relative;
		}

		&:focus-visible {
			@apply z-10 ring-2;
		}

		&[data-state='active'] {
			@apply focus:visible;
			background-color: white;
			color: theme('colors.royal-indigo.800');
		}
	}
</style>
