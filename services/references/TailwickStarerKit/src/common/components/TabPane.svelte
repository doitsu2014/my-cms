<script>
    import { getContext, onMount } from "svelte";
    export let className = "tab-pane ";
    export let active;
    export let tabId = undefined;

    // className = active ? "block" : "hidden";

    // const tabs = getContext("tabs");
    const activeTabId = getContext("tabContent");

    let tabOpen = active;

    if (active) {
        $activeTabId.currTab = tabId;
        tabOpen = "block";
    } else {
        tabOpen = "hidden";
    }

    // tabOpen = $activeTabId.currTab == tabId ? "block" : "hidden";
    console.log("activeTabId", tabOpen, active, tabId);
    // $: className = $activeTabId == tabId ? className + "block" : className + "hidden";
    onMount(() => {
        if (active) $activeTabId.setActiveTab(tabId);
    });

    if ($activeTabId.currTab == tabId) {
        console.log("if", tabId);
        tabOpen = "block";
    } else {
        console.log("else", tabId);
        tabOpen = "hidden";
    }
</script>

<div class="{className}{tabOpen}">
    <slot />
</div>

{#if $activeTabId.currTab}
    <p>{$activeTabId.currTab} {tabId} {$activeTabId.currTab == tabId}</p>
{/if}
