<script>
    import HeadTitle from "../../../../../common/components/HeadTitle.svelte";
	import Breadcrumb from "../../../../../common/components/Breadcrumb.svelte";
	import LucideIcon from "../../../../../common/components/LucideIcon.svelte";
	import CounterPage from "../../../../../common/components/CounterPage.svelte";
    import {LeaveManageEmployeeData} from "../../../../../common/data/hrmanagement"
</script>
<HeadTitle title="Leave Manage (Employee)" />

<CounterPage />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto relative">
    <Breadcrumb title="Leave Manage (Employee)" pagetitle="Leaves Manage" />

    <div class="grid grid-cols-1 gap-x-5 md:grid-cols-2 xl:grid-cols-12">
        <div class="xl:col-span-3">
            <div class="card">
                <div class="flex items-center gap-3 card-body">
                    <div class="flex items-center justify-center size-12 text-red-500 bg-red-100 rounded-md text-15 dark:bg-red-500/20 shrink-0"><LucideIcon name="FileBarChart2"/></div>
                    <div class="grow">
                        <h5 class="mb-1 text-16"><span class="counter-value" data-target="23">0</span></h5>
                        <p class="text-slate-500 dark:text-zink-200">Total Leave Balance</p>
                    </div>
                </div>
            </div>
        </div><!--end col-->
        <div class="xl:col-span-3">
            <div class="card">
                <div class="flex items-center gap-3 card-body">
                    <div class="flex items-center justify-center size-12 text-green-500 bg-green-100 rounded-md text-15 dark:bg-green-500/20 shrink-0"><LucideIcon name="CalendarDays"/></div>
                    <div class="grow">
                        <h5 class="mb-1 text-16"><span class="counter-value" data-target="12">0</span></h5>
                        <p class="text-slate-500 dark:text-zink-200">Annual Leave</p>
                    </div>
                </div>
            </div>
        </div><!--end col-->
        <div class="xl:col-span-3">
            <div class="card">
                <div class="flex items-center gap-3 card-body">
                    <div class="flex items-center justify-center size-12 text-purple-500 bg-purple-100 rounded-md text-15 dark:bg-purple-500/20 shrink-0"><LucideIcon name="Stethoscope"/></div>
                    <div class="grow">
                        <h5 class="mb-1 text-16"><span class="counter-value" data-target="4">0</span></h5>
                        <p class="text-slate-500 dark:text-zink-200">Medical Leave</p>
                    </div>
                </div>
            </div>
        </div><!--end col-->
        <div class="xl:col-span-3">
            <div class="card">
                <div class="flex items-center gap-3 card-body">
                    <div class="flex items-center justify-center size-12 rounded-md text-sky-500 bg-sky-100 text-15 dark:bg-sky-500/20 shrink-0"><LucideIcon name="Anchor"/></div>
                    <div class="grow">
                        <h5 class="mb-1 text-16"><span class="counter-value" data-target="11">0</span></h5>
                        <p class="text-slate-500 dark:text-zink-200">Remaining Leave</p>
                    </div>
                </div>
            </div>
        </div><!--end col-->
    </div><!--end grid-->

    <div class="card" id="ordersTable">
        <div class="card-body">
            <div class="grid grid-cols-1 gap-4 mb-5 lg:grid-cols-2 xl:grid-cols-12">
                <div class="xl:col-span-3">
                    <div class="relative">
                        <input type="text" class="ltr:pl-8 rtl:pr-8 search form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Search for ..." autocomplete="off">
                        <LucideIcon name="Search" class="inline-block size-4 absolute ltr:left-2.5 rtl:right-2.5 top-2.5 text-slate-500 dark:text-zink-200 fill-slate-100 dark:fill-zink-600"/>
                    </div>
                </div><!--end col-->
                <div class="xl:col-span-2 xl:col-start-11">
                    <div class="ltr:lg:text-right rtl:lg:text-left">
                        <a href="/apps/hr/create-leave" type="button" class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"><LucideIcon name="Plus" class="inline-block size-4"/> <span class="align-middle">Add Leave</span></a>
                    </div>
                </div>
            </div><!--col grid-->
            <div class="overflow-x-auto">
                <div class="overflow-x-auto">
                    <table class="w-full whitespace-nowrap">
                        <thead class="ltr:text-left rtl:text-right bg-slate-100 text-slate-500 dark:bg-zink-600 dark:text-zink-200">
                            <tr>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">#</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">Leave Type</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">Reason</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">No Of Days</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">From</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">to</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">Approved By</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">Status</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500 ltr:text-right rtl:text-left">Action</th>
                            </tr>
                        </thead>
                        <tbody>
                            {#each LeaveManageEmployeeData as empLeave}
                            <tr>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">{empLeave.id}</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">{empLeave.leaveType}</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">{empLeave.reason}</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">{empLeave.noOfDays}</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">{empLeave.from}</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">{empLeave.to}</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">{empLeave.approvedBy}</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">

                                    {#if empLeave.status == "Approved"}
                                    <span class="px-2.5 py-0.5 inline-block text-xs font-medium rounded border bg-green-100 border-transparent text-green-500 dark:bg-green-500/20 dark:border-transparent">{empLeave.status}</span>
                                    {:else if empLeave.status == "Pending"}
                                    <span class="px-2.5 py-0.5 inline-block text-xs font-medium rounded border bg-yellow-100 border-transparent text-yellow-500 dark:bg-yellow-500/20 dark:border-transparent">{empLeave.status}</span>
                                    {:else if empLeave.status == "Declined"}
                                    <span class="px-2.5 py-0.5 inline-block text-xs font-medium rounded border bg-red-100 border-transparent text-red-500 dark:bg-red-500/20 dark:border-transparent">{empLeave.status}</span>
                                    {/if}
                                </td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">
                                    <div class="flex justify-end gap-2">
                                        {#if empLeave.status == "Pending"}
                                        <a href="#!" class="flex items-center justify-center size-8 transition-all duration-200 ease-linear rounded-md text-slate-500 bg-slate-100 hover:text-white hover:bg-slate-500 dark:text-zink-200 dark:bg-zink-600 dark:hover:text-white dark:hover:bg-zink-400"><LucideIcon name="Pencil" class="size-4"/></a>
                                        {/if}
                                        <a href="#!" class="flex items-center justify-center size-8 transition-all duration-200 ease-linear rounded-md text-custom-500 bg-custom-100 hover:text-white hover:bg-custom-500 dark:bg-custom-500/20 dark:hover:bg-custom-500"><LucideIcon name="Info" class="size-4"/></a>
                                    </div>
                                </td>
                            </tr>
                            {/each}
                        </tbody>
                    </table>
                </div>
            </div>
            <div class="flex flex-col items-center mt-5 md:flex-row">
                <div class="mb-4 grow md:mb-0">
                    <p class="text-slate-500 dark:text-zink-200">Showing <b>10</b> of <b>17</b> Results</p>
                </div>
                <ul class="flex flex-wrap items-center gap-2 shrink-0">
                    <li>
                        <a href="#!" class="inline-flex items-center justify-center bg-white dark:bg-zink-700 h-8 px-3 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-500 dark:[&.active]:text-custom-500 [&.active]:bg-custom-50 dark:[&.active]:bg-custom-500/10 [&.active]:border-custom-50 dark:[&.active]:border-custom-500/10 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"><LucideIcon class="size-4 mr-1 rtl:rotate-180" name="ChevronLeft"/> Prev</a>
                    </li>
                    <li>
                        <a href="#!" class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-500 dark:[&.active]:text-custom-500 [&.active]:bg-custom-50 dark:[&.active]:bg-custom-500/10 [&.active]:border-custom-50 dark:[&.active]:border-custom-500/10 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto">1</a>
                    </li>
                    <li>
                        <a href="#!" class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-500 dark:[&.active]:text-custom-500 [&.active]:bg-custom-50 dark:[&.active]:bg-custom-500/10 [&.active]:border-custom-50 dark:[&.active]:border-custom-500/10 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto active">2</a>
                    </li>
                    <li>
                        <a href="#!" class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-500 dark:[&.active]:text-custom-500 [&.active]:bg-custom-50 dark:[&.active]:bg-custom-500/10 [&.active]:border-custom-50 dark:[&.active]:border-custom-500/10 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto">3</a>
                    </li>
                    <li>
                        <a href="#!" class="inline-flex items-center justify-center bg-white dark:bg-zink-700 h-8 px-3 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-500 dark:[&.active]:text-custom-500 [&.active]:bg-custom-50 dark:[&.active]:bg-custom-500/10 [&.active]:border-custom-50 dark:[&.active]:border-custom-500/10 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto">Next <LucideIcon class="size-4 ml-1 rtl:rotate-180" name="ChevronRight"/></a>
                    </li>
                </ul>
            </div>
        </div>
    </div>
</div>