<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import LucideIcon from '../../../../common/components/LucideIcon.svelte';

</script>

<HeadTitle title="Basic Table" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Basic Table" pagetitle="Tables" />

    <div class="grid grid-cols-1 gap-x-5 xl:grid-cols-2">
        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">Default Table</h6>

                <div class="overflow-x-auto">
                    <table class="w-full">
                        <thead class="ltr:text-left rtl:text-right">
                            <tr>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">Order ID</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">Shop</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">Customer</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">Price</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">Action</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#541254265</a></td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Amezon</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Cleo Carson</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">$4,521</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                            <tr>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#744145235</a></td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Shoppers</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Juston Eichmann</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">$7,546</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                            <tr>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#9855126598</a></td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Flipkart</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Bettie Johson</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">$1,350</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                            <tr>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#847512653</a></td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Tailwick</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Maritza Blanda</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">$4,521</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        </div><!--end card-->
        
        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">Without Borders</h6>
        
                <div class="overflow-x-auto">
                    <table class="w-full">
                        <thead class="ltr:text-left rtl:text-right">
                            <tr>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-transparent">Order ID</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-transparent">Shop</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-transparent">Customer</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-transparent">Price</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-transparent">Action</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td class="px-3.5 py-2.5 border-y border-transparent"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#541254265</a></td>
                                <td class="px-3.5 py-2.5 border-y border-transparent">Amezon</td>
                                <td class="px-3.5 py-2.5 border-y border-transparent">Cleo Carson</td>
                                <td class="px-3.5 py-2.5 border-y border-transparent">$4,521</td>
                                <td class="px-3.5 py-2.5 border-y border-transparent"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                            <tr>
                                <td class="px-3.5 py-2.5 border-y border-transparent"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#744145235</a></td>
                                <td class="px-3.5 py-2.5 border-y border-transparent">Shoppers</td>
                                <td class="px-3.5 py-2.5 border-y border-transparent">Juston Eichmann</td>
                                <td class="px-3.5 py-2.5 border-y border-transparent">$7,546</td>
                                <td class="px-3.5 py-2.5 border-y border-transparent"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                            <tr>
                                <td class="px-3.5 py-2.5 border-y border-transparent"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#9855126598</a></td>
                                <td class="px-3.5 py-2.5 border-y border-transparent">Flipkart</td>
                                <td class="px-3.5 py-2.5 border-y border-transparent">Bettie Johson</td>
                                <td class="px-3.5 py-2.5 border-y border-transparent">$1,350</td>
                                <td class="px-3.5 py-2.5 border-y border-transparent"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                            <tr>
                                <td class="px-3.5 py-2.5 border-y border-transparent"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#847512653</a></td>
                                <td class="px-3.5 py-2.5 border-y border-transparent">Tailwick</td>
                                <td class="px-3.5 py-2.5 border-y border-transparent">Maritza Blanda</td>
                                <td class="px-3.5 py-2.5 border-y border-transparent">$4,521</td>
                                <td class="px-3.5 py-2.5 border-y border-transparent"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        </div><!--end card-->

        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">Striped Even Rows</h6>

                <div class="overflow-x-auto">
                    <table class="w-full">
                        <thead class="ltr:text-left rtl:text-right">
                            <tr>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">Order ID</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">Shop</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">Customer</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">Price</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">Action</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr class="odd:bg-white even:bg-slate-50 dark:odd:bg-zink-700 dark:even:bg-zink-600">
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#541254265</a></td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Amezon</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Cleo Carson</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">$4,521</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                            <tr class="odd:bg-white even:bg-slate-50 dark:odd:bg-zink-700 dark:even:bg-zink-600">
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#744145235</a></td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Shoppers</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Juston Eichmann</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">$7,546</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                            <tr class="odd:bg-white even:bg-slate-50 dark:odd:bg-zink-700 dark:even:bg-zink-600">
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#9855126598</a></td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Flipkart</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Bettie Johson</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">$1,350</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                            <tr class="odd:bg-white even:bg-slate-50 dark:odd:bg-zink-700 dark:even:bg-zink-600">
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#847512653</a></td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Tailwick</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Maritza Blanda</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">$4,521</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        </div><!--end card-->

        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">Striped Odd Rows</h6>
        
                <div class="overflow-x-auto">
                    <table class="w-full">
                        <thead class="ltr:text-left rtl:text-right ">
                            <tr class="">
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">Order ID</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">Shop</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">Customer</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">Price</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">Action</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr class="odd:bg-slate-50 even:bg-white dark:odd:bg-zink-600 dark:even:bg-zink-700">
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#541254265</a></td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Amezon</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Cleo Carson</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">$4,521</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                            <tr class="odd:bg-slate-50 even:bg-white dark:odd:bg-zink-600 dark:even:bg-zink-700">
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#744145235</a></td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Shoppers</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Juston Eichmann</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">$7,546</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                            <tr class="odd:bg-slate-50 even:bg-white dark:odd:bg-zink-600 dark:even:bg-zink-700">
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#9855126598</a></td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Flipkart</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Bettie Johson</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">$1,350</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                            <tr class="odd:bg-slate-50 even:bg-white dark:odd:bg-zink-600 dark:even:bg-zink-700">
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#847512653</a></td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Tailwick</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Maritza Blanda</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">$4,521</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        </div><!--end card-->

        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">Hoverable Rows</h6>
        
                <div class="overflow-x-auto">
                    <table class="w-full">
                        <thead class="ltr:text-left rtl:text-right ">
                            <tr>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">Order ID</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">Shop</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">Customer</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">Price</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">Action</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr class="transition-all duration-300 ease-linear hover:bg-slate-50 dark:hover:bg-zink-600">
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#541254265</a></td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Amezon</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Cleo Carson</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">$4,521</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                            <tr class="transition-all duration-300 ease-linear hover:bg-slate-50 dark:hover:bg-zink-600">
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#744145235</a></td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Shoppers</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Juston Eichmann</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">$7,546</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                            <tr class="transition-all duration-300 ease-linear hover:bg-slate-50 dark:hover:bg-zink-600">
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#9855126598</a></td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Flipkart</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Bettie Johson</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">$1,350</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                            <tr class="transition-all duration-300 ease-linear hover:bg-slate-50 dark:hover:bg-zink-600">
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#847512653</a></td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Tailwick</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Maritza Blanda</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">$4,521</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        </div><!--end card-->

        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">Hoverable Striped Rows</h6>
        
                <div class="overflow-x-auto">
                    <table class="w-full">
                        <thead class="ltr:text-left rtl:text-right ">
                            <tr>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">Order ID</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">Shop</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">Customer</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">Price</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500">Action</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr class="even:bg-slate-50 hover:bg-slate-50 even:hover:bg-slate-100 dark:even:bg-zink-600/50 dark:hover:bg-zink-600 dark:even:hover:bg-zink-600">
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#541254265</a></td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Amezon</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Cleo Carson</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">$4,521</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                            <tr class="even:bg-slate-50 hover:bg-slate-50 even:hover:bg-slate-100 dark:even:bg-zink-600/50 dark:hover:bg-zink-600 dark:even:hover:bg-zink-600">
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#744145235</a></td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Shoppers</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Juston Eichmann</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">$7,546</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                            <tr class="even:bg-slate-50 hover:bg-slate-50 even:hover:bg-slate-100 dark:even:bg-zink-600/50 dark:hover:bg-zink-600 dark:even:hover:bg-zink-600">
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#9855126598</a></td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Flipkart</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Bettie Johson</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">$1,350</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                            <tr class="even:bg-slate-50 hover:bg-slate-50 even:hover:bg-slate-100 dark:even:bg-zink-600/50 dark:hover:bg-zink-600 dark:even:hover:bg-zink-600">
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#847512653</a></td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Tailwick</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">Maritza Blanda</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">$4,521</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        </div><!--end card-->

        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">Border Colors</h6>
        
                <div class="overflow-x-auto">
                    <table class="w-full">
                        <thead class="ltr:text-left rtl:text-right ">
                            <tr>
                                <th class="px-3.5 py-2.5 font-semibold border border-custom-500 dark:border-custom-800">Order ID</th>
                                <th class="px-3.5 py-2.5 font-semibold border border-custom-500 dark:border-custom-800">Shop</th>
                                <th class="px-3.5 py-2.5 font-semibold border border-custom-500 dark:border-custom-800">Customer</th>
                                <th class="px-3.5 py-2.5 font-semibold border border-custom-500 dark:border-custom-800">Price</th>
                                <th class="px-3.5 py-2.5 font-semibold border border-custom-500 dark:border-custom-800">Action</th>
                            </tr>
                        </thead>
                        <tbody class="">
                            <tr>
                                <td class="px-3.5 py-2.5 border border-custom-500 dark:border-custom-800"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#541254265</a></td>
                                <td class="px-3.5 py-2.5 border border-custom-500 dark:border-custom-800">Amezon</td>
                                <td class="px-3.5 py-2.5 border border-custom-500 dark:border-custom-800">Cleo Carson</td>
                                <td class="px-3.5 py-2.5 border border-custom-500 dark:border-custom-800">$4,521</td>
                                <td class="px-3.5 py-2.5 border border-custom-500 dark:border-custom-800"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                            <tr>
                                <td class="px-3.5 py-2.5 border border-custom-500 dark:border-custom-800"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#744145235</a></td>
                                <td class="px-3.5 py-2.5 border border-custom-500 dark:border-custom-800">Shoppers</td>
                                <td class="px-3.5 py-2.5 border border-custom-500 dark:border-custom-800">Juston Eichmann</td>
                                <td class="px-3.5 py-2.5 border border-custom-500 dark:border-custom-800">$7,546</td>
                                <td class="px-3.5 py-2.5 border border-custom-500 dark:border-custom-800"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                            <tr>
                                <td class="px-3.5 py-2.5 border border-custom-500 dark:border-custom-800"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#9855126598</a></td>
                                <td class="px-3.5 py-2.5 border border-custom-500 dark:border-custom-800">Flipkart</td>
                                <td class="px-3.5 py-2.5 border border-custom-500 dark:border-custom-800">Bettie Johson</td>
                                <td class="px-3.5 py-2.5 border border-custom-500 dark:border-custom-800">$1,350</td>
                                <td class="px-3.5 py-2.5 border border-custom-500 dark:border-custom-800"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                            <tr>
                                <td class="px-3.5 py-2.5 border border-custom-500 dark:border-custom-800"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#847512653</a></td>
                                <td class="px-3.5 py-2.5 border border-custom-500 dark:border-custom-800">Tailwick</td>
                                <td class="px-3.5 py-2.5 border border-custom-500 dark:border-custom-800">Maritza Blanda</td>
                                <td class="px-3.5 py-2.5 border border-custom-500 dark:border-custom-800">$4,521</td>
                                <td class="px-3.5 py-2.5 border border-custom-500 dark:border-custom-800"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        </div><!--end card-->

        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">Bordered Table</h6>
                <div class="overflow-x-auto">
                    <table class="w-full">
                        <thead class="ltr:text-left rtl:text-right ">
                            <tr>
                                <th class="px-3.5 py-2.5 font-semibold border border-slate-200 dark:border-zink-500">Order ID</th>
                                <th class="px-3.5 py-2.5 font-semibold border border-slate-200 dark:border-zink-500">Shop</th>
                                <th class="px-3.5 py-2.5 font-semibold border border-slate-200 dark:border-zink-500">Customer</th>
                                <th class="px-3.5 py-2.5 font-semibold border border-slate-200 dark:border-zink-500">Price</th>
                                <th class="px-3.5 py-2.5 font-semibold border border-slate-200 dark:border-zink-500">Action</th>
                            </tr>
                        </thead>
                        <tbody class="">
                            <tr>
                                <td class="px-3.5 py-2.5 border border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#541254265</a></td>
                                <td class="px-3.5 py-2.5 border border-slate-200 dark:border-zink-500">Amezon</td>
                                <td class="px-3.5 py-2.5 border border-slate-200 dark:border-zink-500">Cleo Carson</td>
                                <td class="px-3.5 py-2.5 border border-slate-200 dark:border-zink-500">$4,521</td>
                                <td class="px-3.5 py-2.5 border border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                            <tr>
                                <td class="px-3.5 py-2.5 border border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#744145235</a></td>
                                <td class="px-3.5 py-2.5 border border-slate-200 dark:border-zink-500">Shoppers</td>
                                <td class="px-3.5 py-2.5 border border-slate-200 dark:border-zink-500">Juston Eichmann</td>
                                <td class="px-3.5 py-2.5 border border-slate-200 dark:border-zink-500">$7,546</td>
                                <td class="px-3.5 py-2.5 border border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                            <tr>
                                <td class="px-3.5 py-2.5 border border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#9855126598</a></td>
                                <td class="px-3.5 py-2.5 border border-slate-200 dark:border-zink-500">Flipkart</td>
                                <td class="px-3.5 py-2.5 border border-slate-200 dark:border-zink-500">Bettie Johson</td>
                                <td class="px-3.5 py-2.5 border border-slate-200 dark:border-zink-500">$1,350</td>
                                <td class="px-3.5 py-2.5 border border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                            <tr>
                                <td class="px-3.5 py-2.5 border border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#847512653</a></td>
                                <td class="px-3.5 py-2.5 border border-slate-200 dark:border-zink-500">Tailwick</td>
                                <td class="px-3.5 py-2.5 border border-slate-200 dark:border-zink-500">Maritza Blanda</td>
                                <td class="px-3.5 py-2.5 border border-slate-200 dark:border-zink-500">$4,521</td>
                                <td class="px-3.5 py-2.5 border border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        </div><!--end card-->

        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">Primary Colors Table</h6>
        
                <div class="overflow-x-auto">
                    <table class="w-full bg-custom-50 dark:bg-custom-500/10">
                        <thead class="ltr:text-left rtl:text-right bg-custom-100 dark:bg-custom-500/10">
                            <tr>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-custom-200 dark:border-custom-900">Order ID</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-custom-200 dark:border-custom-900">Shop</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-custom-200 dark:border-custom-900">Customer</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-custom-200 dark:border-custom-900">Price</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-custom-200 dark:border-custom-900">Action</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td class="px-3.5 py-2.5 border-y border-custom-200 dark:border-custom-900"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#541254265</a></td>
                                <td class="px-3.5 py-2.5 border-y border-custom-200 dark:border-custom-900">Amezon</td>
                                <td class="px-3.5 py-2.5 border-y border-custom-200 dark:border-custom-900">Cleo Carson</td>
                                <td class="px-3.5 py-2.5 border-y border-custom-200 dark:border-custom-900">$4,521</td>
                                <td class="px-3.5 py-2.5 border-y border-custom-200 dark:border-custom-900"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                            <tr>
                                <td class="px-3.5 py-2.5 border-y border-custom-200 dark:border-custom-900"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#744145235</a></td>
                                <td class="px-3.5 py-2.5 border-y border-custom-200 dark:border-custom-900">Shoppers</td>
                                <td class="px-3.5 py-2.5 border-y border-custom-200 dark:border-custom-900">Juston Eichmann</td>
                                <td class="px-3.5 py-2.5 border-y border-custom-200 dark:border-custom-900">$7,546</td>
                                <td class="px-3.5 py-2.5 border-y border-custom-200 dark:border-custom-900"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                            <tr>
                                <td class="px-3.5 py-2.5 border-y border-custom-200 dark:border-custom-900"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#9855126598</a></td>
                                <td class="px-3.5 py-2.5 border-y border-custom-200 dark:border-custom-900">Flipkart</td>
                                <td class="px-3.5 py-2.5 border-y border-custom-200 dark:border-custom-900">Bettie Johson</td>
                                <td class="px-3.5 py-2.5 border-y border-custom-200 dark:border-custom-900">$1,350</td>
                                <td class="px-3.5 py-2.5 border-y border-custom-200 dark:border-custom-900"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                            <tr>
                                <td class="px-3.5 py-2.5 border-y border-custom-200 dark:border-custom-900"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#847512653</a></td>
                                <td class="px-3.5 py-2.5 border-y border-custom-200 dark:border-custom-900">Tailwick</td>
                                <td class="px-3.5 py-2.5 border-y border-custom-200 dark:border-custom-900">Maritza Blanda</td>
                                <td class="px-3.5 py-2.5 border-y border-custom-200 dark:border-custom-900">$4,521</td>
                                <td class="px-3.5 py-2.5 border-y border-custom-200 dark:border-custom-900"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        </div><!--end card-->
        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">Dark Table</h6>
        
                <div class="overflow-x-auto">
                    <table class="w-full bg-slate-800 dark:zink-600">
                        <thead class="ltr:text-left rtl:text-right bg-white/5">
                            <tr>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-700 dark:border-zink-500 text-slate-400 dark:text-zink-200">Order ID</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-700 dark:border-zink-500 text-slate-400 dark:text-zink-200">Shop</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-700 dark:border-zink-500 text-slate-400 dark:text-zink-200">Customer</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-700 dark:border-zink-500 text-slate-400 dark:text-zink-200">Price</th>
                                <th class="px-3.5 py-2.5 font-semibold border-b border-slate-700 dark:border-zink-500 text-slate-400 dark:text-zink-200">Action</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td class="px-3.5 py-2.5 border-y border-slate-700 dark:border-zink-500 text-slate-400 dark:text-zink-200"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#541254265</a></td>
                                <td class="px-3.5 py-2.5 border-y border-slate-700 dark:border-zink-500 text-slate-400 dark:text-zink-200">Amezon</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-700 dark:border-zink-500 text-slate-400 dark:text-zink-200">Cleo Carson</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-700 dark:border-zink-500 text-slate-400 dark:text-zink-200">$4,521</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-700 dark:border-zink-500 text-slate-400 dark:text-zink-200"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                            <tr>
                                <td class="px-3.5 py-2.5 border-y border-slate-700 dark:border-zink-500 text-slate-400 dark:text-zink-200"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#744145235</a></td>
                                <td class="px-3.5 py-2.5 border-y border-slate-700 dark:border-zink-500 text-slate-400 dark:text-zink-200">Shoppers</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-700 dark:border-zink-500 text-slate-400 dark:text-zink-200">Juston Eichmann</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-700 dark:border-zink-500 text-slate-400 dark:text-zink-200">$7,546</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-700 dark:border-zink-500 text-slate-400 dark:text-zink-200"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                            <tr>
                                <td class="px-3.5 py-2.5 border-y border-slate-700 dark:border-zink-500 text-slate-400 dark:text-zink-200"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#9855126598</a></td>
                                <td class="px-3.5 py-2.5 border-y border-slate-700 dark:border-zink-500 text-slate-400 dark:text-zink-200">Flipkart</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-700 dark:border-zink-500 text-slate-400 dark:text-zink-200">Bettie Johson</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-700 dark:border-zink-500 text-slate-400 dark:text-zink-200">$1,350</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-700 dark:border-zink-500 text-slate-400 dark:text-zink-200"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                            <tr>
                                <td class="px-3.5 py-2.5 border-y border-slate-700 dark:border-zink-500 text-slate-400 dark:text-zink-200"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#847512653</a></td>
                                <td class="px-3.5 py-2.5 border-y border-slate-700 dark:border-zink-500 text-slate-400 dark:text-zink-200">Tailwick</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-700 dark:border-zink-500 text-slate-400 dark:text-zink-200">Maritza Blanda</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-700 dark:border-zink-500 text-slate-400 dark:text-zink-200">$4,521</td>
                                <td class="px-3.5 py-2.5 border-y border-slate-700 dark:border-zink-500 text-slate-400 dark:text-zink-200"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        </div><!--end card-->

        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">Custom Table</h6>
        
                <div class="overflow-x-auto">
                    <table class="w-full border-separate table-custom border-spacing-y-1">
                        <thead class="ltr:text-left rtl:text-right ">
                            <tr class="relative rounded-md bg-slate-50 after:absolute after:border-l-2 after:left-0 after:top-0 after:bottom-0 after:border-transparent dark:bg-zink-600 [&.active]:after:border-custom-500">
                                <th class="px-3.5 py-2.5 font-semibold">Order ID</th>
                                <th class="px-3.5 py-2.5 font-semibold">Shop</th>
                                <th class="px-3.5 py-2.5 font-semibold">Customer</th>
                                <th class="px-3.5 py-2.5 font-semibold">Price</th>
                                <th class="px-3.5 py-2.5 font-semibold">Action</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr class="relative rounded-md bg-slate-50 after:absolute after:border-l-2 after:left-0 after:top-0 after:bottom-0 after:border-transparent dark:bg-zink-600 [&.active]:after:border-custom-500">
                                <td class="px-3.5 py-2.5"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#541254265</a></td>
                                <td class="px-3.5 py-2.5">Amezon</td>
                                <td class="px-3.5 py-2.5">Cleo Carson</td>
                                <td class="px-3.5 py-2.5">$4,521</td>
                                <td class="px-3.5 py-2.5"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                            <tr class="relative rounded-md bg-slate-50 after:absolute after:border-l-2 after:left-0 after:top-0 after:bottom-0 after:border-transparent dark:bg-zink-600 [&.active]:after:border-custom-500 active">
                                <td class="px-3.5 py-2.5"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#744145235</a></td>
                                <td class="px-3.5 py-2.5">Shoppers</td>
                                <td class="px-3.5 py-2.5">Juston Eichmann</td>
                                <td class="px-3.5 py-2.5">$7,546</td>
                                <td class="px-3.5 py-2.5"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                            <tr class="relative rounded-md bg-slate-50 after:absolute after:border-l-2 after:left-0 after:top-0 after:bottom-0 after:border-transparent dark:bg-zink-600 [&.active]:after:border-custom-500">
                                <td class="px-3.5 py-2.5"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#9855126598</a></td>
                                <td class="px-3.5 py-2.5">Flipkart</td>
                                <td class="px-3.5 py-2.5">Bettie Johson</td>
                                <td class="px-3.5 py-2.5">$1,350</td>
                                <td class="px-3.5 py-2.5"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                            <tr class="relative rounded-md bg-slate-50 after:absolute after:border-l-2 after:left-0 after:top-0 after:bottom-0 after:border-transparent dark:bg-zink-600 [&.active]:after:border-custom-500">
                                <td class="px-3.5 py-2.5"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#847512653</a></td>
                                <td class="px-3.5 py-2.5">Tailwick</td>
                                <td class="px-3.5 py-2.5">Maritza Blanda</td>
                                <td class="px-3.5 py-2.5">$4,521</td>
                                <td class="px-3.5 py-2.5"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        </div><!--end card-->

        <div>
            <div class="card">
                <div class="card-body">
                    <h6 class="mb-4 text-15">Card Table</h6>
            
                    <div class="-mx-5 -mb-5 overflow-x-auto">
                        <table class="w-full whitespace-nowrap">
                            <thead class="ltr:text-left rtl:text-right ">
                                <tr>
                                    <th class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold border-b border-slate-200 dark:border-zink-500">Order ID</th>
                                    <th class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold border-b border-slate-200 dark:border-zink-500">Shop</th>
                                    <th class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold border-b border-slate-200 dark:border-zink-500">Customer</th>
                                    <th class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold border-b border-slate-200 dark:border-zink-500">Price</th>
                                    <th class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold border-b border-slate-200 dark:border-zink-500">Action</th>
                                </tr>
                            </thead>
                            <tbody>
                                <tr>
                                    <td class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#541254265</a></td>
                                    <td class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500">Amezon</td>
                                    <td class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500">Cleo Carson</td>
                                    <td class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500">$4,521</td>
                                    <td class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                                </tr>
                                <tr>
                                    <td class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#744145235</a></td>
                                    <td class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500">Shoppers</td>
                                    <td class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500">Juston Eichmann</td>
                                    <td class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500">$7,546</td>
                                    <td class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                                </tr>
                                <tr>
                                    <td class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#9855126598</a></td>
                                    <td class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500">Flipkart</td>
                                    <td class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500">Bettie Johson</td>
                                    <td class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500">$1,350</td>
                                    <td class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                                </tr>
                                <tr>
                                    <td class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600">#847512653</a></td>
                                    <td class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500">Tailwick</td>
                                    <td class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500">Maritza Blanda</td>
                                    <td class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500">$4,521</td>
                                    <td class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"><a href="#!" class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"><i class="ri-download-2-line"></i></a></td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </div><!--end card-->
        </div>
    </div>
</div>
