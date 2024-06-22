<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	import { generateData } from './ChartDataGenerate';
	export let chartColors;

	onMount(() => {
		var options = {
			series: [
				{
					name: 'Metric1',
					data: generateData(18, {
						min: 0,
						max: 90
					})
				},
				{
					name: 'Metric2',
					data: generateData(18, {
						min: 0,
						max: 90
					})
				},
				{
					name: 'Metric3',
					data: generateData(18, {
						min: 0,
						max: 90
					})
				},
				{
					name: 'Metric4',
					data: generateData(18, {
						min: 0,
						max: 90
					})
				},
				{
					name: 'Metric5',
					data: generateData(18, {
						min: 0,
						max: 90
					})
				},
				{
					name: 'Metric6',
					data: generateData(18, {
						min: 0,
						max: 90
					})
				},
				{
					name: 'Metric7',
					data: generateData(18, {
						min: 0,
						max: 90
					})
				},
				{
					name: 'Metric8',
					data: generateData(18, {
						min: 0,
						max: 90
					})
				},
				{
					name: 'Metric9',
					data: generateData(18, {
						min: 0,
						max: 90
					})
				}
			],
			chart: {
				height: 350,
				type: 'heatmap'
			},
			dataLabels: {
				enabled: false
			},
			colors: getChartColorsArray(chartColors)
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#basicHeatmap'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="basicHeatmap" class="apex-charts"></div>
