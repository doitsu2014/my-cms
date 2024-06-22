<script>
	import Dropdown from '../../common/components/Dropdown.svelte';
	import DropdownMenu from '../../common/components/DropdownMenu.svelte';
	import DropdownToggle from '../../common/components/DropdownToggle.svelte';
	import LucideIcon from '../../common/components/LucideIcon.svelte';
	import Progress from '../../common/components/Progress.svelte';
</script>

<div class="col-span-12 card lg:col-span-6 2xl:col-span-3">
	<div class="card-body">
		<div class="flex items-center mb-3">
			<h6 class="grow text-15">Customer Service</h6>

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

		<div>
			<div class="flex items-center justify-between mt-5 mb-2">
				<p class="text-slate-500 dark:text-zink-200">28% of the Goal Reached ($25k)</p>
			</div>
			<Progress barClass="h-2 bg-green-500" value="28" />
			<div class="grid mt-3 xl:grid-cols-2">
				<div class="flex items-center gap-2">
					<div class="shrink-0">
						<LucideIcon name="CalendarDays" class="inline-block size-4 mb-1 align-middle" />
					</div>
					<p class="mb-0 text-slate-500 dark:text-zink-200">
						This Month: <span class="font-medium text-slate-800 dark:text-zink-50">$13,741</span>
					</p>
				</div>
			</div>
		</div>
		<h6 class="mt-4 mb-3">Top Customer</h6>
		<ul class="divide-y divide-slate-200 dark:divide-zink-500">
			<li class="flex items-center gap-3 py-2 first:pt-0 last:pb-0">
				<div class="size-8 rounded-full shrink-0 bg-slate-100 dark:bg-zink-600">
					<img src="/assets/images/users/avatar-2.png" alt="" class="size-8 rounded-full" />
				</div>
				<div class="grow">
					<h6 class="font-medium">Urrie Arthur</h6>
					<p class="text-slate-500 dark:text-zink-200">arthur@tailwick.com</p>
				</div>
				<div class="shrink-0">
					<h6>$2,452</h6>
				</div>
			</li>
			<li class="flex items-center gap-3 py-2 first:pt-0 last:pb-0">
				<div class="size-8 rounded-full shrink-0 bg-slate-100 dark:bg-zink-600">
					<img src="/assets/images/users/avatar-3.png" alt="" class="size-8 rounded-full" />
				</div>
				<div class="grow">
					<h6 class="font-medium">Natalie Christy</h6>
					<p class="text-slate-500 dark:text-zink-200">natalie@tailwick.com</p>
				</div>
				<div class="shrink-0">
					<h6>$1,893</h6>
				</div>
			</li>
			<li class="flex items-center gap-3 py-2 first:pt-0 last:pb-0">
				<div class="size-8 rounded-full shrink-0 bg-slate-100 dark:bg-zink-600">
					<img src="/assets/images/users/avatar-4.png" alt="" class="size-8 rounded-full" />
				</div>
				<div class="grow">
					<h6 class="font-medium">Laurie Jackson</h6>
					<p class="text-slate-500 dark:text-zink-200">jackson@tailwick.com</p>
				</div>
				<div class="shrink-0">
					<h6>$1,196</h6>
				</div>
			</li>
			<li class="flex items-center gap-3 py-2 first:pt-0 last:pb-0">
				<div class="size-8 rounded-full shrink-0 bg-slate-100 dark:bg-zink-600">
					<img src="/assets/images/users/avatar-5.png" alt="" class="size-8 rounded-full" />
				</div>
				<div class="grow">
					<h6 class="font-medium">Michael Torres</h6>
					<p class="text-slate-500 dark:text-zink-200">torres@tailwick.com</p>
				</div>
				<div class="shrink-0">
					<h6>$976</h6>
				</div>
			</li>
		</ul>
	</div>
</div>
