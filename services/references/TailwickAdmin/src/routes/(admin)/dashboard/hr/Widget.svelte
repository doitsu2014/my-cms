<script>
	import ApplicationReceivedChart from "./ApplicationReceivedChart.svelte";
	import HiredCandidatesChart from "./HiredCandidatesChart.svelte";
	import RejectedCandidatesChart from "./RejectedCandidatesChart.svelte";
    import TotalApplicationChart from "./TotalApplicationChart.svelte";
    import TotalEmployeeChart from "./TotalEmployeeChart.svelte";

</script>
<div class="col-span-12 md:order-3 lg:col-span-6 2xl:col-span-3 card">
    <div class="card-body">
        <div class="grid grid-cols-12">
            <div class="col-span-8 md:col-span-9">
                <p class="text-slate-500 dark:text-slate-200">Total Employee</p>
                <h5 class="mt-3 mb-4"><span class="counter-value" data-target="615">0</span></h5>
            </div>
            <div class="col-span-4 md:col-span-3">
                <TotalEmployeeChart chartColors={["bg-custom-500"]} />
            </div>
        </div>
        <div class="flex items-center gap-3 mt-3">
            <p class="text-slate-500 dark:text-slate-200 grow"><span class="font-medium text-green-500">15%</span> Increase</p>
            <p class="text-slate-500 dark:text-slate-200">This Month</p>
        </div>
    </div>
</div>
<div class="col-span-12 md:order-4 lg:col-span-6 2xl:col-span-3 card">
    <div class="card-body">
        <div class="grid grid-cols-12">
            <div class="col-span-8 md:col-span-9">
                <p class="text-slate-500 dark:text-slate-200">Total Application</p>
                <h5 class="mt-3 mb-4"><span class="counter-value" data-target="174">0</span></h5>
            </div>
            <div class="col-span-4 md:col-span-3">
                <TotalApplicationChart chartColors={["bg-purple-500"]}/>
            </div>
        </div>
        <div class="flex items-center gap-3 mt-3">
            <p class="text-slate-500 dark:text-slate-200 grow"><span class="font-medium text-green-500">26%</span> Increase</p>
            <p class="text-slate-500 dark:text-slate-200">This Month</p>
        </div>
    </div>
</div>
<div class="col-span-12 md:order-7 2xl:order-5 lg:col-span-12 2xl:col-span-6 2xl:row-span-2 card">
    <div class="card-body">
        <div class="flex items-center gap-2 MB-3">
            <h6 class="mb-0 text-15 grow">Application Received</h6>
            <div class="relative flex items-center gap-2 dropdown shrink-0">
                <button type="button" class="flex items-center justify-center size-8 p-0 text-xs text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20">All</button>
                <button type="button" class="flex items-center justify-center size-8 p-0 text-xs transition-all duration-200 ease-linear text-sky-500 btn bg-sky-100 hover:text-white hover:bg-sky-600 focus:text-white focus:bg-sky-600 focus:ring focus:ring-sky-100 active:text-white active:bg-sky-600 active:ring active:ring-sky-100 dark:bg-sky-500/20 dark:text-sky-400 dark:hover:bg-sky-500 dark:hover:text-white dark:focus:bg-sky-500 dark:focus:text-white dark:active:bg-sky-500 dark:active:text-white dark:ring-sky-400/20">1M</button>
                <button type="button" class="flex items-center justify-center size-8 p-0 text-xs transition-all duration-200 ease-linear text-sky-500 btn bg-sky-100 hover:text-white hover:bg-sky-600 focus:text-white focus:bg-sky-600 focus:ring focus:ring-sky-100 active:text-white active:bg-sky-600 active:ring active:ring-sky-100 dark:bg-sky-500/20 dark:text-sky-400 dark:hover:bg-sky-500 dark:hover:text-white dark:focus:bg-sky-500 dark:focus:text-white dark:active:bg-sky-500 dark:active:text-white dark:ring-sky-400/20">6M</button>
                <button type="button" class="flex items-center justify-center size-8 p-0 text-xs transition-all duration-200 ease-linear text-sky-500 btn bg-sky-100 hover:text-white hover:bg-sky-600 focus:text-white focus:bg-sky-600 focus:ring focus:ring-sky-100 active:text-white active:bg-sky-600 active:ring active:ring-sky-100 dark:bg-sky-500/20 dark:text-sky-400 dark:hover:bg-sky-500 dark:hover:text-white dark:focus:bg-sky-500 dark:focus:text-white dark:active:bg-sky-500 dark:active:text-white dark:ring-sky-400/20">1Y</button>
            </div>
        </div>
        <ApplicationReceivedChart chartColors={["bg-custom-500", "bg-green-500"]}/>
    </div>
</div>
<div class="col-span-12 md:order-5 2xl:order-6 lg:col-span-6 2xl:col-span-3 card">
    <div class="card-body">
        <div class="grid grid-cols-12">
            <div class="col-span-8 md:col-span-9">
                <p class="text-slate-500 dark:text-slate-200">Hired Candidates</p>
                <h5 class="mt-3 mb-4"><span class="counter-value" data-target="64">0</span></h5>
            </div>
            <div class="col-span-4 md:col-span-3">
                <HiredCandidatesChart chartColors={["bg-green-500"]}/>
            </div>
        </div>
        <div class="flex items-center gap-3 mt-3">
            <p class="text-slate-500 dark:text-slate-200 grow"><span class="font-medium text-red-500">05%</span> Increase</p>
            <p class="text-slate-500 dark:text-slate-200">This Month</p>
        </div>
    </div>
</div>
<div class="col-span-12 md:order-6 2xl:order-7 lg:col-span-6 2xl:col-span-3 card">
    <div class="card-body">
        <div class="grid grid-cols-12">
            <div class="col-span-8 md:col-span-9">
                <p class="text-slate-500 dark:text-slate-200">Rejected Candidates</p>
                <h5 class="mt-3 mb-4"><span class="counter-value" data-target="110">0</span></h5>
            </div>
            <div class="col-span-4 md:col-span-3">
                <RejectedCandidatesChart chartColors={["bg-red-500"]}/>
            </div>
        </div>
        <div class="flex items-center gap-3 mt-3">
            <p class="text-slate-500 dark:text-slate-200 grow"><span class="font-medium text-red-500">16%</span> Increase</p>
            <p class="text-slate-500 dark:text-slate-200">This Month</p>
        </div>
    </div>
</div>