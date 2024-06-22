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
				},
				{
					name: 'Series 2',
					data: [20, 30, 40, 80, 20, 80]
				},
				{
					name: 'Series 3',
					data: [44, 76, 78, 13, 43, 10]
				}
			],
			chart: {
				height: 350,
				type: 'radar',
				dropShadow: {
					enabled: true,
					blur: 1,
					left: 1,
					top: 1
				}
			},
			stroke: {
				width: 2
			},
			colors: getChartColorsArray(chartColors),
			fill: {
				opacity: 0.1
			},
			markers: {
				size: 0
			},
			xaxis: {
				categories: ['2011', '2012', '2013', '2014', '2015', '2016']
			}
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#radarMultipleSeries'), options);
			chart.render();
		}, 100);
	});
</script>

basicRadar
<div id="radarMultipleSeries" class="apex-charts"></div>
