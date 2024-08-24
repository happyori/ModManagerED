<script
	lang="ts"
	context="module">
	const { rpc } = await import('$lib/utilities/rpc');
</script>

<script lang="ts">
	import { type ModInfo } from '$generated/binding';
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
		if (!$SelectedProfile) return;
		rpc.api.mod.manage.fetch_active($SelectedProfile.id).then((enabledMods) => {
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
		} catch (e) {
			addToast({ data: { title: 'Mod Enable failed', type: 'error', content: String(e) } });
			return false;
		}

		return check;
	};
</script>

<div
	use:melt={$root}
	class="relative h-full w-full p-4 text-black">
	<section
		use:melt={$viewport}
		class="h-full w-full">
		<div
			use:melt={$content}
			class="h-full space-y-2">
			{#each mods as mod (mod.id)}
				<div class="flex flex-row">
					<a
						class="flex shrink flex-col rounded-md bg-neutral-200/40 px-4 py-px hover:bg-neutral-200/70"
						href="/mods/{mod.id}">
						<span
							class="first-letter:text-royal-indigo-400 text-lg font-bold tracking-tighter capitalize">
							{mod.name}
						</span>
					</a>
					<div class="flex flex-grow flex-row items-center justify-end gap-3">
						<span class="select-none">Enable:</span>
						<Switch
							disable={$SelectedProfile === undefined}
							switched={mod.enabled}
							onSwitchChange={async (next) => await handleModActivationChange(mod.id, next)} />
					</div>
				</div>
			{:else}
				<span class="w-full flex select-none justify-center text-center font-bold">
					Please add new mod using + in the bottom right corner
				</span>
			{/each}
		</div>
	</section>
	<div
		use:melt={$scrollbarY}
		class="h-full w-2.5 flex touch-none select-none border-l border-l-transparent bg-neutral-600/15 p-px transition-colors">
		<span
			use:melt={$thumbY}
			class="relative flex-1 rounded-full bg-neutral-600/50 transition-colors hover:bg-neutral-600/60" />
	</div>
	<span use:melt={$corner} />
</div>
