<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
    import Flatpickr from 'svelte-flatpickr';
    import 'flatpickr/dist/flatpickr.css';
</script>

<HeadTitle title="Time Picker" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Time Picker" pagetitle="Forms" />

    <div class="grid grid-cols-1 gap-x-5 xl:grid-cols-2">
        <div class="card">
            <div class="card-body">
                <h6 class="mb-1 text-15">Timepicker</h6>
                <p class="mb-4">Set <code class="text-xs text-pink-500 select-all">data-provider="timepickr" data-time-basic="true"</code> attribute.</p>
                <Flatpickr options={{enableTime: true,noCalendar: true,time_24hr: false}} class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" name="date" placeholder="Select Time" />
            </div>
        </div><!--end card-->
        <div class="card">
            <div class="card-body">
                <h6 class="mb-1 text-15">24-hour Time Picker</h6>
                <p class="mb-4">Set <code class="text-xs text-pink-500 select-all">data-provider="timepickr" data-time-hrs="true"</code> attribute.</p>
                <Flatpickr options={{enableTime: true,noCalendar: true,time_24hr: true}} class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" name="date" placeholder="Select Time" />
            </div>
        </div><!--end card-->
        <div class="card">
            <div class="card-body">
                <h6 class="mb-1 text-15">Time Picker w/ Limits</h6>
                <p class="mb-4">Set <code class="text-xs text-pink-500 select-all">data-provider="timepickr" data-min-time="Min.Time" data-max-time="Max.Time"</code> attribute.</p>
                <Flatpickr options={{enableTime: true,noCalendar: true,maxTime:"16:00",minTime:"11:00"}} class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" name="date" placeholder="Select Time" />
            </div>
        </div><!--end card-->
        <div class="card">
            <div class="card-body">
                <h6 class="mb-1 text-15">Preloading Time</h6>
                <p class="mb-4">Set <code class="text-xs text-pink-500 select-all">data-provider="timepickr" data-default-time="Your Default Time"</code> attribute.</p>
                <Flatpickr options={{enableTime: true,noCalendar: true,defaultDate: "13:45"}} class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" name="date" placeholder="Select Time" />
            </div>
        </div><!--end card-->
        <div class="card">
            <div class="card-body">
                <h6 class="mb-1 text-15">Inline</h6>
                <p class="mb-4">Set <code class="text-xs text-pink-500 select-all">data-provider="timepickr" data-time-inline="Your Default Time"</code> attribute.</p>
                <Flatpickr options={{enableTime: true,noCalendar: true,inline:true}} class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" name="date" placeholder="Select Time" />
            </div>
        </div><!--end card-->
    </div>
</div>
