<script>
	import HeadTitle from '../../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../../common/components/Breadcrumb.svelte';
	import LucideIcon from '../../../../../common/components/LucideIcon.svelte';
	import Dropdown from '../../../../../common/components/Dropdown.svelte';
	import DropdownToggle from '../../../../../common/components/DropdownToggle.svelte';
	import DropdownMenu from '../../../../../common/components/DropdownMenu.svelte';
	import Modal from '../../../../../common/components/Modal.svelte';
</script>

<HeadTitle title="Order Overview" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto relative">
	<Breadcrumb title="Order Overview" pagetitle="Ecommerce" />

	<div class="grid grid-cols-1 2xl:grid-cols-12 gap-x-5">
		<div class="2xl:col-span-3">
			<div class="card">
				<div class="card-body">
					<div
						class="flex items-center justify-center size-12 bg-purple-100 rounded-md dark:bg-purple-500/20 ltr:float-right rtl:float-left"
					>
						<LucideIcon
							name="Truck"
							class="text-purple-500 fill-purple-200 dark:fill-purple-500/30"
						/>
					</div>
					<h6 class="mb-4 text-15">Shipping Details</h6>

					<h6 class="mb-1">Pauline Hylton</h6>
					<p class="mb-1 text-slate-500 dark:text-zink-200">1235 Crossing Meadows Dr, Onalaska</p>
					<p class="text-slate-500 dark:text-zink-200">West Virginia, USA</p>
				</div>
			</div>
		</div>
		<!--end col-->
		<div class="2xl:col-span-3">
			<div class="card">
				<div class="card-body">
					<div
						class="flex items-center justify-center size-12 bg-orange-100 rounded-md dark:bg-orange-500/20 ltr:float-right rtl:float-left"
					>
						<LucideIcon
							name="CreditCard"
							class="text-orange-500 fill-orange-200 dark:fill-orange-500/30"
						/>
					</div>
					<h6 class="mb-4 text-15">Billing Details</h6>

					<h6 class="mb-1">Pauline Hylton</h6>
					<p class="mb-1 text-slate-500 dark:text-zink-200">1235 Crossing Meadows Dr, Onalaska</p>
					<p class="text-slate-500 dark:text-zink-200">West Virginia, USA</p>
				</div>
			</div>
		</div>
		<!--end col-->
		<div class="2xl:col-span-3">
			<div class="card">
				<div class="card-body">
					<div
						class="flex items-center justify-center size-12 rounded-md ltr:float-right rtl:float-left bg-sky-100 dark:bg-sky-500/20"
					>
						<LucideIcon
							name="CircleDollarSign"
							class="text-sky-500 fill-sky-200 dark:fill-sky-500/30"
						/>
					</div>
					<h6 class="mb-4 text-15">Payment Details</h6>

					<h6 class="mb-1">ID: #TSD456DF41SD5</h6>
					<p class="mb-1 text-slate-500 dark:text-zink-200">Payment Method: <b>Credit Card</b></p>
					<p class="mb-1 text-slate-500 dark:text-zink-200">
						Card Number: <b>xxxx xxxx xxxx 1501</b>
					</p>
				</div>
			</div>
		</div>
		<!--end col-->
		<div class="2xl:col-span-3">
			<div class="card">
				<div class="card-body">
					<div
						class="size-12 bg-yellow-100 rounded-md ltr:float-right rtl:float-left dark:bg-yellow-500/20"
					>
						<img src="/assets/images/users/avatar-4.png" alt="" class="h-12 rounded-md" />
					</div>
					<h6 class="mb-4 text-15">Customer Info</h6>

					<h6 class="mb-1">Pauline Hylton</h6>
					<p class="mb-1 text-slate-500 dark:text-zink-200">pauline@tailwick.com</p>
					<p class="text-slate-500 dark:text-zink-200">+(78) 120 4843 4714</p>
				</div>
			</div>
		</div>
		<!--end col-->
	</div>

	<div class="grid grid-cols-1 2xl:grid-cols-12 gap-x-5">
		<div class="2xl:col-span-9">
			<div class="grid grid-cols-1 2xl:grid-cols-5 gap-x-5">
				<div>
					<div class="card">
						<div class="text-center card-body">
							<h6 class="mb-1 underline">#TWT5015100366</h6>
							<p class="uppercase text-slate-500 dark:text-zink-200">Order ID</p>
						</div>
					</div>
				</div>
				<!--end col-->
				<div>
					<div class="card">
						<div class="text-center card-body">
							<h6 class="mb-1">02 Nov, 2023</h6>
							<p class="uppercase text-slate-500 dark:text-zink-200">Order Date</p>
						</div>
					</div>
				</div>
				<!--end col-->
				<div>
					<div class="card">
						<div class="text-center card-body">
							<h6 class="mb-1">09 Nov, 2023</h6>
							<p class="uppercase text-slate-500 dark:text-zink-200">Delivery Date</p>
						</div>
					</div>
				</div>
				<!--end col-->
				<div>
					<div class="card">
						<div class="text-center card-body">
							<h6 class="mb-1">$843.49</h6>
							<p class="uppercase text-slate-500 dark:text-zink-200">Order Amount</p>
						</div>
					</div>
				</div>
				<!--end col-->
				<div>
					<div class="card">
						<div class="text-center card-body">
							<span
								class="delivery_status px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-purple-100 border-purple-200 text-purple-500 dark:bg-purple-500/20 dark:border-purple-500/20"
								>Shipping</span
							>
							<p class="uppercase text-slate-500 dark:text-zink-200">Order Status</p>
						</div>
					</div>
				</div>
				<!--end col-->
			</div>
			<!--end grid-->
			<div class="card">
				<div class="card-body">
					<div class="flex items-center gap-3 mb-4">
						<h6 class="text-15 grow">Order Summary</h6>
						<a href="#!" class="text-red-500 underline shrink-0">Cancelled Order</a>
					</div>
					<div class="overflow-x-auto">
						<table class="w-full">
							<tbody>
								<tr>
									<td
										class="px-3.5 py-4 border-b border-dashed first:pl-0 last:pr-0 border-slate-200 dark:border-zink-500"
									>
										<div class="flex items-center gap-3">
											<div
												class="flex items-center justify-center size-12 rounded-md bg-slate-100 shrink-0"
											>
												<img src="/assets/images/product/img-08.png" alt="" class="h-8" />
											</div>
											<div class="grow">
												<h6 class="mb-1 text-15">
													<a
														href="/apps/ecommerce/products-overview"
														class="transition-all duration-300 ease-linear hover:text-custom-500"
														>Roar Twill Blue Baseball Cap</a
													>
												</h6>
												<p class="text-slate-500 dark:text-zink-200">$149.99 x 02</p>
											</div>
										</div>
									</td>
									<td
										class="px-3.5 py-4 border-b border-dashed first:pl-0 last:pr-0 border-slate-200 dark:border-zink-500 ltr:text-right rtl:text-left"
										>$299.98</td
									>
								</tr>
								<tr>
									<td
										class="px-3.5 py-4 border-b border-dashed first:pl-0 last:pr-0 border-slate-200 dark:border-zink-500"
									>
										<div class="flex items-center gap-3">
											<div
												class="flex items-center justify-center size-12 rounded-md bg-slate-100 shrink-0"
											>
												<img src="/assets/images/product/img-04.png" alt="" class="h-8" />
											</div>
											<div class="grow">
												<h6 class="mb-1 text-15">
													<a
														href="/apps/ecommerce/products-overview"
														class="transition-all duration-300 ease-linear hover:text-custom-500"
														>Mesh Ergonomic Green Chair</a
													>
												</h6>
												<p class="text-slate-500 dark:text-zink-200">$362.21 x 1</p>
											</div>
										</div>
									</td>
									<td
										class="px-3.5 py-4 border-b border-dashed first:pl-0 last:pr-0 border-slate-200 dark:border-zink-500 ltr:text-right rtl:text-left"
										>$362.21</td
									>
								</tr>
								<tr>
									<td
										class="px-3.5 py-4 border-b border-dashed first:pl-0 last:pr-0 border-slate-200 dark:border-zink-500"
									>
										<div class="flex items-center gap-3">
											<div
												class="flex items-center justify-center size-12 rounded-md bg-slate-100 shrink-0"
											>
												<img src="/assets/images/product/img-01.png" alt="" class="h-8" />
											</div>
											<div class="grow">
												<h6 class="mb-1 text-15">
													<a
														href="/apps/ecommerce/products-overview"
														class="transition-all duration-300 ease-linear hover:text-custom-500"
														>Smartest Printed T-shirt</a
													>
												</h6>
												<p class="text-slate-500 dark:text-zink-200">$89.99 x 03</p>
											</div>
										</div>
									</td>
									<td
										class="px-3.5 py-4 border-b border-dashed first:pl-0 last:pr-0 border-slate-200 dark:border-zink-500 ltr:text-right rtl:text-left"
										>$269.97</td
									>
								</tr>
								<tr>
									<td
										class="px-3.5 pt-4 pb-3 first:pl-0 last:pr-0 text-slate-500 dark:text-zink-200"
									>
										Sub Total
									</td>
									<td class="px-3.5 pt-4 pb-3 first:pl-0 last:pr-0 ltr:text-right rtl:text-left"
										>$932.16</td
									>
								</tr>
								<tr>
									<td class="px-3.5 py-3 first:pl-0 last:pr-0 text-slate-500 dark:text-zink-200">
										Estimated Tax (18%)
									</td>
									<td class="px-3.5 py-3 first:pl-0 last:pr-0 ltr:text-right rtl:text-left"
										>$167.79</td
									>
								</tr>
								<tr>
									<td class="px-3.5 py-3 first:pl-0 last:pr-0 text-slate-500 dark:text-zink-200">
										Item Discounts (12%)
									</td>
									<td class="px-3.5 py-3 first:pl-0 last:pr-0 ltr:text-right rtl:text-left"
										>-$111.86</td
									>
								</tr>
								<tr>
									<td class="px-3.5 py-3 first:pl-0 last:pr-0 text-slate-500 dark:text-zink-200">
										Shipping Charge
									</td>
									<td class="px-3.5 py-3 first:pl-0 last:pr-0 ltr:text-right rtl:text-left">$0</td>
								</tr>
								<tr class="font-semibold">
									<td class="px-3.5 pt-3 first:pl-0 last:pr-0 text-slate-500 dark:text-zink-200">
										Total Amount (USD)
									</td>
									<td class="px-3.5 pt-3 first:pl-0 last:pr-0 ltr:text-right rtl:text-left"
										>$988.09</td
									>
								</tr>
							</tbody>
						</table>
					</div>
				</div>
			</div>
			<!--end card-->
			<div class="card">
				<div class="card-body">
					<div class="flex items-center gap-3 mb-4">
						<h6 class="text-15 grow">Order Status</h6>
						<div class="shrink-0">
							<a href="#!" class="inline-block text-red-500 underline ltr:mr-2 rtl:ml-2"
								>Cancelled Order</a
							>
							<a
								href="/apps/invoice/overview"
								class="py-1 text-xs text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
								><LucideIcon
									name="Download"
									class="inline-block size-4 align-middle ltr:mr-1 rtl:ml-1"
								/> <span class="align-middle">Invoice</span></a
							>
						</div>
					</div>
					<div>
						<div
							class="relative ltr:pl-6 rtl:pr-6 before:absolute ltr:before:border-l rtl:before:border-r ltr:before:left-[0.1875rem] rtl:before:right-[0.1875rem] before:border-slate-200 [&.done]:before:border-custom-500 before:top-1.5 before:-bottom-1.5 after:absolute after:size-2 after:bg-white after:border after:border-slate-200 after:rounded-full ltr:after:left-0 rtl:after:right-0 after:top-1.5 pb-4 last:before:hidden [&.done]:after:bg-custom-500 [&.done]:after:border-custom-500 done"
						>
							<div class="flex gap-4">
								<div class="grow">
									<h6 class="mb-2 text-gray-800 text-15 dark:text-zink-50">Order Placed</h6>
									<p class="text-gray-400 dark:text-zink-200">
										Your order has been successfully submitted.
									</p>
								</div>
								<p class="text-sm text-gray-400 dark:text-zink-200 shrink-0">02 Nov, 2023</p>
							</div>
						</div>
						<div
							class="relative ltr:pl-6 rtl:pr-6 before:absolute ltr:before:border-l rtl:before:border-r ltr:before:left-[0.1875rem] rtl:before:right-[0.1875rem] before:border-slate-200 [&.done]:before:border-custom-500 before:top-1.5 before:-bottom-1.5 after:absolute after:size-2 after:bg-white after:border after:border-slate-200 after:rounded-full ltr:after:left-0 rtl:after:right-0 after:top-1.5 pb-4 last:before:hidden [&.done]:after:bg-custom-500 [&.done]:after:border-custom-500 done"
						>
							<div class="flex gap-4">
								<div class="grow">
									<h6 class="mb-2 text-gray-800 text-15 dark:text-zink-50">Order Processing</h6>
									<p class="text-gray-400 dark:text-zink-200">
										Once the order is received, it goes through the processing stage. During this
										time, the items are gathered, and the order is prepared for shipment.
									</p>
								</div>
								<p class="text-sm text-gray-400 dark:text-zink-200 shrink-0">02 Nov, 2023</p>
							</div>
						</div>
						<div
							class="relative ltr:pl-6 rtl:pr-6 before:absolute ltr:before:border-l rtl:before:border-r ltr:before:left-[0.1875rem] rtl:before:right-[0.1875rem] before:border-slate-200 [&.done]:before:border-custom-500 before:top-1.5 before:-bottom-1.5 after:absolute after:size-2 after:bg-white after:border after:border-slate-200 after:rounded-full ltr:after:left-0 rtl:after:right-0 after:top-1.5 pb-4 last:before:hidden [&.done]:after:bg-custom-500 [&.done]:after:border-custom-500 done"
						>
							<div class="flex gap-4">
								<div class="grow">
									<h6 class="mb-2 text-gray-800 text-15 dark:text-zink-50">Shipped Order</h6>
									<p class="text-gray-400 dark:text-zink-200">
										The order is shipped out to the customer's designated delivery address.
									</p>
								</div>
								<p class="text-sm text-gray-400 dark:text-zink-200 shrink-0">04 Nov, 2023</p>
							</div>
						</div>
						<div
							class="relative ltr:pl-6 rtl:pr-6 before:absolute ltr:before:border-l rtl:before:border-r ltr:before:left-[0.1875rem] rtl:before:right-[0.1875rem] before:border-slate-200 [&.done]:before:border-custom-500 before:top-1.5 before:-bottom-1.5 after:absolute after:size-2 after:bg-white after:border after:border-slate-200 after:rounded-full ltr:after:left-0 rtl:after:right-0 after:top-1.5 pb-4 last:before:hidden [&.done]:after:bg-custom-500 [&.done]:after:border-custom-500 done"
						>
							<div class="flex gap-4">
								<div class="grow">
									<h6 class="mb-2 text-gray-800 text-15 dark:text-zink-50">Out for Delivery</h6>
									<p class="text-gray-400 dark:text-zink-200">
										This status indicates that the order is currently out for delivery by the
										shipping or courier company.
									</p>
								</div>
								<p class="text-sm text-gray-400 dark:text-zink-200 shrink-0">06 Nov, 2023</p>
							</div>
						</div>
						<div
							class="relative ltr:pl-6 rtl:pr-6 before:absolute ltr:before:border-l rtl:before:border-r ltr:before:left-[0.1875rem] rtl:before:right-[0.1875rem] before:border-slate-200 [&.done]:before:border-custom-500 before:top-1.5 before:-bottom-1.5 after:absolute after:size-2 after:bg-white after:border after:border-slate-200 after:rounded-full ltr:after:left-0 rtl:after:right-0 after:top-1.5 pb-4 last:before:hidden [&.done]:after:bg-custom-500 [&.done]:after:border-custom-500 done"
						>
							<div class="flex gap-4">
								<div class="grow">
									<h6 class="mb-2 text-gray-800 text-15 dark:text-zink-50">Delivered</h6>
									<p class="text-gray-400 dark:text-zink-200">
										Finally, when the order successfully reaches the customer's address and is
										handed over, the status changes to "Delivered."
									</p>
								</div>
								<p class="text-sm text-gray-400 dark:text-zink-200 shrink-0">09 Nov, 2023</p>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
		<!--end col-->
		<div class="2xl:col-span-3">
			<div class="card">
				<div class="card-body">
					<h6 class="mb-3 text-gray-800 text-15 dark:text-white">Documents</h6>
					<div class="overflow-x-auto">
						<table class="w-full">
							<tbody>
								<tr>
									<td class="px-3.5 first:pl-0 last:pr-0 py-2 border-y border-transparent"
										>Invoice No.</td
									>
									<td class="px-3.5 first:pl-0 last:pr-0 py-2 border-y border-transparent"
										><a href="/apps/invoice/overview" class="text-custom-500">#TWI154203</a></td
									>
								</tr>
								<tr>
									<td class="px-3.5 first:pl-0 last:pr-0 py-2 border-y border-transparent"
										>Shipping No</td
									>
									<td class="px-3.5 first:pl-0 last:pr-0 py-2 border-y border-transparent"
										><a href="#!" class="text-custom-500">#TWS987102301</a></td
									>
								</tr>
							</tbody>
						</table>
					</div>
				</div>
			</div>
			<div class="card">
				<div class="card-body">
					<div class="flex items-center gap-3 mb-4">
						<h6 class="text-15 grow">Logistics Details</h6>
						<a href="#!" class="underline text-custom-500 shrink-0">Track Order</a>
					</div>
					<div class="flex gap-4">
						<div class="shrink-0">
							<img src="/assets/images/brand/delivery-1.png" alt="" class="h-10" />
						</div>
						<div class="grow">
							<h6 class="mb-1 text-15">Express Delivery</h6>
							<p class="text-slate-500 dark:text-zink-200">ID: EDTW1400457854</p>
						</div>
					</div>
				</div>
			</div>
		</div>
		<!--end col-->
	</div>
</div>
