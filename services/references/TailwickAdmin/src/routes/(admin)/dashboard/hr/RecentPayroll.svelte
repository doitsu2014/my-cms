<script>
	import Dropdown from '../../../../common/components/Dropdown.svelte';
	import DropdownMenu from '../../../../common/components/DropdownMenu.svelte';
	import DropdownToggle from '../../../../common/components/DropdownToggle.svelte';
	import LucideIcon from '../../../../common/components/LucideIcon.svelte';
</script>

<div class="col-span-12 md:order-12 lg:col-span-12 xl:col-span-8 2xl:col-span-3">
	<div class="grid grid-cols-12 gap-x-5">
		<div
			class="relative col-span-12 card bg-gradient-to-r to-custom-100 dark:to-custom-500/20 from-transparent"
		>
			<div
				class="bg-[url('/assets/images/hr-dashboard.png')] absolute inset-0 bg-cover opacity-30"
			></div>
			<div class="relative card-body">
				<div class="flex gap-3 mb-4">
					<div class="size-10 bg-purple-100 rounded-full dark:bg-purple-500/20 shrink-0">
						<img src="/assets/images/users/avatar-6.png" alt="" class="h-10 rounded-full" />
					</div>
					<div class="grow">
						<h6 class="mb-1">Nakisha Short</h6>
						<p class="text-slate-500 dark:text-zink-200">Her Birthday Today</p>
					</div>
				</div>
				<button
					type="button"
					class="px-2 py-1.5 text-xs text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
					>Wish Her</button
				>
			</div>
			<img src="/assets/images/birthday.png" alt="" class="absolute bottom-0 right-0" />
		</div>
		<div class="col-span-12 card">
			<div class="!pb-0 card-body">
				<div class="flex items-center gap-2 mb-3">
					<h6 class="text-15 grow">Recent Payroll</h6>
					<Dropdown class="relative shrink-0" direction="bottom-start">
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
									href="#!">Today</a
								>
							</li>
							<li>
								<a
									class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
									href="#!">Yesterday</a
								>
							</li>
							<li>
								<a
									class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
									href="#!">Thursday</a
								>
							</li>
						</DropdownMenu>
					</Dropdown>
				</div>
			</div>
			<div class="pb-5">
				<div data-overlayscrollbars-initialize class="flex flex-col h-[198px] gap-4 px-5">
					<div class="flex flex-col gap-3">
						<div class="flex flex-wrap items-center gap-3">
							<div
								class="flex items-center justify-center size-6 text-red-500 rounded-full shrink-0"
							>
								<LucideIcon name="MoveUpRight" class="size-4" />
							</div>
							<div class="grow">
								<h6 class="mb-0">Christopher Horn</h6>
							</div>
							<div class="shrink-0">
								<h6>$145.32</h6>
							</div>
							<div class="w-20 ltr:text-right rtl:text-left shrink-0">
								<span
									class="px-2.5 py-0.5 inline-block text-[11px] font-medium rounded border bg-green-100 border-transparent text-green-500 dark:bg-green-500/20 dark:border-transparent"
									>Paid</span
								>
							</div>
						</div>
						<div class="flex flex-wrap items-center gap-3">
							<div
								class="flex items-center justify-center size-6 text-green-500 rounded-full shrink-0"
							>
								<LucideIcon name="MoveDownLeft" class="size-4" />
							</div>
							<div class="grow">
								<h6 class="mb-0">Richard Peters</h6>
							</div>
							<div class="shrink-0">
								<h6>$4512.99</h6>
							</div>
							<div class="w-20 ltr:text-right rtl:text-left shrink-0">
								<span
									class="px-2.5 py-0.5 inline-block text-[11px] font-medium rounded border bg-yellow-100 border-transparent text-yellow-500 dark:bg-yellow-500/20 dark:border-transparent"
									>Pending</span
								>
							</div>
						</div>
						<div class="flex flex-wrap items-center gap-3">
							<div
								class="flex items-center justify-center size-6 text-green-500 rounded-full shrink-0"
							>
								<LucideIcon name="MoveDownLeft" class="size-4" />
							</div>
							<div class="grow">
								<h6 class="mb-0">James Perez</h6>
							</div>
							<div class="shrink-0">
								<h6>$879.99</h6>
							</div>
							<div class="w-20 ltr:text-right rtl:text-left shrink-0">
								<span
									class="px-2.5 py-0.5 inline-block text-[11px] font-medium rounded border bg-green-100 border-transparent text-green-500 dark:bg-green-500/20 dark:border-transparent"
									>Paid</span
								>
							</div>
						</div>
						<div class="flex flex-wrap items-center gap-3">
							<div
								class="flex items-center justify-center size-6 text-red-500 rounded-full shrink-0"
							>
								<LucideIcon name="MoveUpRight" class="size-4" />
							</div>
							<div class="grow">
								<h6 class="mb-0">Myrtle Velez</h6>
							</div>
							<div class="shrink-0">
								<h6>$978.14</h6>
							</div>
							<div class="w-20 ltr:text-right rtl:text-left shrink-0">
								<span
									class="px-2.5 py-0.5 inline-block text-[11px] font-medium rounded border bg-red-100 border-transparent text-red-500 dark:bg-red-500/20 dark:border-transparent"
									>Cancelled</span
								>
							</div>
						</div>
						<div class="flex flex-wrap items-center gap-3">
							<div
								class="flex items-center justify-center size-6 text-green-500 rounded-full shrink-0"
							>
								<LucideIcon name="MoveDownLeft" class="size-4" />
							</div>
							<div class="grow">
								<h6 class="mb-0">Brad Castillo</h6>
							</div>
							<div class="shrink-0">
								<h6>$412.59</h6>
							</div>
							<div class="w-20 ltr:text-right rtl:text-left shrink-0">
								<span
									class="px-2.5 py-0.5 inline-block text-[11px] font-medium rounded border bg-yellow-100 border-transparent text-yellow-500 dark:bg-yellow-500/20 dark:border-transparent"
									>Pending</span
								>
							</div>
						</div>
						<div class="flex flex-wrap items-center gap-3">
							<div
								class="flex items-center justify-center size-6 text-green-500 rounded-full shrink-0"
							>
								<LucideIcon name="MoveDownLeft" class="size-4" />
							</div>
							<div class="grow">
								<h6 class="mb-0">Robert Jump</h6>
							</div>
							<div class="shrink-0">
								<h6>$666.99</h6>
							</div>
							<div class="w-20 ltr:text-right rtl:text-left shrink-0">
								<span
									class="px-2.5 py-0.5 inline-block text-[11px] font-medium rounded border bg-green-100 border-transparent text-green-500 dark:bg-green-500/20 dark:border-transparent"
									>Paid</span
								>
							</div>
						</div>
						<div class="flex flex-wrap items-center gap-3">
							<div
								class="flex items-center justify-center size-6 text-red-500 rounded-full shrink-0"
							>
								<LucideIcon name="MoveUpRight" class="size-4" />
							</div>
							<div class="grow">
								<h6 class="mb-0">Myrtle Velez</h6>
							</div>
							<div class="shrink-0">
								<h6>$978.14</h6>
							</div>
							<div class="w-20 ltr:text-right rtl:text-left shrink-0">
								<span
									class="px-2.5 py-0.5 inline-block text-[11px] font-medium rounded border bg-red-100 border-transparent text-red-500 dark:bg-red-500/20 dark:border-transparent"
									>Cancelled</span
								>
							</div>
						</div>
						<div class="flex flex-wrap items-center gap-3">
							<div
								class="flex items-center justify-center size-6 text-red-500 rounded-full shrink-0"
							>
								<LucideIcon name="MoveUpRight" class="size-4" />
							</div>
							<div class="grow">
								<h6 class="mb-0">Christopher Horn</h6>
							</div>
							<div class="shrink-0">
								<h6>$145.32</h6>
							</div>
							<div class="w-20 ltr:text-right rtl:text-left shrink-0">
								<span
									class="px-2.5 py-0.5 inline-block text-[11px] font-medium rounded border bg-green-100 border-transparent text-green-500 dark:bg-green-500/20 dark:border-transparent"
									>Paid</span
								>
							</div>
						</div>
						<div class="flex flex-wrap items-center gap-3">
							<div
								class="flex items-center justify-center size-6 text-green-500 rounded-full shrink-0"
							>
								<LucideIcon name="MoveDownLeft" class="size-4" />
							</div>
							<div class="grow">
								<h6 class="mb-0">Richard Peters</h6>
							</div>
							<div class="shrink-0">
								<h6>$4512.99</h6>
							</div>
							<div class="w-20 ltr:text-right rtl:text-left shrink-0">
								<span
									class="px-2.5 py-0.5 inline-block text-[11px] font-medium rounded border bg-yellow-100 border-transparent text-yellow-500 dark:bg-yellow-500/20 dark:border-transparent"
									>Pending</span
								>
							</div>
						</div>
						<div class="flex flex-wrap items-center gap-3">
							<div
								class="flex items-center justify-center size-6 text-green-500 rounded-full shrink-0"
							>
								<LucideIcon name="MoveDownLeft" class="size-4" />
							</div>
							<div class="grow">
								<h6 class="mb-0">James Perez</h6>
							</div>
							<div class="shrink-0">
								<h6>$879.99</h6>
							</div>
							<div class="w-20 ltr:text-right rtl:text-left shrink-0">
								<span
									class="px-2.5 py-0.5 inline-block text-[11px] font-medium rounded border bg-green-100 border-transparent text-green-500 dark:bg-green-500/20 dark:border-transparent"
									>Paid</span
								>
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>
