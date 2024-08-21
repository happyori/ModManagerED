<script
	lang="ts"
	context="module">
	const tauri = await import('@tauri-apps/api');
</script>

<script lang="ts">
	import type { Profile } from '$generated/Profile';
	import Commands from '$lib/commands';

	import { findProfileById, ProfileStore } from '$lib/stores/profiles';
	import { getContext } from 'svelte';
	import type { PageData } from '../[id]/$types';
	import { melt, createLabel } from '@melt-ui/svelte';
	import { ToasterContext, type ToasterContextReturn } from '$lib/components/Toster.svelte';
	import { goto, invalidate } from '$app/navigation';

	export let data: PageData;
	const { addToast } = getContext<ToasterContextReturn>(ToasterContext);

	const {
		elements: { root }
	} = createLabel();

	const handleSave = async () => {
		try {
			await tauri.invoke<Profile>(Commands.UpdateProfile, { profile });
			await invalidate('profiles:data');
			addToast({
				data: {
					title: 'Successfully updated the profile',
					type: 'success'
				}
			});
		} catch (error) {
			addToast({
				data: {
					title: 'Failed to update profile',
					content: `Profile ${profile?.name} was not updated\n${error}`,
					type: 'error'
				}
			});
		}
	};

	const handleDelete = async () => {
		try {
			await tauri.invoke<Profile>(Commands.DeleteProfile, { profile });
			await invalidate('profiles:data');
			await goto('/profiles');
		} catch (error) {
			addToast({
				data: {
					title: 'Failed to delete the profile',
					content: `Profile ${profile?.name} was not deleted\n${error}`,
					type: 'error'
				}
			});
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
