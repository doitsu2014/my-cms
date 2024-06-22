<script>
    import { tooltip } from "@svelte-plugins/tooltips";
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
</script>

<HeadTitle title="Tooltip" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Tooltip" pagetitle="UI Elements" />

    <div class="card">
        <div class="card-body">
            <h6 class="mb-4 text-gray-800 text-15 dark:text-white">Default</h6>
            <button title="Test tooltip" use:tooltip type="button" class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20">Default Tooltips</button>
        </div>
    </div><!--end card-->
    <div class="card">
        <div class="card-body">
            <h6 class="mb-4 text-gray-800 text-15 dark:text-white">Placements</h6>
            
            <div class="flex flex-wrap items-center gap-2">
                <button use:tooltip title="Default Tooltips" type="button" class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20">Default</button>
                <button use:tooltip={{
                    content:'Bottom',
                    position: 'bottom',
                    animation: 'slide',
                    arrow: true
                  }} type="button" class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20">Bottom</button>
                <button use:tooltip={{
                    content:'Right',
                    position: 'right',
                    animation: 'slide',
                    arrow: true
                  }} data-tooltip-placement="right" title="Right" type="button" class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20">Right</button>
                <button use:tooltip={{
                    content:'left',
                    position: 'left',
                    animation: 'slide',
                    arrow: true
                  }} title="Left" type="button" class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20">Left</button>
            </div>
        </div>
    </div>
</div>
