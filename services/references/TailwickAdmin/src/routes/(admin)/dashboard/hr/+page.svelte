<script>
    import HeadTitle from "../../../../common/components/HeadTitle.svelte";
    import Breadcrumb from "../../../../common/components/Breadcrumb.svelte";
    import CounterPage from "../../../../common/components/CounterPage.svelte";
    import Widget from "./Widget.svelte";
    import EmployeePerformance from "./EmployeePerformance.svelte";
    import UpcomingScheduled from "./UpcomingScheduled.svelte";
    import TotalProjectsChart from "./TotalProjectsChart.svelte";
    import UpcomingInterview from "./UpcomingInterview.svelte";
    import RecentPayroll from "./RecentPayroll.svelte";
</script>

<HeadTitle title="HR" />
<CounterPage />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
    <Breadcrumb title="HR" pagetitle="Dashboards" />
    <div class="grid grid-cols-12 2xl:grid-cols-12 gap-x-5">
        <div class="col-span-12 md:order-1 xl:col-span-8 2xl:col-span-6">
            <h5 class="mb-2">Welcome Paula Keenan 🎉</h5>
            <p class="mb-5 text-slate-500 dark:text-zink-200">The salary of <a href="#!"
                    class="underline text-slate-800 dark:text-zink-50">Glennie Langosh</a> is pending since 05 Dec,
                2023. the documentation of the tasks, workflows, and activities that make up a process managed by the HR
                or People Ops team. <a href="#!" class="text-red-500">Learn More</a></p>
        </div>
        <div class="col-span-12 md:order-2 xl:col-span-4 2xl:col-start-9 card">
            <div class="p-4">
                <div class="grid grid-cols-3">
                    <div
                        class="px-4 text-center ltr:border-r rtl:border-l border-slate-200 dark:border-zink-500 ltr:last:border-r-0 rtl:last:border-l-0">
                        <h6 class="mb-1 font-bold"><span class="counter-value" data-target="36"></span></h6>
                        <p class="text-slate-500 dark:text-zink-200">Absent</p>
                    </div>
                    <div
                        class="px-4 text-center ltr:border-r rtl:border-l border-slate-200 dark:border-zink-500 ltr:last:border-r-0 rtl:last:border-l-0">
                        <h6 class="mb-1 font-bold"><span class="counter-value" data-target="465"></span></h6>
                        <p class="text-slate-500 dark:text-zink-200">Attendance</p>
                    </div>
                    <div
                        class="px-4 text-center ltr:border-r rtl:border-l border-slate-200 dark:border-zink-500 ltr:last:border-r-0 rtl:last:border-l-0">
                        <h6 class="mb-1 font-bold"><span class="counter-value" data-target="50"></span></h6>
                        <p class="text-slate-500 dark:text-zink-200">Late</p>
                    </div>
                </div>
            </div>
        </div>

        <Widget />

        <EmployeePerformance />

        <UpcomingScheduled />

        <TotalProjectsChart chartColors={["bg-custom-500", "bg-yellow-500" , "bg-green-400" , "bg-red-400" ]} />

        <UpcomingInterview />

        <RecentPayroll />
    </div>

</div>