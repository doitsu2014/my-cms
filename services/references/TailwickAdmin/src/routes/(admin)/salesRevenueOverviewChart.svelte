<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	var options = {
		series: [
			{
				name: 'Total Sales',
				data: [44, 55, 41, 67, 22, 43, 21, 49, 20, 41, 67, 22]
			},
			{
				name: 'Total Profit',
				data: [11, 17, 15, 15, 21, 14, 15, 13, 5, 15, 15, 21]
			}
		],
		chart: {
			type: 'bar',
			height: 300,
			stacked: true,
			stackType: '100%',
			toolbar: {
				show: false
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
			]
		},
		tooltip: {
			y: {
				formatter: function (val) {
					return '$' + val + 'k';
				}
			}
		},
		grid: {
			show: true,
			padding: {
				top: -20,
				right: -10
			}
		},
		plotOptions: {
			bar: {
				horizontal: false,
				columnWidth: '50%'
			}
		},
		colors: getChartColorsArray(chartColors),
		fill: {
			opacity: 1
		},
		legend: {
			position: 'bottom'
		}
	};

	onMount(() => {
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#salesRevenueOverviewChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="salesRevenueOverviewChart" class="apex-charts"></div>
