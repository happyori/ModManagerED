<script lang="ts">
	import { ProfileStore } from '$lib/stores/profiles';
	import type { PageData } from '../[id]/$types';

	export let data: PageData;

	const profile = $ProfileStore.find((p) => p.id === data.id);
</script>

{#if profile}
	<main class="grid grid-cols-8 grid-rows-4 m-4 gap-2 items-center">
		<h1 class="col-span-4 text-lg">Profile Name:</h1>
		<input
			class="font-bold ml-2 bg-transparent col-span-6 outline outline-2 outline-royal-indigo-400 rounded px-5 py-2"
			bind:value={profile.name} />
		<div class="row-start-2 -col-start-1 col-span-3 p-2 flex flex-row gap-2 justify-around">
			<button class="text-xl px-2 py-5 uppercase text-green-500">SAVE</button>
			<button class="text-xl px-2 py-5 uppercase text-red-600">DELETE</button>
		</div>
	</main>
{:else}
	<span>Failed to fetch profile! Something is wrong with this id: {data.id}</span>
{/if}
