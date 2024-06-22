<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	onMount(() => {
		var options = {
			series: [
				{
					name: 'Series 1',
					data: [80, 50, 30, 40, 100, 20]
				}
			],
			chart: {
				height: 350,
				type: 'radar'
			},
			colors: getChartColorsArray(chartColors),
			xaxis: {
				categories: ['January', 'February', 'March', 'April', 'May', 'June']
			}
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#basicRadarChart'), options);
			chart.render();
		}, 100);
	});
</script>

basicRadar
<div id="basicRadarChart" class="apex-charts"></div>
