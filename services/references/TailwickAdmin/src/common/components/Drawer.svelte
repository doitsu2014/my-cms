<script>
	import { onDestroy, onMount } from 'svelte';

	export let isOpen = false;
	export let className = '';
	export let position = 'drawer-end';
	export let toggle;
	let backDrop;

	onMount(() => {
		backDrop = document.getElementById('backDropDiv');
	});

	onDestroy(() => {
		if (backDrop) {
			backDrop.classList.add("hidden");
			removeBackDropEventListener();
		}
	});

	$: {
		className = getPositionClass(position);
		className = isOpen ? className : className + " hidden";
		if(isOpen){
			onOpened();
		}else{
			if(backDrop){
				backDrop.classList.add('hidden');
				removeBackDropEventListener()
			}
		}
	}

	function getPositionClass(position) {
		switch (position.toLowerCase()) {
			case 'drawer-top':
				return ' top-0 left-0 right-0 md:h-80';
				break;
			case 'drawer-bottom':
				return ' bottom-0 left-0 right-0 md:h-80';
				break;
			case 'drawer-end':
				return ' ltr:right-0 rtl:left-0 md:w-96 inset-y-0';
				break;
			case 'drawer-start':
				return ' ltr:left-0 rtl:right-0 md:w-80 inset-y-0';
				break;
			default:
				break;
		}
	}

	function onOpened() {
		if (backDrop) {
			isOpen ? backDrop.classList.remove('hidden') : backDrop.classList.add('hidden');
			backDrop.addEventListener('click', onClickBackDrop);
		}
	}

	function onClickBackDrop(e) {
		toggle();
		backDrop.classList.add('hidden');
		removeBackDropEventListener();
	}

	function removeBackDropEventListener() {
		if (backDrop) {
			backDrop.removeEventListener('click', onClickBackDrop);
		}
	}
</script>

<div
	class='fixed flex flex-col w-full transition-transform duration-300 ease-in-out transform bg-white shadow dark:bg-zink-600 z-drawer {className}'
	
	{...$$restProps}
>
	<slot />
</div>
