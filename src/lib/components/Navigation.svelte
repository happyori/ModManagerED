<script lang="ts">
	import Icon from '@iconify/svelte';
</script>

<nav class="col-span-3 row-span-10 h-auto max-w-auto flex flex-col justify-between font-default">
	<div class="flex flex-col *:ml-2 *:mr-6 space-y-lg">
		<a
			href="/"
			class="link">
			<Icon
				icon="line-md:home-simple"
				width="24px"
				height="24px" />
			<span>Home</span>
		</a>
		<a
			class="link"
			href="/profiles">
			<Icon
				icon="line-md:list-3"
				width="24px"
				height="24px" />
			<span>Profiles</span>
		</a>
		<a
			class="link"
			href="/mods">
			<Icon
				icon="line-md:switch"
				width="24px"
				height="24px" />
			<span>Mods</span>
		</a>
	</div>
	<a
		href="/settings"
		class="link mb-2 ml-2 mr-4 !px-3">
		<Icon
			icon="line-md:cog-filled-loop"
			width="24px"
			height="24px" />
		<span>Settings</span>
	</a>
</nav>

<style lang="scss">
	@use 'sass:color';

	.link {
		display: flex;
		flex-direction: row;
		align-items: center;
		--at-apply: transition rounded px-4 py-1;
		transition-timing-function: ease-in-out;
		width: fit-content;

		span {
			margin-left: 4px;
		}

		&:hover {
			background-color: color.scale(#b83113, $alpha: -80%);
			backdrop-filter: blur(10px);
			--at-apply: shadow-lg;
		}
	}
</style>
