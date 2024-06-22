<script>
	import Dropdown from '../../../../common/components/Dropdown.svelte';
	import DropdownToggle from '../../../../common/components/DropdownToggle.svelte';
	import DropdownMenu from '../../../../common/components/DropdownMenu.svelte';
	import LucideIcon from '../../../../common/components/LucideIcon.svelte';
	import EmailDataChart from './EmailDataChart.svelte';
</script>

<div class="col-span-12 card 2xl:col-span-6">
	<div class="card-body">
		<div class="flex items-center gap-2">
			<h6 class="text-15 grow">Email Data</h6>
			<Dropdown class="relative"  direction="bottom-start">
				<DropdownToggle
					className="px-2 py-1.5 text-xs bg-white border-dashed text-custom-500 btn border-custom-500 hover:text-custom-500 hover:bg-custom-50 hover:border-custom-600 focus:text-custom-600 focus:bg-custom-50 focus:border-custom-600 active:text-custom-600 active:bg-custom-50 active:border-custom-600 dark:bg-zink-700 dark:ring-custom-400/20 dark:hover:bg-custom-800/20 dark:focus:bg-custom-800/20 dark:active:bg-custom-800/20 "
				>
					This Yearly
					<LucideIcon name="ChevronDown" class="inline-block size-4 ltr:ml-1 rlt:mr-1" />
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

		<!-- <div id="emailDataChart" class="apex-charts" data-chart-colors='["bg-custom-500", "bg-purple-500"]' dir="ltr"></div> -->
		<EmailDataChart chartColors={['bg-custom-500', 'bg-purple-500']} />
	</div>
</div>
