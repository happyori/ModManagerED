<script lang="ts">
	import Icon from '@iconify/svelte';
	import { fade } from 'svelte/transition';
	import { tweened } from 'svelte/motion';
	import { cubicInOut } from 'svelte/easing';
	import { ProfileStore, SelectedProfile } from '$lib/stores/profiles';
	import { createSelect, melt } from '@melt-ui/svelte';
	import { getContext } from 'svelte';
	import { ToasterContext, type ToasterContextReturn } from './Toster.svelte';
	import { createTauRPCProxy, type Profile } from '$generated/binding';

	const { addSuccessfulToast, addFailedToast } = getContext<ToasterContextReturn>(ToasterContext);

	const {
		elements: { menu, trigger, option },
		states: { open, selected },
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
	class="mr-2 h-28px flex items-center rounded bg-pallete-bg transition-colors"
	class:rounded-bl-none={$open}>
	<div>
		<button
			class="flex flex-row select-none items-center gap-1 pl-1.5"
			use:melt={$trigger}>
			{$SelectedProfile?.name || 'Select Profile'}
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
				class="z-10 max-h-[300px] flex flex-col overflow-y-auto rounded-lg rounded-t-none bg-pallete-bg px-0 pb-2 pt-1.5 shadow-xl space-y-0.5">
				{#each profiles as profile (profile.id)}
					<span
						use:melt={$option({ value: profile, label: profile.name })}
						class="cursor-pointer px-2 py-1 text-size-base text-pallete-text leading-tight transition-colors hover:text-pallete-accent">
						{profile.name}
					</span>
				{/each}
			</div>
		{/if}
	</div>
	<button
		on:click={handlePlayClicked}
		disabled={$SelectedProfile === undefined}
		class="ml-1 h-full w-5.5 flex items-center justify-center rounded-r bg-green-600 disabled:cursor-not-allowed disabled:bg-pallete-bg">
		<Icon icon="ant-design:caret-right-outlined" />
	</button>
</div>
