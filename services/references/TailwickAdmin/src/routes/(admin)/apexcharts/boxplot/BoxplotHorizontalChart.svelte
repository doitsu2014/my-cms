<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	onMount(() => {
		var options = {
			series: [
				{
					data: [
						{
							x: 'Category A',
							y: [54, 66, 69, 75, 88]
						},
						{
							x: 'Category B',
							y: [43, 65, 69, 76, 81]
						},
						{
							x: 'Category C',
							y: [31, 39, 45, 51, 59]
						},
						{
							x: 'Category D',
							y: [39, 46, 55, 65, 71]
						},
						{
							x: 'Category E',
							y: [29, 31, 35, 39, 44]
						},
						{
							x: 'Category F',
							y: [41, 49, 58, 61, 67]
						},
						{
							x: 'Category G',
							y: [54, 59, 66, 71, 88]
						}
					]
				}
			],
			chart: {
				type: 'boxPlot',
				height: 350
			},
			plotOptions: {
				bar: {
					horizontal: true,
					barHeight: '50%'
				},
				boxPlot: {
					colors: {
						upper: getChartColorsArray(chartColors)[0],
						lower: getChartColorsArray(chartColors)[1]
					}
				}
			}
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#boxplotHorizontalChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="boxplotHorizontalChart" class="apex-charts"></div>
