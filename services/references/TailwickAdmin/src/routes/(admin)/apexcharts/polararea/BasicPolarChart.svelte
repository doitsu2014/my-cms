<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	onMount(() => {
		var options = {
			series: [14, 23, 21, 17, 15, 10, 12, 17, 21],
			chart: {
				height: 350,
				type: 'polarArea'
			},
			stroke: {
				colors: ['#fff']
			},
			colors: getChartColorsArray(chartColors),
			fill: {
				opacity: 0.8
			},
			legend: {
				position: 'bottom'
			}
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#basicPolarChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="basicPolarChart" class="apex-charts"></div>
