<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import LucideIcon from '../../../../common/components/LucideIcon.svelte';

</script>

<HeadTitle title="Progressbar" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Progressbar" pagetitle="UI Elements" />
  
    <div class="card">
        <div class="card-body">
            <h6 class="mb-4 text-15">Default & Color Variants</h6>
            <div class="grid grid-cols-1 gap-x-4 xl:grid-cols-2">
                <div>
                    <div class="w-full bg-slate-200 rounded-full h-2.5 mb-4 dark:bg-zink-600">
                        <div class="bg-custom-500 h-2.5 rounded-full" style="width: 25%"></div>
                    </div>
                    <div class="w-full bg-slate-200 rounded-full h-2.5 mb-4 dark:bg-zink-600">
                        <div class="bg-green-500 h-2.5 rounded-full" style="width: 50%"></div>
                    </div>
                    <div class="w-full bg-slate-200 rounded-full h-2.5 mb-4 dark:bg-zink-600">
                        <div class="bg-orange-500 h-2.5 rounded-full" style="width: 75%"></div>
                    </div>
                    <div class="w-full bg-slate-200 rounded-full h-2.5 dark:bg-zink-600">
                        <div class="bg-sky-500 h-2.5 rounded-full" style="width: 100%"></div>
                    </div>
                </div>
                <div>
                    <div class="w-full bg-slate-200 rounded-full h-2.5 mb-4 dark:bg-zink-600">
                        <div class="bg-yellow-500 h-2.5 rounded-full" style="width: 25%"></div>
                    </div>
                    <div class="w-full bg-slate-200 rounded-full h-2.5 mb-4 dark:bg-zink-600">
                        <div class="bg-red-500 h-2.5 rounded-full" style="width: 50%"></div>
                    </div>
                    <div class="w-full bg-slate-200 rounded-full h-2.5 mb-4 dark:bg-zink-600">
                        <div class="bg-purple-500 h-2.5 rounded-full" style="width: 75%"></div>
                    </div>
                    <div class="w-full bg-slate-200 rounded-full h-2.5 dark:bg-zink-600">
                        <div class="bg-slate-500 dark:bg-zink-200 h-2.5 rounded-full" style="width: 100%"></div>
                    </div>
                </div>
            </div>
        </div>
    </div><!--end card-->

    <div class="grid grid-cols-1 gap-x-4 xl:grid-cols-2">
        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">Progress with Label</h6>
                <div class="pt-2">
                    <div class="w-full bg-slate-200 rounded-full h-2.5 mb-7 dark:bg-zink-600">
                        <div class="bg-custom-500 h-2.5 rounded-full animate-progress relative" style="width: 25%"><div class="absolute ltr:right-0 rtl:left-0 inline-block px-2 py-0.5 text-[10px] text-white bg-custom-500 rounded -top-6 after:absolute after:border-4 ltr:after:right-1/2 rtl:after:left-1/2 after:-bottom-2 after:border-transparent after:border-t-custom-500">15%</div></div>
                    </div>
                    <div class="w-full bg-slate-200 rounded-full h-2.5 mb-7 dark:bg-zink-600">
                        <div class="bg-green-500 h-2.5 rounded-full animate-progress relative" style="width: 70%"><div class="absolute ltr:right-0 rtl:left-0 inline-block px-2 py-0.5 text-[10px] text-white bg-green-500 rounded -top-6 after:absolute after:border-4 ltr:after:right-1/2 rtl:after:left-1/2 after:-bottom-2 after:border-transparent after:border-t-green-500">70%</div></div>
                    </div>
                    <div class="w-full bg-slate-200 rounded-full h-2.5 mb-0 dark:bg-zink-600">
                        <div class="bg-orange-500 h-2.5 rounded-full animate-progress relative" style="width: 40%"><div class="absolute ltr:right-0 rtl:left-0 inline-block px-2 py-0.5 text-[10px] text-white bg-orange-500 rounded -top-6 after:absolute after:border-4 ltr:after:right-1/2 rtl:after:left-1/2 after:-bottom-2 after:border-transparent after:border-t-orange-500">40%</div></div>
                    </div>
                </div>
            </div>
        </div><!--end card-->
        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">Animated Progress</h6>
                <div>
                    <div class="w-full bg-slate-200 rounded-full h-2.5 mb-4 dark:bg-zink-600">
                        <div class="bg-yellow-500 h-2.5 rounded-full animate-progress" style="width: 25%"></div>
                    </div>
                    <div class="w-full bg-slate-200 rounded-full h-2.5 mb-4 dark:bg-zink-600">
                        <div class="bg-red-500 h-2.5 rounded-full animate-progress" style="width: 50%"></div>
                    </div>
                    <div class="w-full bg-slate-200 rounded-full h-2.5 mb-4 dark:bg-zink-600">
                        <div class="bg-purple-500 h-2.5 rounded-full animate-progress" style="width: 75%"></div>
                    </div>
                    <div class="w-full bg-slate-200 rounded-full h-2.5 dark:bg-zink-600">
                        <div class="bg-slate-500 dark:bg-zink-200 h-2.5 rounded-full animate-progress" style="width: 100%"></div>
                    </div>
                </div>
            </div>
        </div><!--end card-->
    </div><!--end grid-->

    <div class="card">
        <div class="card-body">
            <h6 class="mb-4 text-15">Progress Sizes</h6>
            <div class="w-full h-1.5 mb-4 rounded-full bg-slate-200 dark:bg-zink-600">
                <div class="h-1.5 rounded-full bg-custom-500" style="width: 25%"></div>
            </div>
            <div class="w-full h-2 mb-4 rounded-full bg-slate-200 dark:bg-zink-600">
                <div class="h-2 rounded-full bg-custom-500" style="width: 25%"></div>
            </div>
            <div class="w-full h-3 mb-4 rounded-full bg-slate-200 dark:bg-zink-600">
                <div class="h-3 rounded-full bg-custom-500" style="width: 25%"></div>
            </div>
            <div class="w-full h-3.5 mb-4 rounded-full bg-slate-200 dark:bg-zink-600">
                <div class="h-3.5 rounded-full bg-custom-500" style="width: 25%"></div>
            </div>
            <div class="w-full h-5 mb-4 rounded-full bg-slate-200 dark:bg-zink-600">
                <div class="h-5 rounded-full bg-custom-500" style="width: 25%"></div>
            </div>
            <div class="w-full h-6 rounded-full bg-slate-200 dark:bg-zink-600">
                <div class="h-6 rounded-full bg-custom-500" style="width: 25%"></div>
            </div>
        </div>
    </div><!--end card-->

    <div class="card">
        <div class="card-body">
            <h6 class="mb-4 text-15">Progress with Content</h6>
            <div>
                <div class="flex items-center justify-between mb-2">
                    <h6 class="mb-0">Tailwick - Admin & Dashboard</h6>
                    <h6 class="mb-0 text-custom-500">24%</h6>
                </div>
                <div class="w-full bg-slate-200 rounded-full h-2.5 mb-4 dark:bg-zink-600">
                    <div class="bg-custom-500 h-2.5 rounded-full" style="width: 24%"></div>
                </div>
            </div>
            <div>
                <div class="flex items-center justify-between mb-2">
                    <h6 class="mb-0">Nazox - Admin & Dashboard</h6>
                    <h6 class="mb-0 text-green-500">66%</h6>
                </div>
                <div class="w-full bg-slate-200 rounded-full h-2.5 mb-4 dark:bg-zink-600">
                    <div class="bg-green-500 h-2.5 rounded-full" style="width: 66%"></div>
                </div>
            </div>
        </div>
    </div><!--end card-->

    <div class="card">
        <div class="card-body">
            <h6 class="mb-4 text-15">With Label Inside</h6>
            <div class="w-full h-3 mb-4 rounded-full bg-slate-200 dark:bg-zink-600">
                <div class="bg-custom-500 h-3 rounded-full text-[8px] text-white text-center" style="width: 24%">24%</div>
            </div>
            <div class="w-full h-3.5 mb-4 rounded-full bg-slate-200 dark:bg-zink-600">
                <div class="h-3.5 rounded-full bg-custom-500 text-[10px] text-center text-white" style="width: 25%">25%</div>
            </div>
            <div class="w-full h-5 rounded-full bg-slate-200 dark:bg-zink-600">
                <div class="h-5 text-xs text-center text-white rounded-full bg-custom-500" style="width: 25%">25%</div>
            </div>
        </div>
    </div>
</div>
