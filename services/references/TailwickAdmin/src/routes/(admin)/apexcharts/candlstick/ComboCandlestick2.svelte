<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	import { seriesDataLinear } from '../../../../common/data/ohlc';
	export let chartColors;

	onMount(() => {
		var options = {
			series: [
				{
					name: 'volume',
					data: seriesDataLinear
				}
			],
			chart: {
				height: 140,
				type: 'bar',
				brush: {
					enabled: true,
					target: 'candles'
				},
				selection: {
					enabled: true,
					xaxis: {
						min: new Date('20 Jan 2017').getTime(),
						max: new Date('10 Dec 2017').getTime()
					},
					fill: {
						color: '#ccc',
						opacity: 0.4
					},
					stroke: {
						color: '#0D47A1'
					}
				}
			},
			dataLabels: {
				enabled: false
			},
			plotOptions: {
				bar: {
					columnWidth: '80%',
					colors: {
						ranges: [
							{
								from: -1000,
								to: 0,
								color: getChartColorsArray(chartColors)[0]
							},
							{
								from: 1,
								to: 10000,
								color: getChartColorsArray(chartColors)[1]
							}
						]
					}
				}
			},
			stroke: {
				width: 0
			},
			xaxis: {
				type: 'datetime',
				axisBorder: {
					offsetX: 13
				}
			},
			yaxis: {
				labels: {
					show: false
				}
			}
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#combo_candlestick2'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="combo_candlestick2" class="apex-charts"></div>
