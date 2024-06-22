<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	var options = {
		series: [
			{
				name: 'Delivered Rate',
				data: [11, 30, 24, 26, 33, 38, 14, 20, 10, 9, 34, 27, 22]
			}
		],
		chart: {
			id: 'area-datetime',
			type: 'bar',
			height: 80,
			sparkline: {
				enabled: true
			},
			zoom: {
				autoScaleYaxis: true
			}
		},
		colors: getChartColorsArray(chartColors),
		stroke: {
			width: 1,
			curve: 'smooth'
		},
		dataLabels: {
			enabled: false
		}
	};

	onMount(() => {
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#deliveredRate'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="deliveredRate" class="grow apex-charts"></div>
