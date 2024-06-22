<script>
    import { Body, classList, style } from 'svelte-body';
	import { browser } from '$app/environment';

	if (browser) {
		document.body.setAttribute("class","");
		document.documentElement.classList.add("overflow-x-hidden")
	}
</script>
<svelte:body use:classList={'text-base bg-white text-body font-public dark:text-zink-50 dark:bg-zinc-900'} />
<slot />