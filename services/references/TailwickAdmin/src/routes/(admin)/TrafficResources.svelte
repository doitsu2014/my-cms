<script>
	import LucideIcon from "../../common/components/LucideIcon.svelte";
	import Progress from "../../common/components/Progress.svelte";
	import TrafficResourcesChart from "./trafficResourcesChart.svelte";
</script>
<div class="grid grid-cols-12 gap-x-5">
    <div class="col-span-12 card lg:col-span-6 2xl:col-span-12">
        <div class="card-body">
            <div class="flex items-center mb-3">
                <h6 class="grow text-15">Traffic Resources</h6>
                <div class="relative">
                    <a href="#!" class="transition-all duration-300 ease-linear text-custom-500 hover:text-custom-700">View Status 
                        <LucideIcon name="MoveRightIcon" class="inline-block size-4 align-middle ltr:ml-1 rtl:mr-1"/>
                    </a>
                </div>
            </div>
            <div class="grid grid-cols-12">
                <div class="col-span-12 md:col-span-6 2xl:col-span-7">
                    <TrafficResourcesChart chartColors={["bg-sky-500", "bg-purple-500", "bg-green-500", "bg-yellow-500"]} />
                </div>
                <div class="col-span-12 md:col-span-6 2xl:col-span-5">
                    <ul class="flex flex-col gap-3">
                        <li class="flex items-center gap-2">
                            <div class="size-3 bg-green-500 shrink-0 clip-triangle"></div>
                            <p class="text-green-500">Search Engine (22%)</p>
                        </li>
                        <li class="flex items-center gap-2">
                            <div class="size-3 bg-purple-500 shrink-0 clip-triangle"></div>
                            <p class="text-purple-500">Referral (34%)</p>
                        </li>
                        <li class="flex items-center gap-2">
                            <div class="size-3 bg-sky-500 shrink-0 clip-triangle"></div>
                            <p class="text-sky-500">Direct (44%)</p>
                        </li>
                    </ul>
                </div>
            </div>
        </div>
    </div><!--end col-->
    <div class="col-span-12 card lg:col-span-6 2xl:col-span-12">
        <div class="card-body">
            <div class="flex items-center mb-2">
                <h5 class="grow"><span class="counter-value" data-target="1596">0</span></h5>
                <span class="px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-white border-red-100 text-red-500 dark:bg-zink-700 dark:border-red-900">
                    <LucideIcon name="TrendingDownIcon" class="inline-block size-3 ltr:mr-1 rtl:ml-1"/>
                     6.8%</span>
            </div>
            <h6 class="mb-0">Monthly Orders Goal (20000+)</h6>
            <div>
                <div class="flex items-center justify-between mt-5 mb-2">
                    <p class="text-slate-500 dark:text-zink-200">Total Orders</p>
                    <h6 class="mb-0 text-custom-500">85%</h6>
                </div>
                <Progress barClass="bg-custom-500 h-2.5 " value="85"/>
            </div>
        </div>
    </div><!--end col-->
</div>