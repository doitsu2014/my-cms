<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;
	export let seriesData;

	onMount(() => {
		var options = {
			series: [
				{
					data: seriesData
				}
			],
			chart: {
				id: 'chart1',
				height: 130,
				type: 'area',
				brush: {
					target: 'chart2',
					enabled: true
				},
				selection: {
					enabled: true,
					xaxis: {
						min: new Date('19 Jun 2017').getTime(),
						max: new Date('14 Aug 2017').getTime()
					}
				}
			},
			colors: getChartColorsArray(chartColors),
			fill: {
				type: 'gradient',
				gradient: {
					opacityFrom: 0.91,
					opacityTo: 0.1
				}
			},
			xaxis: {
				type: 'datetime',
				tooltip: {
					enabled: false
				}
			},
			yaxis: {
				tickAmount: 2
			}
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#brushChartLine2'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="brushChartLine2" class="apex-charts"></div>
