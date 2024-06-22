<script>
	import HeadTitle from '../../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../../common/components/Breadcrumb.svelte';
	import LucideIcon from '../../../../../common/components/LucideIcon.svelte';
	import data from '../../../../../common/data/product-list';
	import Dropdown from '../../../../../common/components/Dropdown.svelte';
	import DropdownToggle from '../../../../../common/components/DropdownToggle.svelte';
	import DropdownMenu from '../../../../../common/components/DropdownMenu.svelte';
	import Filter from './Filter.svelte';

	let isGridview = false;
	const toggleGridview = () => (isGridview = !isGridview);
</script>

<HeadTitle title="Grid View" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto relative">
	<Breadcrumb title="Grid View" pagetitle="Products" />
	<div class="grid grid-cols-1 2xl:grid-cols-12 gap-x-5">
		<Filter />

		<div class="2xl:col-span-9">
			<div class="flex flex-wrap items-center gap-2">
				<p class="grow">Showing all <b>7,410</b> items results</p>
				<div class="flex gap-2 shrink-0 items-center">
					<Dropdown class="relative shrink-0 items-center" direction="bottom-start">
						<DropdownToggle
							className="bg-white text-custom-500 btn border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:bg-zink-700 dark:hover:bg-custom-500 dark:ring-custom-400/20 dark:focus:bg-custom-500"
						>
							Sort by: <b class="font-medium">Highest Price</b>
							<LucideIcon name="ChevronDown" class="inline-block size-4 ltr:ml-1 rtl:mr-1" />
						</DropdownToggle>
						<DropdownMenu
							tag="ul"
							class="absolute z-50 py-2 mt-1 ltr:text-left rtl:text-right list-none bg-white rounded-md shadow-md dropdown-menu min-w-[10rem] dark:bg-zink-600"
						>
							<li>
								<a
									class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
									href="#!">Lowest Price</a
								>
							</li>
							<li>
								<a
									class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
									href="#!">Highest Price</a
								>
							</li>
							<li>
								<a
									class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
									href="#!">Hight to Low</a
								>
							</li>
							<li class="pt-2 mt-2 border-t dark:border-zink-300/50">
								<a
									class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
									href="#!">Low to High</a
								>
							</li>
						</DropdownMenu>
					</Dropdown>
					<button
						type="button"
						id="listView"
						on:click={toggleGridview}
						class="flex items-center justify-center size-[37.5px] p-0 text-sky-500 btn bg-sky-100 hover:text-white hover:bg-sky-600 focus:text-white focus:bg-sky-600 [&.active]:text-white [&.active]:bg-sky-600 dark:bg-sky-500/20 dark:text-sky-400 dark:hover:bg-sky-500 dark:hover:text-white dark:focus:bg-sky-500 dark:focus:text-white dark:[&.active]:bg-sky-500 dark:[&.active]:text-white dark:ring-sky-400/20 {isGridview
							? 'active'
							: ''}"><LucideIcon name="List" class="size-4" /></button
					>
					<button
						type="button"
						id="gridView"
						on:click={toggleGridview}
						class="flex items-center justify-center size-[37.5px] p-0 text-sky-500 btn bg-sky-100 hover:text-white hover:bg-sky-600 focus:text-white focus:bg-sky-600 [&.active]:text-white [&.active]:bg-sky-600 dark:bg-sky-500/20 dark:text-sky-400 dark:hover:bg-sky-500 dark:hover:text-white dark:focus:bg-sky-500 dark:focus:text-white dark:[&.active]:bg-sky-500 dark:[&.active]:text-white dark:ring-sky-400/20 {!isGridview
							? 'active'
							: ''}"><LucideIcon name="LayoutGrid" class="size-4" /></button
					>
				</div>
			</div>

			<div class="flex items-center gap-3 mt-3">
				<span
					class="px-2.5 py-0.5 text-sm font-medium rounded-full border bg-slate-100 border-slate-300 text-slate-500 inline-flex items-center dark:bg-zink-800 dark:border-zink-500 dark:text-zink-200"
					>High to Low <a
						href="#!"
						class="transition text-slate-500 dark:text-zink-200 hover:text-slate-600 dark:hover:text-zink-100"
						><LucideIcon name="X" class="size-3 ltr:ml-1 rtl:mr-1" /></a
					></span
				>
				<span
					class="px-2.5 py-0.5 text-sm font-medium rounded-full border bg-slate-100 border-slate-300 text-slate-500 inline-flex items-center dark:bg-zink-800 dark:border-zink-500 dark:text-zink-200"
					>New <a
						href="#!"
						class="transition text-slate-500 dark:text-zink-200 hover:text-slate-600 dark:hover:text-zink-100"
						><LucideIcon name="X" class="size-3 ltr:ml-1 rtl:mr-1" /></a
					></span
				>
				<a
					href="#!"
					class="px-2.5 py-0.5 text-sm font-medium rounded border bg-transparent border-transparent text-slate-500 transition hover:bg-slate-200 dark:bg-zink-800 dark:hover:bg-zink-600 dark:text-zink-200"
					>All Clear</a
				>
			</div>

			<div
				class="grid grid-cols-1 mt-5 md:grid-cols-2 [&.gridView]:grid-cols-1 xl:grid-cols-4 group [&.gridView]:xl:grid-cols-1 gap-x-5 {isGridview
					? 'gridView'
					: ''}"
				id="cardGridView"
			>
				{#each data.PRODUCTSLIST as product, index}
					{#if index < 8}
						<div class="card md:group-[.gridView]:flex relative">
							<div class="relative group-[.gridView]:static p-8 group-[.gridView]:p-5">
								<a
									href="#!"
									class="absolute group/item toggle-button top-6 ltr:right-6 rtl:left-6 {product.isFavorite
										? 'active'
										: ''}"
									><LucideIcon
										name="Heart"
										class="size-5 text-slate-400 fill-slate-200 transition-all duration-150 ease-linear dark:text-zink-200 dark:fill-zink-600 group-[.active]/item:text-red-500 dark:group-[.active]/item:text-red-500 group-[.active]/item:fill-red-200 dark:group-[.active]/item:fill-red-500/20 group-hover/item:text-red-500 dark:group-hover/item:text-red-500 group-hover/item:fill-red-200 dark:group-hover/item:fill-red-500/20"
									/></a
								>
								<div
									class="group-[.gridView]:p-3 group-[.gridView]:bg-slate-100 dark:group-[.gridView]:bg-zink-600 group-[.gridView]:inline-block rounded-md"
								>
									<img src={product.img} alt="" class="group-[.gridView]:h-16" />
								</div>
							</div>
							<div
								class="card-body !pt-0 md:group-[.gridView]:flex group-[.gridView]:!p-5 group-[.gridView]:gap-3 group-[.gridView]:grow"
							>
								<div class="group-[.gridView]:grow">
									<h6
										class="mb-1 truncate transition-all duration-200 ease-linear text-15 hover:text-custom-500"
									>
										<a href="/apps/ecommerce/products-overview">{product.product_name}</a>
									</h6>

									<div class="flex items-center text-slate-500 dark:text-zink-200">
										<div class="mr-1 text-yellow-500 shrink-0 text-15">
											<i class="ri-star-fill" />
											<i class="ri-star-fill" />
											<i class="ri-star-fill" />
											<i class="ri-star-fill" />
											<i class="ri-star-half-line" />
										</div>
										({(Math.random() * 1000).toFixed(0)})
									</div>
									<h5 class="mt-4 text-16">
										{product.price}<small
											class="font-normal line-through text-slate-500 dark:text-zink-200"
											>{product.original_price}</small
										>
									</h5>
								</div>

								<div
									class="flex items-center gap-2 mt-4 group-[.gridView]:mt-0 group-[.gridView]:self-end"
								>
									<button
										type="button"
										class="w-full bg-white border-dashed text-slate-500 btn border-slate-500 hover:text-slate-500 hover:bg-slate-50 hover:border-slate-600 focus:text-slate-600 focus:bg-slate-50 focus:border-slate-600 active:text-slate-600 active:bg-slate-50 active:border-slate-600 dark:bg-zink-700 dark:text-zink-200 dark:border-zink-400 dark:ring-zink-400/20 dark:hover:bg-zink-600 dark:hover:text-zink-100 dark:focus:bg-zink-600 dark:focus:text-zink-100 dark:active:bg-zink-600 dark:active:text-zink-100"
										><LucideIcon name="ShoppingCart" class="inline-block size-3 leading-none" />
										<span class="align-middle">Add to Cart</span></button
									>
									<Dropdown class="relative float-right" direction="bottom-start">
										<DropdownToggle
											className="flex items-center justify-center w-[38.39px] h-[38.39px] dropdown-toggle p-0 text-slate-500 btn bg-slate-100 hover:text-white hover:bg-slate-600 focus:text-white focus:bg-slate-600 focus:ring focus:ring-slate-100 active:text-white active:bg-slate-600 active:ring active:ring-slate-100 dark:bg-slate-500/20 dark:text-slate-400 dark:hover:bg-slate-500 dark:hover:text-white dark:focus:bg-slate-500 dark:focus:text-white dark:active:bg-slate-500 dark:active:text-white dark:ring-slate-400/20"
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
													><LucideIcon
														name="Trash2"
														class="inline-block size-3 ltr:mr-1 rtl:ml-1"
													/> <span class="align-middle">Delete</span></a
												>
											</li>
										</DropdownMenu>
									</Dropdown>
								</div>
							</div>
						</div>
						<!--end col & card-->
					{/if}
				{/each}
			</div>
			<!--end grid-->

			<div class="flex flex-col items-center mb-5 md:flex-row">
				<div class="mb-4 grow md:mb-0">
					<p class="text-slate-500 dark:text-zink-200">Showing <b>12</b> of <b>44</b> Results</p>
				</div>
				<ul class="flex flex-wrap items-center gap-2 shrink-0">
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 h-8 px-3 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-100 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-white dark:[&.active]:text-white [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
							><LucideIcon class="size-4 mr-1 rtl:rotate-180" name="ChevronLeft" /> Prev</a
						>
					</li>
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-100 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-white dark:[&.active]:text-white [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
							>1</a
						>
					</li>
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-100 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-white dark:[&.active]:text-white [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto active"
							>2</a
						>
					</li>
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-100 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-white dark:[&.active]:text-white [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
							>3</a
						>
					</li>
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-100 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-white dark:[&.active]:text-white [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
							>4</a
						>
					</li>
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-100 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-white dark:[&.active]:text-white [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
							>5</a
						>
					</li>
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 h-8 px-3 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-100 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-white dark:[&.active]:text-white [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
							>Next <LucideIcon class="size-4 ml-1 rtl:rotate-180" name="ChevronRight" /></a
						>
					</li>
				</ul>
			</div>
		</div>
	</div>
</div>
