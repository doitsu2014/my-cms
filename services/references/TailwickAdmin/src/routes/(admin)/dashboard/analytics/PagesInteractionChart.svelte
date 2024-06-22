<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	import Dropdown from '../../../../common/components/Dropdown.svelte';
	import DropdownToggle from '../../../../common/components/DropdownToggle.svelte';
	import DropdownMenu from '../../../../common/components/DropdownMenu.svelte';
	import LucideIcon from '../../../../common/components/LucideIcon.svelte';
	let chartColors = ['bg-custom-500', 'bg-purple-500'];

	let options = {
		series: [
			{
				name: 'Viewers',
				data: [20, 13, 19, 23, 29, 42, 33, 29, 37, 46, 40, 49]
			}
		],
		chart: {
			height: 350,
			type: 'bar',
			toolbar: {
				show: false
			}
		},
		plotOptions: {
			bar: {
				borderRadius: 10,
				dataLabels: {
					position: 'top' // top, center, bottom
				}
			}
		},
		dataLabels: {
			enabled: true,
			offsetY: -20,
			style: {
				fontSize: '12px',
				colors: ['#304758']
			}
		},

		xaxis: {
			categories: [
				'Jan',
				'Feb',
				'Mar',
				'Apr',
				'May',
				'Jun',
				'Jul',
				'Aug',
				'Sep',
				'Oct',
				'Nov',
				'Dec'
			],
			position: 'bottom',
			axisBorder: {
				show: false
			},
			axisTicks: {
				show: false
			},
			crosshairs: {
				fill: {
					type: 'gradient',
					gradient: {
						colorFrom: '#D8E3F0',
						colorTo: '#BED1E6',
						stops: [0, 100],
						opacityFrom: 0.4,
						opacityTo: 0.5
					}
				}
			},
			tooltip: {
				enabled: true
			}
		},
		yaxis: {
			axisBorder: {
				show: false
			},
			axisTicks: {
				show: false
			},
			labels: {
				show: false
			}
		},
		stroke: {
			show: true,
			width: 4,
			colors: ['transparent']
		},
		grid: {
			show: false,
			padding: {
				top: 0,
				right: 0,
				bottom: 0,
				left: -10
			}
		},
		colors: getChartColorsArray(chartColors)
	};

	onMount(() => {
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#pagesInteraction'), options);
			chart.render();
		}, 100);
	});
</script>

<div class="order-7 col-span-12 2xl:order-1 card 2xl:col-span-7">
	<div class="card-body">
		<div class="flex items-center gap-2">
			<h6 class="mb-3 text-15 grow">Pages Interaction</h6>

			<Dropdown className="relative shrink-0" direction="bottom-end">
				<DropdownToggle
					className="px-2 py-1.5 text-xs bg-text-slate-500 btn bg-slate-100 hover:text-white hover:bg-slate-600 focus:text-white focus:bg-slate-600 focus:ring focus:ring-slate-100 active:text-white active:bg-slate-600 active:ring active:ring-slate-100 dark:bg-slate-500/20 dark:text-slate-400 dark:hover:bg-slate-500 dark:hover:text-white dark:focus:bg-slate-500 dark:focus:text-white dark:active:bg-slate-500 dark:active:text-white dark:ring-slate-400/20 "
				>
					This Yearly <LucideIcon
						name="ChevronDown"
						class="inline-block size-4 ltr:ml-1 rlt:mr-1"
					/>
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
		<div id="pagesInteraction" class="apex-charts" dir="ltr"></div>
	</div>
</div>
