<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	var options = {
		series: [
			{
				data: [400, 430, 448, 470, 540, 580, 690, 1100, 1200, 1380]
			}
		],
		chart: {
			type: 'bar',
			height: 350
		},
		plotOptions: {
			bar: {
				borderRadius: 4,
				horizontal: true
			}
		},
		colors: getChartColorsArray(chartColors),
		dataLabels: {
			enabled: false
		},
		xaxis: {
			categories: [
				'South Korea',
				'Canada',
				'United Kingdom',
				'Netherlands',
				'Italy',
				'France',
				'Japan',
				'United States',
				'China',
				'Germany'
			]
		}
	};

	onMount(() => {
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#basicBarChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="basicBarChart" class="apex-charts"></div>
