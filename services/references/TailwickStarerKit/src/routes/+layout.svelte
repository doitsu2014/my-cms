<style lang="scss">
	@import '/assets/scss/tailwind.scss';
	@import '/assets/scss/icons.scss';
</style>

<svelte:head>
	<script src="https://cdn.jsdelivr.net/npm/apexcharts"></script>
</svelte:head>

<slot />
<div class="fixed inset-0 bg-slate-900/40 dark:bg-zink-800/70 z-[1049] backdrop-overlay hidden" id="backDropDiv"></div>
