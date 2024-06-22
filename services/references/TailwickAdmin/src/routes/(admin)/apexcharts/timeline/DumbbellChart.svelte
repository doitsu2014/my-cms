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
							x: 'Operations',
							y: [2800, 4500]
						},
						{
							x: 'Customer Success',
							y: [3200, 4100]
						},
						{
							x: 'Engineering',
							y: [2950, 7800]
						},
						{
							x: 'Marketing',
							y: [3000, 4600]
						},
						{
							x: 'Product',
							y: [3500, 4100]
						},
						{
							x: 'Data Science',
							y: [4500, 6500]
						},
						{
							x: 'Sales',
							y: [4100, 5600]
						}
					]
				}
			],
			chart: {
				height: 350,
				type: 'rangeBar',
				zoom: {
					enabled: false
				}
			},
			plotOptions: {
				bar: {
					horizontal: true,
					isDumbbell: true,
					dumbbellColors: [
						[
							getChartColorsArray(chartColors)[0],
							getChartColorsArray(chartColors)[1]
						]
					]
				}
			},
			title: {
				text: 'Paygap Disparity'
			},
			legend: {
				show: true,
				showForSingleSeries: true,
				position: 'top',
				horizontalAlign: 'left',
				customLegendItems: ['Female', 'Male']
			},
			fill: {
				type: 'gradient',
				gradient: {
					gradientToColors: [getChartColorsArray(chartColors)[1]],
					inverseColors: false,
					stops: [0, 100]
				}
			},
			grid: {
				xaxis: {
					lines: {
						show: true
					}
				},
				yaxis: {
					lines: {
						show: false
					}
				}
			}
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#dumbbellChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="dumbbellChart" class="apex-charts"></div>
