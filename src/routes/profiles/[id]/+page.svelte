<script lang="ts">
	import { findProfileById, ProfileStore } from '$lib/stores/profiles';
	import { getContext } from 'svelte';
	import type { PageData } from './$types';
	import { melt, createLabel } from '@melt-ui/svelte';
	import { ToasterContext, type ToasterContextReturn } from '$lib/components/Toster.svelte';
	import { goto, invalidate } from '$app/navigation';
	import { createTauRPCProxy } from '$generated/binding';

	export let data: PageData;
	const { addFailedToast, addSuccessfulToast } = getContext<ToasterContextReturn>(ToasterContext);

	const {
		elements: { root }
	} = createLabel();

	const handleSave = async () => {
		if (profile === undefined) {
			addFailedToast('Profile is not found!');
			return;
		}
		const rpc = await createTauRPCProxy();
		try {
			await rpc.api.profile.update(profile);
			await invalidate('profiles:data');
			addSuccessfulToast('Successfully updated the profile');
		} catch (error) {
			addFailedToast(
				'Failed to update profile',
				`Profile ${profile?.name} was not updated\n${error}`
			);
		}
	};

	const handleDelete = async () => {
		if (profile === undefined) {
			addFailedToast('Profile is not found!');
			return;
		}
		const rpc = await createTauRPCProxy();
		try {
			await rpc.api.profile.delete(profile);
			await invalidate('profiles:data');
			await goto('/profiles');
		} catch (error) {
			addFailedToast(
				'Failed to delete the profile',
				`Profile ${profile?.name} was not deleted\n${error}`
			);
		}
	};

	const profile = findProfileById($ProfileStore, data.id);
</script>

{#if profile}
	<main class="grid grid-cols-8 grid-rows-4 m-4 gap-2 items-center">
		<label
			use:melt={$root}
			class="col-span-4 text-lg"
			for="name">
			Profile Name:
		</label>
		<input
			id="name"
			class="font-bold ml-2 bg-transparent col-span-6 outline outline-2 outline-royal-indigo-400 rounded px-5 py-2"
			bind:value={profile.name} />
		<div class="row-start-2 -col-start-1 col-span-3 p-2 flex flex-row gap-2 justify-around">
			<button
				class="text-xl px-2 py-5 uppercase text-green-500"
				on:click={handleSave}>
				SAVE
			</button>
			<button
				class="text-xl px-2 py-5 uppercase text-red-600"
				on:click={handleDelete}>
				DELETE
			</button>
		</div>
	</main>
{:else}
	<span>Failed to fetch profile! Something is wrong with this id: {data.id}</span>
{/if}
