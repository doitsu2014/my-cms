<script>
	import Dropdown from '../../../../common/components/Dropdown.svelte';
	import DropdownMenu from '../../../../common/components/DropdownMenu.svelte';
	import DropdownToggle from '../../../../common/components/DropdownToggle.svelte';
	import LucideIcon from '../../../../common/components/LucideIcon.svelte';
</script>

<div class="col-span-12 md:order-11 lg:col-span-6 xl:col-span-4 2xl:col-span-3 card">
	<div class="!pb-0 card-body">
		<h6 class="mb-3 text-15">Upcoming Interview</h6>
	</div>
	<div class="pb-5">
		<div data-overlayscrollbars-initialize class="flex flex-col h-[350px] gap-4 px-5">
			<div class="flex flex-col gap-3">
				<div class="border rounded-md border-slate-200 dark:border-zink-500">
					<div class="flex flex-wrap items-center gap-3 p-2">
						<div class="size-10 rounded-full shrink-0">
							<img src="/assets/images/users/user-1.jpg" alt="" class="h-10 rounded-full" />
						</div>
						<div class="grow">
							<h6 class="mb-1"><a href="#!">James Krogman</a></h6>
							<p class="text-slate-500 dark:text-zink-200">james@tailwick.com</p>
						</div>
						<Dropdown class="relative shrink-0" direction="bottom-end">
							<DropdownToggle
								className="flex items-center justify-center  size-[30px] p-0 bg-white text-slate-500 btn hover:text-slate-500 hover:bg-slate-100 focus:text-slate-500 focus:bg-slate-100 active:text-slate-500 active:bg-slate-100 dark:bg-zink-700 dark:hover:bg-slate-500/10 dark:focus:bg-slate-500/10 dark:active:bg-slate-500/10"
							>
								<LucideIcon name="MoreVertical" class="inline-block size-4" />
							</DropdownToggle>
							<DropdownMenu
								tag="ul"
								class="absolute z-50 py-2 mt-1 ltr:text-left rtl:text-right list-none bg-white rounded-md shadow-md dropdown-menu min-w-[10rem] dark:bg-zink-600"
							>
								<li>
									<a
										class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!">Overview</a
									>
								</li>
								<li>
									<a
										class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!">Edit</a
									>
								</li>
								<li>
									<a
										class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!">Delete</a
									>
								</li>
							</DropdownMenu>
						</Dropdown>
					</div>
					<div class="p-2 border-t border-slate-200 dark:border-zink-500">
						<div class="flex flex-col gap-3 md:items-center md:flex-row">
							<p class="text-slate-500 dark:text-zink-200 shrink-0">
								<LucideIcon name="Calendar" class="inline-block size-4 ltr:mr-1 rtl::ml-1" />
								<span class="align-middle">25 Nov</span>
							</p>
							<p class="text-slate-500 dark:text-zink-200 grow">
								<LucideIcon name="Clock4" class="inline-block size-4 ltr:mr-1 rtl::ml-1" />
								<span class="align-middle">02:41 PM</span>
							</p>
							<span
								class="px-2.5 py-0.5 text-xs inline-block text-center font-medium shrink-0 rounded border bg-white border-green-400 text-green-500 dark:bg-zink-700 dark:border-green-700"
								>Confirm</span
							>
						</div>
					</div>
				</div>
				<div class="border rounded-md border-slate-200 dark:border-zink-500">
					<div class="flex flex-wrap items-center gap-3 p-2">
						<div class="size-10 rounded-full shrink-0">
							<img src="/assets/images/users/user-2.jpg" alt="" class="h-10 rounded-full" />
						</div>
						<div class="grow">
							<h6 class="mb-1"><a href="#!">Michael Scott</a></h6>
							<p class="text-slate-500 dark:text-zink-200">michaelScott@tailwick.com</p>
						</div>
						<Dropdown class="relative shrink-0" direction="bottom-end">
							<DropdownToggle
								className="flex items-center justify-center  size-[30px] p-0 bg-white text-slate-500 btn hover:text-slate-500 hover:bg-slate-100 focus:text-slate-500 focus:bg-slate-100 active:text-slate-500 active:bg-slate-100 dark:bg-zink-700 dark:hover:bg-slate-500/10 dark:focus:bg-slate-500/10 dark:active:bg-slate-500/10"
							>
								<LucideIcon name="MoreVertical" class="inline-block size-4" />
							</DropdownToggle>
							<DropdownMenu
								tag="ul"
								class="absolute z-50 py-2 mt-1 ltr:text-left rtl:text-right list-none bg-white rounded-md shadow-md dropdown-menu min-w-[10rem] dark:bg-zink-600"
							>
								<li>
									<a
										class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!">Overview</a
									>
								</li>
								<li>
									<a
										class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!">Edit</a
									>
								</li>
								<li>
									<a
										class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!">Delete</a
									>
								</li>
							</DropdownMenu>
						</Dropdown>
					</div>
					<div class="p-2 border-t border-slate-200 dark:border-zink-500">
						<div class="flex flex-col gap-3 md:items-center md:flex-row">
							<p class="text-slate-500 dark:text-zink-200 shrink-0">
								<LucideIcon name="Calendar" class="inline-block size-4 ltr:mr-1 rtl::ml-1" />
								<span class="align-middle">05 Dec</span>
							</p>
							<p class="text-slate-500 dark:text-zink-200 grow">
								<LucideIcon name="Clock4" class="inline-block size-4 ltr:mr-1 rtl::ml-1" />
								<span class="align-middle">01:23 PM</span>
							</p>
							<span
								class="px-2.5 py-0.5 text-xs text-center inline-block font-medium shrink-0 rounded border bg-white border-custom-400 text-custom-500 dark:bg-zink-700 dark:border-custom-700"
								>Re-scheduled</span
							>
						</div>
					</div>
				</div>
				<div class="border rounded-md border-slate-200 dark:border-zink-500">
					<div class="flex flex-wrap items-center gap-3 p-2">
						<div class="size-10 rounded-full shrink-0">
							<img src="/assets/images/users/user-3.jpg" alt="" class="h-10 rounded-full" />
						</div>
						<div class="grow">
							<h6 class="mb-1"><a href="#!">Denise Ledford</a></h6>
							<p class="text-slate-500 dark:text-zink-200">ledford@tailwick.com</p>
						</div>
						<Dropdown class="relative shrink-0" direction="bottom-end">
							<DropdownToggle
								className="flex items-center justify-center  size-[30px] p-0 bg-white text-slate-500 btn hover:text-slate-500 hover:bg-slate-100 focus:text-slate-500 focus:bg-slate-100 active:text-slate-500 active:bg-slate-100 dark:bg-zink-700 dark:hover:bg-slate-500/10 dark:focus:bg-slate-500/10 dark:active:bg-slate-500/10"
							>
								<LucideIcon name="MoreVertical" class="inline-block size-4" />
							</DropdownToggle>
							<DropdownMenu
								tag="ul"
								class="absolute z-50 py-2 mt-1 ltr:text-left rtl:text-right list-none bg-white rounded-md shadow-md dropdown-menu min-w-[10rem] dark:bg-zink-600"
							>
								<li>
									<a
										class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!">Overview</a
									>
								</li>
								<li>
									<a
										class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!">Edit</a
									>
								</li>
								<li>
									<a
										class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!">Delete</a
									>
								</li>
							</DropdownMenu>
						</Dropdown>
					</div>
					<div class="p-2 border-t border-slate-200 dark:border-zink-500">
						<div class="flex flex-col gap-3 md:items-center md:flex-row">
							<p class="text-slate-500 dark:text-zink-200 shrink-0">
								<LucideIcon name="Calendar" class="inline-block size-4 ltr:mr-1 rtl::ml-1" />
								<span class="align-middle">27 Nov</span>
							</p>
							<p class="text-slate-500 dark:text-zink-200 grow">
								<LucideIcon name="Clock4" class="inline-block size-4 ltr:mr-1 rtl::ml-1" />
								<span class="align-middle">11:59 PM</span>
							</p>
							<span
								class="px-2.5 py-0.5 text-xs inline-block text-center font-medium shrink-0 rounded border bg-white border-sky-400 text-sky-500 dark:bg-zink-700 dark:border-sky-700"
								>Scheduled</span
							>
						</div>
					</div>
				</div>
				<div class="border rounded-md border-slate-200 dark:border-zink-500">
					<div class="flex flex-wrap items-center gap-3 p-2">
						<div class="size-10 rounded-full shrink-0">
							<img src="/assets/images/users/avatar-5.png" alt="" class="h-10 rounded-full" />
						</div>
						<div class="grow">
							<h6 class="mb-1"><a href="#!">Gladys Smith</a></h6>
							<p class="text-slate-500 dark:text-zink-200">gap-4@tailwick.com</p>
						</div>
						<Dropdown class="relative shrink-0" direction="bottom-end">
							<DropdownToggle
								className="flex items-center justify-center  size-[30px] p-0 bg-white text-slate-500 btn hover:text-slate-500 hover:bg-slate-100 focus:text-slate-500 focus:bg-slate-100 active:text-slate-500 active:bg-slate-100 dark:bg-zink-700 dark:hover:bg-slate-500/10 dark:focus:bg-slate-500/10 dark:active:bg-slate-500/10"
							>
								<LucideIcon name="MoreVertical" class="inline-block size-4" />
							</DropdownToggle>
							<DropdownMenu
								tag="ul"
								class="absolute z-50 py-2 mt-1 ltr:text-left rtl:text-right list-none bg-white rounded-md shadow-md dropdown-menu min-w-[10rem] dark:bg-zink-600"
							>
								<li>
									<a
										class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!">Overview</a
									>
								</li>
								<li>
									<a
										class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!">Edit</a
									>
								</li>
								<li>
									<a
										class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!">Delete</a
									>
								</li>
							</DropdownMenu>
						</Dropdown>
					</div>
					<div class="p-2 border-t border-slate-200 dark:border-zink-500">
						<div class="flex flex-col gap-3 md:items-center md:flex-row">
							<p class="text-slate-500 dark:text-zink-200 shrink-0">
								<LucideIcon name="Calendar" class="inline-block size-4 ltr:mr-1 rtl::ml-1" />
								<span class="align-middle">07 Dec</span>
							</p>
							<p class="text-slate-500 dark:text-zink-200 grow">
								<LucideIcon name="Clock4" class="inline-block size-4 ltr:mr-1 rtl::ml-1" />
								<span class="align-middle">05:19 PM</span>
							</p>
							<span
								class="px-2.5 py-0.5 text-xs inline-block text-center font-medium shrink-0 rounded border bg-white border-red-400 text-red-500 dark:bg-zink-700 dark:border-red-700"
								>Cancelled</span
							>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>
