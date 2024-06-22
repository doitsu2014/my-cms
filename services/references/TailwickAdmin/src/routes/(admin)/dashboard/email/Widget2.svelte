<script>
	import LucideIcon from "../../../../common/components/LucideIcon.svelte";
	import DeliveredRateChart from "./DeliveredRateChart.svelte";
	import HardBounceRateChart from "./HardBounceRateChart.svelte";
	import SpanReportRateChart from "./SpanReportRateChart.svelte";
	import UnsubscribedRateChart from "./UnsubscribedRateChart.svelte";

</script>
<div class="col-span-12 card md:col-span-6 2xl:col-span-3">
    <div class="card-body">
        <a href="#!" class="ltr:float-right rtl:float-left text-slate-500 dark:text-zink-200">
            <LucideIcon name="Info" class="size-4"/></a>
        <p class="mb-3 text-slate-500 dark:text-zink-200">Delivered Rate</p>
        <h5 class="mb-4"><span class="counter-value" data-target="100">0</span>%</h5>

        <DeliveredRateChart chartColors={["bg-sky-500"]}/>
    </div>
</div><!--end col-->
<div class="col-span-12 card md:col-span-6 2xl:col-span-3">
    <div class="card-body">
        <a href="#!" class="ltr:float-right rtl:float-left text-slate-500 dark:text-zink-200"><LucideIcon name="Info" class="size-4"/></a>
        <p class="mb-3 text-slate-500 dark:text-zink-200">Hard Bounce Rate</p>
        <h5 class="mb-4"><span class="counter-value" data-target="89">0</span>%</h5>

        <HardBounceRateChart chartColors={["bg-green-500"]}/>
    </div>
</div><!--end col-->
<div class="col-span-12 card md:col-span-6 2xl:col-span-3">
    <div class="card-body">
        <a href="#!" class="ltr:float-right rtl:float-left text-slate-500 dark:text-zink-200"><LucideIcon name="Info" class="size-4"/></a>
        <p class="mb-3 text-slate-500 dark:text-zink-200">Unsubscribed Rate</p>
        <h5 class="mb-4"><span class="counter-value" data-target="33">0</span>%</h5>

        <UnsubscribedRateChart chartColors={["bg-yellow-500"]}/>
    </div>
</div><!--end col-->
<div class="col-span-12 card md:col-span-6 2xl:col-span-3">
    <div class="card-body">
        <a href="#!" class="ltr:float-right rtl:float-left text-slate-500 dark:text-zink-200"><LucideIcon name="Info" class="size-4"/></a>
        <p class="mb-3 text-slate-500 dark:text-zink-200">Spam Report Rate</p>
        <h5 class="mb-4"><span class="counter-value" data-target="11.8">0</span>%</h5>

        <SpanReportRateChart chartColors={["bg-purple-500"]}/>
    </div>
</div>