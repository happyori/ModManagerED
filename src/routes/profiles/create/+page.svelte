<script lang="ts">
	import Commands from '$lib/commands';
	import { tauri } from '@tauri-apps/api';
	import { goto } from '$app/navigation';

	let profile_name = '';

	async function createProfile() {
		try {
			if (profile_name === '') {
				return console.error('Profile name needs to be set');
			}
			await tauri.invoke(Commands.CreateProfile, { profileDataModel: { name: profile_name } });
			goto('/profiles');
		} catch (e) {
			console.error(e);
		}
	}
</script>

<main class="flex flex-col justify-center items-center w-full h-full">
	<div class="flex flex-col items-center justify-start w-fit gap-2">
		<h1 class="text-2xl font-bold tracking-wide text-left">Profile Creation</h1>
		<input
			type="text"
			bind:value={profile_name}
			placeholder="Profile Name"
			class="outline outline-2 outline-royal-indigo-400 px-2 bg-zinc-800 m-4" />
		<button
			class="rounded outline-royal-indigo-800 outline-2 self-end mr-4 outline px-4 py-2"
			on:click={createProfile}>
			Create
		</button>
	</div>
</main>
