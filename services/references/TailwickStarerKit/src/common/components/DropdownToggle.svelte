<script>
    import { getContext } from "svelte";
    const context = getContext("dropdownContext");
    export let tag = null;
    export let disabled = false;
    export let className = "";

    function toggleButton(e) {
        if (disabled) {
            e.preventDefault();
            return;
        }
        $context.toggle(e);
    }

   const getClassNameButton = className;
    const getClassNameLink = className;
    
</script>

{#if tag === "a"}
    <a
        use:$context.popperRef
        {...$$restProps}
        on:click
        on:click={toggleButton}
        href={"#"}
        aria-expanded={$context.isOpen}
        class={getClassNameLink}
    >
        <slot />
    </a>
{:else if tag === "div"}
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div
        use:$context.popperRef
        {...$$restProps}
        on:click
        on:click={toggleButton}
        aria-expanded={$context.isOpen}
        class={getClassNameButton}
    >
        <slot />
    </div>
{:else}
    <button
        use:$context.popperRef
        {...$$restProps}
        type="button"
        on:click
        on:click={toggleButton}
        aria-expanded={$context.isOpen}
        class={getClassNameButton}
    >
        <slot />
    </button>
{/if}
