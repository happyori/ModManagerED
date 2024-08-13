<script lang="ts">
	import { cn } from '$lib/utilities/cn';

	const classNames = '';
	export { classNames as class };

	const maincss = cn(
		'absolute bottom-2 right-2 bg-gradient-to-r from-cyan-600 to-royal-indigo-500 rounded-full shadow px-4 py-2.5 m-0',
		classNames
	);
</script>

<button
	on:click
	class={maincss}>
	<slot />
</button>
