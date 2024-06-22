<script lang="ts">
	import type { BlogData } from '$lib/types';
	export let data: BlogData;
</script>

<h1>Blog</h1>

<ul>
	{#each data.summaries as { title, slug }}
		<li><a href="/blog/{slug}">{title}</a></li>
	{/each}
</ul>
