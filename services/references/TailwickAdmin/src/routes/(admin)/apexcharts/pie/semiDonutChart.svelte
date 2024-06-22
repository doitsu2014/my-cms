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
					endAngle: 90,
					offsetY: 5
				}
			},
			grid: {
				padding: {
					bottom: -80
				}
			},
			colors: getChartColorsArray(chartColors),
			legend: {
				position: 'bottom'
			}
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#semiDonut'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="semiDonut" class="apex-charts"></div>
