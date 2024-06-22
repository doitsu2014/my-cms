<script>
	import Dropdown from '../../common/components/Dropdown.svelte';
	import DropdownMenu from '../../common/components/DropdownMenu.svelte';
	import DropdownToggle from '../../common/components/DropdownToggle.svelte';
	import LucideIcon from '../../common/components/LucideIcon.svelte';
</script>

<div class="card-body">
	<div class="grid items-center grid-cols-1 gap-3 mb-5 2xl:grid-cols-12">
		<div class="2xl:col-span-3">
			<h6 class="text-15">Product Orders</h6>
		</div>
		<!--end col-->
		<div class="2xl:col-span-3 2xl:col-start-10">
			<div class="flex gap-3">
				<div class="relative grow">
					<input
						type="text"
						class="ltr:pl-8 rtl:pr-8 search form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
						placeholder="Search for ..."
						autocomplete="off"
					/>
					<LucideIcon
						name="Search"
						class="inline-block size-4 absolute ltr:left-2.5 rtl:right-2.5 top-2.5 text-slate-500 dark:text-zink-200 fill-slate-100 dark:fill-zink-600"
					/>
				</div>
				<button
					type="button"
					class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
					><i class="align-baseline ltr:pr-1 rtl:pl-1 ri-download-2-line"></i> Export</button
				>
			</div>
		</div>
		<!--end col-->
	</div>
	<!--end grid-->
	<div class="overflow-x-auto">
		<table class="w-full whitespace-nowrap">
			<thead
				class="ltr:text-left rtl:text-right bg-slate-100 text-slate-500 dark:text-zink-200 dark:bg-zink-600"
			>
				<tr>
					<th
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold border-y border-slate-200 dark:border-zink-500"
					>
						#
					</th>
					<th
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold border-y border-slate-200 dark:border-zink-500"
						>Order ID</th
					>
					<th
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold border-y border-slate-200 dark:border-zink-500"
						>Customer Name</th
					>
					<th
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold border-y border-slate-200 dark:border-zink-500"
						>Location</th
					>
					<th
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold border-y border-slate-200 dark:border-zink-500"
						>Order Date</th
					>
					<th
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold border-y border-slate-200 dark:border-zink-500"
						>Payments</th
					>
					<th
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold border-y border-slate-200 dark:border-zink-500"
						>Quantity</th
					>
					<th
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold border-y border-slate-200 dark:border-zink-500"
						>Price</th
					>
					<th
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold border-y border-slate-200 dark:border-zink-500"
						>Total Amount</th
					>
					<th
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold border-y border-slate-200 dark:border-zink-500"
						>Status</th
					>
					<th
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold border-y border-slate-200 dark:border-zink-500"
						>Action</th
					>
				</tr>
			</thead>
			<tbody>
				<tr>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
					>
						01
					</td>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						><a href="/apps/ecommerce/order-overview">#TWT5015100365</a></td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>Marie Prohaska</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>Germany</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>08 Jun, 2023</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>Credit Card</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>05</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>$146.99</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>$749.95</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
					>
						<span
							class="delivery_status px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-green-100 border-green-200 text-green-500 dark:bg-green-500/20 dark:border-green-500/20"
							>Delivered</span
						>
					</td>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
					>
						<Dropdown class="relative" direction="bottom-start">
							<DropdownToggle
								className="flex items-center justify-center  size-[30px] dropdown-toggle p-0 text-slate-500 btn bg-slate-100 hover:text-white hover:bg-slate-600 focus:text-white focus:bg-slate-600 focus:ring focus:ring-slate-100 active:text-white active:bg-slate-600 active:ring active:ring-slate-100 dark:bg-slate-500/20 dark:text-slate-400 dark:hover:bg-slate-500 dark:hover:text-white dark:focus:bg-slate-500 dark:focus:text-white dark:active:bg-slate-500 dark:active:text-white dark:ring-slate-400/20"
							>
								<LucideIcon name="MoreHorizontal" class="size-3" />
							</DropdownToggle>
							<DropdownMenu
								tag="ul"
								class="absolute z-50 py-2 mt-1 ltr:text-left rtl:text-right list-none bg-white rounded-md shadow-md dropdown-menu min-w-[10rem] dark:bg-zink-600"
							>
								<li>
									<a
										class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!"
									>
										<LucideIcon name="Eye" class="inline-block size-3 ltr:mr-1 rtl:ml-1" />
										<span class="align-middle">Overview</span></a
									>
								</li>
								<li>
									<a
										class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!"
									>
										<LucideIcon name="FileEditIcon" class="inline-block size-3 ltr:mr-1 rtl:ml-1" />
										<span class="align-middle">Edit</span></a
									>
								</li>
								<li>
									<a
										class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!"
									>
										<LucideIcon name="Trash2Icon" class="inline-block size-3 ltr:mr-1 rtl:ml-1" />
										<span class="align-middle">Delete</span></a
									>
								</li>
							</DropdownMenu>
						</Dropdown>
					</td>
				</tr>
				<tr>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
					>
						02
					</td>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						><a href="/apps/ecommerce/order-overview">#TWT5015100366</a></td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>Jaqueline Hammes</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>France</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>11 July, 2023</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>Paypal</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>02</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>$450.00</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>$900.00</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
					>
						<span
							class="delivery_status px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-purple-100 border-purple-200 text-purple-500 dark:bg-purple-500/20 dark:border-purple-500/20"
							>Shipping</span
						>
					</td>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
					>
						<Dropdown class="relative" direction="bottom-start">
							<DropdownToggle
								className="flex items-center justify-center  size-[30px] dropdown-toggle p-0 text-slate-500 btn bg-slate-100 hover:text-white hover:bg-slate-600 focus:text-white focus:bg-slate-600 focus:ring focus:ring-slate-100 active:text-white active:bg-slate-600 active:ring active:ring-slate-100 dark:bg-slate-500/20 dark:text-slate-400 dark:hover:bg-slate-500 dark:hover:text-white dark:focus:bg-slate-500 dark:focus:text-white dark:active:bg-slate-500 dark:active:text-white dark:ring-slate-400/20"
							>
								<LucideIcon name="MoreHorizontal" class="size-3" />
							</DropdownToggle>
							<DropdownMenu
								tag="ul"
								class="absolute z-50 py-2 mt-1 ltr:text-left rtl:text-right list-none bg-white rounded-md shadow-md dropdown-menu min-w-[10rem] dark:bg-zink-600"
							>
								<li>
									<a
										class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!"
									>
										<LucideIcon name="Eye" class="inline-block size-3 ltr:mr-1 rtl:ml-1" />
										<span class="align-middle">Overview</span></a
									>
								</li>
								<li>
									<a
										class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!"
									>
										<LucideIcon name="FileEditIcon" class="inline-block size-3 ltr:mr-1 rtl:ml-1" />
										<span class="align-middle">Edit</span></a
									>
								</li>
								<li>
									<a
										class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!"
									>
										<LucideIcon name="Trash2Icon" class="inline-block size-3 ltr:mr-1 rtl:ml-1" />
										<span class="align-middle">Delete</span></a
									>
								</li>
							</DropdownMenu>
						</Dropdown>
					</td>
				</tr>
				<tr>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
					>
						03
					</td>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						><a href="/apps/ecommerce/order-overview">#TWT5015100367</a></td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>Marlene Hirthe</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>Argentina</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>21 Aug, 2023</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>Visa Card</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>03</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>$147.23</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>$294.46</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
					>
						<span
							class="delivery_status px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-sky-100 border-sky-200 text-sky-500 dark:bg-sky-500/20 dark:border-sky-500/20"
							>New</span
						>
					</td>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
					>
						<Dropdown class="relative" direction="bottom-start">
							<DropdownToggle
								className="flex items-center justify-center  size-[30px] dropdown-toggle p-0 text-slate-500 btn bg-slate-100 hover:text-white hover:bg-slate-600 focus:text-white focus:bg-slate-600 focus:ring focus:ring-slate-100 active:text-white active:bg-slate-600 active:ring active:ring-slate-100 dark:bg-slate-500/20 dark:text-slate-400 dark:hover:bg-slate-500 dark:hover:text-white dark:focus:bg-slate-500 dark:focus:text-white dark:active:bg-slate-500 dark:active:text-white dark:ring-slate-400/20"
							>
								<LucideIcon name="MoreHorizontal" class="size-3" />
							</DropdownToggle>
							<DropdownMenu
								tag="ul"
								class="absolute z-50 py-2 mt-1 ltr:text-left rtl:text-right list-none bg-white rounded-md shadow-md dropdown-menu min-w-[10rem] dark:bg-zink-600"
							>
								<li>
									<a
										class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!"
									>
										<LucideIcon name="Eye" class="inline-block size-3 ltr:mr-1 rtl:ml-1" />
										<span class="align-middle">Overview</span></a
									>
								</li>
								<li>
									<a
										class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!"
									>
										<LucideIcon name="FileEditIcon" class="inline-block size-3 ltr:mr-1 rtl:ml-1" />
										<span class="align-middle">Edit</span></a
									>
								</li>
								<li>
									<a
										class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!"
									>
										<LucideIcon name="Trash2Icon" class="inline-block size-3 ltr:mr-1 rtl:ml-1" />
										<span class="align-middle">Delete</span></a
									>
								</li>
							</DropdownMenu>
						</Dropdown>
					</td>
				</tr>
				<tr>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
					>
						04
					</td>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						><a href="/apps/ecommerce/order-overview">#TWT5015100368</a></td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>Reagan Larson</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>Belgium</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>28 Nov, 2023</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>American Express</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>01</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>$579.12</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>$579.12</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
					>
						<span
							class="delivery_status px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-green-100 border-green-200 text-green-500 dark:bg-green-500/20 dark:border-green-500/20"
							>Delivered</span
						>
					</td>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
					>
						<Dropdown class="relative" direction="bottom-start">
							<DropdownToggle
								className="flex items-center justify-center  size-[30px] dropdown-toggle p-0 text-slate-500 btn bg-slate-100 hover:text-white hover:bg-slate-600 focus:text-white focus:bg-slate-600 focus:ring focus:ring-slate-100 active:text-white active:bg-slate-600 active:ring active:ring-slate-100 dark:bg-slate-500/20 dark:text-slate-400 dark:hover:bg-slate-500 dark:hover:text-white dark:focus:bg-slate-500 dark:focus:text-white dark:active:bg-slate-500 dark:active:text-white dark:ring-slate-400/20"
							>
								<LucideIcon name="MoreHorizontal" class="size-3" />
							</DropdownToggle>
							<DropdownMenu
								tag="ul"
								class="absolute z-50 py-2 mt-1 ltr:text-left rtl:text-right list-none bg-white rounded-md shadow-md dropdown-menu min-w-[10rem] dark:bg-zink-600"
							>
								<li>
									<a
										class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!"
									>
										<LucideIcon name="Eye" class="inline-block size-3 ltr:mr-1 rtl:ml-1" />
										<span class="align-middle">Overview</span></a
									>
								</li>
								<li>
									<a
										class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!"
									>
										<LucideIcon name="FileEditIcon" class="inline-block size-3 ltr:mr-1 rtl:ml-1" />
										<span class="align-middle">Edit</span></a
									>
								</li>
								<li>
									<a
										class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!"
									>
										<LucideIcon name="Trash2Icon" class="inline-block size-3 ltr:mr-1 rtl:ml-1" />
										<span class="align-middle">Delete</span></a
									>
								</li>
							</DropdownMenu>
						</Dropdown>
					</td>
				</tr>
				<tr>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
					>
						05
					</td>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						><a href="/apps/ecommerce/order-overview">#TWT5015100369</a></td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>Glennie Langosh</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>Australia</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>11 Oct, 2023</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>American Express</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>01</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>$349.00</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>$349.00</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
					>
						<span
							class="delivery_status px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-yellow-100 border-yellow-200 text-yellow-500 dark:bg-yellow-500/20 dark:border-yellow-500/20"
							>Pending</span
						>
					</td>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
					>
						<Dropdown class="relative" direction="bottom-start">
							<DropdownToggle
								className="flex items-center justify-center  size-[30px] dropdown-toggle p-0 text-slate-500 btn bg-slate-100 hover:text-white hover:bg-slate-600 focus:text-white focus:bg-slate-600 focus:ring focus:ring-slate-100 active:text-white active:bg-slate-600 active:ring active:ring-slate-100 dark:bg-slate-500/20 dark:text-slate-400 dark:hover:bg-slate-500 dark:hover:text-white dark:focus:bg-slate-500 dark:focus:text-white dark:active:bg-slate-500 dark:active:text-white dark:ring-slate-400/20"
							>
								<LucideIcon name="MoreHorizontal" class="size-3" />
							</DropdownToggle>
							<DropdownMenu
								tag="ul"
								class="absolute z-50 py-2 mt-1 ltr:text-left rtl:text-right list-none bg-white rounded-md shadow-md dropdown-menu min-w-[10rem] dark:bg-zink-600"
							>
								<li>
									<a
										class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!"
									>
										<LucideIcon name="Eye" class="inline-block size-3 ltr:mr-1 rtl:ml-1" />
										<span class="align-middle">Overview</span></a
									>
								</li>
								<li>
									<a
										class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!"
									>
										<LucideIcon name="FileEditIcon" class="inline-block size-3 ltr:mr-1 rtl:ml-1" />
										<span class="align-middle">Edit</span></a
									>
								</li>
								<li>
									<a
										class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!"
									>
										<LucideIcon name="Trash2Icon" class="inline-block size-3 ltr:mr-1 rtl:ml-1" />
										<span class="align-middle">Delete</span></a
									>
								</li>
							</DropdownMenu>
						</Dropdown>
					</td>
				</tr>
				<tr>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
					>
						06
					</td>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						><a href="/apps/ecommerce/order-overview">#TWT5015100370</a></td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>Rickie Cremin</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>United States</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>16 Feb, 2023</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>COD</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>03</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>$89.49</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>$268.47</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
					>
						<span
							class="delivery_status px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-green-100 border-green-200 text-green-500 dark:bg-green-500/20 dark:border-green-500/20"
							>Delivered</span
						>
					</td>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
					>
						<Dropdown class="relative" direction="bottom-start">
							<DropdownToggle
								className="flex items-center justify-center  size-[30px] dropdown-toggle p-0 text-slate-500 btn bg-slate-100 hover:text-white hover:bg-slate-600 focus:text-white focus:bg-slate-600 focus:ring focus:ring-slate-100 active:text-white active:bg-slate-600 active:ring active:ring-slate-100 dark:bg-slate-500/20 dark:text-slate-400 dark:hover:bg-slate-500 dark:hover:text-white dark:focus:bg-slate-500 dark:focus:text-white dark:active:bg-slate-500 dark:active:text-white dark:ring-slate-400/20"
							>
								<LucideIcon name="MoreHorizontal" class="size-3" />
							</DropdownToggle>
							<DropdownMenu
								tag="ul"
								class="absolute z-50 py-2 mt-1 ltr:text-left rtl:text-right list-none bg-white rounded-md shadow-md dropdown-menu min-w-[10rem] dark:bg-zink-600"
							>
								<li>
									<a
										class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!"
									>
										<LucideIcon name="Eye" class="inline-block size-3 ltr:mr-1 rtl:ml-1" />
										<span class="align-middle">Overview</span></a
									>
								</li>
								<li>
									<a
										class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!"
									>
										<LucideIcon name="FileEditIcon" class="inline-block size-3 ltr:mr-1 rtl:ml-1" />
										<span class="align-middle">Edit</span></a
									>
								</li>
								<li>
									<a
										class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!"
									>
										<LucideIcon name="Trash2Icon" class="inline-block size-3 ltr:mr-1 rtl:ml-1" />
										<span class="align-middle">Delete</span></a
									>
								</li>
							</DropdownMenu>
						</Dropdown>
					</td>
				</tr>
				<tr>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
					>
						07
					</td>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						><a href="/apps/ecommerce/order-overview">#TWT5015100371</a></td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>Domenic Tromp</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>Afghanistan</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>21 Jan, 2024</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>PayPal</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>02</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>$739.79</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
						>$1,479.58</td
					>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
					>
						<span
							class="delivery_status px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-sky-100 border-sky-200 text-sky-500 dark:bg-sky-500/20 dark:border-sky-500/20"
							>New</span
						>
					</td>
					<td
						class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
					>
						<Dropdown class="relative" direction="bottom-start">
							<DropdownToggle
								className="flex items-center justify-center  size-[30px] dropdown-toggle p-0 text-slate-500 btn bg-slate-100 hover:text-white hover:bg-slate-600 focus:text-white focus:bg-slate-600 focus:ring focus:ring-slate-100 active:text-white active:bg-slate-600 active:ring active:ring-slate-100 dark:bg-slate-500/20 dark:text-slate-400 dark:hover:bg-slate-500 dark:hover:text-white dark:focus:bg-slate-500 dark:focus:text-white dark:active:bg-slate-500 dark:active:text-white dark:ring-slate-400/20"
							>
								<LucideIcon name="MoreHorizontal" class="size-3" />
							</DropdownToggle>
							<DropdownMenu
								tag="ul"
								class="absolute z-50 py-2 mt-1 ltr:text-left rtl:text-right list-none bg-white rounded-md shadow-md dropdown-menu min-w-[10rem] dark:bg-zink-600"
							>
								<li>
									<a
										class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!"
									>
										<LucideIcon name="Eye" class="inline-block size-3 ltr:mr-1 rtl:ml-1" />
										<span class="align-middle">Overview</span></a
									>
								</li>
								<li>
									<a
										class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!"
									>
										<LucideIcon name="FileEditIcon" class="inline-block size-3 ltr:mr-1 rtl:ml-1" />
										<span class="align-middle">Edit</span></a
									>
								</li>
								<li>
									<a
										class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!"
									>
										<LucideIcon name="Trash2Icon" class="inline-block size-3 ltr:mr-1 rtl:ml-1" />
										<span class="align-middle">Delete</span></a
									>
								</li>
							</DropdownMenu>
						</Dropdown>
					</td>
				</tr>
			</tbody>
		</table>
	</div>
	<div class="flex flex-col items-center mt-5 md:flex-row">
		<div class="mb-4 grow md:mb-0">
			<p class="text-slate-500 dark:text-zink-200">Showing <b>07</b> of <b>19</b> Results</p>
		</div>
		<ul class="flex flex-wrap items-center gap-2 shrink-0">
			<li>
				<a
					href="#!"
					class="inline-flex items-center justify-center bg-white dark:bg-zink-700 h-8 px-3 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-500 dark:[&.active]:text-custom-500 [&.active]:bg-custom-50 dark:[&.active]:bg-custom-500/10 [&.active]:border-custom-50 dark:[&.active]:border-custom-500/10 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
					><LucideIcon name="ChevronLeftIcon" class="size-4 mr-1 rtl:rotate-180" /> Prev</a
				>
			</li>
			<li>
				<a
					href="#!"
					class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-500 dark:[&.active]:text-custom-500 [&.active]:bg-custom-50 dark:[&.active]:bg-custom-500/10 [&.active]:border-custom-50 dark:[&.active]:border-custom-500/10 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
					>1</a
				>
			</li>
			<li>
				<a
					href="#!"
					class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-500 dark:[&.active]:text-custom-500 [&.active]:bg-custom-50 dark:[&.active]:bg-custom-500/10 [&.active]:border-custom-50 dark:[&.active]:border-custom-500/10 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto active"
					>2</a
				>
			</li>
			<li>
				<a
					href="#!"
					class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-500 dark:[&.active]:text-custom-500 [&.active]:bg-custom-50 dark:[&.active]:bg-custom-500/10 [&.active]:border-custom-50 dark:[&.active]:border-custom-500/10 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
					>3</a
				>
			</li>
			<li>
				<a
					href="#!"
					class="inline-flex items-center justify-center bg-white dark:bg-zink-700 h-8 px-3 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-500 dark:[&.active]:text-custom-500 [&.active]:bg-custom-50 dark:[&.active]:bg-custom-500/10 [&.active]:border-custom-50 dark:[&.active]:border-custom-500/10 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
					>Next <LucideIcon name="ChevronRightIcon" class="size-4 ml-1 rtl:rotate-180" /></a
				>
			</li>
		</ul>
	</div>
</div>
