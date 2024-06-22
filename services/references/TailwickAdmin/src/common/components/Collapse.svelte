<script>
    import { slide } from "svelte/transition";

    export let isOpen = false;
</script>

<div class="card mt-2 mb-0 collapsible-content {!isOpen ? "hidden":""}" transition:slide|global={{duration:500}}>
    <div class="card-body" >
        <slot />
    </div>
</div>