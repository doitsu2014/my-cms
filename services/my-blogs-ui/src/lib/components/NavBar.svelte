<script>
	export let title = 'D Tech';

	function toggleDarkMode() {
		document.documentElement.classList.toggle('dark');
	}
</script>

<nav class="bg-surface-100 p-4 shadow-lg transition-colors duration-200 dark:bg-surface-900">
	<div class="container mx-auto flex items-center justify-between">
		<div class="text-2xl font-bold text-black dark:text-white">
			<a href="/">{title}</a>
		</div>
		<div class="space-x-4">
			<a
				href="/"
				class="border-2 border-surface-100 p-2 text-black hover:border-2 hover:border-primary dark:border-surface-900 dark:text-white dark:hover:border-primary-50 transition-colors duration-200"
				>Home</a
			>
			<a
				href="/blog"
				class="border-2 border-surface-100 p-2 text-black hover:border-2 hover:border-primary dark:border-surface-900 dark:text-white dark:hover:border-primary-50 transition-colors duration-200"
				>Blog</a
			>
			<a
				href="/about"
				class="border-2 border-surface-100 p-2 text-black hover:border-2 hover:border-primary dark:border-surface-900 dark:text-white dark:hover:border-primary-50 transition-colors duration-200"
				>About</a
			>
		</div>
		<button
			class="rounded bg-primary px-4 py-2 font-bold text-white dark:bg-primary-50 dark:text-black"
			on:click={toggleDarkMode}
		>
			Toggle Dark Mode
		</button>
	</div>
</nav>

<style>
</style>
