<script lang="ts">
	import { findProfileById, ProfileStore, SelectedProfile } from '$lib/stores/profiles';
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
			if ($SelectedProfile && $SelectedProfile.id === profile.id) {
				$SelectedProfile = profile;
			}
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
	<main class="flex flex-col gap-2 space-y-2xl">
		<h1 class="mt-24 golden-text text-lg">Edit Profile</h1>
		<div class="flex flex-row items-center">
			<label
				use:melt={$root}
				class="labels"
				for="name">
				<span>Name:</span>
				<input
					id="name"
					class="fancy-input"
					bind:value={profile.name} />
			</label>
		</div>
		<div class="mr-150px flex flex-row justify-end space-x-3">
			<button
				on:click={handleSave}
				class="w-24 rounded text-base tracking-wide outline-1 outline-green-400 outline transition-all duration-150 active:outline-pallete-accent hover:outline-green-700">
				Save
			</button>
			<button
				on:click={handleDelete}
				class="w-24 rounded text-center tracking-wide outline-1 outline-rose-500 outline transition-all duration-150 active:outline-pallete-accent hover:outline-rose-700">
				Delete
			</button>
		</div>
	</main>
{:else}
	<span>Failed to fetch profile! Something is wrong with this id: {data.id}</span>
{/if}

<style lang="scss">
	.labels {
		display: grid;
		grid-template-columns: 120px auto 1fr;
		--at-apply: tracking-tight text-sm items-center;
		span {
			font-weight: bold;
		}
	}
</style>
