<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	import { generateData } from './ChartDataGenerate';
	export let chartColors;

	onMount(() => {
		var options = {
			series: [
				{
					name: 'Jan',
					data: generateData(20, {
						min: -30,
						max: 55
					})
				},
				{
					name: 'Feb',
					data: generateData(20, {
						min: -30,
						max: 55
					})
				},
				{
					name: 'Mar',
					data: generateData(20, {
						min: -30,
						max: 55
					})
				},
				{
					name: 'Apr',
					data: generateData(20, {
						min: -30,
						max: 55
					})
				},
				{
					name: 'May',
					data: generateData(20, {
						min: -30,
						max: 55
					})
				},
				{
					name: 'Jun',
					data: generateData(20, {
						min: -30,
						max: 55
					})
				},
				{
					name: 'Jul',
					data: generateData(20, {
						min: -30,
						max: 55
					})
				},
				{
					name: 'Aug',
					data: generateData(20, {
						min: -30,
						max: 55
					})
				},
				{
					name: 'Sep',
					data: generateData(20, {
						min: -30,
						max: 55
					})
				}
			],
			chart: {
				height: 350,
				type: 'heatmap'
			},
			plotOptions: {
				heatmap: {
					shadeIntensity: 0.5,
					radius: 0,
					useFillColorAsStroke: true
				}
			},
			colors: getChartColorsArray(chartColors),
			dataLabels: {
				enabled: false
			},
			stroke: {
				width: 1
			},
			title: {
				text: 'HeatMap Chart with Color Range'
			}
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#colorsRangeHeatmap'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="colorsRangeHeatmap" class="apex-charts"></div>
