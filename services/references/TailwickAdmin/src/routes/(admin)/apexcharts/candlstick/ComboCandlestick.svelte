<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	import { seriesData } from '../../../../common/data/ohlc';
	export let chartColors;

	onMount(() => {
		var options = {
			series: [
				{
					data: seriesData
				}
			],
			chart: {
				type: 'candlestick',
				height: 200,
				id: 'candles',
				toolbar: {
					autoSelected: 'pan',
					show: false
				},
				zoom: {
					enabled: false
				}
			},
			plotOptions: {
				candlestick: {
					colors: {
						upward: getChartColorsArray(chartColors)[0],
						downward: getChartColorsArray(chartColors)[1]
					}
				}
			},
			xaxis: {
				type: 'datetime'
			}
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#combo_candlestick'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="combo_candlestick" class="apex-charts"></div>