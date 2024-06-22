<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
    import ColorPicker, { ChromeVariant } from 'svelte-awesome-color-picker';
</script>

<HeadTitle title="Color Picker" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Color Picker" pagetitle="Forms" />

    <div class="grid grid-cols-1 gap-x-5 xl:grid-cols-2">
        <div class="card">
            <div class="card-body">
                <h6 class="mb-1 text-gray-800 text-15 dark:text-white">Default Demo</h6>
                
                <ColorPicker hex={'#405189'} label="" />
            </div>
        </div>
        <div class="card">
            <div class="card-body">
                <h6 class="mb-1 text-gray-800 text-15 dark:text-white">Chrome Variant Demo</h6>
                
                <ColorPicker hex={'#418a8a'} components={ChromeVariant} label="" />
            </div>
        </div>
    </div>
</div>
