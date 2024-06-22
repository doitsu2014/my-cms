<script>
    import HeadTitle from "../../../../../common/components/HeadTitle.svelte";
	import Breadcrumb from "../../../../../common/components/Breadcrumb.svelte";
	import LucideIcon from "../../../../../common/components/LucideIcon.svelte";
	import CounterPage from "../../../../../common/components/CounterPage.svelte";
	import Modal from "../../../../../common/components/Modal.svelte";
    import { MainAttendanceData } from "../../../../../common/data/hrmanagement"
	import Flatpickr from "svelte-flatpickr";
    import 'flatpickr/dist/flatpickr.css';

    let isAttendanceInfoModal = false;
    const toggleAttendanceInfo =() => {
        isAttendanceInfoModal = !isAttendanceInfoModal;
    }

</script>
<HeadTitle title="Main Attendance" />

<CounterPage />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto relative">
    <Breadcrumb title="Main Attendance" pagetitle="HR Management" />

    <div class="grid grid-cols-1 gap-x-5 md:grid-cols-2 xl:grid-cols-12">
        <div class="xl:col-span-3">
            <div class="card">
                <div class="flex items-center gap-4 card-body">
                    <div class="flex items-center justify-center size-12 rounded-md text-sky-500 bg-sky-100 text-15 dark:bg-sky-500/20 shrink-0"><LucideIcon name="Users2"/></div>
                    <div class="overflow-hidden grow">
                        <h5 class="mb-1 text-16"><span class="counter-value" data-target="43">0</span></h5>
                        <p class="truncate text-slate-500 dark:text-zink-200">Total Employee</p>
                    </div>
                </div>
            </div>
        </div><!--end col-->
        <div class="xl:col-span-3">
            <div class="card">
                <div class="flex items-center gap-4 card-body">
                    <div class="flex items-center justify-center size-12 text-red-500 bg-red-100 rounded-md text-15 dark:bg-red-500/20 shrink-0"><LucideIcon name="UserX2"/></div>
                    <div class="overflow-hidden grow">
                        <h5 class="mb-1 text-16"><span class="counter-value" data-target="6">0</span></h5>
                        <p class="truncate text-slate-500 dark:text-zink-200">Absent Employee (Today)</p>
                    </div>
                </div>
            </div>
        </div><!--end col-->
        <div class="xl:col-span-3">
            <div class="card">
                <div class="flex items-center gap-4 card-body">
                    <div class="flex items-center justify-center size-12 text-green-500 bg-green-100 rounded-md text-15 dark:bg-green-500/20 shrink-0"><LucideIcon name="UserCheck2"/></div>
                    <div class="overflow-hidden grow">
                        <h5 class="mb-1 text-16"><span class="counter-value" data-target="32">0</span></h5>
                        <p class="truncate text-slate-500 dark:text-zink-200">Present Employee (Today)</p>
                    </div>
                </div>
            </div>
        </div><!--end col-->
        <div class="xl:col-span-3">
            <div class="card">
                <div class="flex items-center gap-4 card-body">
                    <div class="flex items-center justify-center size-12 rounded-md text-custom-500 bg-custom-100 text-15 dark:bg-custom-500/20 shrink-0"><LucideIcon name="Briefcase"/></div>
                    <div class="overflow-hidden grow">
                        <h5 class="mb-1 text-16"><span class="counter-value" data-target="22">0</span></h5>
                        <p class="truncate text-slate-500 dark:text-zink-200">Working Days (Current Month)</p>
                    </div>
                </div>
            </div>
        </div><!--end col-->
    </div>

    <div class="card">
        <div class="card-body">
            <div class="grid grid-cols-1 gap-5 mb-5 xl:grid-cols-12">
                <div class="xl:col-span-3">
                    <div class="relative">
                        <input type="text" class="ltr:pl-8 rtl:pr-8 search form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Search for ..." autocomplete="off">
                        <LucideIcon name="Search" class="inline-block size-4 absolute ltr:left-2.5 rtl:right-2.5 top-2.5 text-slate-500 dark:text-zink-200 fill-slate-100 dark:fill-zink-600"/>
                    </div>
                </div><!--end col-->
                <div class="xl:col-span-2 xl:col-start-11">
                    <div class="relative">
                        <LucideIcon name="CalendarRange" class="absolute size-4 ltr:left-3 rtl:right-3 top-3 text-slate-500 dark:text-zink-200"/>
                        <Flatpickr type="text" options={{
                            dateFormat:"d M, Y"
                        }} id="fromInput" class="ltr:pl-10 rtl:pr-10 form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Select Date" />
                    </div>
                </div>
            </div><!--end grid-->
            <div class="overflow-x-auto">
                <table class="w-full whitespace-nowrap">
                    <thead class="ltr:text-left rtl:text-right bg-slate-100 text-slate-500 dark:text-zink-200 dark:bg-zink-600">
                        <tr class="*:px-3.5 *:py-2.5 *:font-semibold *:border-b *:border-slate-200 *:dark:border-zink-500">
                            <th>Employee Name</th>
                            <th>01</th>
                            <th>02</th>
                            <th>03</th>
                            <th>04</th>
                            <th>05</th>
                            <th class="active">06</th>
                            <th>07</th>
                            <th>08</th>
                            <th>09</th>
                            <th>10</th>
                            <th>11</th>
                            <th>12</th>
                            <th>13</th>
                            <th>14</th>
                            <th>15</th>
                            <th>16</th>
                            <th>17</th>
                            <th>18</th>
                            <th>19</th>
                            <th>20</th>
                            <th>21</th>
                            <th>22</th>
                            <th>23</th>
                            <th>24</th>
                            <th>25</th>
                            <th>26</th>
                            <th>27</th>
                            <th>28</th>
                            <th>29</th>
                            <th>30</th>
                        </tr>
                    </thead>
                    <tbody>
                        {#each MainAttendanceData as attendance}
                            
                        <tr class="*:px-3.5 *:py-2.5 *:border-y *:border-slate-200 *:dark:border-zink-500">
                            <td ><a href="#!" class="transition-all duration-200 ease-linear">{attendance.employeeName}</a></td>
                            <td >
                                {#if attendance.Day1}
                                <a href="#!" on:click={toggleAttendanceInfo} class="text-green-500 transition-all duration-200 ease-linear hover:text-green-600"><LucideIcon name="Check" class="size-4"/></a>
                                {:else}
                                    <LucideIcon name="X" class="size-4 text-red-500"/>
                                {/if}
                            </td>
                            <td >
                                {#if attendance.Day2}
                                <a href="#!" on:click={toggleAttendanceInfo} class="text-green-500 transition-all duration-200 ease-linear hover:text-green-600"><LucideIcon name="Check" class="size-4"/></a>
                                {:else}
                                    <LucideIcon name="X" class="size-4 text-red-500"/>
                                {/if}
                            </td>
                            <td >-</td>
                            <td >-</td>
                            <td >
                                 {#if attendance.Day5}
                                <a href="#!" on:click={toggleAttendanceInfo} class="text-green-500 transition-all duration-200 ease-linear hover:text-green-600"><LucideIcon name="Check" class="size-4"/></a>
                                {:else}
                                    <LucideIcon name="X" class="size-4 text-red-500"/>
                                {/if}
                            </td>
                            <td >
                                 {#if attendance.Day6}
                                <a href="#!" on:click={toggleAttendanceInfo} class="text-green-500 transition-all duration-200 ease-linear hover:text-green-600"><LucideIcon name="Check" class="size-4"/></a>
                                {:else}
                                    <LucideIcon name="X" class="size-4 text-red-500"/>
                                {/if}
                            </td>
                            <td >
                                 {#if attendance.Day7}
                                <a href="#!" on:click={toggleAttendanceInfo} class="text-green-500 transition-all duration-200 ease-linear hover:text-green-600"><LucideIcon name="Check" class="size-4"/></a>
                                {:else}
                                    <LucideIcon name="X" class="size-4 text-red-500"/>
                                {/if}
                            </td>
                            <td >
                                 {#if attendance.Day8}
                                <a href="#!" on:click={toggleAttendanceInfo} class="text-green-500 transition-all duration-200 ease-linear hover:text-green-600"><LucideIcon name="Check" class="size-4"/></a>
                                {:else}
                                    <LucideIcon name="X" class="size-4 text-red-500"/>
                                {/if}
                            </td>
                            <td >
                                {#if attendance.Day9}
                                <a href="#!" on:click={toggleAttendanceInfo} class="text-green-500 transition-all duration-200 ease-linear hover:text-green-600"><LucideIcon name="Check" class="size-4"/></a>
                                {:else}
                                    <LucideIcon name="X" class="size-4 text-red-500"/>
                                {/if}
                            </td>
                            <td >-</td>
                            <td >-</td>
                            <td >
                                {#if attendance.Day12}
                                <a href="#!" on:click={toggleAttendanceInfo} class="text-green-500 transition-all duration-200 ease-linear hover:text-green-600"><LucideIcon name="Check" class="size-4"/></a>
                                {:else}
                                    <LucideIcon name="X" class="size-4 text-red-500"/>
                                {/if}
                            </td>
                            <td >
                                {#if attendance.Day13}
                                <a href="#!" on:click={toggleAttendanceInfo} class="text-green-500 transition-all duration-200 ease-linear hover:text-green-600"><LucideIcon name="Check" class="size-4"/></a>
                                {:else}
                                    <LucideIcon name="X" class="size-4 text-red-500"/>
                                {/if}
                            </td>
                            <td >
                                {#if attendance.Day14}
                                <a href="#!" on:click={toggleAttendanceInfo} class="text-green-500 transition-all duration-200 ease-linear hover:text-green-600"><LucideIcon name="Check" class="size-4"/></a>
                                {:else}
                                    <LucideIcon name="X" class="size-4 text-red-500"/>
                                {/if}
                            </td>
                            <td >
                                {#if attendance.Day15}
                                <a href="#!" on:click={toggleAttendanceInfo} class="text-green-500 transition-all duration-200 ease-linear hover:text-green-600"><LucideIcon name="Check" class="size-4"/></a>
                                {:else}
                                    <LucideIcon name="X" class="size-4 text-red-500"/>
                                {/if}
                            </td>
                            <td >
                                {#if attendance.Day16}
                                <a href="#!" on:click={toggleAttendanceInfo} class="text-green-500 transition-all duration-200 ease-linear hover:text-green-600"><LucideIcon name="Check" class="size-4"/></a>
                                {:else}
                                    <LucideIcon name="X" class="size-4 text-red-500"/>
                                {/if}
                            </td>
                            <td >-</td>
                            <td >-</td>
                            <td >
                                {#if attendance.Day19}
                                <a href="#!" on:click={toggleAttendanceInfo} class="text-green-500 transition-all duration-200 ease-linear hover:text-green-600"><LucideIcon name="Check" class="size-4"/></a>
                                {:else}
                                    <LucideIcon name="X" class="size-4 text-red-500"/>
                                {/if}
                            </td>
                            <td >
                                {#if attendance.Day20}
                                <a href="#!" on:click={toggleAttendanceInfo} class="text-green-500 transition-all duration-200 ease-linear hover:text-green-600"><LucideIcon name="Check" class="size-4"/></a>
                                {:else}
                                    <LucideIcon name="X" class="size-4 text-red-500"/>
                                {/if}
                            </td>
                            <td >
                                {#if attendance.Day21}
                                <a href="#!" on:click={toggleAttendanceInfo} class="text-green-500 transition-all duration-200 ease-linear hover:text-green-600"><LucideIcon name="Check" class="size-4"/></a>
                                {:else}
                                    <LucideIcon name="X" class="size-4 text-red-500"/>
                                {/if}
                            </td>
                            <td >
                                {#if attendance.Day22}
                                <a href="#!" on:click={toggleAttendanceInfo} class="text-green-500 transition-all duration-200 ease-linear hover:text-green-600"><LucideIcon name="Check" class="size-4"/></a>
                                {:else}
                                    <LucideIcon name="X" class="size-4 text-red-500"/>
                                {/if}
                            </td>
                            <td >
                                {#if attendance.Day23}
                                <a href="#!" on:click={toggleAttendanceInfo} class="text-green-500 transition-all duration-200 ease-linear hover:text-green-600"><LucideIcon name="Check" class="size-4"/></a>
                                {:else}
                                    <LucideIcon name="X" class="size-4 text-red-500"/>
                                {/if}
                            </td>
                            <td >-</td>
                            <td >-</td>
                            <td >
                                {#if attendance.Day26}
                                <a href="#!" on:click={toggleAttendanceInfo} class="text-green-500 transition-all duration-200 ease-linear hover:text-green-600"><LucideIcon name="Check" class="size-4"/></a>
                                {:else}
                                    <LucideIcon name="X" class="size-4 text-red-500"/>
                                {/if}
                            </td>
                            <td >
                                {#if attendance.Day27}
                                <a href="#!" on:click={toggleAttendanceInfo} class="text-green-500 transition-all duration-200 ease-linear hover:text-green-600"><LucideIcon name="Check" class="size-4"/></a>
                                {:else}
                                    <LucideIcon name="X" class="size-4 text-red-500"/>
                                {/if}
                            </td>
                            <td >
                                {#if attendance.Day28}
                                <a href="#!" on:click={toggleAttendanceInfo} class="text-green-500 transition-all duration-200 ease-linear hover:text-green-600"><LucideIcon name="Check" class="size-4"/></a>
                                {:else}
                                    <LucideIcon name="X" class="size-4 text-red-500"/>
                                {/if}
                            </td>
                            <td >
                                {#if attendance.Day29}
                                <a href="#!" on:click={toggleAttendanceInfo} class="text-green-500 transition-all duration-200 ease-linear hover:text-green-600"><LucideIcon name="Check" class="size-4"/></a>
                                {:else}
                                    <LucideIcon name="X" class="size-4 text-red-500"/>
                                {/if}
                            </td>
                            <td >
                                {#if attendance.Day30}
                                <a href="#!" on:click={toggleAttendanceInfo} class="text-green-500 transition-all duration-200 ease-linear hover:text-green-600"><LucideIcon name="Check" class="size-4"/></a>
                                {:else}
                                    <LucideIcon name="X" class="size-4 text-red-500"/>
                                {/if}
                            </td>
                        </tr>
                        {/each}
                    </tbody>
                </table>
            </div>
        </div>
    </div>
</div>

<Modal modal-center className="-translate-y-2/4" isOpen={isAttendanceInfoModal} toggle={toggleAttendanceInfo}>
    <div class="w-screen md:w-[30rem] bg-white shadow rounded-md dark:bg-zink-600">
        <div class="flex items-center justify-between p-4 border-b dark:border-zink-300/20">
            <h5 class="text-16">Attendance Info (05 June, 2023)</h5>
            <button data-modal-close="addOrderModal" class="transition-all duration-200 ease-linear text-slate-400 hover:text-red-500"><LucideIcon name="X" class="size-5"/></button>
        </div>
        <div class="max-h-[calc(theme('height.screen')_-_180px)] p-4 overflow-y-auto">
            <div class="rounded-md bg-slate-100 dark:bg-zink-500/50">
                <div class="p-3">
                    <p class="mb-1 text-slate-500 dark:text-zink-200">Check In Date & Time</p>
                    <h6>Monday, 05 June, 2023 <b>08:29 AM</b></h6>
                </div>
                <div class="p-5">
                    <div class="flex items-center justify-center size-24 mx-auto bg-white rounded-full dark:bg-zink-600 text-16 outline outline-white dark:outline-zink-600 outline-offset-4">
                        <b>08:15</b> <small class="ltr:ml-1 rtl:mr-1">Hrs</small>
                    </div>
                </div>
                <div class="p-3">
                    <p class="mb-1 text-slate-500 dark:text-zink-200">Check Out Date & Time</p>
                    <h6>Monday, 05 June, 2023 <b>06:11 AM</b></h6>
                </div>
            </div>
        </div>
    </div>
</Modal>