<script>
    import { afterUpdate, getContext, onMount } from "svelte";
    export let className = "";
    export let tabId = undefined;
    const tabs = getContext("tabs");
    // const { activeTabId, setActiveTab } = getContext("tabContent");
    const activeTabId = getContext("tabContent");

    onMount(() => {
    });

    function setActiveTab(activeTab){
        $activeTabId.setActiveTab(activeTab)
    }
</script>

<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
<li {...$$restProps} class={className} on:click={() => setActiveTab(tabId)}>
    <slot />
</li>
