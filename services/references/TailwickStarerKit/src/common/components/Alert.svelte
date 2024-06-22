<!-- Alert.svelte -->
<script>
	import LucideIcon from "./LucideIcon.svelte";

    // export let type; // 'success', 'error', 'info', etc.
    let classname = "";
    export let isOpen = true;
    export { classname as class };
    export let dismissible = false;
    export let closeAriaLabel = "Close";
    export let dismissibleClass = "";
    export let toggle = undefined;
    export let customCloseButton = undefined;

    $: showClose = dismissible;
    $: closeClassNames = "alert-close " + (dismissibleClass ? dismissibleClass : "");
    $: handleToggle = toggle || (() => (isOpen = false));
</script>

{#if isOpen}
    <div class={classname}>
        {#if showClose}
        {#if customCloseButton}
        <slot name="customCloseButton" />
        {:else}
        <button
        type="button"
        class={closeClassNames}
        aria-label={closeAriaLabel}
        on:click={handleToggle}
        >
        <LucideIcon name="X" class="h-5"/>
    </button>
    
    {/if}
    {/if}
    <slot />
    </div>
{/if}
