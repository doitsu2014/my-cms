<script>
	import HeadTitle from '../../../../../common/components/HeadTitle.svelte';
	import { onMount } from 'svelte';
	import Breadcrumb from '../../../../../common/components/Breadcrumb.svelte';
	import Modal from '../../../../../common/components/Modal.svelte';
	import LucideIcon from '../../../../../common/components/LucideIcon.svelte';
	import data from '../../../../../common/data/product-list';
	import Flatpickr from 'svelte-flatpickr';
	import 'flatpickr/dist/flatpickr.css';
	import Dropdown from '../../../../../common/components/Dropdown.svelte';
	import DropdownToggle from '../../../../../common/components/DropdownToggle.svelte';
	import DropdownMenu from '../../../../../common/components/DropdownMenu.svelte';

	let isDeleteModal = false;
	const toggleDelete = () => (isDeleteModal = !isDeleteModal);
</script>

<HeadTitle title="List View" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto relative">
	<Breadcrumb title="List View" pagetitle="Products" />

	<div class="card" id="productListTable">
		<div class="card-body">
			<div class="grid grid-cols-1 gap-4 lg:grid-cols-2 xl:grid-cols-12">
				<div class="xl:col-span-3">
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
				<div class="xl:col-span-2">
					<div>
						<Flatpickr
							name="date"
							class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
							readonly="readonly"
							placeholder="Select Date"
						/>
					</div>
				</div>
				<!--end col-->
				<div class="lg:col-span-2 ltr:lg:text-right rtl:lg:text-left xl:col-span-2 xl:col-start-11">
					<a
						href="/apps/ecommerce/products-new"
						type="button"
						class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
						><LucideIcon name="Plus" class="inline-block size-4" />
						<span class="align-middle">Add Product</span></a
					>
				</div>
			</div>
			<!--end grid-->
		</div>
		<div class="!pt-1 card-body">
			<div class="overflow-x-auto">
				<table class="w-full whitespace-nowrap" id="productTable">
					<thead class="ltr:text-left rtl:text-right bg-slate-100 dark:bg-zink-600">
						<tr>
							<th
								class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500 sort product_code"
								data-sort="product_code">Product Code</th
							>
							<th
								class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500 sort product_name"
								data-sort="product_name">Product Name</th
							>
							<th
								class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500 sort category"
								data-sort="category">Category</th
							>
							<th
								class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500 sort price"
								data-sort="price">Price</th
							>
							<th
								class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500 sort stock"
								data-sort="stock">Stock</th
							>
							<th
								class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500 sort revenue"
								data-sort="revenue">Revenue</th
							>
							<th
								class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500 sort status"
								data-sort="status">Status</th
							>
							<th
								class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500 action"
								>Actions</th
							>
						</tr>
					</thead>
					<tbody class="list">
						{#each data.PRODUCTSLIST as product}
							<tr>
								<td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">
									<a
										href="#!"
										class="transition-all duration-150 ease-linear product_code text-custom-500 hover:text-custom-600"
										>{product.product_code}</a
									>
								</td>
								<td
									class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500 product_name"
								>
									<a href="/apps/ecommerce/products-overview" class="flex items-center gap-2">
										<img src={product.img} alt="Product images" class="h-6" />
										<h6 class="product_name">{product.product_name}</h6>
									</a>
								</td>
								<td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500 category">
									<span
										class="category px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-slate-100 border-slate-200 text-slate-500 dark:bg-slate-500/20 dark:border-slate-500/20 dark:text-zink-200"
										>{product.category}</span
									>
								</td>
								<td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500 price"
									>{product.price}</td
								>
								<td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500 stock"
									>{product.stock}</td
								>
								<td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500 revenue"
									>{product.revenue}</td
								>
								<td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500 status">
									{#if product.status == 'Scheduled'}
										<span
											class="status px-2.5 py-0.5 inline-block text-xs font-medium rounded border bg-orange-100 border-transparent text-orange-500 dark:bg-orange-500/20 dark:border-transparent"
											>{product.status}</span
										>
									{:else if product.status == 'Publish'}
										<span
											class="status px-2.5 py-0.5 inline-block text-xs font-medium rounded border bg-green-100 border-transparent text-green-500 dark:bg-green-500/20 dark:border-transparent"
											>{product.status}</span
										>
									{:else if product.status == 'Inactive'}
										<span
											class="status px-2.5 py-0.5 inline-block text-xs font-medium rounded border bg-red-100 border-transparent text-red-500 dark:bg-red-500/20 dark:border-transparent"
											>{product.status}</span
										>
									{/if}
								</td>
								<td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500 action">
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
													href="/apps/ecommerce/products-overview"
													><LucideIcon name="Eye" class="inline-block size-3 ltr:mr-1 rtl:ml-1" />
													<span class="align-middle">Overview</span></a
												>
											</li>
											<li>
												<a
													class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
													href="/apps/ecommerce/products-new"
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
							name="Search"
							class="size-6 mx-auto mb-3 text-sky-500 fill-sky-100 dark:fill-sky-500/20"
						/>
						<h5 class="mt-2 mb-1">Sorry! No Result Found</h5>
						<p class="mb-0 text-slate-500 dark:text-zink-200">
							We've searched more than 199+ product We did not find any product for you search.
						</p>
					</div>
				</div>
			</div>

			<div class="flex flex-col items-center gap-4 px-4 mt-4 md:flex-row" id="pagination-element">
				<div class="grow">
					<p class="text-slate-500 dark:text-zink-200">
						Showing <b class="showing">10</b> of <b class="total-records">38</b> Results
					</p>
				</div>

				<div class="col-sm-auto mt-sm-0">
					<div class="flex gap-2 pagination-wrap justify-content-center">
						<a
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 h-8 px-3 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-500 dark:[&.active]:text-custom-500 [&.active]:bg-custom-50 dark:[&.active]:bg-custom-500/10 [&.active]:border-custom-50 dark:[&.active]:border-custom-500/10 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto page-item pagination-prev disabled"
							href={'javascript:void(0)'}
						>
							<LucideIcon class="size-4 mr-1 rtl:rotate-180" name="ChevronLeft" /> Prev
						</a>
						<ul class="flex flex-wrap items-center gap-2 pagination listjs-pagination">
							<li class="active"><a class="page" href="#!">1</a></li>
							<li><a class="page" href="#!">2</a></li>
						</ul>
						<a
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 h-8 px-3 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-500 dark:[&.active]:text-custom-500 [&.active]:bg-custom-50 dark:[&.active]:bg-custom-500/10 [&.active]:border-custom-50 dark:[&.active]:border-custom-500/10 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto page-item pagination-next"
							href={'javascript:void(0)'}
						>
							Next <LucideIcon class="size-4 ml-1 rtl:rotate-180" name="ChevronRight" />
						</a>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>

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
