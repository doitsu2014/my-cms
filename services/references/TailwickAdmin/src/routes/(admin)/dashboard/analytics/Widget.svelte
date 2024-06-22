<script>
	import Dropdown from "../../../../common/components/Dropdown.svelte";
	import DropdownMenu from "../../../../common/components/DropdownMenu.svelte";
	import DropdownToggle from "../../../../common/components/DropdownToggle.svelte";
import LucideIcon from "../../../../common/components/LucideIcon.svelte";
	import PlatformPerspectiveChart from "./platformPerspectiveChart.svelte";

</script>
<div class="order-1 md:col-span-6 lg:col-span-3 col-span-12 2xl:order-1 bg-green-100 dark:bg-green-500/20 card 2xl:col-span-2 group-data-[skin=bordered]:border-green-500/20 relative overflow-hidden">
    <div class="card-body">
        <LucideIcon name="Kanban" class="absolute top-0 size-32 stroke-1 text-green-200/50 dark:text-green-500/20 ltr:-right-10 rtl:-left-10"/>
        <div class="flex items-center justify-center size-12 bg-green-500 rounded-md text-15 text-green-50">
            <LucideIcon name="Users" />
        </div>
        <h5 class="mt-5 mb-2"><span class="counter-value" data-target="15876">0</span></h5>
        <p class="text-slate-500 dark:text-slate-200">Total Users</p>
    </div>
</div><!--end col-->
<div class="order-2 md:col-span-6 lg:col-span-3 col-span-12 2xl:order-1 bg-orange-100 dark:bg-orange-500/20 card 2xl:col-span-2 group-data-[skin=bordered]:border-orange-500/20 relative overflow-hidden">
    <div class="card-body">
        <LucideIcon name="ListFilter" class="absolute top-0 size-32 stroke-1 text-orange-200/50 dark:text-orange-500/20 ltr:-right-10 rtl:-left-10"/>
        <div class="flex items-center justify-center size-12 bg-orange-500 rounded-md text-15 text-orange-50">
            <LucideIcon name="CogIcon" />
        </div>
        <h5 class="mt-5 mb-2"><span class="counter-value" data-target="103.15">0</span>k</h5>
        <p class="text-slate-500 dark:text-slate-200">Sessions</p>
    </div>
</div><!--end col-->
<div class="order-5 col-span-12 2xl:order-1 card 2xl:row-span-2 2xl:col-span-8">
    <div class="card-body">
        <div class="flex items-center gap-2 mb-3">
            <h6 class="text-15 grow">Platform Perspective</h6>
            <Dropdown class="relative shrink-0"  direction="bottom-start">
                <span class="ltr:mr-1 rtl:ml-1 text-slate-500 dark:zink-200">Sort by</span>
                <DropdownToggle className="px-2 py-1.5 text-xs bg-white border-dashed text-slate-500 btn border-slate-500 hover:text-slate-500 hover:bg-slate-50 hover:border-slate-600 focus:text-slate-600 focus:bg-slate-50 focus:border-slate-600 active:text-slate-600 active:bg-slate-50 active:border-slate-600 dark:bg-zink-700 dark:text-zink-200 dark:border-zink-400 dark:ring-zink-400/20 dark:hover:bg-zink-600 dark:hover:text-zink-100 dark:focus:bg-zink-600 dark:focus:text-zink-100 dark:active:bg-zink-600 dark:active:text-zink-100 ">
                    This Yearly <LucideIcon name="ChevronDown" class="inline-block size-4 ltr:ml-1 rlt:mr-1"/>
                </DropdownToggle>
                <DropdownMenu tag="ul" class="absolute z-50 py-2 mt-1 ltr:text-left rtl:text-right list-none bg-white rounded-md shadow-md dropdown-menu min-w-[10rem] dark:bg-zink-600">
                    <li>
                        <a class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200" href="#!">1 Weekly</a>
                    </li>
                    <li>
                        <a class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200" href="#!">1 Monthly</a>
                    </li>
                    <li>
                        <a class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200" href="#!">3 Monthly</a>
                    </li>
                    <li>
                        <a class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200" href="#!">6 Monthly</a>
                    </li>
                    <li>
                        <a class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200" href="#!">This Yearly</a>
                    </li>
                    </DropdownMenu>
            </Dropdown>

        </div>
        <PlatformPerspectiveChart chartColors={["bg-custom-500"]} />
        <div id="chartdiv"></div>
    </div>
</div><!--end col-->
<div class="order-3 md:col-span-6 lg:col-span-3 col-span-12 2xl:order-1 bg-sky-100 dark:bg-sky-500/20 card 2xl:col-span-2 group-data-[skin=bordered]:border-sky-500/20 relative overflow-hidden">
    <div class="card-body">
        <LucideIcon name="ListFilter" class="absolute top-0 size-32 stroke-1 text-sky-200/50 dark:text-sky-500/20 ltr:-right-10 rtl:-left-10"/>
        <div class="flex items-center justify-center size-12 rounded-md bg-sky-500 text-15 text-sky-50">
            <LucideIcon name="Coins" />
        </div>
        <h5 class="mt-5 mb-2"><span class="counter-value" data-target="1">0</span>M <span class="counter-value" data-target="29">0</span>sec</h5>
        <p class="text-slate-500 dark:text-slate-200">Avg. Visit Duration</p>
    </div>
</div>
<div class="order-4 md:col-span-6 lg:col-span-3 col-span-12 2xl:order-1 bg-purple-100 dark:bg-purple-500/20 card 2xl:col-span-2 group-data-[skin=bordered]:border-purple-500/20 relative overflow-hidden">
    <div class="card-body">
        <LucideIcon name="Kanban" class="absolute top-0 size-32 stroke-1 text-purple-200/50 dark:text-purple-500/20 ltr:-right-10 rtl:-left-10"/>
        <div class="flex items-center justify-center size-12 bg-purple-500 rounded-md text-15 text-purple-50">
            <LucideIcon name="Users" />
        </div>
        <h5 class="mt-5 mb-2"><span class="counter-value" data-target="49.77">0</span>%</h5>
        <p class="text-slate-500 dark:text-slate-200">Bounce Rate</p>
    </div>
</div>