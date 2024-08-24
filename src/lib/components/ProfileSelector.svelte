<script lang="ts">
	import Icon from '@iconify/svelte';
	import { fade } from 'svelte/transition';
	import { tweened } from 'svelte/motion';
	import { cubicInOut } from 'svelte/easing';
	import { ProfileStore, SelectedProfile } from '$lib/stores/profiles';
	import { createSelect, createSeparator, melt } from '@melt-ui/svelte';
	import { getContext } from 'svelte';
	import { ToasterContext, type ToasterContextReturn } from './Toster.svelte';
	import { createTauRPCProxy, type Profile } from '$generated/binding';

	const { addSuccessfulToast, addFailedToast } = getContext<ToasterContextReturn>(ToasterContext);

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

	export let profiles: Profile[];

	$: SelectedProfile.set($selected?.value);
	$: disabled.set($ProfileStore.length === 0);

	let rotation = tweened(0, {
		duration: 170,
		easing: cubicInOut
	});
	$: rotation.set($open ? 180 : 0);

	const {
		elements: { root: separator }
	} = createSeparator({ decorative: true, orientation: 'vertical' });

	const handlePlayClicked = async () => {
		const rpc = await createTauRPCProxy();
		try {
			if ($SelectedProfile === undefined) {
				addFailedToast('No profile selected, how did you even do this???');
				return;
			}
			await rpc.api.mod.manage.launch_game($SelectedProfile);
			addSuccessfulToast('Game launching...');
		} catch (e) {
			addFailedToast('Failed to launch!', `${e}`);
		}
	};
</script>

<div
	class="flex items-center rounded bg-green-700 pr-1 transition-colors"
	class:rounded-bl-none={$open}>
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
				class="z-10 max-h-[300px] flex flex-col overflow-y-auto rounded-lg rounded-t-none bg-green-700 px-0 pb-2 shadow space-y-0.5">
				{#each profiles as profile (profile.id)}
					<span
						use:melt={$option({ value: profile, label: profile.name })}
						class="cursor-pointer px-2 py-1 text-sm text-white leading-tight tracking-tight hover:bg-green-800">
						{profile.name}
					</span>
				{/each}
			</div>
		{/if}
	</div>
	<span
		use:melt={$separator}
		class="h-[80%] w-[3px] bg-green-800" />
	<button
		on:click={handlePlayClicked}
		disabled={$SelectedProfile === undefined}
		class="disabled:cursor-not-allowed">
		<Icon icon="ant-design:caret-right-outlined" />
	</button>
</div>
