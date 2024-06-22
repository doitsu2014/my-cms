<script>
	import LucideIcon from '../../../../common/components/LucideIcon.svelte';
	import Dropdown from '../../../../common/components/Dropdown.svelte';
	import DropdownToggle from '../../../../common/components/DropdownToggle.svelte';
	import DropdownMenu from '../../../../common/components/DropdownMenu.svelte';
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	var options = {
		series: [
			{
				name: 'New',
				data: [44, 55, 41, 67, 22, 43, 14, 55, 41]
			},
			{
				name: 'Pending',
				data: [13, 23, 20, 8, 13, 27, 8, 20, 8]
			},
			{
				name: 'Completed',
				data: [11, 17, 15, 15, 21, 14, 24, 11, 17]
			},
			{
				name: 'Rejected',
				data: [21, 7, 25, 13, 22, 8, 13, 7, 25]
			}
		],
		chart: {
			type: 'bar',
			height: 350,
			stacked: true,
			zoom: {
				enabled: true
			},
			toolbar: {
				show: false
			}
		},
		plotOptions: {
			bar: {
				horizontal: false,
				borderRadius: 2,
				columnWidth: '25%'
			}
		},
		grid: {
			padding: {
				top: -15,
				bottom: 5,
				right: 0
			}
		},
		xaxis: {
			categories: ['01', '02', '03', '04', '05', '06', '07', '08', '09']
		},
		dataLabels: {
			enabled: false
		},
		colors: getChartColorsArray(chartColors),
		legend: {
			position: 'bottom'
		},
		fill: {
			opacity: 1
		}
	};

	onMount(() => {
		var chart = new ApexCharts(document.querySelector('#totalProjectChart'), options);
		chart.render();
	});
</script>

<div class="col-span-12 md:order-10 lg:col-span-6 xl:col-span-4 2xl:col-span-3 card">
	<div class="card-body">
		<div class="flex items-center gap-2 mb-3">
			<h6 class="text-15 grow">Total Projects (247)</h6>
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
		<div id="totalProjectChart" class="-ml-2 apex-charts" dir="ltr"></div>
	</div>
</div>
