<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';

</script>

<HeadTitle title="Lists" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Lists" pagetitle="UI Elements" />
  
    <div class="grid grid-cols-1 gap-x-5 md:grid-cols-2 xl:grid-cols-3">
        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">Basic List</h6>
                <ul class="space-y-5 rounded-md">
                    <li>Send the billing agreement</li>
                    <li>Send over all the documentation.</li>
                    <li>Meeting with daron to review the intake form</li>
                    <li>Check tailwick theme and give customer support</li>
                </ul>
            </div>
        </div><!--end card-->
        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">Bullet List</h6>
                <ul class="space-y-5 list-disc list-inside rounded-md">
                    <li>Send the billing agreement</li>
                    <li>Send over all the documentation.</li>
                    <li>Meeting with daron to review the intake form</li>
                    <li>Check tailwick theme and give customer support</li>
                </ul>
            </div>
        </div><!--end card-->
        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">List with Numbered</h6>
                <ul class="space-y-5 list-decimal list-inside rounded-md">
                    <li>Send the billing agreement</li>
                    <li>Send over all the documentation.</li>
                    <li>Meeting with daron to review the intake form</li>
                    <li>Check tailwick theme and give customer support</li>
                </ul>
            </div>
        </div><!--end card-->
        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">Marker Colors</h6>
                <ul class="space-y-5 list-disc list-inside rounded-md marker:text-red-500">
                    <li>Send the billing agreement</li>
                    <li>Send over all the documentation.</li>
                    <li>Meeting with daron to review the intake form</li>
                    <li>Check tailwick theme and give customer support</li>
                </ul>
            </div>
        </div><!--end card-->
        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">Marker Colors</h6>
                <ul class="space-y-5 list-decimal list-inside rounded-md marker:text-custom-500">
                    <li>Send the billing agreement</li>
                    <li>Send over all the documentation.</li>
                    <li>Meeting with daron to review the intake form</li>
                    <li>Check tailwick theme and give customer support</li>
                </ul>
            </div>
        </div><!--end card-->
        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">Custom Square List</h6>
                <ul class="list-[square] rounded-md list-inside space-y-5">
                    <li>Send the billing agreement</li>
                    <li>Send over all the documentation.</li>
                    <li>Meeting with daron to review the intake form</li>
                    <li>Check tailwick theme and give customer support</li>
                </ul>
            </div>
        </div><!--end card-->
        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">Bordered List</h6>
                <ul class="border rounded-md border-slate-200 dark:border-zink-500">
                    <li class="px-3.5 py-2.5 border-b dark:border-zink-500">Send the billing agreement</li>
                    <li class="px-3.5 py-2.5 border-b dark:border-zink-500 text-custom-50 bg-custom-500">Send over all the documentation.</li>
                    <li class="px-3.5 py-2.5 border-b dark:border-zink-500">Meeting with daron to review the intake form</li>
                    <li class="px-3.5 py-2.5">Check tailwick theme and give customer support</li>
                </ul>
            </div>
        </div><!--end card-->
        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">Flush List</h6>
                <ul>
                    <li class="px-3.5 py-2.5 border-b dark:border-zink-500">Send the billing agreement</li>
                    <li class="px-3.5 py-2.5 border-b dark:border-zink-500">Send over all the documentation.</li>
                    <li class="px-3.5 py-2.5 border-b dark:border-zink-500">Meeting with daron to review the intake form</li>
                    <li class="px-3.5 py-2.5">Check tailwick theme and give customer support</li>
                </ul>
            </div>
        </div><!--end card-->
        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">List with Link</h6>
                <div class="flex flex-col border rounded-md dark:border-zink-500">
                    <a href="#!" class="block px-3.5 py-2.5 transition border-b dark:border-zink-500 hover:bg-slate-100 dark:hover:bg-zink-600">Category Download</a>
                    <a href="#!" class="block px-3.5 py-2.5 transition border-b dark:border-zink-500 bg-custom-500 text-custom-50">Security Access</a>
                    <a href="#!" class="block px-3.5 py-2.5 transition border-b dark:border-zink-500 hover:bg-slate-100 dark:hover:bg-zink-600">Storage folder</a>
                    <a href="#!" class="block px-3.5 py-2.5 transition pointer-events-none bg-slate-100 text-slate-500 dark:text-zink-200 dark:bg-zink-600">Push Notification disabled</a>
                </div>
            </div>
        </div><!--end card-->
        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">List with Checkbox</h6>
                <div class="flex flex-col border rounded-md dark:border-zink-300/20">
                    <label class="px-3.5 py-2.5 border-b dark:border-zink-300/20 flex items-center gap-2">
                        <input class="size-4 border rounded-sm appearance-none bg-slate-100 border-slate-200 ltr:mr-1 rtl:ml-1 checked:bg-custom-500 checked:border-custom-500 dark:bg-zink-600 dark:border-zink-500 dark:checked:bg-custom-500 dark:checked:border-custom-500" type="checkbox" value="" checked>
                        Declined Payment
                    </label>
                    <label class="px-3.5 py-2.5 border-b dark:border-zink-300/20 flex items-center gap-2">
                        <input class="size-4 border rounded-sm appearance-none bg-slate-100 border-slate-200 ltr:mr-1 rtl:ml-1 checked:bg-custom-500 checked:border-custom-500 dark:bg-zink-600 dark:border-zink-500 dark:checked:bg-custom-500 dark:checked:border-custom-500" type="checkbox" value="">
                        Delivery Error
                    </label>
                    <label class="px-3.5 py-2.5 border-b dark:border-zink-300/20 flex items-center gap-2">
                        <input class="size-4 border rounded-sm appearance-none bg-slate-100 border-slate-200 ltr:mr-1 rtl:ml-1 checked:bg-custom-500 checked:border-custom-500 dark:bg-zink-600 dark:border-zink-500 dark:checked:bg-custom-500 dark:checked:border-custom-500" type="checkbox" value="">
                        Wrong Amount
                    </label>
                    <label class="px-3.5 py-2.5 border-b dark:border-zink-300/20 flex items-center gap-2">
                        <input class="size-4 border rounded-sm appearance-none bg-slate-100 border-slate-200 ltr:mr-1 rtl:ml-1 checked:bg-custom-500 checked:border-custom-500 dark:bg-zink-600 dark:border-zink-500 dark:checked:bg-custom-500 dark:checked:border-custom-500" type="checkbox" value="" checked>
                        Wrong Address
                    </label>
                    <label class="px-3.5 py-2.5 flex items-center gap-2">
                        <input class="size-4 border rounded-sm appearance-none bg-slate-100 border-slate-200 ltr:mr-1 rtl:ml-1 checked:bg-custom-500 checked:border-custom-500 dark:bg-zink-600 dark:border-zink-500 dark:checked:bg-custom-500 dark:checked:border-custom-500" type="checkbox" value="">
                        Wrong UX/UI Solution
                    </label>
                </div>
            </div>
        </div><!--end card-->
        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">List with Radio</h6>
                <div class="flex flex-col border rounded-md dark:border-zink-300/20">
                    <label class="px-3.5 py-2.5 border-b dark:border-zink-300/20 flex items-center gap-2">
                        <input class="size-4 border rounded-full appearance-none bg-slate-100 border-slate-200 text-custom-600 checked:bg-custom-500 checked:border-custom-500 dark:bg-zink-600 dark:border-zink-500 dark:checked:bg-custom-500 dark:checked:border-custom-500" type="radio" name="listWithRadio" value="">
                        Declined Payment
                    </label>
                    <label class="px-3.5 py-2.5 border-b dark:border-zink-300/20 flex items-center gap-2">
                        <input class="size-4 border rounded-full appearance-none bg-slate-100 border-slate-200 text-custom-600 checked:bg-custom-500 checked:border-custom-500 dark:bg-zink-600 dark:border-zink-500 dark:checked:bg-custom-500 dark:checked:border-custom-500" type="radio" name="listWithRadio" value="" checked>
                        Delivery Error
                    </label>
                    <label class="px-3.5 py-2.5 border-b dark:border-zink-300/20 flex items-center gap-2">
                        <input class="size-4 border rounded-full appearance-none bg-slate-100 border-slate-200 text-custom-600 checked:bg-custom-500 checked:border-custom-500 dark:bg-zink-600 dark:border-zink-500 dark:checked:bg-custom-500 dark:checked:border-custom-500" type="radio" name="listWithRadio" value="">
                        Wrong Amount
                    </label>
                    <label class="px-3.5 py-2.5 border-b dark:border-zink-300/20 flex items-center gap-2">
                        <input class="size-4 border rounded-full appearance-none bg-slate-100 border-slate-200 text-custom-600 checked:bg-custom-500 checked:border-custom-500 dark:bg-zink-600 dark:border-zink-500 dark:checked:bg-custom-500 dark:checked:border-custom-500" type="radio" name="listWithRadio" value="">
                        Wrong Address
                    </label>
                    <label class="px-3.5 py-2.5 flex items-center gap-2">
                        <input class="size-4 border rounded-full appearance-none bg-slate-100 border-slate-200 text-custom-600 checked:bg-custom-500 checked:border-custom-500 dark:bg-zink-600 dark:border-zink-500 dark:checked:bg-custom-500 dark:checked:border-custom-500" type="radio" name="listWithRadio" value="" checked>
                        Wrong UX/UI Solution
                    </label>
                </div>
            </div>
        </div><!--end card-->
    </div>
</div>
