<script
	lang="ts"
	context="module">
	import { ProfileStore, SelectedProfile } from '$lib/stores/profiles';
	import { createSelect, createSeparator, melt } from '@melt-ui/svelte';

	let profilePromise = ProfileStore.fetchAllMods();
</script>

<script lang="ts">
	import type { Profile } from '$generated/Profile';
	import Icon from '@iconify/svelte';
	import { fade } from 'svelte/transition';
	import { tweened } from 'svelte/motion';
	import { cubicInOut } from 'svelte/easing';
	import { cn } from '$lib/utilities/cn';

	const {
		elements: { menu, trigger, option },
		states: { selectedLabel, open, selected },
		options: { disabled }
	} = createSelect<Profile>({
		positioning: {
			placement: 'bottom',
			sameWidth: true,
			gutter: 0
		},
		forceVisible: true,
		disabled: $ProfileStore.length === 0
	});

	$: SelectedProfile.set($selected?.value);
	$: disabled.set($ProfileStore.length === 0);

	let rotation = tweened(0, {
		duration: 170,
		easing: cubicInOut
	});
	$: rotation.set($open ? 180 : 0);
	$: rounded = $open ? 'rounded-bl-none' : '';

	const {
		elements: { root: separator }
	} = createSeparator({ decorative: true, orientation: 'vertical' });

	const handlePlayClicked = () => {};
</script>

<div class={cn('flex bg-green-700 rounded pr-1 items-center transition-colors', rounded)}>
	<div>
		<button
			class="flex flex-row items-center gap-1 pl-1.5"
			use:melt={$trigger}>
			{$selectedLabel || 'Select Profile'}
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="1em"
				height="1em"
				style="transform: rotate({$rotation}deg);"
				viewBox="0 0 24 24">
				<path
					fill="none"
					stroke="currentColor"
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="m17 10l-5 5l-5-5" />
			</svg>
		</button>
		{#if $open}
			<div
				transition:fade={{ duration: 150 }}
				use:melt={$menu}
				class="z-10 flex max-h-[300px] px-0 pb-2 space-y-0.5 bg-green-700 flex-col overflow-y-auto rounded-lg rounded-t-none shadow">
				{#await profilePromise then profiles}
					{#each profiles as profile (profile.id)}
						<span
							use:melt={$option({ value: profile, label: profile.name })}
							class="text-white text-sm tracking-tight leading-tight cursor-pointer hover:bg-green-800 px-2 py-1">
							{profile.name}
						</span>
					{/each}
				{/await}
			</div>
		{/if}
	</div>
	<span
		use:melt={$separator}
		class="bg-green-800 w-[3px] h-[80%]" />
	<button on:click={handlePlayClicked}>
		<Icon icon="ant-design:caret-right-outlined" />
	</button>
</div>
