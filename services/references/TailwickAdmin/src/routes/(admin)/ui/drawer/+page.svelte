<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import Drawer from '../../../../common/components/Drawer.svelte';
	import LucideIcon from '../../../../common/components/LucideIcon.svelte';

    let drawerEnd = false;
    let drawerStart = false;
    let drawerBottom = false;
    let drawerTop = false;
    const toggleTop = () => drawerTop = !drawerTop;
</script>

<HeadTitle title="Drawer" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Drawer" pagetitle="UI Elements" />
    <div>
        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">Default</h6>
                <div class="flex flex-wrap gap-2">
                    <button type="button" class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20" on:click={() => drawerEnd = !drawerEnd}>End Button</button>
                    
                    <button type="button" class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20" on:click={() => drawerStart = !drawerStart}>Start Drawer</button>
                    
                    
                    <button type="button" class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20" on:click={() => drawerBottom = !drawerBottom}>Bottom Drawer</button>
                    
                    
                    <button type="button" class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20" on:click={() => drawerTop = !drawerTop}>Top Drawer</button>
                    
                </div>
            </div>
        </div><!--end card-->
        
    </div>
</div>

<Drawer position="drawer-end" isOpen={drawerEnd} toggle={() => drawerEnd = !drawerEnd}>
    <div class="flex items-center justify-between p-4 border-b card-body border-slate-200 dark:border-zink-500">
        <h6 class="text-15">Drawer Heading</h6>
        <button on:click={() => drawerEnd = !drawerEnd}>
            <LucideIcon name="X" class="size-4 transition-all duration-200 ease-linear text-slate-500 hover:text-slate-700 dark:text-zink-200 dark:hover:text-zink-50"/></button>
    </div>
    <div class="h-full p-4 overflow-y-auto">
        <div class="card-body">
            <h6 class="mb-4 text-15">Drawer Content</h6>
            <p class="text-slate-500 dark:text-zink-200">They all have something to say beyond the words on the page. They can come across as casual or neutral, exotic or graphic.</p>
        </div>
    </div>
    <div class="flex items-center justify-between p-4 border-t border-slate-200 dark:border-zink-500">
        <h6 class="text-15">Drawer Footer</h6>
    </div>
</Drawer>

<Drawer position="drawer-start" isOpen={drawerStart} toggle={() => drawerStart = !drawerStart}>
    <div class="flex items-center justify-between p-4 border-b card-body border-slate-200 dark:border-zink-500">
        <h6 class="text-15">Drawer Heading</h6>
        <button on:click={() => drawerStart = !drawerStart}>
            <LucideIcon name="X" class="size-4 transition-all duration-200 ease-linear text-slate-500 hover:text-slate-700 dark:text-zink-200 dark:hover:text-zink-50"/></button>
    </div>
    <div class="h-full p-4 overflow-y-auto">
        <div class="card-body">
            <h6 class="mb-4 text-15">Drawer Content</h6>
            <p class="text-slate-500 dark:text-zink-200">They all have something to say beyond the words on the page. They can come across as casual or neutral, exotic or graphic.</p>
        </div>
    </div>
    <div class="flex items-center justify-between p-4 border-t border-slate-200 dark:border-zink-500">
        <h6 class="text-15">Drawer Footer</h6>
    </div>
</Drawer>

<Drawer position="drawer-bottom" isOpen={drawerBottom} toggle={() => drawerBottom = !drawerBottom}>
    <div class="flex items-center justify-between p-4 border-b card-body border-slate-200 dark:border-zink-500">
        <h6 class="text-15">Drawer Heading</h6>
        <button on:click={() => drawerBottom = !drawerBottom}>
            <LucideIcon name="X" class="size-4 transition-all duration-200 ease-linear text-slate-500 hover:text-slate-700 dark:text-zink-200 dark:hover:text-zink-50"/></button>
    </div>
    <div class="h-full p-4 overflow-y-auto">
        <div class="card-body">
            <h6 class="mb-4 text-15">Drawer Content</h6>
            <p class="text-slate-500 dark:text-zink-200">They all have something to say beyond the words on the page. They can come across as casual or neutral, exotic or graphic.</p>
        </div>
    </div>
    <div class="flex items-center justify-between p-4 border-t border-slate-200 dark:border-zink-500">
        <h6 class="text-15">Drawer Footer</h6>
    </div>
</Drawer>

<Drawer position="drawer-top" isOpen={drawerTop} toggle={toggleTop}>
    <div class="flex items-center justify-between p-4 border-b card-body border-slate-200 dark:border-zink-500">
        <h6 class="text-15">Drawer Heading</h6>
        <button on:click={toggleTop}>
            <LucideIcon name="X" class="size-4 transition-all duration-200 ease-linear text-slate-500 hover:text-slate-700 dark:text-zink-200 dark:hover:text-zink-50"/></button>
    </div>
    <div class="h-full p-4 overflow-y-auto">
        <div class="card-body">
            <h6 class="mb-4 text-15">Drawer Content</h6>
            <p class="text-slate-500 dark:text-zink-200">They all have something to say beyond the words on the page. They can come across as casual or neutral, exotic or graphic.</p>
        </div>
    </div>
    <div class="flex items-center justify-between p-4 border-t border-slate-200 dark:border-zink-500">
        <h6 class="text-15">Drawer Footer</h6>
    </div>
</Drawer>