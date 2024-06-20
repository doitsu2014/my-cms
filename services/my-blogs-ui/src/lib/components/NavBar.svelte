<script>
	export let title = 'My Website';

	function toggleDarkMode() {
		document.documentElement.classList.toggle('dark');
	}
</script>

<nav class="bg-surface-light p-4 shadow-lg dark:bg-surface-dark">
	<div class="container mx-auto flex items-center justify-between">
		<div class="text-2xl font-bold text-primary-light dark:text-primary-dark">
			{title}
		</div>
		<div class="space-x-4">
			<a href="/" class="text-secondary-light dark:text-secondary-dark">Home</a>
			<a href="/about" class="text-tertiary-light dark:text-tertiary-dark">About</a>
			<a href="/blog" class="text-success-light dark:text-success-dark">Blog</a>
		</div>
		<button
			class="rounded bg-primary-light px-4 py-2 font-bold text-white dark:bg-primary-dark"
			on:click={toggleDarkMode}
		>
			Toggle Dark Mode
		</button>
	</div>
</nav>

<!-- <style> -->
<!-- 	:global(body) { -->
<!-- 		@apply bg-white dark:bg-black; -->
<!-- 	} -->
<!-- </style> -->
