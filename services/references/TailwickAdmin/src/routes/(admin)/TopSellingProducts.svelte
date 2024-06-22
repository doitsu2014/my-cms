<script>
	import Dropdown from '../../common/components/Dropdown.svelte';
	import DropdownMenu from '../../common/components/DropdownMenu.svelte';
	import DropdownToggle from '../../common/components/DropdownToggle.svelte';
	import LucideIcon from '../../common/components/LucideIcon.svelte';
</script>

<div class="col-span-12 card lg:col-span-6 2xl:col-span-3">
	<div class="card-body">
		<div class="flex items-center mb-3">
			<h6 class="grow text-15">Top Selling Products</h6>
			<Dropdown class="relative shrink-0" direction="bottom-start">
				<DropdownToggle
					className="flex items-center justify-center  size-[30px] p-0 bg-white text-slate-500 btn hover:text-slate-500 hover:bg-slate-100 focus:text-slate-500 focus:bg-slate-100 active:text-slate-500 active:bg-slate-100 dark:bg-zink-700 dark:hover:bg-slate-500/10 dark:focus:bg-slate-500/10 dark:active:bg-slate-500/10"
				>
					<LucideIcon name="MoreVertical" class="size-4" />
				</DropdownToggle>
				<DropdownMenu
					tag="ul"
					class="absolute z-50 py-2 mt-1 ltr:text-left rtl:text-right list-none bg-white rounded-md shadow-md dropdown-menu min-w-[10rem] dark:bg-zink-600"
				>
					<li>
						<a
							class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
							href="#!">1 Weekly</a
						>
					</li>
					<li>
						<a
							class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
							href="#!">1 Monthly</a
						>
					</li>
					<li>
						<a
							class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
							href="#!">3 Monthly</a
						>
					</li>
					<li>
						<a
							class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
							href="#!">6 Monthly</a
						>
					</li>
					<li>
						<a
							class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
							href="#!">This Yearly</a
						>
					</li>
				</DropdownMenu>
			</Dropdown>
		</div>
		<ul class="flex flex-col gap-5">
			<li class="flex items-center gap-3">
				<div
					class="flex items-center justify-center size-10 rounded-md bg-slate-100 dark:bg-zink-600"
				>
					<img src="/assets/images/product/img-02.png" alt="" class="h-6" />
				</div>
				<div class="overflow-hidden grow">
					<h6 class="truncate">Mesh Ergonomic Black Chair</h6>
					<div class="text-yellow-500">
						<i class="ri-star-fill"></i>
						<i class="ri-star-fill"></i>
						<i class="ri-star-fill"></i>
						<i class="ri-star-fill"></i>
						<i class="ri-star-half-fill"></i>
					</div>
				</div>
				<h6 class="shrink-0">
					<LucideIcon
						name="ShoppingCartIcon"
						class="inline-block size-4 align-middle text-slate-500 dark:text-zink-200 ltr:mr-1 rtl:ml-1"
					/>
					798
				</h6>
			</li>
			<li class="flex items-center gap-3">
				<div
					class="flex items-center justify-center size-10 rounded-md bg-slate-100 dark:bg-zink-600"
				>
					<img src="/assets/images/product/img-03.png" alt="" class="h-6" />
				</div>
				<div class="overflow-hidden grow">
					<h6 class="truncate">Fastcolors Typography Men</h6>
					<div class="text-yellow-500">
						<i class="ri-star-fill"></i>
						<i class="ri-star-fill"></i>
						<i class="ri-star-fill"></i>
						<i class="ri-star-fill"></i>
						<i class="ri-star-half-fill"></i>
					</div>
				</div>
				<h6 class="shrink-0">
					<LucideIcon
						name="ShoppingCartIcon"
						class="inline-block size-4 align-middle text-slate-500 dark:text-zink-200 ltr:mr-1 rtl:ml-1"
					/> 695
				</h6>
			</li>
			<li class="flex items-center gap-3">
				<div
					class="flex items-center justify-center size-10 rounded-md bg-slate-100 dark:bg-zink-600"
				>
					<img src="/assets/images/product/img-04.png" alt="" class="h-6" />
				</div>
				<div class="overflow-hidden grow">
					<h6 class="truncate">Mesh Ergonomic Green Chair</h6>
					<div class="text-yellow-500">
						<i class="ri-star-fill"></i>
						<i class="ri-star-fill"></i>
						<i class="ri-star-fill"></i>
						<i class="ri-star-fill"></i>
						<i class="ri-star-half-fill"></i>
					</div>
				</div>
				<h6 class="shrink-0">
					<LucideIcon
						name="ShoppingCartIcon"
						class="inline-block size-4 align-middle text-slate-500 dark:text-zink-200 ltr:mr-1 rtl:ml-1"
					/> 985
				</h6>
			</li>
			<li class="flex items-center gap-3">
				<div
					class="flex items-center justify-center size-10 rounded-md bg-slate-100 dark:bg-zink-600"
				>
					<img src="/assets/images/product/img-05.png" alt="" class="h-6" />
				</div>
				<div class="overflow-hidden grow">
					<h6 class="truncate">Techel Black Bluetooth Soundbar</h6>
					<div class="text-yellow-500">
						<i class="ri-star-fill"></i>
						<i class="ri-star-fill"></i>
						<i class="ri-star-fill"></i>
						<i class="ri-star-fill"></i>
						<i class="ri-star-half-fill"></i>
					</div>
				</div>
				<h6 class="shrink-0">
					<LucideIcon
						name="ShoppingCartIcon"
						class="inline-block size-4 align-middle text-slate-500 dark:text-zink-200 ltr:mr-1 rtl:ml-1"
					/> 813
				</h6>
			</li>
			<li class="flex items-center gap-3">
				<div
					class="flex items-center justify-center size-10 rounded-md bg-slate-100 dark:bg-zink-600"
				>
					<img src="/assets/images/product/img-06.png" alt="" class="h-6" />
				</div>
				<div class="overflow-hidden grow">
					<h6 class="truncate">Bovet Fleurier AIFSQ029</h6>
					<div class="text-yellow-500">
						<i class="ri-star-fill"></i>
						<i class="ri-star-fill"></i>
						<i class="ri-star-fill"></i>
						<i class="ri-star-fill"></i>
						<i class="ri-star-half-fill"></i>
					</div>
				</div>
				<h6 class="shrink-0">
					<LucideIcon
						name="ShoppingCartIcon"
						class="inline-block size-4 align-middle text-slate-500 dark:text-zink-200 ltr:mr-1 rtl:ml-1"
					/> 915
				</h6>
			</li>
			<li class="flex items-center gap-3">
				<div
					class="flex items-center justify-center size-10 rounded-md bg-slate-100 dark:bg-zink-600"
				>
					<img src="/assets/images/product/img-03.png" alt="" class="h-6" />
				</div>
				<div class="overflow-hidden grow">
					<h6 class="truncate">Fastcolors Typography Men</h6>
					<div class="text-yellow-500">
						<i class="ri-star-fill"></i>
						<i class="ri-star-fill"></i>
						<i class="ri-star-fill"></i>
						<i class="ri-star-fill"></i>
						<i class="ri-star-half-fill"></i>
					</div>
				</div>
				<h6 class="shrink-0">
					<LucideIcon
						name="ShoppingCartIcon"
						class="inline-block size-4 align-middle text-slate-500 dark:text-zink-200 ltr:mr-1 rtl:ml-1"
					/> 785
				</h6>
			</li>
		</ul>
	</div>
</div>
