<script>
    import Flatpickr from 'svelte-flatpickr';
    import 'flatpickr/dist/flatpickr.css';
</script>

<div class="col-span-12 md:order-9 lg:col-span-6 lg:row-span-2 xl:col-span-4 xl:row-span-2 2xl:row-span-2 2xl:col-span-3 card">
    <div class="card-body">
        <h6 class="mb-3 text-15 grow">Upcoming Scheduled</h6>
        <Flatpickr name="date" options={{inline:true}} class="hidden"/>
        <div class="flex flex-col gap-4 mt-3">
            <div class="flex gap-3">
                <div class="flex flex-col items-center justify-center size-12 border rounded-sm border-slate-200 dark:border-zink-500 shrink-0"><h6>28</h6>  <span class="text-sm text-slate-500 dark:text-zink-200">July</span></div>
                <div class="grow">
                    <h6 class="mb-1">Meeting with Designer <small class="inline-block px-2 font-medium border border-transparent rounded text-[11px] py-0.5 bg-slate-100 text-slate-500 dark:bg-slate-500/20 dark:text-zink-200 dark:border-transparent">09:57 AM</small></h6>
                    <p class="text-slate-500 dark:text-zink-200">Created by Admin</p>
                </div>
            </div>
            <div class="flex gap-3">
                <div class="flex flex-col items-center justify-center size-12 border rounded-sm border-slate-200 dark:border-zink-500 shrink-0">
                    <h6>08</h6> <span class="text-sm text-slate-500 dark:text-zink-200">June</span>
                </div>
                <div class="grow">
                    <h6 class="mb-1">Developing Line Managers Conference <small class="inline-block px-2 font-medium border border-transparent rounded text-[11px] py-0.5 bg-slate-100 text-slate-500 dark:bg-slate-500/20 dark:text-zink-200 dark:border-transparent">10:54 AM</small></h6>
                    <p class="text-slate-500 dark:text-zink-200">Created by HR</p>
                </div>
            </div>
            <div class="flex gap-3">
                <div class="flex flex-col items-center justify-center size-12 border rounded-sm border-slate-200 dark:border-zink-500 shrink-0">
                    <h6>17</h6> <span class="text-sm text-slate-500 dark:text-zink-200">July</span>
                </div>
                <div class="grow">
                    <h6 class="mb-1">Airplane in Las Vegas <small class="inline-block px-2 font-medium border border-transparent rounded text-[11px] py-0.5 bg-slate-100 text-slate-500 dark:bg-slate-500/20 dark:text-zink-200 dark:border-transparent">12:00 PM</small></h6>
                    <p class="text-slate-500 dark:text-zink-200">Created by HR</p>
                </div>
            </div>
            <div class="flex gap-3">
                <div class="flex flex-col items-center justify-center size-12 border rounded-sm border-slate-200 dark:border-zink-500 shrink-0">
                    <h6>11</h6> <span class="text-sm text-slate-500 dark:text-zink-200">Nov</span>
                </div>
                <div class="grow">
                    <h6 class="mb-1">Hospitality Project Discuses</h6>
                    <p class="text-slate-500 dark:text-zink-200">Created by Admin</p>
                </div>
            </div>
            <div class="flex gap-3">
                <div class="flex flex-col items-center justify-center size-12 border rounded-sm border-slate-200 dark:border-zink-500 shrink-0">
                    <h6>20</h6> <span class="text-sm text-slate-500 dark:text-zink-200">Nov</span>
                </div>
                <div class="grow">
                    <h6 class="mb-1">Gartner Digital Workplace <small class="inline-block px-2 font-medium border border-transparent rounded text-[11px] py-0.5 bg-slate-100 text-slate-500 dark:bg-slate-500/20 dark:text-zink-200 dark:border-transparent">03:49 PM</small></h6>
                    <p class="text-slate-500 dark:text-zink-200">Created by HR</p>
                </div>
            </div>
            <div class="flex gap-3">
                <div class="flex flex-col items-center justify-center size-12 border rounded-sm border-slate-200 dark:border-zink-500 shrink-0">
                    <h6>04</h6> <span class="text-sm text-slate-500 dark:text-zink-200">Dec</span>
                </div>
                <div class="grow">
                    <h6 class="mb-1">Nordic People Analytics <small class="inline-block px-2 font-medium border border-transparent rounded text-[11px] py-0.5 bg-slate-100 text-slate-500 dark:bg-slate-500/20 dark:text-zink-200 dark:border-transparent">11:00 AM</small></h6>
                    <p class="text-slate-500 dark:text-zink-200">Created by Admin</p>
                </div>
            </div>
            <!-- <div class="flex gap-3">
                <div class="flex flex-col items-center justify-center size-12 border rounded-sm border-slate-200 dark:border-zink-500 shrink-0">
                    <h6>17</h6> <span class="text-sm text-slate-500 dark:text-zink-200">Jan</span>
                </div>
                <div class="grow">
                    <h6 class="mb-1">CIPD Festival of Work <small class="inline-block px-2 font-medium border border-transparent rounded text-[11px] py-0.5 bg-slate-100 text-slate-500 dark:bg-slate-500/20 dark:text-zink-200 dark:border-transparent">01:29 PM</small></h6>
                    <p class="text-slate-500 dark:text-zink-200">Created by HR</p>
                </div>
            </div> -->
            <!-- <div class="flex gap-3">
                <div class="flex flex-col items-center justify-center size-12 border rounded-sm border-slate-200 dark:border-zink-500 shrink-0">
                    <h6>03</h6> <span class="text-sm text-slate-500 dark:text-zink-200">Feb</span>
                </div>
                <div class="grow">
                    <h6 class="mb-1">HRO Today Forum <small class="inline-block px-2 font-medium border border-transparent rounded text-[11px] py-0.5 bg-slate-100 text-slate-500 dark:bg-slate-500/20 dark:text-zink-200 dark:border-transparent">02:15 PM</small></h6>
                    <p class="text-slate-500 dark:text-zink-200">Created by Admin</p>
                </div>
            </div> -->
        </div>
        <div class="flex gap-3 p-2 mt-3 rounded-md bg-custom-500">
            <div class="shrink-0">
                <img src="/assets/images/support.png" alt="" class="h-24">
            </div>
            <div>
                <h6 class="mb-1 text-15 text-custom-50">Need Help ?</h6>
                <p class="text-custom-200">If you would like to learn more about transferring the license to a customer</p>
            </div>
        </div>
    </div>
</div>