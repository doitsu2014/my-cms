<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	var options = {
		series: [
			{
				type: 'rangeArea',
				name: 'Profit Range',

				data: [
					{
						x: 'Mar',
						y: [900, 2900]
					},
					{
						x: 'Apr',
						y: [1400, 2700]
					},
					{
						x: 'May',
						y: [2600, 3900]
					},
					{
						x: 'Jun',
						y: [500, 1700]
					},
					{
						x: 'Jul',
						y: [1900, 2300]
					},
					{
						x: 'Aug',
						y: [1000, 1500]
					}
				]
			},

			{
				type: 'rangeArea',
				name: 'Expense Range',
				data: [
					{
						x: 'Mar',
						y: [3900, 4900]
					},
					{
						x: 'Apr',
						y: [3400, 3900]
					},
					{
						x: 'May',
						y: [5100, 5900]
					},
					{
						x: 'Jun',
						y: [5400, 6700]
					},
					{
						x: 'Jul',
						y: [4300, 4600]
					},
					{
						x: 'Aug',
						y: [2100, 2900]
					}
				]
			},

			{
				type: 'line',
				name: 'Profit Median',
				data: [
					{
						x: 'Mar',
						y: 1900
					},
					{
						x: 'Apr',
						y: 2200
					},
					{
						x: 'May',
						y: 3000
					},
					{
						x: 'Jun',
						y: 1000
					},
					{
						x: 'Jul',
						y: 2100
					},
					{
						x: 'Aug',
						y: 1200
					},
					{
						x: 'Sep',
						y: 2250
					},
					{
						x: 'Oct',
						y: 2900
					}
				]
			},
			{
				type: 'line',
				name: 'Expense Median',
				data: [
					{
						x: 'Mar',
						y: 4300
					},
					{
						x: 'Apr',
						y: 3700
					},
					{
						x: 'May',
						y: 5500
					},
					{
						x: 'Jun',
						y: 5900
					},
					{
						x: 'Jul',
						y: 4500
					},
					{
						x: 'Aug',
						y: 3500
					},
					{
						x: 'Sep',
						y: 2000
					},
					{
						x: 'Oct',
						y: 1800
					}
				]
			}
		],
		chart: {
			height: 285,
			type: 'rangeArea',
			animations: {
				speed: 500
			},
			toolbar: {
				show: false
			}
		},
		colors: getChartColorsArray(chartColors),
		dataLabels: {
			enabled: false
		},
		fill: {
			opacity: [0.24, 0.24, 1, 1]
		},
		forecastDataPoints: {
			count: 2
		},
		yaxis: {
			show: false
		},
		stroke: {
			curve: 'straight',
			width: [0, 0, 2, 2]
		},
		grid: {
			show: true,
			padding: {
				top: -8,
				left: 10,
				right: 0
			}
		},
		legend: {
			show: true,
			customLegendItems: ['Team B', 'Team A'],
			inverseOrder: true
		},
		markers: {
			hover: {
				sizeOffset: 5
			}
		}
	};

	onMount(() => {
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#salesThisMonthChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="salesThisMonthChart" class="apex-charts"></div>
