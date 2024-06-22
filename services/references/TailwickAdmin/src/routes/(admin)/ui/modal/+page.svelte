<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import LucideIcon from '../../../../common/components/LucideIcon.svelte';
	import Modal from '../../../../common/components/Modal.svelte';

    let isOpen = false;
    const toggle = () => isOpen = !isOpen;

    let isTopOpen = false;
    const toggleTop = () => isTopOpen = !isTopOpen;

    let isBottomOpen = false;
    const toggleBottom = () => isBottomOpen = !isBottomOpen;

    let isExtraSmallOpen = false;
    const toggleExtraSmallOpen = () => isExtraSmallOpen = !isExtraSmallOpen;

    let isSmallOpen = false;
    const toggleSmallOpen = () => isSmallOpen = !isSmallOpen;

    let isLargeOpen = false;
    const toggleLargeOpen = () => isLargeOpen = !isLargeOpen;

    let isExtraLargeOpen = false;
    const toggleExtraLargeOpen = () => isExtraLargeOpen = !isExtraLargeOpen;

    let isFullScreenOpen = false;
    const toggleFullScreenOpen = () => isFullScreenOpen = !isFullScreenOpen;
</script>

<HeadTitle title="Modal" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Modal" pagetitle="UI Elements" />
    <div>
        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">Default</h6>
                <div class="flex flex-wrap gap-2">
                    <button type="button" class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20" on:click={toggle}>Default Modal</button>

                    <button type="button" class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20" on:click={toggleTop}>Top Modal</button>

                    <button type="button" class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20" on:click={toggleBottom}>Bottom Modal</button>

                </div>
            </div>
        </div>

        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-gray-800 text-15 dark:text-white">Modal Sizes</h6>
                
                <div class="flex flex-wrap gap-2">
                    <button  type="button" class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20" on:click={toggleExtraSmallOpen}>Extra Small Modal</button>
                    
                    <button  type="button" class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20" on:click={toggleSmallOpen}>Small Modal</button>
                    
                    <button  type="button" class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20" on:click={toggle}>Default Modal</button>

                    <button  type="button" class="text-white transition-all duration-200 ease-linear btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:focus:ring-custom-400/20" on:click={toggleLargeOpen}>Large Modal</button>

                    <button  type="button" class="text-white transition-all duration-200 ease-linear btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:focus:ring-custom-400/20" on:click={toggleExtraLargeOpen}>Extra Large Modal</button>

                    <button  type="button" class="text-white transition-all duration-200 ease-linear btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:focus:ring-custom-400/20" on:click={toggleFullScreenOpen}>Full Screen Modal</button>

                </div>
        
            </div>
        </div>
    </div>

    <Modal id="defaultModal" modal-center className="-translate-y-2/4" {toggle} {isOpen}>
        <div class="w-screen md:w-[30rem] bg-white shadow rounded-md dark:bg-zink-600 flex flex-col h-full">
            <div class="flex items-center justify-between p-4 border-b border-slate-200 dark:border-zink-500">
                <h5 class="text-16">Modal Heading</h5>
                <button class="transition-all duration-200 ease-linear text-slate-500 hover:text-red-500 dark:text-zink-200 dark:hover:text-red-500" on:click={toggle}><LucideIcon name="X" class="size-5"/></button>
            </div>
            <div class="max-h-[calc(theme('height.screen')_-_180px)] p-4 overflow-y-auto">
                <h5 class="mb-3 text-16">Modal Content</h5>
                <p class="text-slate-500 dark:text-zink-200">They all have something to say beyond the words on the page. They can come across as casual or neutral, exotic or graphic.</p>
            </div>
            <div class="flex items-center justify-between p-4 mt-auto border-t border-slate-200 dark:border-zink-500">
                <h5 class="text-16">Modal Footer</h5>
            </div>
        </div>
    </Modal>

    <Modal id="TopModal" modal-top className="" toggle={toggleTop} isOpen={isTopOpen}>
        <div class="w-screen md:w-[30rem] bg-white shadow rounded-md dark:bg-zink-600 flex flex-col">
            <div class="flex items-center justify-between p-4 border-b border-slate-200 dark:border-zink-500">
                <h5 class="text-16">Modal Heading</h5>
                <button class="transition-all duration-200 ease-linear text-slate-500 hover:text-red-500 dark:text-zink-200 dark:hover:text-red-500" on:click={toggleTop}><LucideIcon name="X"  class="size-5"/></button>
            </div>
            <div class="max-h-[calc(theme('height.screen')_-_180px)] p-4 overflow-y-auto">
                <h5 class="mb-3 text-16">Modal Content</h5>
                <p class="text-slate-500 dark:text-zink-200">They all have something to say beyond the words on the page. They can come across as casual or neutral, exotic or graphic.</p>
                <p class="text-slate-500 dark:text-zink-200">They all have something to say beyond the words on the page. They can come across as casual or neutral, exotic or graphic.</p>
                <p class="text-slate-500 dark:text-zink-200">They all have something to say beyond the words on the page. They can come across as casual or neutral, exotic or graphic.</p>
                <p class="text-slate-500 dark:text-zink-200">They all have something to say beyond the words on the page. They can come across as casual or neutral, exotic or graphic.</p>
                <p class="text-slate-500 dark:text-zink-200">They all have something to say beyond the words on the page. They can come across as casual or neutral, exotic or graphic.</p>
                <p class="text-slate-500 dark:text-zink-200">They all have something to say beyond the words on the page. They can come across as casual or neutral, exotic or graphic.</p>
                <p class="text-slate-500 dark:text-zink-200">They all have something to say beyond the words on the page. They can come across as casual or neutral, exotic or graphic.</p>
                <p class="text-slate-500 dark:text-zink-200">They all have something to say beyond the words on the page. They can come across as casual or neutral, exotic or graphic.</p>
                <p class="text-slate-500 dark:text-zink-200">They all have something to say beyond the words on the page. They can come across as casual or neutral, exotic or graphic.</p>
                <p class="text-slate-500 dark:text-zink-200">They all have something to say beyond the words on the page. They can come across as casual or neutral, exotic or graphic.</p>
                <p class="text-slate-500 dark:text-zink-200">They all have something to say beyond the words on the page. They can come across as casual or neutral, exotic or graphic.</p>
                <p class="text-slate-500 dark:text-zink-200">They all have something to say beyond the words on the page. They can come across as casual or neutral, exotic or graphic.</p>
                <p class="text-slate-500 dark:text-zink-200">They all have something to say beyond the words on the page. They can come across as casual or neutral, exotic or graphic.</p>
                <p class="text-slate-500 dark:text-zink-200">They all have something to say beyond the words on the page. They can come across as casual or neutral, exotic or graphic.</p>
                <p class="text-slate-500 dark:text-zink-200">They all have something to say beyond the words on the page. They can come across as casual or neutral, exotic or graphic.</p>
                <p class="text-slate-500 dark:text-zink-200">They all have something to say beyond the words on the page. They can come across as casual or neutral, exotic or graphic.</p>
                <p class="text-slate-500 dark:text-zink-200">They all have something to say beyond the words on the page. They can come across as casual or neutral, exotic or graphic.</p>
                <p class="text-slate-500 dark:text-zink-200">They all have something to say beyond the words on the page. They can come across as casual or neutral, exotic or graphic.</p>
                <p class="text-slate-500 dark:text-zink-200">They all have something to say beyond the words on the page. They can come across as casual or neutral, exotic or graphic.</p>
            </div>
            <div class="flex items-center justify-between p-4 mt-auto border-t border-slate-200 dark:border-zink-500">
                <h5 class="text-16">Modal Footer</h5>
            </div>
        </div>
    </Modal>

    <Modal id="BottomModal" modal-bottom className="" toggle={toggleBottom} isOpen={isBottomOpen}>
        <div class="w-screen md:w-[30rem] bg-white shadow rounded-md dark:bg-zink-600 flex flex-col h-full">
            <div class="flex items-center justify-between p-4 border-b border-slate-200 dark:border-zink-500">
                <h5 class="text-16">Modal Heading</h5>
                <button class="transition-all duration-200 ease-linear text-slate-500 hover:text-red-500 dark:text-zink-200 dark:hover:text-red-500" on:click={toggleBottom}><LucideIcon name="X"  class="size-5"/></button>
            </div>
            <div class="max-h-[calc(theme('height.screen')_-_180px)] p-4 overflow-y-auto">
                <h5 class="mb-3 text-16">Modal Content</h5>
                <p class="text-slate-500 dark:text-zink-200">They all have something to say beyond the words on the page. They can come across as casual or neutral, exotic or graphic.</p>
            </div>
            <div class="flex items-center justify-between p-4 mt-auto border-t border-slate-200 dark:border-zink-500">
                <h5 class="text-16">Modal Footer</h5>
            </div>
        </div>
    </Modal>

    <Modal modal-center className="-translate-y-2/4" toggle={toggleExtraSmallOpen} isOpen={isExtraSmallOpen}>
        <div class="w-screen md:w-[20rem] bg-white shadow rounded-md dark:bg-zink-600 flex flex-col h-full">
            <div class="flex items-center justify-between p-4 border-b border-slate-200 dark:border-zink-500">
                <h5 class="text-16">Modal Heading</h5>
                <button data-modal-close="extraSmallModal" class="transition-all duration-200 ease-linear text-slate-500 hover:text-red-500 dark:text-zink-200 dark:hover:text-red-500" on:click={toggleExtraSmallOpen}><LucideIcon name="X" class="size-5"/></button>
            </div>
            <div class="max-h-[calc(theme('height.screen')_-_180px)] p-4 overflow-y-auto">
                <h5 class="mb-3 text-16">Modal Content</h5>
                <p class="text-slate-500 dark:text-zink-200">They all have something to say beyond the words on the page. They can come across as casual or neutral, exotic or graphic.</p>
            </div>
            <div class="flex items-center justify-between p-4 mt-auto border-t border-slate-200 dark:border-zink-500">
                <h5 class="text-16">Modal Footer</h5>
            </div>
        </div>
    </Modal>

    <Modal modal-center className="-translate-y-2/4" toggle={toggleSmallOpen} isOpen={isSmallOpen}>
        <div class="w-screen md:w-[25rem] bg-white shadow rounded-md dark:bg-zink-600 flex flex-col h-full">
            <div class="flex items-center justify-between p-4 border-b border-slate-200 dark:border-zink-500">
                <h5 class="text-16">Modal Heading</h5>
                <button data-modal-close="smallModal" class="transition-all duration-200 ease-linear text-slate-500 hover:text-red-500 dark:text-zink-200 dark:hover:text-red-500" on:click={toggleSmallOpen}><LucideIcon name="X" class="size-5"/></button>
            </div>
            <div class="max-h-[calc(theme('height.screen')_-_180px)] p-4 overflow-y-auto">
                <h5 class="mb-3 text-16">Modal Content</h5>
                <p class="text-slate-500 dark:text-zink-200">They all have something to say beyond the words on the page. They can come across as casual or neutral, exotic or graphic.</p>
            </div>
            <div class="flex items-center justify-between p-4 mt-auto border-t border-slate-200 dark:border-zink-500">
                <h5 class="text-16">Modal Footer</h5>
            </div>
        </div>
    </Modal>

    <Modal modal-center className="-translate-y-2/4" toggle={toggleLargeOpen} isOpen={isLargeOpen}>
        <div class="w-screen md:w-[40rem] bg-white shadow rounded-md dark:bg-zink-600 flex flex-col h-full">
            <div class="flex items-center justify-between p-4 border-b border-slate-200 dark:border-zink-500">
                <h5 class="text-16">Modal Heading</h5>
                <button data-modal-close="largeModal" class="transition-all duration-200 ease-linear text-slate-500 hover:text-red-500 dark:text-zink-200 dark:hover:text-red-500" on:click={toggleLargeOpen}><LucideIcon name="X" class="size-5"/></button>
            </div>
            <div class="max-h-[calc(theme('height.screen')_-_180px)] p-4 overflow-y-auto">
                <h5 class="mb-3 text-16">Modal Content</h5>
                <p class="text-slate-500 dark:text-zink-200">They all have something to say beyond the words on the page. They can come across as casual or neutral, exotic or graphic.</p>
            </div>
            <div class="flex items-center justify-between p-4 mt-auto border-t border-slate-200 dark:border-zink-500">
                <h5 class="text-16">Modal Footer</h5>
            </div>
        </div>
    </Modal>

    <Modal modal-center className="-translate-y-2/4" toggle={toggleExtraLargeOpen} isOpen={isExtraLargeOpen}>
        <div class="w-screen lg:w-[55rem] bg-white shadow rounded-md dark:bg-zink-600 flex flex-col h-full">
            <div class="flex items-center justify-between p-4 border-b border-slate-200 dark:border-zink-500">
                <h5 class="text-16">Modal Heading</h5>
                <button data-modal-close="extraLargeModal" class="transition-all duration-200 ease-linear text-slate-500 hover:text-red-500 dark:text-zink-200 dark:hover:text-red-500" on:click={toggleExtraLargeOpen}><LucideIcon name="X" class="size-5"/></button>
            </div>
            <div class="max-h-[calc(theme('height.screen')_-_180px)] p-4 overflow-y-auto">
                <h5 class="mb-3 text-16">Modal Content</h5>
                <p class="text-slate-500 dark:text-zink-200">They all have something to say beyond the words on the page. They can come across as casual or neutral, exotic or graphic.</p>
            </div>
            <div class="flex items-center justify-between p-4 mt-auto border-t border-slate-200 dark:border-zink-500">
                <h5 class="text-16">Modal Footer</h5>
            </div>
        </div>
    </Modal>

    <Modal modal-center className="inset-0" toggle={toggleFullScreenOpen} isOpen={isFullScreenOpen} fullscreen>
        <div class="flex flex-col w-full h-full bg-white dark:bg-zink-600">
            <div class="flex items-center justify-between p-4 border-b border-slate-200 dark:border-zink-500">
                <h5 class="text-16">Modal Heading</h5>
                <button data-modal-close="fullScreenModal" class="transition-all duration-200 ease-linear text-slate-500 hover:text-red-500 dark:text-zink-200 dark:hover:text-red-500" on:click={toggleFullScreenOpen}>
                    <LucideIcon name="X" class="size-5"/></button>
            </div>
            <div class="p-4">
                <h5 class="mb-3 text-16">Modal Content</h5>
                <p class="text-slate-500 dark:text-zink-200">They all have something to say beyond the words on the page. They can come across as casual or neutral, exotic or graphic.</p>
            </div>
            <div class="flex items-center justify-between p-4 mt-auto border-t border-slate-200 dark:border-zink-500">
                <h5 class="text-16">Modal Footer</h5>
            </div>
        </div>
    </Modal>
</div>
