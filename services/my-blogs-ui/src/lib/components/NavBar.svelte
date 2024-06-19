<script>
	let isOpen = false;

	function toggleNavbar() {
		isOpen = !isOpen;
	}
</script>

<nav class="rounded border-gray-200 bg-background px-2 py-2.5 dark:bg-gray-800 sm:px-4">
	<div class="container mx-auto flex flex-wrap items-center justify-between">
		<a href="/" class="flex items-center">
			<img src="/logo.svg" class="mr-3 h-6 sm:h-9" alt="Logo" />
			<span
				class="self-center whitespace-nowrap text-xl font-semibold text-text dark:text-secondary"
				>Your Brand</span
			>
		</a>
		<button
			on:click={toggleNavbar}
			type="button"
			class="ml-3 inline-flex items-center rounded-lg p-2 text-sm text-text hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-secondary dark:hover:bg-gray-700 dark:focus:ring-gray-600 md:hidden"
			aria-controls="navbar-default"
			aria-expanded={isOpen ? 'true' : 'false'}
		>
			<span class="sr-only">Open main menu</span>
			<svg
				class="h-6 w-6"
				aria-hidden="true"
				fill="currentColor"
				viewBox="0 0 20 20"
				xmlns="http://www.w3.org/2000/svg"
			>
				<path
					fill-rule="evenodd"
					d="M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 15a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z"
					clip-rule="evenodd"
				></path>
			</svg>
		</button>
		<div class={`w-full md:block md:w-auto ${isOpen ? '' : 'hidden'}`} id="navbar-default">
			<ul class="mt-4 flex flex-col md:mt-0 md:flex-row md:space-x-8 md:text-sm md:font-medium">
				<li>
					<a
						href="/"
						class="block rounded bg-primary py-2 pl-3 pr-4 text-text dark:bg-primary-dark dark:text-secondary md:bg-transparent md:p-0 md:text-primary dark:md:bg-transparent"
						aria-current="page">Home</a
					>
				</li>
				<li>
					<a
						href="/about"
						class="block border-b border-gray-100 py-2 pl-3 pr-4 text-text hover:bg-gray-50 dark:border-gray-700 dark:text-secondary dark:hover:bg-gray-700 dark:hover:text-secondary md:border-0 md:p-0 md:hover:bg-transparent md:hover:text-primary dark:md:hover:bg-transparent"
						>About</a
					>
				</li>
				<li>
					<a
						href="/blog"
						class="block border-b border-gray-100 py-2 pl-3 pr-4 text-text hover:bg-gray-50 dark:border-gray-700 dark:text-secondary dark:hover:bg-gray-700 dark:hover:text-secondary md:border-0 md:p-0 md:hover:bg-transparent md:hover:text-primary dark:md:hover:bg-transparent"
						>Blog</a
					>
				</li>
			</ul>
		</div>
	</div>
</nav>

<style>
	/* Add your custom styles here */
</style>
