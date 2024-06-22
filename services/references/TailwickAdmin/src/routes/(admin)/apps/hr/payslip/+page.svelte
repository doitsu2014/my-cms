<script>
    import HeadTitle from "../../../../../common/components/HeadTitle.svelte";
	import Breadcrumb from "../../../../../common/components/Breadcrumb.svelte";
</script>
<HeadTitle title="Payslip" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto relative">
    <Breadcrumb title="Payslip" pagetitle="Payroll" />

    <div class="grid grid-cols-1 2xl:grid-cols-12">
        <div class="relative card 2xl:col-span-8 2xl:col-start-3">
            <div class="p-8">
                <div class="absolute top-0 ltr:right-0 rtl:left-0 opacity-30">
                    <img src="/assets/images/logo-sm.png" alt="">
                </div>
                <div class="text-center">
                    <h5 class="relative before:absolute before:h-[1px] before:inset-x-0 before:-bottom-2.5 inline-block before:bg-gradient-to-r before:from-white dark:before:from-zink-700 before:via-custom-500 before:to-white dark:before:to-zink-700 dark:before:via-custom-500">Salary Slip</h5>
                </div>
                
                <div class="mt-16">
                    <p class="mb-2 text-slate-500 dark:text-zink-200">Employee ID: <span class="font-semibold text-slate-800 dark:text-zink-50">#TWE1001524</span></p>
                    <p class="mb-2 text-slate-500 dark:text-zink-200">Employee Name: <span class="font-semibold text-slate-800 dark:text-zink-50">Patricia Garcia</span></p>
                    <p class="mb-2 text-slate-500 dark:text-zink-200">Experience: <span class="font-semibold text-slate-800 dark:text-zink-50">2 Year</span></p>
                    <p class="text-slate-500 dark:text-zink-200">Create Date: <span class="font-semibold text-slate-800 dark:text-zink-50">08 Oct 2023</span></p>
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
                                <th class="px-3.5 py-2.5 font-semibold border border-transparent">Status</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td class="px-3.5 py-2.5 border border-transparent">Oct, 2023</td>
                                <td class="px-3.5 py-2.5 border border-transparent">$25,000</td>
                                <td class="px-3.5 py-2.5 border border-transparent">$800</td>
                                <td class="px-3.5 py-2.5 border border-transparent">$150</td>
                                <td class="px-3.5 py-2.5 border border-transparent">$500</td>
                                <td class="px-3.5 py-2.5 border border-transparent">$23,550</td>
                                <td class="px-3.5 py-2.5 border border-transparent"><span class="px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-green-100 border-green-200 text-green-500 dark:bg-green-500/20 dark:border-green-500/20">Paid</span></td>
                            </tr>
                            <tr>
                                <td class="px-3.5 py-2.5 border border-transparent">Nov, 2023</td>
                                <td class="px-3.5 py-2.5 border border-transparent">$25,000</td>
                                <td class="px-3.5 py-2.5 border border-transparent">$800</td>
                                <td class="px-3.5 py-2.5 border border-transparent">$150</td>
                                <td class="px-3.5 py-2.5 border border-transparent">$500</td>
                                <td class="px-3.5 py-2.5 border border-transparent">$23,550</td>
                                <td class="px-3.5 py-2.5 border border-transparent"><span class="px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-green-100 border-green-200 text-green-500 dark:bg-green-500/20 dark:border-green-500/20">Paid</span></td>
                            </tr>
                            <tr>
                                <td class="px-3.5 py-2.5 border border-transparent">Dec, 2023</td>
                                <td class="px-3.5 py-2.5 border border-transparent">$25,000</td>
                                <td class="px-3.5 py-2.5 border border-transparent">$800</td>
                                <td class="px-3.5 py-2.5 border border-transparent">$150</td>
                                <td class="px-3.5 py-2.5 border border-transparent">$500</td>
                                <td class="px-3.5 py-2.5 border border-transparent">$23,550</td>
                                <td class="px-3.5 py-2.5 border border-transparent"><span class="px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-green-100 border-green-200 text-green-500 dark:bg-green-500/20 dark:border-green-500/20">Paid</span></td>
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
