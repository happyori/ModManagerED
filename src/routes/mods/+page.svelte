<script lang="ts">
	import { goto } from '$app/navigation';
	import ActionButton from '$lib/components/ActionButton.svelte';
	import { createTabs, melt } from '@melt-ui/svelte';
	import { cubicInOut } from 'svelte/easing';
	import { crossfade } from 'svelte/transition';
	import ModContent from '$lib/components/ModContent.svelte';
	import { ExtraModStore, MainModStore } from '$lib/stores/mods';
	import Icon from '@iconify/svelte';

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
	class="mx-2 mt-3 h-[90%] max-w-full flex flex-col overflow-hidden rounded-xl shadow-lg data-[orientation=vertical]:flex-row">
	<div
		use:melt={$list}
		class="flex shrink-0 overflow-x-auto bg-pallete-bg data-[orientation=vertical]:flex-col data-[orientation=vertical]:border-r"
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
						class="absolute bottom-1 left-1/2 h-1 w-6 rounded-full bg-accents-1 -translate-x-1/2" />
				{/if}
			</button>
		{/each}
	</div>
	<div
		use:melt={$content('mods')}
		aria-label="Main Mods"
		class="h-full w-full bg-pallete-bg">
		<ModContent modStore={MainModStore} />
	</div>
	<div
		use:melt={$content('extra')}
		aria-label="Extra Mods"
		class="h-full w-full bg-pallete-bg">
		<ModContent modStore={ExtraModStore} />
	</div>
</div>
<ActionButton on:click={handleCreate}>
	<Icon
		icon="line-md:plus"
		width="24"
		height="24" />
</ActionButton>

<style lang="scss">
	.trigger {
		display: flex;
		align-items: center;
		justify-content: center;

		cursor: default;
		user-select: none;

		border-radius: 0;
		background-color: theme('colors.pallete.darker');

		color: theme('colors.neutral.100');
		font-weight: 500;
		line-height: 1;

		flex: 1;
		height: 3rem;
		padding-inline: 0.5rem;

		&:focus {
			position: relative;
		}

		&:focus-visible {
			--at-apply: z-10 ring-2;
		}

		&[data-state='active'] {
			--at-apply: 'focus:visible';
			background-color: theme('colors.pallete.bg');
			color: theme('colors.accents.2');
		}
	}
</style>
