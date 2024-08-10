<script>
	import '../app.css';
</script>

<div class="grid grid-cols-12 grid-rows-10 w-screen h-screen bg-zinc-900 text-white">
	<div
		id="action_corner"
		class="col-span-3 row-span-3 flex z-10 rounded-br-3xl drop-shadow-xl justify-center items-center flex-col bg-zinc-700">
		<enhanced:img
			src="$lib/assets/eldenringlogo.png?w=100"
			class="mb-4"
			alt="Golden Order Logo" />
		<button class="bg-royal-indigo-700 p-2 rounded-full px-4 cursor-auto text-white">Start</button>
	</div>
	<div class="col-start-4 col-end-13 flex flex-col w-full">
		<div class="flex flex-row justify-between w-full h-24 align-middle bg-zinc-800">
			<h1
				class="bg-clip-text bg-gradient-to-br text-4xl text-transparent font-bold from-yellow-500 px-5 cursor-default select-none text-center self-center to-amber-600 to-60%">
				Golden Order
			</h1>
		</div>
	</div>

	<nav
		class="row-span-10 row-start-4 col-span-3 max-w-fit h-auto bg-zinc-800 gap-2 pt-5 pr-20 flex flex-col">
		<a
			class="text-white p-2"
			href="/profiles">
			Profiles
		</a>
		<a
			class="text-white p-2"
			href="/mods">
			Main Mods
		</a>
		<a
			class="text-white p-2"
			href="/dlls">
			DLL Mods
		</a>
	</nav>
	<section class="row-start-2 row-span-10 col-start-4 col-span-12 w-full h-full">
		<slot />
	</section>
</div>
