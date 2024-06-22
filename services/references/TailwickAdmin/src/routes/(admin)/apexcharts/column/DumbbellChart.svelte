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
							x: '2008',
							y: [2800, 4500]
						},
						{
							x: '2009',
							y: [3200, 4100]
						},
						{
							x: '2010',
							y: [2950, 7800]
						},
						{
							x: '2011',
							y: [3000, 4600]
						},
						{
							x: '2012',
							y: [3500, 4100]
						},
						{
							x: '2013',
							y: [4500, 6500]
						},
						{
							x: '2014',
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
					isDumbbell: true,
					columnWidth: 3,
					dumbbellColors: [
						[
							getChartColorsArray(chartColors)[0],
							getChartColorsArray(chartColors)[1]
						]
					]
				}
			},
			legend: {
				show: true,
				showForSingleSeries: true,
				position: 'top',
				horizontalAlign: 'left',
				customLegendItems: ['Product A', 'Product B']
			},
			fill: {
				type: 'gradient',
				gradient: {
					type: 'vertical',
					gradientToColors: [getChartColorsArray(chartColors)[1]],
					inverseColors: true,
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
			},
			xaxis: {
				tickPlacement: 'on'
			}
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#dumbbellChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="dumbbellChart" class="apex-charts"></div>
