<script>
	import { onDestroy, onMount } from "svelte";

    export let isOpen = false;
    export let className = "";
    export let toggle;
    export let fullscreen;

    let _mouseDownElement;
    let _removeEscListener;
    let backDropElement;

    onMount(()=>{
        backDropElement = document.getElementById('backDropDiv');
    })

    onDestroy(() => {
		if (backDropElement) {
			backDropElement.classList.add("hidden");
			removeBackDropEventListener();
		}
	});

    $: {
		if(isOpen){
			onOpened();
		}else{
			if(backDropElement){
				backDropElement.classList.add('hidden');
				removeBackDropEventListener()
			}
		}
	}

    function onClickBackDrop(e) {
		toggle();
		backDropElement.classList.add('hidden');
		removeBackDropEventListener();
	}

    function removeBackDropEventListener() {
		if (backDropElement) {
			backDropElement.removeEventListener('click', onClickBackDrop);
		}
	}

    function onOpened() {
		if (backDropElement) {
			isOpen ? backDropElement.classList.remove('hidden') : backDropElement.classList.add('hidden');
			backDropElement.addEventListener('click', onClickBackDrop);
		}
	}

    // backdrop event handlers
    function handleBackdropClick(event) {
        if (event.target === event.currentTarget) {
            // Clicked on the backdrop, close the modal
            toggle();
        }
    }

    
    function onModalOpened() {
        console.log("Modal opened");
        _removeEscListener = document.addEventListener("keydown", (event) => {
            console.log("event.key", event.key);
            if (event.key && event.key === "Escape") {
                toggle(event);
            }
        });
    }

    function handleKeyDown(event) {
        // Close modal if Esc key is pressed
        if (event.key === "Escape") {
            closeModal();
        }
    }

    function closeModal() {
        console.log("Closing modal");
        // Remove the event listener when the modal is closed
        _removeEscListener();
        // Additional logic to close the modal (you can use the provided `toggle` prop)
        if (toggle) {
            toggle();
        }
    }

    function handleBackdropMouseDown(e) {
        _mouseDownElement = e.target;
    }
</script>

{#if isOpen}
    <!-- svelte-ignore a11y-no-static-element-interactions -->
    <div
        class="fixed flex flex-col transition-all duration-300 ease-in-out  z-drawer  {className} {!fullscreen ? "-translate-x-2/4 left-2/4":""}"
               
        {toggle}
        {...$$restProps}
        on:introstart={onModalOpened}
        on:mousedown={handleBackdropMouseDown}
    >
        <slot />
    </div>
{/if}
