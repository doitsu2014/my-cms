<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import LucideIcon from '../../../../common/components/LucideIcon.svelte';
	import Counter from '../../../../common/components/CounterPage.svelte';
</script>

<HeadTitle title="Countdown" />
<Counter />
<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Countdown" pagetitle="UI Elements" />

    <div>
        <div class="card">
            <div class="card-body">
            <h6 class="mb-4 text-15">Basic Examples</h6>
                <div class="grid grid-cols-1 gap-5 md:grid-cols-2 xl:grid-cols-4">
                    <div class="text-center">
                        <h4 class="mb-2 fs-4xl"><span class="counter-value" data-target="745">0</span></h4>
                        <p class="mb-0 text-slate-500 dark:text-zink-200">Working Hours</p>
                    </div>
                    <div class="text-center text-custom-500">
                        <h4 class="mb-2 fs-4xl "><span class="counter-value" data-target="365">0</span></h4>
                        <p class="mb-0 text-custom-500">Completed Projects</p>
                    </div>
                    <div class="text-center">
                        <h4 class="mb-2 fs-4xl"><span class="counter-value" data-target="4645">0</span></h4>
                        <p class="mb-0 text-slate-500 dark:text-zink-200">Happy Users & Clients</p>
                    </div>
                    <div class="text-center">
                        <h4 class="mb-2 fs-4xl"><span class="counter-value" data-target="130">0</span></h4>
                        <p class="mb-0 text-slate-500 dark:text-zink-200">Awards Received</p>
                    </div>
                </div>
            </div>
        </div><!--end card-->
        <div class="card">
            <div class="card-body">
            <h6 class="mb-4 text-15">Counter with Symbols</h6>
                <div class="grid grid-cols-1 gap-5 md:grid-cols-2 xl:grid-cols-4">
                    <div class="text-center">
                        <h4 class="mb-2 fs-4xl">$<span class="counter-value" data-target="674.3">0</span>k</h4>
                        <p class="mb-0 text-slate-500 dark:text-zink-200">Total Earnings</p>
                    </div>
                    <div class="text-center">
                        <h4 class="mb-2 fs-4xl"><span class="counter-value" data-target="54.41">0</span>k</h4>
                        <p class="mb-0 text-slate-500 dark:text-zink-200">Total Orders</p>
                    </div>
                    <div class="text-center">
                        <h4 class="mb-2 fs-4xl"><span class="counter-value" data-target="4645">0</span></h4>
                        <p class="mb-0 text-slate-500 dark:text-zink-200">Customers</p>
                    </div>
                    <div class="text-center">
                        <h4 class="mb-2 fs-4xl">$<span class="counter-value" data-target="165.89">0</span>k</h4>
                        <p class="mb-0 text-slate-500 dark:text-zink-200">My Balance</p>
                    </div>
                </div>
            </div>
        </div><!--end card-->
        <div class="card">
            <div class="card-body">
            <h6 class="mb-4 text-15">Counter with Grid</h6>
                <div class="grid grid-cols-1 gap-5 md:grid-cols-2 xl:grid-cols-4">
                    <div class="px-3 py-6 text-center border rounded-md border-slate-200 dark:border-zink-500">
                        <h4 class="mb-2 fs-4xl">$<span class="counter-value" data-target="674.3">0</span>k</h4>
                        <p class="mb-0 text-slate-500 dark:text-zink-200">Total Earnings</p>
                    </div>
                    <div class="px-3 py-6 text-center border rounded-md border-slate-200 dark:border-zink-500">
                        <h4 class="mb-2 fs-4xl"><span class="counter-value" data-target="54.41">0</span>k</h4>
                        <p class="mb-0 text-slate-500 dark:text-zink-200">Total Orders</p>
                    </div>
                    <div class="px-3 py-6 text-center border rounded-md border-slate-200 dark:border-zink-500">
                        <h4 class="mb-2 fs-4xl"><span class="counter-value" data-target="4645">0</span></h4>
                        <p class="mb-0 text-slate-500 dark:text-zink-200">Customers</p>
                    </div>
                    <div class="px-3 py-6 text-center border rounded-md border-slate-200 dark:border-zink-500">
                        <h4 class="mb-2 fs-4xl">$<span class="counter-value" data-target="165.89">0</span>k</h4>
                        <p class="mb-0 text-slate-500 dark:text-zink-200">My Balance</p>
                    </div>
                </div>
            </div>
        </div><!--end card-->
        <div class="card">
            <div class="card-body">
            <h6 class="mb-4 text-gray-800 text-15 dark:text-zink-50">Counter with Icons</h6>
                <div class="grid grid-cols-1 gap-5 md:grid-cols-2 xl:grid-cols-4">
                    <div class="px-3 py-6 text-center border rounded-md dark:border-zink-500 border-slate-200">
                        <div class="flex items-center justify-center size-12 mx-auto mb-4 rounded-md text-15 bg-custom-50 dark:bg-custom-500/20"><LucideIcon name="Smile" class="text-custom-500 fill-custom-200 dark:fill-custom-500/30"/></div>
                        <h4 class="mb-2 fs-4xl"><span class="counter-value" data-target="8000">0</span>k</h4>
                        <p class="mb-0 text-slate-500 dark:text-zink-200">Happy Clients</p>
                    </div>
                    <div class="px-3 py-6 text-center border rounded-md dark:border-zink-500 border-slate-200">
                        <div class="flex items-center justify-center size-12 mx-auto mb-4 rounded-md text-15 bg-red-50 dark:bg-red-500/20"><LucideIcon name="Download" class="text-red-500 fill-red-200 dark:fill-red-500/30"/></div>
                        <h4 class="mb-2 fs-4xl"><span class="counter-value" data-target="3000">0</span>k</h4>
                        <p class="mb-0 text-slate-500 dark:text-zink-200">Downloads</p>
                    </div>
                    <div class="px-3 py-6 text-center border rounded-md dark:border-zink-500 border-slate-200">
                        <div class="flex items-center justify-center size-12 mx-auto mb-4 rounded-md text-15 bg-green-50 dark:bg-green-500/20"><LucideIcon name="Monitor" class="text-green-500 fill-green-200 dark:fill-green-500/30"/></div>
                        <h4 class="mb-2 fs-4xl"><span class="counter-value" data-target="4645">784</span>k</h4>
                        <p class="mb-0 text-slate-500 dark:text-zink-200">Project Completed</p>
                    </div>
                    <div class="px-3 py-6 text-center border rounded-md dark:border-zink-500 border-slate-200">
                        <div class="flex items-center justify-center size-12 mx-auto mb-4 rounded-md text-15 bg-sky-50 dark:bg-sky-500/20"><LucideIcon name="Clock4" class="text-sky-500 fill-sky-200 dark:fill-sky-500/30"/></div>
                        <h4 class="mb-2 fs-4xl"><span class="counter-value" data-target="365">0</span></h4>
                        <p class="mb-0 text-slate-500 dark:text-zink-200">Working Days</p>
                    </div>
                </div>
            </div>
        </div><!--end card-->
        <div class="card">
            <div class="card-body">
            <h6 class="mb-4 text-15">Counter with Card Colors</h6>
                <div class="grid grid-cols-1 gap-5 md:grid-cols-2 xl:grid-cols-4">
                    <div class="px-3 py-6 text-center border rounded-md border-custom-200 dark:border-zink-500">
                        <div class="flex items-center justify-center size-12 mx-auto mb-4 rounded-md text-15 bg-custom-50 dark:bg-custom-400/20"><LucideIcon name="Smile" class="text-custom-500 fill-custom-200 dark:fill-custom-500/20"/></div>
                        <h4 class="mb-1 fs-4xl"><span class="counter-value" data-target="8000">0</span>k</h4>
                        <p class="mb-0 text-slate-500 dark:text-zink-200">Happy Clients</p>
                    </div>
                    <div class="px-3 py-6 text-center bg-red-100 border border-red-200 rounded-md dark:bg-red-500/20 dark:border-red-800">
                        <div class="flex items-center justify-center size-12 mx-auto mb-4 rounded-md bg-red-50 text-15 dark:bg-red-400/20"><LucideIcon name="Download" class="text-red-500 fill-red-200 dark:fill-red-500/30"/></div>
                        <h4 class="mb-1 text-red-500 fs-4xl"><span class="counter-value" data-target="3000">0</span>k</h4>
                        <p class="mb-0 text-red-400">Downloads</p>
                    </div>
                    <div class="px-3 py-6 text-center bg-green-100 border border-green-100 rounded-md dark:bg-green-900 dark:border-zink-800">
                        <div class="flex items-center justify-center size-12 mx-auto mb-4 rounded-md bg-green-50 text-15 dark:bg-green-400/20"><LucideIcon name="Monitor" class="text-green-500 fill-green-200 dark:fill-green-500/20"/></div>
                        <h4 class="mb-1 text-green-500 fs-4xl"><span class="counter-value" data-target="4645">784</span>k</h4>
                        <p class="mb-0 text-green-400">Project Completed</p>
                    </div>
                    <div class="px-3 py-6 text-center border rounded-md bg-sky-500 border-sky-500 dark:bg-sky-900 dark:border-zink-800">
                        <div class="flex items-center justify-center size-12 mx-auto mb-4 rounded-md bg-white/90 text-15 dark:bg-sky-400/20"><LucideIcon name="Clock4" class="text-sky-500 fill-sky-200 dark:fill-sky-500/20"/></div>
                        <h4 class="mb-1 text-sky-50 fs-4xl"><span class="counter-value" data-target="365">0</span></h4>
                        <p class="mb-0 text-sky-200">Working Days</p>
                    </div>
                </div>
            </div>
        </div><!--end card-->
    </div>
</div>
