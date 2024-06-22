<script>
	import { getContext } from 'svelte';
	export let tag;
	const context = getContext('dropdownContext');

	$: popperOptions = {
		modifiers: [
			{ name: 'flip' },
			{
				name: 'offset',
				options: {
					offset: [0, 2]
				}
			}
		],
		placement: $context.direction
	};
	
</script>

{#if $context.isOpen}
	{#if tag == 'ul'}
		<ul {...$$restProps} use:$context.popperContent={popperOptions}>
			<slot />
		</ul>
	{:else}
		<div {...$$restProps} use:$context.popperContent={popperOptions}>
			<slot />
		</div>
	{/if}
{/if}
