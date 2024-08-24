<script
	lang="ts"
	context="module">
	const { rpc } = await import('$lib/utilities/rpc');
</script>

<script lang="ts">
	import { createTauRPCProxy, type ModInfo } from '$generated/binding';
	import Switch from '$lib/components/Switch.svelte';
	import { ToasterContext, type ToasterContextReturn } from '$lib/components/Toster.svelte';
	import { SelectedProfile } from '$lib/stores/profiles';
	import { createScrollArea, melt } from '@melt-ui/svelte';
	import { getContext } from 'svelte';
	import type { Readable } from 'svelte/store';

	export let modStore: Readable<ModInfo[]>;

	type WithEnabled<T> = T & { enabled: boolean };
	let mods: WithEnabled<ModInfo>[] = $modStore.map((v) => ({ ...v, enabled: false }));

	const fetchModEnabledStatus = () => {
		rpc.api.mod.manage.fetch_active($SelectedProfile?.id!).then((enabledMods) => {
			for (const enabledMod of enabledMods) {
				let mod = mods.find((mod) => mod.id === enabledMod.id);
				if (mod === undefined) continue;
				mod.enabled = true;
			}
			console.log(mods);
			mods = [...mods];
		});
	};

	$: if ($SelectedProfile) fetchModEnabledStatus();

	const { addToast } = getContext<ToasterContextReturn>(ToasterContext);

	const {
		elements: { root, content, viewport, corner, scrollbarY, thumbY }
	} = createScrollArea({
		type: 'auto',
		dir: 'ltr'
	});

	const handleModActivationChange = async (id: string, check: boolean): Promise<boolean> => {
		let profile = $SelectedProfile;
		if (profile === undefined) {
			return false;
		}

		try {
			if (check) {
				console.log(`Mod: ${id} activated for ${profile?.name}`);
				await rpc.api.mod.manage.enable(profile.id, id);
			} else {
				console.log(`Mod: ${id} deactivated for ${profile?.name}`);
				await rpc.api.mod.manage.disable(profile.id, id);
			}
		} catch (e: any) {
			addToast({ data: { title: 'Mod Enable failed', type: 'error', content: e } });
			return false;
		}

		return check;
	};
</script>

<div
	use:melt={$root}
	class="relative w-full h-full p-4 text-black">
	<section
		use:melt={$viewport}
		class="w-full h-full">
		<div
			use:melt={$content}
			class="h-full space-y-2">
			{#each mods as mod (mod.id)}
				<div class="flex flex-row">
					<a
						class="flex flex-col shrink bg-neutral-200/40 px-4 py-px rounded-md hover:bg-neutral-200/70"
						href="/mods/{mod.id}">
						<span
							class="text-lg font-bold capitalize tracking-tighter first-letter:text-royal-indigo-400">
							{mod.name}
						</span>
					</a>
					<div class="flex-grow flex flex-row justify-end items-center gap-3">
						<span class="select-none">Enable:</span>
						<Switch
							disable={$SelectedProfile === undefined}
							switched={mod.enabled}
							onSwitchChange={async (next) => await handleModActivationChange(mod.id, next)} />
					</div>
				</div>
			{:else}
				<span class="flex justify-center select-none font-bold text-center w-full">
					Please add new mod using + in the bottom right corner
				</span>
			{/each}
		</div>
	</section>
	<div
		use:melt={$scrollbarY}
		class="flex h-full w-2.5 touch-none select-none border-l border-l-transparent bg-neutral-600/15 p-px transition-colors">
		<span
			use:melt={$thumbY}
			class="relative flex-1 rounded-full bg-neutral-600/50 hover:bg-neutral-600/60 transition-colors" />
	</div>
	<span use:melt={$corner} />
</div>
