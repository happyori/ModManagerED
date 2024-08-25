<script lang="ts">
	import { goto, invalidate } from '$app/navigation';
	import { createTauRPCProxy } from '$generated/binding';

	let profile_name = '';

	async function createProfile() {
		const rpc = await createTauRPCProxy();
		try {
			if (profile_name === '') {
				return console.error('Profile name needs to be set');
			}
			await rpc.api.profile.create({ name: profile_name });
			await invalidate('profiles:data');
			await goto('/profiles');
		} catch (e) {
			console.error(e);
		}
	}
</script>

<main class="h-full w-full flex flex-col items-center justify-center">
	<div class="w-fit flex flex-col items-center justify-start gap-2">
		<h1 class="text-left text-2xl font-bold tracking-wide">Profile Creation</h1>
		<input
			type="text"
			bind:value={profile_name}
			placeholder="Profile Name"
			class="m-4 rounded bg-zinc-800 px-2 py-0.5 outline-2 outline-pallete-accent outline" />
		<button
			class="mr-4 self-end rounded px-4 py-2 outline-2 outline-pallete-accent outline transition-colors hover:bg-pallete-accent hover:text-black"
			on:click={createProfile}>
			Create
		</button>
	</div>
</main>
