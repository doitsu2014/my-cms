<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	onMount(() => {
		var options = {
			series: [44, 55, 41, 17, 15],
			chart: {
				height: 280,
				type: 'donut'
			},
			plotOptions: {
				pie: {
					startAngle: -90,
					endAngle: 270
				}
			},
			dataLabels: {
				enabled: false
			},
			fill: {
				type: 'gradient'
			},
			colors: getChartColorsArray(chartColors),
			legend: {
				formatter: function (val, opts) {
					return val + ' - ' + opts.w.globals.series[opts.seriesIndex];
				}
			},
			legend: {
				position: 'bottom'
			}
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#gradientDonut'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="gradientDonut" class="apex-charts"></div>
