<script>
    import HeadTitle from "../../../../../common/components/HeadTitle.svelte";
	import Breadcrumb from "../../../../../common/components/Breadcrumb.svelte";
	import LucideIcon from "../../../../../common/components/LucideIcon.svelte";
</script>
<HeadTitle title="Marketplace" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto relative">
    <Breadcrumb title="Add New" pagetitle="Invoices" />

    <div class="grid items-center grid-cols-1 gap-5 mb-5 xl:grid-cols-12">
        <div class="xl:col-span-2">
            <h5 class="text-16">New Invoice</h5>
        </div><!--end col-->
        <div class="xl:col-span-3 xl:col-start-10">
            <div class="flex justify-end gap-2">
                <button type="button" class="text-slate-500 btn bg-slate-200 border-slate-200 hover:text-slate-600 hover:bg-slate-300 hover:border-slate-300 focus:text-slate-600 focus:bg-slate-300 focus:border-slate-300 focus:ring focus:ring-slate-100 active:text-slate-600 active:bg-slate-300 active:border-slate-300 active:ring active:ring-slate-100 dark:bg-zink-600 dark:hover:bg-zink-500 dark:border-zink-600 dark:hover:border-zink-500 dark:text-zink-200 dark:ring-zink-400/50"><LucideIcon name="Eye" class="inline-block size-4 mr-1"/> <span class="align-middle">Preview</span></button>
                <button type="button" class="text-white bg-purple-500 border-purple-500 btn hover:text-white hover:bg-purple-600 hover:border-purple-600 focus:text-white focus:bg-purple-600 focus:border-purple-600 focus:ring focus:ring-purple-100 active:text-white active:bg-purple-600 active:border-purple-600 active:ring active:ring-purple-100 dark:ring-purple-400/10">Save Draft</button>
                <button type="button" class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"><LucideIcon name="Save" class="inline-block size-4 mr-1"/> <span class="align-middle">Save & Download</span></button>
            </div>
        </div>
    </div><!--end grid-->

    <div class="card">
        <div class="card-body">
            <form action="#!">
                <h6 class="mb-4 text-gray-800 underline text-16 dark:text-zink-50">Generale Info:</h6>
                <div class="grid grid-cols-1 gap-5 xl:grid-cols-12">
                    <div class="xl:col-span-3">
                        <label for="invoiceID" class="inline-block mb-2 text-base font-medium">Invoice No.</label>
                        <input type="text" id="invoiceID" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Enter invoice no." value="#TW15090251" disabled required>
                    </div><!--end col-->
                    <div class="xl:col-span-3">
                        <label for="invoiceNo" class="inline-block mb-2 text-base font-medium">Invoice Date</label>
                        <input type="text" id="invoiceNo" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Invoice Date" data-provider="flatpickr" data-date-format="d M, Y" readonly="readonly" required>
                    </div><!--end col-->
                    <div class="xl:col-span-3">
                        <label for="invoiceDue" class="inline-block mb-2 text-base font-medium">Invoice Due</label>
                        <input type="text" id="invoiceDue" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Invoice Due" data-provider="flatpickr" data-date-format="d M, Y" readonly="readonly" required>
                    </div><!--end col-->
                    <div class="xl:col-span-3">
                        <label for="legalRegistrationNo" class="inline-block mb-2 text-base font-medium">Legal Registration No.</label>
                        <input type="number" id="legalRegistrationNo" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Legal Registration No" required>
                    </div><!--end col-->
                    <div class="xl:col-span-3">
                        <label for="emailInvoiceInput" class="inline-block mb-2 text-base font-medium">Email</label>
                        <input type="email" id="emailInvoiceInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="tailwick@themesdesign.com" required>
                    </div><!--end col-->
                    <div class="xl:col-span-3">
                        <label for="websiteInput" class="inline-block mb-2 text-base font-medium">Website</label>
                        <input type="text" id="websiteInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="www.themesdesign.in" required>
                    </div><!--end col-->
                    <div class="xl:col-span-3">
                        <label for="contactInput" class="inline-block mb-2 text-base font-medium">Contact US</label>
                        <input type="number" id="contactInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="(241) 1234 567 8900" required>
                    </div><!--end col-->
                    <div class="xl:col-span-3">
                        <label for="paymentStatus" class="inline-block mb-2 text-base font-medium">Payment Status</label>
                        <select class="form-select border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" id="paymentStatus">
                            <option selected>Select Status</option>
                            <option value="Paid">Paid</option>
                            <option value="Unpaid">Unpaid</option>
                            <option value="Cancel">Cancel</option>
                            <option value="Refund">Refund</option>
                        </select>
                    </div><!--end col-->
                </div><!--end grid-->

                <h6 class="my-5 underline text-16">Shipping Info:</h6>
                <div class="grid grid-cols-1 gap-5 xl:grid-cols-12 changeAddress">
                    <div class="xl:col-span-3">
                        <label for="fullNameShippingInput" class="inline-block mb-2 text-base font-medium">Full Name</label>
                        <input type="text" id="fullNameShippingInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Full Name" required>
                    </div><!--end col-->
                    <div class="xl:col-span-3">
                        <label for="phoneNoShippingInput" class="inline-block mb-2 text-base font-medium">Phone No.</label>
                        <input type="number" id="phoneNoShippingInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="(00) 1234 567 8956" required>
                    </div><!--end col-->
                    <div class="xl:col-span-3">
                        <label for="alternativeNoShippingInput" class="inline-block mb-2 text-base font-medium">Alternative No.</label>
                        <input type="number" id="alternativeNoShippingInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="(00) 1234 567 8956" required>
                    </div><!--end col-->
                    <div class="xl:col-span-3">
                        <label for="taxShippingInput" class="inline-block mb-2 text-base font-medium">TAX No.</label>
                        <input type="number" id="taxShippingInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="12-34567974" required>
                    </div><!--end col-->
                    <div class="xl:col-span-12">
                        <label for="addressShippingInput" class="inline-block mb-2 text-base font-medium">Address</label>
                        <textarea class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Address" id="addressShippingInput" rows="3"></textarea>
                    </div><!--end col-->
                </div><!--end grid-->

                <div class="flex items-center mt-5">
                    <div class="relative inline-block w-10 mr-2 align-middle transition duration-200 ease-in">
                        <input type="checkbox" name="customSoftSwitch" id="customSoftSwitch" class="absolute block size-5 transition duration-300 ease-linear border-2 rounded-full appearance-none cursor-pointer border-slate-200 dark:border-zink-600 bg-white/80 dark:bg-zink-400 peer/published checked:bg-custom-500 dark:checked:bg-custom-500 ltr:checked:right-0 rtl:checked:left-0 checked:border-custom-100 dark:checked:border-custom-900 arrow-none checked:bg-none" >
                        <label for="customSoftSwitch" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-full cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-custom-100 dark:peer-checked/published:bg-custom-900 peer-checked/published:border-custom-100 dark:peer-checked/published:border-custom-900"></label>
                    </div>
                    <label for="customSoftSwitch" class="inline-block text-base font-medium">Will your Shipping & Billing address same?</label>
                </div>

                <h6 class="my-5 underline text-16">Billing Info:</h6>
                <div class="grid grid-cols-1 gap-5 xl:grid-cols-12">
                    <div class="xl:col-span-3">
                        <label for="fullNameBillingInput" class="inline-block mb-2 text-base font-medium">Full Name</label>
                        <input type="text" id="fullNameBillingInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Full Name" required>
                    </div><!--end col-->
                    <div class="xl:col-span-3">
                        <label for="phoneNoBillingInput" class="inline-block mb-2 text-base font-medium">Phone No.</label>
                        <input type="text" id="phoneNoBillingInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="(00) 1234 567 8956" required>
                    </div><!--end col-->
                    <div class="xl:col-span-3">
                        <label for="alternativeNoBillingInput" class="inline-block mb-2 text-base font-medium">Alternative No.</label>
                        <input type="text" id="alternativeNoBillingInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="(00) 1234 567 8956" required>
                    </div><!--end col-->
                    <div class="xl:col-span-3">
                        <label for="taxBillingInput" class="inline-block mb-2 text-base font-medium">TAX No.</label>
                        <input type="text" id="taxBillingInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="12-34567974" required>
                    </div><!--end col-->
                    <div class="xl:col-span-12">
                        <label for="addressBillingInput" class="inline-block mb-2 text-base font-medium">Address</label>
                        <textarea class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Address" id="addressBillingInput" rows="3"></textarea>
                    </div><!--end col-->
                </div><!--end grid-->

                <h6 class="my-5 underline text-16">Products Info:</h6>

                <div class="overflow-x-auto">
                    <table class="w-full whitespace-nowrap" >
                        <thead>
                            <tr>
                                <th class="px-3.5 py-2.5 font-medium text-sm text-slate-500 uppercase border border-slate-200 dark:text-zink-200 dark:border-zink-500">Item Name</th>
                                <th class="px-3.5 py-2.5 font-medium text-sm text-slate-500 uppercase border border-slate-200 dark:text-zink-200 dark:border-zink-500">Quantity</th>
                                <th class="px-3.5 py-2.5 font-medium text-sm text-slate-500 uppercase border border-slate-200 dark:text-zink-200 dark:border-zink-500">Price</th>
                                <th class="px-3.5 py-2.5 font-medium text-sm text-slate-500 uppercase border border-slate-200 dark:text-zink-200 dark:border-zink-500">Discount</th>
                                <th class="px-3.5 py-2.5 font-medium text-sm text-slate-500 uppercase border border-slate-200 dark:text-zink-200 dark:border-zink-500">TAX</th>
                                <th class="px-3.5 py-2.5 font-medium text-sm text-slate-500 uppercase border border-slate-200 dark:text-zink-200 dark:border-zink-500 w-44">Total</th>
                            </tr>
                        </thead>
                        <tbody class="before:block before:h-3 item-list">
                            <tr class="item">
                                <td class="border border-slate-200 dark:border-zink-500">
                                    <input type="text" id="itemName1" class="px-3.5 py-2.5 border-none form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Item Name" required>
                                </td>
                                <td class="w-40 border border-slate-200 dark:border-zink-500">
                                    <div class="flex justify-center text-center input-step">
                                        <button type="button" class="border size-9 leading-[15px] btn-minus bg-white dark:bg-zink-700 dark:border-zink-500 ltr:rounded-l rtl:rounded-r transition-all duration-200 ease-linear border-slate-200 text-slate-500 dark:text-zink-200 hover:bg-custom-500 dark:hover:bg-custom-500 hover:text-custom-50 dark:hover:text-custom-50 hover:border-custom-500 dark:hover:border-custom-500 focus:bg-custom-500 dark:focus:bg-custom-500 focus:border-custom-500 dark:focus:border-custom-500 focus:text-custom-50 dark:focus:text-custom-50"><LucideIcon name="Minus" class="inline-block size-4"/></button>
                                        <input type="number" class="w-12 text-center ltr:pl-2 rtl:pr-2 h-9 border-y product-quantity dark:bg-zink-700 focus:shadow-none dark:border-zink-500 item-quantity" value="0" min="0" max="100" readonly="">
                                        <button type="button" class="transition-all duration-200 ease-linear bg-white border dark:bg-zink-700 dark:border-zink-500 ltr:rounded-r rtl:rounded-l size-9 border-slate-200 btn-plus text-slate-500 dark:text-zink-200 hover:bg-custom-500 dark:hover:bg-custom-500 hover:text-custom-50 dark:hover:text-custom-50 hover:border-custom-500 dark:hover:border-custom-500 focus:bg-custom-500 dark:focus:bg-custom-500 focus:border-custom-500 dark:focus:border-custom-500 focus:text-custom-50 dark:focus:text-custom-50"><LucideIcon name="Plus" class="inline-block size-4"/></button>
                                    </div>
                                </td>
                                <td class="w-40 border border-slate-200 dark:border-zink-500">
                                    <input type="number" id="itemName1" class="px-3.5 py-2.5 border-none form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200 item-price" placeholder="$00.00" required>
                                </td>
                                <td class="w-40 border border-slate-200 dark:border-zink-500">
                                    <input type="text" id="itemName1" class="px-3.5 py-2.5 border-none form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200 item-discount" placeholder="0%" required>
                                </td>
                                <td class="w-40 border border-slate-200 dark:border-zink-500">
                                    <input type="text" id="itemName1" class="px-3.5 py-2.5 border-none form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200 item-text" placeholder="0%" required>
                                </td>
                                <td class="border border-slate-200 dark:border-zink-500" rowspan="2">
                                    <div class="mb-1">
                                        <input type="text" id="ItemTotal" class="px-3.5 py-2.5 border-none form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200 item-line-price" placeholder="$00.00" readonly>
                                    </div>
                                    <input type="text" id="itemDiscountsInput" class="px-3.5 pb-2.5 pt-0 border-none form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200 cart-discount" placeholder="-$00.00" readonly>
                                </td>
                            </tr>
                            <tr>
                                <td class="border border-slate-200 dark:border-zink-500">
                                    <input type="text" id="itemdescription1" class="px-3.5 py-2.5 border-none form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Enter Description (Optional)" required>
                                </td>
                                <td class="border border-slate-200 px-3.5 py-1.5 text-center dark:border-zink-500">
                                    <button type="button" class="px-2 py-1.5 text-xs text-red-500 bg-red-100 btn hover:text-white hover:bg-red-600 focus:text-white focus:bg-red-600 focus:ring focus:ring-red-100 active:text-white active:bg-red-600 active:ring active:ring-red-100 dark:bg-red-500/20 dark:text-red-500 dark:hover:bg-red-500 dark:hover:text-white dark:focus:bg-red-500 dark:focus:text-white dark:active:bg-red-500 dark:active:text-white dark:ring-red-400/20 product-removal"><LucideIcon name="Trash2" class="inline-block size-3 mr-1 align-middle"/> <span class="align-middle">Delete</span></button>
                                </td>
                                <td colspan="3" class="border border-slate-200 dark:border-zink-500"></td>
                            </tr>
                        </tbody>
                        <tbody class="before:block before:h-3 item-list">
                            <tr class="item">
                                <td class="border border-slate-200 dark:border-zink-500">
                                    <input type="text" id="itemName1" class="px-3.5 py-2.5 border-none form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Item Name" required>
                                </td>
                                <td class="w-40 border border-slate-200 dark:border-zink-500">
                                    <div class="flex justify-center text-center input-step">
                                        <button type="button" class="border size-9 leading-[15px] btn-minus bg-white dark:bg-zink-700 dark:border-zink-500 ltr:rounded-l rtl:rounded-r transition-all duration-200 ease-linear border-slate-200 text-slate-500 dark:text-zink-200 hover:bg-custom-500 dark:hover:bg-custom-500 hover:text-custom-50 dark:hover:text-custom-50 hover:border-custom-500 dark:hover:border-custom-500 focus:bg-custom-500 dark:focus:bg-custom-500 focus:border-custom-500 dark:focus:border-custom-500 focus:text-custom-50 dark:focus:text-custom-50"><LucideIcon name="Minus" class="inline-block size-4"/></button>
                                        <input type="number" class="w-12 text-center ltr:pl-2 rtl:pr-2 h-9 border-y product-quantity dark:bg-zink-700 focus:shadow-none dark:border-zink-500 item-quantity" value="0" min="0" max="100" readonly="">
                                        <button type="button" class="transition-all duration-200 ease-linear bg-white border dark:bg-zink-700 dark:border-zink-500 ltr:rounded-r rtl:rounded-l size-9 border-slate-200 btn-plus text-slate-500 dark:text-zink-200 hover:bg-custom-500 dark:hover:bg-custom-500 hover:text-custom-50 dark:hover:text-custom-50 hover:border-custom-500 dark:hover:border-custom-500 focus:bg-custom-500 dark:focus:bg-custom-500 focus:border-custom-500 dark:focus:border-custom-500 focus:text-custom-50 dark:focus:text-custom-50"><LucideIcon name="Plus" class="inline-block size-4"/></button>
                                    </div>
                                </td>
                                <td class="w-40 border border-slate-200 dark:border-zink-500">
                                    <input type="number" id="itemName1" class="px-3.5 py-2.5 border-none form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200 item-price" placeholder="$00.00" required>
                                </td>
                                <td class="w-40 border border-slate-200 dark:border-zink-500">
                                    <input type="text" id="itemName1" class="px-3.5 py-2.5 border-none form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200 item-discount" placeholder="0%" required>
                                </td>
                                <td class="w-40 border border-slate-200 dark:border-zink-500">
                                    <input type="text" id="itemName1" class="px-3.5 py-2.5 border-none form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200 item-text" placeholder="0%" required>
                                </td>
                                <td class="border border-slate-200 dark:border-zink-500" rowspan="2">
                                    <div class="mb-1">
                                        <input type="text" id="ItemTotal" class="px-3.5 py-2.5 border-none form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200 item-line-price" placeholder="$00.00" readonly>
                                    </div>
                                    <input type="text" id="itemDiscountsInput" class="px-3.5 pb-2.5 pt-0 border-none form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200 cart-discount" placeholder="-$00.00" readonly>
                                </td>
                            </tr>
                            <tr>
                                <td class="border border-slate-200 dark:border-zink-500">
                                    <input type="text" id="itemdescription1" class="px-3.5 py-2.5 border-none form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Enter Description (Optional)" required>
                                </td>
                                <td class="border border-slate-200 px-3.5 py-1.5 text-center dark:border-zink-500">
                                    <button type="button" class="px-2 py-1.5 text-xs text-red-500 bg-red-100 btn hover:text-white hover:bg-red-600 focus:text-white focus:bg-red-600 focus:ring focus:ring-red-100 active:text-white active:bg-red-600 active:ring active:ring-red-100 dark:bg-red-500/20 dark:text-red-500 dark:hover:bg-red-500 dark:hover:text-white dark:focus:bg-red-500 dark:focus:text-white dark:active:bg-red-500 dark:active:text-white dark:ring-red-400/20 product-removal"><LucideIcon name="Trash2" class="inline-block size-3 mr-1 align-middle"/> <span class="align-middle">Delete</span></button>
                                </td>
                                <td colspan="3" class="border border-slate-200 dark:border-zink-500"></td>
                            </tr>
                        </tbody>
                  
                        <tbody class="before:block before:h-4" id="invoiceTable">
                            <tr>
                                <td colspan="6">
                                    <a href={"javascript:void(0)"} id="addBtn"><button type="button" class="bg-white border-dashed text-custom-500 btn border-custom-500 hover:text-custom-500 hover:bg-custom-50 hover:border-custom-600 focus:text-custom-600 focus:bg-custom-50 focus:border-custom-600 active:text-custom-600 active:bg-custom-50 active:border-custom-600 dark:bg-zink-700 dark:ring-custom-400/20 dark:hover:bg-custom-800/20 dark:focus:bg-custom-800/20 dark:active:bg-custom-800/20"><LucideIcon name="Plus" class="inline-block size-3 mr-1 align-middle"/> <span class="align-middle">Add Item</span></button></a>
                                </td>
                            </tr>
                        </tbody>
                        <tbody class="before:block before:h-3" id="totalAmount">
                            <tr>
                                <td colspan="4"></td>
                                <td class="border-b border-slate-200 px-3.5 py-2.5 text-slate-500 dark:text-zink-200 dark:border-zink-500">
                                    Sub Total
                                </td>
                                <td class="font-medium border-b border-slate-200 dark:border-zink-500">
                                    <input type="text" id="subTotale" class="px-3.5 py-2.5 border-none form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200 cart-subtotal" placeholder="$00.00" readonly>
                                </td>
                            </tr>
                            <tr>
                                <td colspan="4"></td>
                                <td class="border-b border-slate-200 px-3.5 py-2.5 text-slate-500 dark:text-zink-200 dark:border-zink-500">
                                    Estimated Tax (18%)
                                </td>
                                <td class="font-medium border-b border-slate-200 dark:border-zink-500">
                                    <input type="text" id="estimatedTax" class="px-3.5 py-2.5 border-none form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200 cart-tax" placeholder="$00.00" readonly>
                                </td>
                            </tr>
                            <tr>
                                <td colspan="4"></td>
                                <td class="border-b border-slate-200 px-3.5 py-2.5 text-slate-500 dark:text-zink-200 dark:border-zink-500">
                                    Item Discounts
                                </td>
                                <td class="font-medium border-b border-slate-200 dark:border-zink-500 text-slate-500">
                                    <input type="text" id="itemDiscounts" class="px-3.5 py-2.5 border-none form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200 cart-discount" placeholder="-$00.00" readonly>
                                </td>
                            </tr>
                            <tr>
                                <td colspan="4"></td>
                                <td class="border-b border-slate-200 px-3.5 py-2.5 text-slate-500 dark:text-zink-200 dark:border-zink-500">
                                    Shipping Charge
                                </td>
                                <td class="font-medium border-b border-slate-200 dark:border-zink-500">
                                    <input type="text" id="shippingCharge" class="px-3.5 py-2.5 border-none form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200 cart-shipping" placeholder="$00.00" readonly>
                                </td>
                            </tr>
                            <tr>
                                <td colspan="4"></td>
                                <td class="border-b border-slate-200 px-3.5 py-2.5 text-slate-500 dark:text-zink-200 dark:border-zink-500">
                                    Total Amount
                                </td>
                                <td class="font-medium border-b border-slate-200 dark:border-zink-500">
                                    <input type="text" id="totalAmount" class="px-3.5 py-2.5 border-none form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200 cart-total" placeholder="$00.00" readonly>
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </div>

                <h6 class="my-5 underline text-16">Payments Details:</h6>
                <div class="grid grid-cols-1 gap-5 xl:grid-cols-12">
                    <div class="xl:col-span-3">
                        <label for="paymentMethod" class="inline-block mb-2 text-base font-medium">Full Name</label>
                        <select class="form-select border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" name="paymentMethod" id="paymentMethod">
                            <option value="">Select Method</option>
                            <option value="Credit Card">Credit Card</option>
                            <option value="Paypal">Paypal</option>
                            <option value="Paypal">Paypal</option>
                            <option value="American Express">American Express</option>
                        </select>
                    </div><!--end col-->
                    <div class="xl:col-span-3">
                        <label for="cardHolderName" class="inline-block mb-2 text-base font-medium">Card Holder Name</label>
                        <input type="number" id="cardHolderName" inputmode="numeric" pattern="[0-9\s]{13,19}" autocomplete="cc-number" maxlength="19" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Full Name" required>
                    </div><!--end col-->
                    <div class="xl:col-span-3">
                        <label for="cardNumber" class="inline-block mb-2 text-base font-medium">Card Number</label>
                        <input type="text" id="cardNumber" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="xxxx xxxx xxxx xxxx" required>
                    </div><!--end col-->
                    <div class="xl:col-span-3">
                        <label for="totalPayment" class="inline-block mb-2 text-base font-medium ">Total Payment</label>
                        <input type="text" id="totalPayment" class=" tform-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200otalPaymentinput" placeholder="$00.00" required>
                    </div><!--end col-->
                    <div class="xl:col-span-12">
                        <label for="taxBillingInput" class="inline-block mb-2 text-base font-medium">Notes</label>
                        <textarea class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Enter notes pertaining to the customer or payment" id="taxBillingInput" rows="3"></textarea>
                    </div><!--end col-->
                </div><!--end grid-->

                <div class="flex justify-end gap-2 mt-5">
                    <button type="button" class="text-slate-500 btn bg-slate-200 border-slate-200 hover:text-slate-600 hover:bg-slate-300 hover:border-slate-300 focus:text-slate-600 focus:bg-slate-300 focus:border-slate-300 focus:ring focus:ring-slate-100 active:text-slate-600 active:bg-slate-300 active:border-slate-300 active:ring active:ring-slate-100 dark:bg-zink-600 dark:hover:bg-zink-500 dark:border-zink-600 dark:hover:border-zink-500 dark:text-zink-200 dark:ring-zink-400/50"><LucideIcon name="RefreshCcw" class="inline-block size-4 mr-1"/> <span class="align-middle">Reset</span></button>
                    <button type="button" class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"><LucideIcon name="Save" class="inline-block size-4 mr-1"/> <span class="align-middle">Save</span></button>
                    <button type="button" class="text-white bg-green-500 border-green-500 btn hover:text-white hover:bg-green-600 hover:border-green-600 focus:text-white focus:bg-green-600 focus:border-green-600 focus:ring focus:ring-green-100 active:text-white active:bg-green-600 active:border-green-600 active:ring active:ring-green-100 dark:ring-green-400/10"><LucideIcon name="Download" class="inline-block size-4 mr-1"/> <span class="align-middle">Download</span></button>
                </div>
            </form>
        </div>
    </div>
</div>
