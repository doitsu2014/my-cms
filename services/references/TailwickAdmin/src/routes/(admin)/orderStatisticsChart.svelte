<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	var options = {
		series: [
			{
				name: 'Pending',
				data: [17, 16, 19, 22, 24, 29, 25, 20, 25, 31, 28, 35]
			},
			{
				name: 'New Orders',
				data: [30, 24, 32, 27, 16, 22, 32, 21, 24, 20, 38, 28]
			}
		],
		chart: {
			type: 'line',
			height: 310,
			toolbar: {
				show: false
			}
		},
		stroke: {
			curve: 'smooth',
			width: 2
		},
		colors: getChartColorsArray(chartColors),
		dataLabels: {
			enabled: false
		},
		grid: {
			show: true,
			padding: {
				top: -20,
				right: 0
			}
		},
		markers: {
			hover: {
				sizeOffset: 4
			}
		}
	};

	onMount(() => {
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#orderStatisticsChart'), options);
			chart.render();
		}, 500);
	});
</script>

<div id="orderStatisticsChart" class="apex-charts"></div>
