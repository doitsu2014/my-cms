<script>
	import LucideIcon from "../../../../common/components/LucideIcon.svelte";

</script>
<div class="col-span-12 lg:col-span-6 order-[13] 2xl:order-1 card 2xl:col-span-3">
    <div class="card-body">
        <h6 class="mb-3 text-15">Status of Monthly Campaign</h6>
        <ul class="flex flex-col gap-5">
            <li class="flex items-center gap-3">
                <div class="flex items-center justify-center size-8 text-red-500 bg-red-100 rounded-md dark:bg-red-500/20 shrink-0">
                    <LucideIcon name="Mail" data-lucide="mail" class="size-4"/>
                </div>
                <h6 class="grow">Clicked</h6>
                <p class="text-slate-500 dark:text-zink-200">9,745</p>
                <div class="w-12 text-green-500 ltr:text-right rtl:text-left">
                    2.5%
                </div>
            </li>
            <li class="flex items-center gap-3">
                <div class="flex items-center justify-center size-8 rounded-md text-sky-500 bg-sky-100 dark:bg-sky-500/20 shrink-0">
                    <LucideIcon name="Users" class="size-4"/>
                </div>
                <h6 class="grow">Subscribe</h6>
                <p class="text-slate-500 dark:text-zink-200">3,352</p>
                <div class="w-12 text-green-500 ltr:text-right rtl:text-left">
                    18.9%
                </div>
            </li>
            <li class="flex items-center gap-3">
                <div class="flex items-center justify-center size-8 text-orange-500 bg-orange-100 rounded-md dark:bg-orange-500/20 shrink-0">
                    <LucideIcon name="UserX2" class="size-4"/>
                </div>
                <h6 class="grow">Unsubscribe</h6>
                <p class="text-slate-500 dark:text-zink-200">85,745</p>
                <div class="w-12 text-red-500 ltr:text-right rtl:text-left">
                    1.2%
                </div>
            </li>
            <li class="flex items-center gap-3">
                <div class="flex items-center justify-center size-8 text-green-500 bg-green-100 rounded-md dark:bg-green-500/20 shrink-0">
                    <LucideIcon name="EyeIcon" class="size-4"/>
                </div>
                <h6 class="grow">Viewers</h6>
                <p class="text-slate-500 dark:text-zink-200">85,745</p>
                <div class="w-12 text-green-500 ltr:text-right rtl:text-left">
                    11.6%
                </div>
            </li>
            <li class="flex items-center gap-3">
                <div class="flex items-center justify-center size-8 rounded-md text-slate-500 bg-slate-100 dark:bg-zink-600 dark:text-zink-200 shrink-0">
                    <LucideIcon name="PackageOpen" class="size-4"/>
                </div>
                <h6 class="grow">Opened</h6>
                <p class="text-slate-500 dark:text-zink-200">85,745</p>
                <div class="w-12 text-red-500 ltr:text-right rtl:text-left">
                    3.1%
                </div>
            </li>
        </ul>
    </div>
</div>