<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	onMount(() => {
		var options = {
			series: [44, 55, 41, 17, 15],
			chart: {
				height: 350,
				type: 'donut'
			},
			colors: getChartColorsArray(chartColors),
			legend: {
				position: 'bottom'
			}
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#simpleDonutChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="simpleDonutChart" class="apex-charts"></div>
