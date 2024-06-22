<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	onMount(() => {
		var options = {
			series: [
				{
					data: [34, 44, 54, 21, 12, 43, 33, 23, 66, 66, 58]
				}
			],
			chart: {
				type: 'line',
				height: 350
			},
			stroke: {
				curve: 'stepline'
			},
			colors: getChartColorsArray(chartColors),
			dataLabels: {
				enabled: false
			},
			title: {
				text: 'Stepline Chart',
				align: 'left'
			},
			markers: {
				hover: {
					sizeOffset: 4
				}
			}
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#chartStepline'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="chartStepline" class="apex-charts"></div>
