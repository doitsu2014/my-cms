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
					data: generateData(20, {
						min: 0,
						max: 90
					})
				},
				{
					name: 'Metric2',
					data: generateData(20, {
						min: 0,
						max: 90
					})
				},
				{
					name: 'Metric3',
					data: generateData(20, {
						min: 0,
						max: 90
					})
				},
				{
					name: 'Metric4',
					data: generateData(20, {
						min: 0,
						max: 90
					})
				},
				{
					name: 'Metric5',
					data: generateData(20, {
						min: 0,
						max: 90
					})
				},
				{
					name: 'Metric6',
					data: generateData(20, {
						min: 0,
						max: 90
					})
				},
				{
					name: 'Metric7',
					data: generateData(20, {
						min: 0,
						max: 90
					})
				},
				{
					name: 'Metric8',
					data: generateData(20, {
						min: 0,
						max: 90
					})
				},
				{
					name: 'Metric8',
					data: generateData(20, {
						min: 0,
						max: 90
					})
				}
			],
			chart: {
				height: 350,
				type: 'heatmap'
			},
			stroke: {
				width: 0
			},
			plotOptions: {
				heatmap: {
					radius: 30,
					enableShades: false
				}
			},
			colors: getChartColorsArray(chartColors),
			dataLabels: {
				enabled: true,
				style: {
					colors: ['#fff']
				}
			},
			xaxis: {
				type: 'category'
			}
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#roundedRangeHeatmap'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="roundedRangeHeatmap" class="apex-charts"></div>
