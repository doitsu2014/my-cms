<script>
	import LucideIcon from "../../../../common/components/LucideIcon.svelte";
	import ResponseTimesChart from "./ResponseTimesChart.svelte";

</script>
<div class="order-6 col-span-12 2xl:order-1 card 2xl:col-span-5">
    <div class="card-body">
        <div class="flex items-center gap-2">
            <h6 class="mb-3 text-15 grow">Location-Based Response Times</h6>
            <div class="shrink-0">
                <button type="button" class="px-2 py-1.5 text-xs text-custom-500 btn bg-custom-100 hover:text-white hover:bg-custom-600 focus:text-white focus:bg-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:ring active:ring-custom-100 dark:bg-custom-500/20 dark:text-custom-500 dark:hover:bg-custom-500 dark:hover:text-white dark:focus:bg-custom-500 dark:focus:text-white dark:active:bg-custom-500 dark:active:text-white dark:ring-custom-400/20">
                    View More <LucideIcon name="MoveRight" data-lucide="move-right" class="inline-block size-4 ltr:ml-1 rlt:mr-1 rtl:-rotate-180"/>
                </button>
            </div>
        </div>
        <ResponseTimesChart chartColors={["bg-red-500"]} />
    </div>
</div>