<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	onMount(() => {
		var options = {
			series: [80],
			chart: {
				height: 350,
				type: 'radialBar'
			},
			plotOptions: {
				radialBar: {
					hollow: {
						size: '70%'
					}
				}
			},
			colors: getChartColorsArray(chartColors),
			labels: ['Themesdesign']
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#simpleRadialbarChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="simpleRadialbarChart" class="apex-charts"></div>
