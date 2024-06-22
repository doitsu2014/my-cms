<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	onMount(() => {
		var options = {
			series: [44, 55, 13, 43, 22],
			chart: {
				height: 350,
				type: 'pie'
			},
			labels: ['Team A', 'Team B', 'Team C', 'Team D', 'Team E'],
			colors: getChartColorsArray(chartColors),
			legend: {
				position: 'bottom'
			}
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#simplePieChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="simplePieChart" class="apex-charts"></div>
