<script>
	import '../app.css';
	import NavBar from '$lib/components/NavBar.svelte';
	let darkMode = false;

	function toggleDarkMode() {
		darkMode = !darkMode;
		if (darkMode) {
			document.documentElement.classList.add('dark');
		} else {
			document.documentElement.classList.remove('dark');
		}
	}
</script>

<button on:click={toggleDarkMode} class="rounded bg-primary p-2 text-secondary">
	Toggle Dark Mode
</button>

<NavBar></NavBar>

<slot></slot>

<style></style>
