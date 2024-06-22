<script>
    import HeadTitle from "../../../../../common/components/HeadTitle.svelte";
	import Breadcrumb from "../../../../../common/components/Breadcrumb.svelte";
	import LucideIcon from "../../../../../common/components/LucideIcon.svelte";
	import CounterPage from "../../../../../common/components/CounterPage.svelte";
    import { AttendanceData } from "../../../../../common/data/hrmanagement"
	import Flatpickr from "svelte-flatpickr";
    import 'flatpickr/dist/flatpickr.css';

</script>
<HeadTitle title="Attendance" />

<CounterPage />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto relative">
    <Breadcrumb title="Attendance" pagetitle="Attendance" />

    <div class="grid grid-cols-1 lg:grid-cols-12 xl:grid-cols-12 gap-x-5">
        <div class="lg:col-span-12 xl:col-span-3 xl:row-span-2">
            <div class="mb-5">
                <label for="deliveryStatusSelect" class="inline-block mb-2 text-base font-medium">Select Employee</label>
                <select class="form-select border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" name="deliveryStatusSelect" id="deliveryStatusSelect">
                    <option value="">Select Employee</option>
                    <option value="Willie Torres">Willie Torres</option>
                    <option value="Patricia Garcia">Patricia Garcia</option>
                    <option value="Juliette Fecteau">Juliette Fecteau</option>
                    <option value="Thomas Hatfield">Thomas Hatfield</option>
                    <option value="Juliette Fecteau">Juliette Fecteau</option>
                    <option value="Nancy Reynolds">Nancy Reynolds</option>
                    <option value="Holly Kavanaugh">Holly Kavanaugh</option>
                    <option value="Jonas Frederiksen">Jonas Frederiksen</option>
                </select>
            </div>
            <div class="card">
                <div class="card-body">
                    <div class="text-center">
                        <div class="size-20 mx-auto rounded-full bg-slate-100 dark:bg-zink-600">
                            <img src="/assets/images/users/user-3.jpg" alt="" class="h-20 rounded-full">
                        </div>
                        <h6 class="mt-3 mb-1 text-16"><a href="#!">Willie Torres</a></h6>
                        <p class="text-slate-500 dark:text-zink-200">Product Designer</p>
                    </div>
                    <div class="mt-5 overflow-x-auto">
                        <table class="w-full mb-0">
                            <tbody>
                                <tr>
                                    <td class="px-3.5 py-2.5 first:pl-0 last:pr-0 border-y border-transparent text-slate-500 dark:text-zink-200">Employee ID</td>
                                    <td class="px-3.5 py-2.5 first:pl-0 last:pr-0 border-y border-transparent font-semibold">#TWE1001501</td>
                                </tr>
                                <tr>
                                    <td class="px-3.5 py-2.5 first:pl-0 last:pr-0 border-y border-transparent text-slate-500 dark:text-zink-200">Experience</td>
                                    <td class="px-3.5 py-2.5 first:pl-0 last:pr-0 border-y border-transparent font-semibold">3 Year</td>
                                </tr>
                                <tr>
                                    <td class="px-3.5 py-2.5 first:pl-0 last:pr-0 border-y border-transparent text-slate-500 dark:text-zink-200">Joining Date</td>
                                    <td class="px-3.5 py-2.5 first:pl-0 last:pr-0 border-y border-transparent font-semibold">05 Feb, 2020</td>
                                </tr>
                                <tr>
                                    <td class="px-3.5 py-2.5 first:pl-0 last:pr-0 border-y border-transparent text-slate-500 dark:text-zink-200">Total Hours (Years)</td>
                                    <td class="px-3.5 py-2.5 first:pl-0 last:pr-0 border-y border-transparent font-semibold">953.8 Hrs</td>
                                </tr>
                                <tr>
                                    <td class="px-3.5 py-2.5 first:pl-0 last:pr-0 border-y border-transparent text-slate-500 dark:text-zink-200">Total Hours</td>
                                    <td class="px-3.5 py-2.5 first:pl-0 last:pr-0 border-y border-transparent font-semibold">218.4 Hrs</td>
                                </tr>
                                <tr>
                                    <td class="px-3.5 py-2.5 first:pl-0 last:pr-0 border-y border-transparent text-slate-500 dark:text-zink-200">Regular Hours</td>
                                    <td class="px-3.5 py-2.5 first:pl-0 last:pr-0 border-y border-transparent font-semibold">172 Hrs</td>
                                </tr>
                                <tr>
                                    <td class="px-3.5 py-2.5 first:pl-0 last:pr-0 border-y border-transparent text-slate-500 dark:text-zink-200">Overtime</td>
                                    <td class="px-3.5 py-2.5 first:pl-0 last:pr-0 border-y border-transparent font-semibold">24 Hrs</td>
                                </tr>
                                <tr>
                                    <td class="px-3.5 py-2.5 first:pl-0 last:pr-0 border-y border-transparent text-slate-500 dark:text-zink-200">Holiday</td>
                                    <td class="px-3.5 py-2.5 first:pl-0 last:pr-0 border-y border-transparent font-semibold">22.40 Hrs</td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        </div>
        <div class="lg:col-span-4 xl:col-span-3">
            <div class="card">
                <div class="flex items-center gap-4 card-body">
                    <div class="flex items-center justify-center size-12 rounded-md text-sky-500 bg-sky-100 text-15 dark:bg-sky-500/20 shrink-0"><LucideIcon name="Clock"/></div>
                    <div class="grow">
                        <h5 class="mb-1 text-16"><span class="counter-value" data-target="187">0</span></h5>
                        <p class="text-slate-500 dark:text-zink-200">Approved Hours</p>
                    </div>
                </div>
            </div>
        </div>
        <div class="lg:col-span-4 xl:col-span-3">
            <div class="card">
                <div class="flex items-center gap-4 card-body">
                    <div class="flex items-center justify-center size-12 text-red-500 bg-red-100 rounded-md text-15 dark:bg-red-500/20 shrink-0"><LucideIcon name="XOctagon"/></div>
                    <div class="grow">
                        <h5 class="mb-1 text-16"><span class="counter-value" data-target="19">0</span></h5>
                        <p class="text-slate-500 dark:text-zink-200">Rejected Hours</p>
                    </div>
                </div>
            </div>
        </div><!--end col-->
        <div class="lg:col-span-4 xl:col-span-3">
            <div class="card">
                <div class="flex items-center gap-4 card-body">
                    <div class="flex items-center justify-center size-12 text-yellow-500 bg-yellow-100 rounded-md text-15 dark:bg-yellow-500/20 shrink-0"><LucideIcon name="RefreshCw"/></div>
                    <div class="grow">
                        <h5 class="mb-1 text-16"><span class="counter-value" data-target="32">0</span></h5>
                        <p class="text-slate-500 dark:text-zink-200">Pending Hours</p>
                    </div>
                </div>
            </div>
        </div>

        <div class="xl:col-span-9 lg:col-span-12">
            <div class="card">
                <div class="card-body">
                    <div class="grid grid-cols-1 gap-4 mb-5 lg:grid-cols-2 xl:grid-cols-12">
                        <div class="xl:col-span-3">
                            <div class="relative">
                                <input type="text" class="ltr:pl-8 rtl:pr-8 search form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Search for ..." autocomplete="off">
                                <LucideIcon name="Search" class="inline-block size-4 absolute ltr:left-2.5 rtl:right-2.5 top-2.5 text-slate-500 dark:text-zink-200 fill-slate-100 dark:fill-zink-600"/>
                            </div>
                        </div><!--end col-->
                        <div class="xl:col-span-3">
                            <div>
                                <Flatpickr type="text" options={{
                                    dateFormat:"d M, Y"
                                }} id="fromInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Select date" />
                            </div>
                        </div><!--end col-->
                        <div class="flex justify-end gap-2 text-right lg:col-span-2 xl:col-span-4 xl:col-start-10">
                            <a href="#!" type="button" class="text-red-500 bg-white border-red-500 border-dashed btn hover:text-red-500 hover:bg-red-50 hover:border-red-600 focus:text-red-600 focus:bg-red-50 focus:border-red-600 active:text-red-600 active:bg-red-50 active:border-red-600 dark:bg-zink-700 dark:ring-red-400/20 dark:hover:bg-red-800/20 dark:focus:bg-red-800/20 dark:active:bg-red-800/20">Rejecte All</a>
                            <a href="#!" type="button" class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20">Approve All</a>
                        </div>
                    </div><!--end grid-->
                    <div class="overflow-x-auto">
                        <table class="w-full whitespace-nowrap">
                            <thead class="ltr:text-left rtl:text-right bg-slate-100 text-slate-500 dark:text-zink-200 dark:bg-zink-600">
                                <tr>
                                    <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">Date</th>
                                    <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">Check In</th>
                                    <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">Check Out</th>
                                    <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">Meal Break</th>
                                    <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">Work Hours</th>
                                    <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">Overtime</th>
                                    <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">Action</th>
                                </tr>
                            </thead>
                            <tbody>
                                {#each AttendanceData as attendance}
                                <tr>
                                    <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">
                                        {attendance.date} <span class="px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-white border-slate-400 text-slate-500 dark:bg-zink-700 dark:border-zink-400 dark:text-zink-200 ltr:ml-1 rtl:mr-1 align-middle">{attendance.day}</span>
                                    </td>
                                    <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">{attendance.checkIn}</td>
                                    <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">{attendance.checkOut}</td>
                                    <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">{attendance.mealBreak}</td>
                                    <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">{attendance.workHours}</td>
                                    <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">{attendance.overtime}</td>
                                    <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">
                                        <div class="flex gap-2">
                                            <a href="#!" class="flex items-center justify-center size-8 text-green-500 transition-all duration-200 ease-linear bg-green-100 rounded-md hover:text-white hover:bg-green-500 dark:bg-green-500/20 dark:hover:bg-green-500"><LucideIcon name="Check" class="size-4"/></a>
                                            <a href="#!" class="flex items-center justify-center size-8 text-red-500 transition-all duration-200 ease-linear bg-red-100 rounded-md hover:text-white hover:bg-red-500 dark:bg-red-500/20 dark:hover:bg-red-500"><LucideIcon name="X" class="size-4"/></a>
                                        </div>
                                    </td>
                                </tr>
                                {/each}
                            </tbody>
                        </table>
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
    </div>
</div>