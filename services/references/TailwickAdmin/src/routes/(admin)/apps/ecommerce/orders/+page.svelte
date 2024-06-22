<script>
	import HeadTitle from '../../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../../common/components/Breadcrumb.svelte';
	import LucideIcon from '../../../../../common/components/LucideIcon.svelte';
	import OrdersOverviewChart from './OrdersOverviewChart.svelte';
	import data from '../../../../../common/data/orders';
	import Dropdown from '../../../../../common/components/Dropdown.svelte';
	import DropdownToggle from '../../../../../common/components/DropdownToggle.svelte';
	import DropdownMenu from '../../../../../common/components/DropdownMenu.svelte';
	import Modal from '../../../../../common/components/Modal.svelte';
	import CounterPage from '../../../../../common/components/CounterPage.svelte';

	let isAddModal = false;
	const toggleAddModal = () => (isAddModal = !isAddModal);

	let isDeleteModal = false;
	const toggleDelete = () => (isDeleteModal = !isDeleteModal);
</script>

<HeadTitle title="Order Lists" />
<CounterPage />
<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto relative">
	<Breadcrumb title="Order Lists" pagetitle="Ecommerce" />

	<div class="grid grid-cols-1 gap-x-5 md:grid-cols-2 2xl:grid-cols-12">
		<div class="2xl:col-span-2 2xl:row-span-1">
			<div class="card">
				<div class="flex items-center gap-3 card-body">
					<div
						class="flex items-center justify-center size-12 rounded-md text-15 bg-custom-50 text-custom-500 dark:bg-custom-500/20 shrink-0"
					>
						<LucideIcon name="Boxes" />
					</div>
					<div class="grow">
						<h5 class="mb-1 text-16 counter-value" data-target="15101">0</h5>
						<p class="text-slate-500 dark:text-zink-200">Total Orders</p>
					</div>
				</div>
			</div>
		</div>
		<!--end col-->
		<div class="2xl:col-span-2 2xl:row-span-1">
			<div class="card">
				<div class="flex items-center gap-3 card-body">
					<div
						class="flex items-center justify-center size-12 rounded-md text-15 bg-sky-50 text-sky-500 dark:bg-sky-500/20 shrink-0"
					>
						<LucideIcon name="PackagePlus" />
					</div>
					<div class="grow">
						<!-- svelte-ignore a11y-missing-content -->
						<h5 class="mb-1 text-16 counter-value" data-target="3874"></h5>
						<p class="text-slate-500 dark:text-zink-200">New Orders</p>
					</div>
				</div>
			</div>
		</div>
		<!--end col-->
		<div class="order-last md:col-span-2 2xl:col-span-8 2xl:row-span-3 card 2xl:order-none">
			<div class="card-body">
				<h6 class="mb-4 text-gray-800 text-15 dark:text-zink-50">Orders Overview</h6>
				<OrdersOverviewChart chartColors={['bg-custom-500']} />
			</div>
		</div>
		<div class="2xl:col-span-2 2xl:row-span-1">
			<div class="card">
				<div class="flex items-center gap-3 card-body">
					<div
						class="flex items-center justify-center size-12 text-yellow-500 rounded-md text-15 bg-yellow-50 dark:bg-yellow-500/20 shrink-0"
					>
						<LucideIcon name="Loader" />
					</div>
					<div class="grow">
						<h5 class="mb-1 text-16 counter-value" data-target="1548">0</h5>
						<p class="text-slate-500 dark:text-zink-200">Pending Orders</p>
					</div>
				</div>
			</div>
		</div>
		<!--end col-->
		<div class="2xl:col-span-2 2xl:row-span-1">
			<div class="card">
				<div class="flex items-center gap-3 card-body">
					<div
						class="flex items-center justify-center size-12 text-purple-500 rounded-md text-15 bg-purple-50 dark:bg-purple-500/20 shrink-0"
					>
						<LucideIcon name="Truck" />
					</div>
					<div class="grow">
						<h5 class="mb-1 text-16 counter-value" data-target="9543">0</h5>
						<p class="text-slate-500 dark:text-zink-200">Shipping Orders</p>
					</div>
				</div>
			</div>
		</div>
		<!--end col-->
		<div class="2xl:col-span-2 2xl:row-span-1">
			<div class="card">
				<div class="flex items-center gap-3 card-body">
					<div
						class="flex items-center justify-center size-12 text-green-500 rounded-md text-15 bg-green-50 dark:bg-green-500/20 shrink-0"
					>
						<LucideIcon name="PackageCheck" />
					</div>
					<div class="grow">
						<h5 class="mb-1 text-16 counter-value" data-target="30914">0</h5>
						<p class="text-slate-500 dark:text-zink-200">Delivered Orders</p>
					</div>
				</div>
			</div>
		</div>
		<!--end col-->
		<div class="2xl:col-span-2 2xl:row-span-1">
			<div class="card">
				<div class="flex items-center gap-3 card-body">
					<div
						class="flex items-center justify-center size-12 text-red-500 rounded-md text-15 bg-red-50 dark:bg-red-500/20 shrink-0"
					>
						<LucideIcon name="PackageX" />
					</div>
					<div class="grow">
						<h5 class="mb-1 text-16 counter-value" data-target="3863">0</h5>
						<p class="text-slate-500 dark:text-zink-200">Cancelled Orders</p>
					</div>
				</div>
			</div>
		</div>
		<!--end col-->
	</div>

	<div class="card" id="ordersTable">
		<div class="card-body">
			<div class="grid grid-cols-1 gap-4 lg:grid-cols-12">
				<div class="lg:col-span-3">
					<div class="relative">
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
				</div>
				<!--end col-->
				<div class="lg:col-span-2 lg:col-start-11">
					<div class="ltr:lg:text-right rtl:lg:text-left">
						<a
							href="#!"
							type="button"
							class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
							on:click={toggleAddModal}
							><LucideIcon name="Plus" class="inline-block size-4" />
							<span class="align-middle">Add Order</span></a
						>
					</div>
				</div>
			</div>
			<!--col grid-->

			<ul class="flex flex-wrap w-full mt-5 text-sm font-medium text-center text-gray-500 nav-tabs">
				<li class="group active">
					<a
						href={'javascript:void(0);'}
						class="inline-block px-4 py-1.5 text-base transition-all duration-300 ease-linear rounded-md text-slate-500 dark:text-zink-200 border border-transparent group-[.active]:bg-custom-500 group-[.active]:text-white dark:group-[.active]:text-white hover:text-custom-500 dark:hover:text-custom-500 active:text-custom-500 dark:active:text-custom-500 -mb-[1px]"
						><LucideIcon name="Boxes" class="inline-block size-4 ltr:mr-1 rtl:ml-1" />
						<span class="align-middle">All Orders</span></a
					>
				</li>
				<li class="group">
					<a
						href={'javascript:void(0);'}
						class="inline-block px-4 py-1.5 text-base transition-all duration-300 ease-linear rounded-md text-slate-500 dark:text-zink-200 border border-transparent group-[.active]:bg-custom-500 group-[.active]:text-white dark:group-[.active]:text-white hover:text-custom-500 dark:hover:text-custom-500 active:text-custom-500 dark:active:text-custom-500 -mb-[1px]"
						><LucideIcon name="Loader" class="inline-block size-4 ltr:mr-1 rtl:ml-1" />
						<span class="align-middle">Pending</span></a
					>
				</li>
				<li class="group">
					<a
						href={'javascript:void(0);'}
						class="inline-block px-4 py-1.5 text-base transition-all duration-300 ease-linear rounded-md text-slate-500 dark:text-zink-200 border border-transparent group-[.active]:bg-custom-500 group-[.active]:text-white dark:group-[.active]:text-white hover:text-custom-500 dark:hover:text-custom-500 active:text-custom-500 dark:active:text-custom-500 -mb-[1px]"
						><LucideIcon name="PackageCheck" class="inline-block size-4 ltr:mr-1 rtl:ml-1" />
						<span class="align-middle">Delivered</span></a
					>
				</li>
				<li class="group">
					<a
						href={'javascript:void(0);'}
						class="inline-block px-4 py-1.5 text-base transition-all duration-300 ease-linear rounded-md text-slate-500 dark:text-zink-200 border border-transparent group-[.active]:bg-custom-500 group-[.active]:text-white dark:group-[.active]:text-white hover:text-custom-500 dark:hover:text-custom-500 active:text-custom-500 dark:active:text-custom-500 -mb-[1px]"
						><LucideIcon name="RefreshCcw" class="inline-block size-4 ltr:mr-1 rtl:ml-1" />
						<span class="align-middle">Returns</span></a
					>
				</li>
				<li class="group">
					<a
						href={'javascript:void(0);'}
						class="inline-block px-4 py-1.5 text-base transition-all duration-300 ease-linear rounded-md text-slate-500 dark:text-zink-200 border border-transparent group-[.active]:bg-custom-500 group-[.active]:text-white dark:group-[.active]:text-white hover:text-custom-500 dark:hover:text-custom-500 active:text-custom-500 dark:active:text-custom-500 -mb-[1px]"
						><LucideIcon name="PackageX" class="inline-block size-4 ltr:mr-1 rtl:ml-1 " />
						<span class="align-middle">Cancelled</span></a
					>
				</li>
			</ul>

			<div class="mt-5 overflow-x-auto">
				<table class="w-full whitespace-nowrap">
					<thead class="ltr:text-left rtl:text-right bg-slate-100 dark:bg-zink-600">
						<tr>
							<th
								class="px-3.5 py-2.5 font-semibold text-slate-500 border-b border-slate-200 dark:border-zink-500 dark:text-zink-200"
							>
								<div class="flex items-center h-full">
									<input
										id="CheckboxAll"
										class="size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800"
										type="checkbox"
									/>
								</div>
							</th>
							<th
								class="px-3.5 py-2.5 font-semibold text-slate-500 border-b border-slate-200 dark:border-zink-500 dark:text-zink-200 sort"
								data-sort="order_id">Order ID</th
							>
							<th
								class="px-3.5 py-2.5 font-semibold text-slate-500 border-b border-slate-200 dark:border-zink-500 dark:text-zink-200 sort"
								data-sort="order_date">Order Date</th
							>
							<th
								class="px-3.5 py-2.5 font-semibold text-slate-500 border-b border-slate-200 dark:border-zink-500 dark:text-zink-200 sort"
								data-sort="delivery_date">Delivery Date</th
							>
							<th
								class="px-3.5 py-2.5 font-semibold text-slate-500 border-b border-slate-200 dark:border-zink-500 dark:text-zink-200 sort"
								data-sort="customer_name">Customer Name</th
							>
							<th
								class="px-3.5 py-2.5 font-semibold text-slate-500 border-b border-slate-200 dark:border-zink-500 dark:text-zink-200 sort"
								data-sort="payment_method">Payment Method</th
							>
							<th
								class="px-3.5 py-2.5 font-semibold text-slate-500 border-b border-slate-200 dark:border-zink-500 dark:text-zink-200 sort"
								data-sort="amount">Amount</th
							>
							<th
								class="px-3.5 py-2.5 font-semibold text-slate-500 border-b border-slate-200 dark:border-zink-500 dark:text-zink-200 sort"
								data-sort="delivery_status">Delivery Status</th
							>
							<th
								class="px-3.5 py-2.5 font-semibold text-slate-500 border-b border-slate-200 dark:border-zink-500 dark:text-zink-200"
								>Action</th
							>
						</tr>
					</thead>
					<tbody>
						{#each data.orderlist as order}
							<tr>
								<td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">
									<div class="flex items-center h-full">
										<input
											id="Checkbox1"
											class="size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800"
											type="checkbox"
										/>
									</div>
								</td>
								<td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"
									><a
										href="#!"
										class="transition-all duration-150 ease-linear order_id text-custom-500 hover:text-custom-600"
										>{order.ID}</a
									></td
								>
								<td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500 order_date"
									>{order.OrderDate}</td
								>
								<td
									class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500 delivery_date"
									>{order.DeliveryDate}</td
								>
								<td
									class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500 customer_name"
									>{order.CustomerName}</td
								>
								<td
									class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500 payment_method"
									>{order.PaymentMethod}</td
								>
								<td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500 amount"
									>{order.Amount}</td
								>
								<td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">
									{#if order.DeliveryStatus == 'Delivered'}
										<span
											class="delivery_status px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-green-100 border-green-200 text-green-500 dark:bg-green-500/20 dark:border-green-500/20"
											>Delivered</span
										>
									{:else if order.DeliveryStatus == 'Pending'}
										<span
											class="delivery_status px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-yellow-100 border-yellow-200 text-yellow-500 dark:bg-yellow-500/20 dark:border-yellow-500/20"
											>Pending</span
										>
									{:else if order.DeliveryStatus == 'Shipping'}
										<span
											class="delivery_status px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-purple-100 border-purple-200 text-purple-500 dark:bg-purple-500/20 dark:border-purple-500/20"
											>Shipping</span
										>
									{:else if order.DeliveryStatus == 'New'}
										<span
											class="delivery_status px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-sky-100 border-sky-200 text-sky-500 dark:bg-sky-500/20 dark:border-sky-500/20"
											>New</span
										>
									{:else if order.DeliveryStatus == 'Return'}
										<span
											class="delivery_status px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-slate-100 border-slate-200 text-slate-500 dark:bg-slate-500/20 dark:border-slate-500/20 dark:text-zink-200"
											>Return</span
										>
									{:else if order.DeliveryStatus == 'Cancelled'}
										<span
											class="delivery_status px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-red-100 border-red-200 text-red-500 dark:bg-red-500/20 dark:border-red-500/20"
											>Cancelled</span
										>
									{/if}
								</td>
								<td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">
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
													href="/apps/ecommerce/order-overview"
													><LucideIcon name="Eye" class="inline-block size-3 ltr:mr-1 rtl:ml-1" />
													<span class="align-middle">Overview</span></a
												>
											</li>
											<li>
												<a
													class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
													href="#!"
													><LucideIcon
														name="FileEdit"
														class="inline-block size-3 ltr:mr-1 rtl:ml-1"
													/> <span class="align-middle">Edit</span></a
												>
											</li>
											<li>
												<a
													class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
													href="#!"
													on:click={toggleDelete}
													><LucideIcon
														name="Trash2"
														class="inline-block size-3 ltr:mr-1 rtl:ml-1"
													/> <span class="align-middle">Delete</span></a
												>
											</li>
										</DropdownMenu>
									</Dropdown>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
				<div class="noresult" style="display: none">
					<div class="py-6 text-center">
						<LucideIcon
							name="search"
							class="size-6 mx-auto text-sky-500 fill-sky-100 dark:sky-500/20"
						/>
						<h5 class="mt-2 mb-1">Sorry! No Result Found</h5>
						<p class="mb-0 text-slate-500 dark:text-zink-200">
							We've searched more than 299+ orders We did not find any orders for you search.
						</p>
					</div>
				</div>
			</div>

			<div class="flex flex-col items-center mt-5 md:flex-row">
				<div class="mb-4 grow md:mb-0">
					<p class="text-slate-500 dark:text-zink-200">Showing <b>10</b> of <b>18</b> Results</p>
				</div>
				<ul class="flex flex-wrap items-center gap-2 shrink-0">
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 h-8 px-3 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-500 dark:[&.active]:text-custom-500 [&.active]:bg-custom-50 dark:[&.active]:bg-custom-500/10 [&.active]:border-custom-50 dark:[&.active]:border-custom-500/10 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
							><LucideIcon class="size-4 mr-1 rtl:rotate-180" name="ChevronLeft" /> Prev</a
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
							>Next <LucideIcon class="size-4 ml-1 rtl:rotate-180" name="ChevronRight" /></a
						>
					</li>
				</ul>
			</div>
		</div>
	</div>
</div>

<Modal modal-center className="-translate-y-2/4" isOpen={isAddModal} toggle={toggleAddModal}>
	<div class="w-screen md:w-[30rem] bg-white shadow rounded-md dark:bg-zink-600">
		<div class="flex items-center justify-between p-4 border-b dark:border-zink-500">
			<h5 class="text-16">Add Order</h5>
			<button
				class="transition-all duration-200 ease-linear text-slate-400 hover:text-red-500"
				on:click={toggleAddModal}><LucideIcon name="X" class="size-5" /></button
			>
		</div>
		<div class="max-h-[calc(theme('height.screen')_-_180px)] p-4 overflow-y-auto">
			<form action="#!">
				<div class="grid grid-cols-1 gap-4 xl:grid-cols-12">
					<div class="xl:col-span-12">
						<label for="orderId" class="inline-block mb-2 text-base font-medium">Order ID</label>
						<input
							type="text"
							id="orderId"
							class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
							disabled
							value="#TWT5015100365"
							required
						/>
					</div>
					<div class="xl:col-span-6">
						<label for="orderDateInput" class="inline-block mb-2 text-base font-medium"
							>Order Date</label
						>
						<input
							type="text"
							id="orderDateInput"
							class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
							placeholder="Select date"
							data-provider="flatpickr"
							data-date-format="d M, Y"
						/>
					</div>
					<div class="xl:col-span-6">
						<label for="deliveryDateInput" class="inline-block mb-2 text-base font-medium"
							>Delivery Date</label
						>
						<input
							type="text"
							id="deliveryDateInput"
							class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
							placeholder="Select date"
							data-provider="flatpickr"
							data-date-format="d M, Y"
						/>
					</div>
					<div class="xl:col-span-12">
						<label for="customerNameInput" class="inline-block mb-2 text-base font-medium"
							>Customer Name</label
						>
						<input
							type="text"
							id="customerNameInput"
							class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
							placeholder="Customer name"
							required
						/>
					</div>
					<div class="xl:col-span-12">
						<label for="paymentMethodSelect" class="inline-block mb-2 text-base font-medium"
							>Payment Method</label
						>
						<select
							class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
							data-choices
							data-choices-search-false
							name="paymentMethodSelect"
							id="paymentMethodSelect"
						>
							<option value="Credit Card">Credit Card</option>
							<option value="Paypal">Paypal</option>
							<option value="Visa Card">Visa Card</option>
							<option value="COD">COD</option>
							<option value="American Express">American Express</option>
						</select>
					</div>
					<div class="xl:col-span-12">
						<label for="amountInput" class="inline-block mb-2 text-base font-medium">Amount</label>
						<input
							type="text"
							id="amountInput"
							class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
							placeholder="12345 67890"
							required
						/>
					</div>
					<div class="xl:col-span-12">
						<label for="deliveryStatusSelect" class="inline-block mb-2 text-base font-medium"
							>Delivery Status</label
						>
						<select
							class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
							name="deliveryStatusSelect"
							id="deliveryStatusSelect"
						>
							<option value="Delivered">Delivered</option>
							<option value="Shipping">Shipping</option>
							<option value="New">New</option>
							<option value="Pending">Pending</option>
							<option value="Return">Return</option>
							<option value="Cancelled">Cancelled</option>
						</select>
					</div>
				</div>
				<div class="flex justify-end gap-2 mt-4">
					<button
						type="reset"
						class="text-red-500 bg-white btn hover:text-red-500 hover:bg-red-100 focus:text-red-500 focus:bg-red-100 active:text-red-500 active:bg-red-100 dark:bg-zink-600 dark:hover:bg-red-500/10 dark:focus:bg-red-500/10 dark:active:bg-red-500/10"
						on:click={toggleAddModal}>Cancel</button
					>
					<button
						type="submit"
						class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
						>Add Order</button
					>
				</div>
			</form>
		</div>
	</div>
</Modal>

<Modal modal-center className="-translate-y-2/4" isOpen={isDeleteModal} toggle={toggleDelete}>
	<div class="w-screen md:w-[25rem] bg-white shadow rounded-md dark:bg-zink-600">
		<div class="max-h-[calc(theme('height.screen')_-_180px)] overflow-y-auto px-6 py-8">
			<div class="float-right">
				<button
					data-modal-close="deleteModal"
					class="transition-all duration-200 ease-linear text-slate-500 hover:text-red-500"
					on:click={toggleDelete}><LucideIcon name="X" class="size-5" /></button
				>
			</div>
			<img src="/assets/images/delete.png" alt="" class="block h-12 mx-auto" />
			<div class="mt-5 text-center">
				<h5 class="mb-1">Are you sure?</h5>
				<p class="text-slate-500 dark:text-zink-200">
					Are you certain you want to delete this record?
				</p>
				<div class="flex justify-center gap-2 mt-6">
					<button
						type="reset"
						data-modal-close="deleteModal"
						class="bg-white text-slate-500 btn hover:text-slate-500 hover:bg-slate-100 focus:text-slate-500 focus:bg-slate-100 active:text-slate-500 active:bg-slate-100 dark:bg-zink-600 dark:hover:bg-slate-500/10 dark:focus:bg-slate-500/10 dark:active:bg-slate-500/10"
						on:click={toggleDelete}>Cancel</button
					>
					<button
						type="submit"
						class="text-white bg-red-500 border-red-500 btn hover:text-white hover:bg-red-600 hover:border-red-600 focus:text-white focus:bg-red-600 focus:border-red-600 focus:ring focus:ring-red-100 active:text-white active:bg-red-600 active:border-red-600 active:ring active:ring-red-100 dark:ring-custom-400/20"
						>Yes, Delete It!</button
					>
				</div>
			</div>
		</div>
	</div>
</Modal>
