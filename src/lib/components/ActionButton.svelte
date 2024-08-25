<script lang="ts">
	import { spring } from 'svelte/motion';

	const rotation = spring(0, {
		damping: 0.12,
		precision: 0.01,
		stiffness: 0.05
	});

	const handleMouseOver = () => rotation.set(90);
	const handleMouseOut = () => rotation.set(0);
</script>

<button
	on:click
	on:mouseover={handleMouseOver}
	on:focus={handleMouseOver}
	on:mouseleave={handleMouseOut}
	class="absolute bottom-2 right-2 m-0 size-10 flex items-center justify-center rounded-full bg-gradient-from-pallete-accent to-pallete-complement bg-gradient-linear shape-br text-pallete-text shadow">
	<span style="transform: rotate({$rotation}deg);">
		<slot />
	</span>
</button>
