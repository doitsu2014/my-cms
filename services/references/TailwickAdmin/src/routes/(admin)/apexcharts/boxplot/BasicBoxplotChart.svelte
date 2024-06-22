<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	
	onMount(() => {
		var options = {
			series: [
				{
					type: 'boxPlot',
					data: [
						{
							x: 'Jan 2015',
							y: [54, 66, 69, 75, 88]
						},
						{
							x: 'Jan 2016',
							y: [43, 65, 69, 76, 81]
						},
						{
							x: 'Jan 2017',
							y: [31, 39, 45, 51, 59]
						},
						{
							x: 'Jan 2018',
							y: [39, 46, 55, 65, 71]
						},
						{
							x: 'Jan 2019',
							y: [29, 31, 35, 39, 44]
						},
						{
							x: 'Jan 2020',
							y: [41, 49, 58, 61, 67]
						},
						{
							x: 'Jan 2021',
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
				boxPlot: {
					colors: {
						upper: getChartColorsArray(chartColors)[0],
						lower: getChartColorsArray(chartColors)[1]
					}
				}
			}
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#basicBoxplotChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="basicBoxplotChart" class="apex-charts"></div>
