<script>
    import HeadTitle from "../../../../../common/components/HeadTitle.svelte";
	import Breadcrumb from "../../../../../common/components/Breadcrumb.svelte";
    import Flatpickr from "svelte-flatpickr";
    import 'flatpickr/dist/flatpickr.css';
	import LucideIcon from "../../../../../common/components/LucideIcon.svelte";
</script>
<HeadTitle title="Create Payslip" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto relative">
    <Breadcrumb title="Create Payslip" pagetitle="Payroll" />

    <div class="grid grid-cols-1 2xl:grid-cols-12">
        <div class="relative card 2xl:col-span-8 2xl:col-start-3">
            <div class="p-8">
                <div class="absolute top-0 ltr:right-0 rtl:left-0 opacity-30">
                    <img src="/assets/images/logo-sm.png" alt="">
                </div>
                <div class="text-center">
                    <h5 class="relative before:absolute before:h-[1px] before:inset-x-0 before:-bottom-2.5 inline-block before:bg-gradient-to-r before:from-white dark:before:from-zink-700 before:via-custom-500 before:to-white dark:before:to-zink-700 dark:before:via-custom-500">Salary Slip</h5>
                </div>
                
                <div class="mt-16 xl:max-w-md">
                    <div class="mb-4">
                        <label for="employeeID" class="inline-block mb-2 text-base font-medium">Employee ID</label>
                        <input type="text" id="employeeID" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" value="#TWE1001524" disabled>
                    </div>
                    <div class="mb-4">
                        <label for="employeeSelect" class="inline-block mb-2 text-base font-medium">Employee Name</label>
                        <select class="form-select border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" name="employeeSelect" id="deliveryStatusSelect">
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
                    <div class="mb-4">
                        <label for="ExperienceInput" class="inline-block mb-2 text-base font-medium">Experience</label>
                        <input type="text" id="ExperienceInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="0.4 year">
                    </div>
                    <div class="mb-4">
                        <label for="createDateInput" class="inline-block mb-2 text-base font-medium">Create Date</label>
                        <Flatpickr type="text" options={{
                            dateFormat:"d M, Y"
                        }} id="fromInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Select date" />
                    </div>
                </div>

                <div class="mt-10 overflow-x-auto">
                    <table class="w-full whitespace-nowrap">
                        <thead class="ltr:text-left rtl:text-right bg-slate-100 dark:bg-zink-600">
                            <tr>
                                <th class="px-3.5 py-2.5 font-semibold border border-transparent">Month</th>
                                <th class="px-3.5 py-2.5 font-semibold border border-transparent">Salary Amount</th>
                                <th class="px-3.5 py-2.5 font-semibold border border-transparent">Deductions(TDS)</th>
                                <th class="px-3.5 py-2.5 font-semibold border border-transparent">Professional Tax</th>
                                <th class="px-3.5 py-2.5 font-semibold border border-transparent">Provident Fund</th>
                                <th class="px-3.5 py-2.5 font-semibold border border-transparent">Net Payable</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td class="px-3.5 py-2.5 border border-transparent">
                                    <Flatpickr type="text" options={{
                                        dateFormat:"d M, Y"
                                    }} id="fromInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Select date" />
                                </td>
                                <td class="px-3.5 py-2.5 border border-transparent">
                                    <input type="number" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="$00.00">
                                </td>
                                <td class="px-3.5 py-2.5 border border-transparent">
                                    <input type="number" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="$00.00">
                                </td>
                                <td class="px-3.5 py-2.5 border border-transparent">
                                    <input type="number" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="$00.00">
                                </td>
                                <td class="px-3.5 py-2.5 border border-transparent">
                                    <input type="number" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="$00.00">
                                </td>
                                <td class="px-3.5 py-2.5 border border-transparent">
                                    <input type="number" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="$00.00">
                                </td>
                            </tr>
                        </tbody>
                        <tbody class="before:block before:h-4">
                            <tr>
                                <td colspan="6">
                                    <button type="button" class="bg-white border-dashed text-custom-500 btn border-custom-500 hover:text-custom-500 hover:bg-custom-50 hover:border-custom-600 focus:text-custom-600 focus:bg-custom-50 focus:border-custom-600 active:text-custom-600 active:bg-custom-50 active:border-custom-600 dark:bg-zink-700 dark:ring-custom-400/20 dark:hover:bg-custom-800/20 dark:focus:bg-custom-800/20 dark:active:bg-custom-800/20"><LucideIcon name="Plus" class="inline-block size-3 mr-1 align-middle"/> <span class="align-middle">Add Item</span></button>
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>
                <div class="grid grid-cols-1 mt-10 2xl:grid-cols-12">
                    <div class="2xl:col-span-5">
                        <p class="mb-2 text-slate-500 dark:text-zink-200">For any inquiries, please contact us at +(252) 0123 456 7890 or reach out via email at tailwick@themesdesign.com.</p>
                        <p class="mb-2 text-slate-500 dark:text-zink-200">Best Regards,</p>
                        <p class="mb-2 text-slate-500 dark:text-zink-200">Themesdesign</p>
                        <p class="text-slate-500 dark:text-zink-200">Founder & CEO: <span class="font-semibold text-slate-800 dark:text-zink-50">Paula Keenan</span></p>
                    </div>
                    <div class="self-end text-center 2xl:col-span-2 2xl:col-start-11">
                        <img src="/assets/images/signature.png" alt="" class="h-12 mx-auto">
                        <h6>Authorized Sign</h6>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>