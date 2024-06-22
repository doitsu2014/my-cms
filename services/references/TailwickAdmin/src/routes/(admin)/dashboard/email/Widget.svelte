<script>
	import LucideIcon from "../../../../common/components/LucideIcon.svelte";
	import EmailClickRateChart from "./EmailClickRateChart.svelte";
	import EmailclickThroughChart from "./EmailclickThroughChart.svelte";
	import EmailopenRateChart from "./EmailopenRateChart.svelte";
	import SentEmailChart from "./SentEmailChart.svelte";

</script>
<div class="col-span-12 card 2xl:col-span-12">
    <div class="card-body">
        <div class="grid grid-cols-1 gap-5 divide-y md:divide-x md:divide-y-0 md:grid-cols-2 2xl:grid-cols-4 rtl:divide-x-reverse divide-slate-200 dark:divide-zink-500">
            <div class="py-5 first:pt-0 md:first:pt-5 md:last:pb-5 last:pb-0 md:px-5 md:ltr:first:pl-0 md:rtl:first:pr-0 md:ltr:last:pr-0 md:rtl:last:pl-0">
                <div class="flex mb-4">
                    <div class="w-36 shrink-0">
                        <p class="mb-6 uppercase text-slate-500 dark:text-zink-200">
                            <LucideIcon name="Rocket" class="inline-block size-4 ltr:mr-1 rtl:ml-1"/> <span class="align-middle">Sent</span></p>
                        <h5 class="mb-0"><span class="counter-value" data-target="1452">0</span></h5>
                    </div>
                    <SentEmailChart chartColors={["bg-custom-500"]}/>
                </div>
                <p class="text-slate-500 dark:text-zink-200">238 Emails (Monthly)</p>

            </div><!--end col-->
            <div class="py-5 first:pt-0 md:first:pt-5 md:last:pb-5 last:pb-0 md:px-5 md:ltr:first:pl-0 md:rtl:first:pr-0 md:ltr:last:pr-0 md:rtl:last:pl-0">
                <div class="flex mb-4">
                    <div class="w-36 shrink-0">
                        <p class="mb-6 uppercase text-slate-500 dark:text-zink-200">
                            <LucideIcon name="MousePointerClick" class="inline-block size-4 ltr:mr-1 rtl:ml-1"/> <span class="align-middle">Open Rate</span></p>
                        <h5 class="mb-0"><span class="counter-value" data-target="92.74">0</span>%</h5>
                    </div>
                    <EmailopenRateChart chartColors={["bg-green-500"]}/>
                </div>
                <p class="text-slate-500 dark:text-zink-200">60 Opened (Monthly)</p>
            </div><!--end col-->
            <div class="py-5 first:pt-0 md:first:pt-5 md:last:pb-5 last:pb-0 md:px-5 md:ltr:first:pl-0 md:rtl:first:pr-0 md:ltr:last:pr-0 md:rtl:last:pl-0">
                <div class="flex mb-4">
                    <div class="w-36 shrink-0">
                        <p class="mb-6 uppercase text-slate-500 dark:text-zink-200">
                            <LucideIcon name="MousePointerSquare" class="inline-block size-4 ltr:mr-1 rtl:ml-1"/> <span class="align-middle">Clicks Rate</span></p>
                        <h5 class="mb-0"><span class="counter-value" data-target="3.79">0</span>%</h5>
                    </div>
                    <EmailClickRateChart chartColors={["bg-red-500"]}/>
                </div>
                <p class="text-slate-500 dark:text-zink-200">29 Clicks (Monthly)</p>
            </div>
            <div class="py-5 first:pt-0 md:first:pt-5 md:last:pb-5 last:pb-0 md:px-5 md:ltr:first:pl-0 md:rtl:first:pr-0 md:ltr:last:pr-0 md:rtl:last:pl-0">
                <div class="flex mb-4">
                    <div class="w-36 shrink-0">
                        <p class="mb-6 uppercase text-slate-500 dark:text-zink-200">
                            <LucideIcon name="Goal" class="inline-block size-4 ltr:mr-1 rtl:ml-1"/> <span class="align-middle">Click Through</span></p>
                        <h5 class="mb-0"><span class="counter-value" data-target="4.06">0</span>%</h5>
                    </div>
                    <EmailclickThroughChart chartColors={["bg-sky-500"]}/>
                </div>
                <p class="text-slate-500 dark:text-zink-200">29 Click Through (Monthly)</p>
            </div>
        </div>
    </div>
</div>