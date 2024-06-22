<script>
    import Dropdown from "./Dropdown.svelte";
</script>


<Dropdown {...$$restProps} on:click>
    <slot />
</Dropdown>