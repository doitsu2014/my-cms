<script>
    import { createEventDispatcher, setContext } from "svelte";
    import { writable } from "svelte/store";

    const dispatch = createEventDispatcher();

    export let className = "";
    let currTab;
    // export let pills = false;

    const activeTabId = writable();
    setContext("tabContent", activeTabId);

    $: {
        activeTabId.update(() => {
            return {
                activeTabId,
                currTab,
                setActiveTab: (tabId) => {
                    // activeTabId.set(tabId);
                    currTab = tabId;
                    
                    activeTabId.set(currTab);
                    console.log("Current tabId", currTab);
                },
            };
        });
    }
</script>

<div {...$$restProps} class={className}>
    <slot />
</div>
