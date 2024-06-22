<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	onMount(() => {
		var options = {
			series: [
				{
					name: 'Desktops',
					data: [10, 41, 35, 51, 49, 62, 69, 91, 148]
				}
			],
			chart: {
				height: 350,
				type: 'line',
				zoom: {
					enabled: false
				},
				margin: {
					left: 0,
					right: 0,
					top: 0,
					bottom: 0
				}
			},
			dataLabels: {
				enabled: false
			},
			stroke: {
				curve: 'straight'
			},
			title: {
				text: 'Product Trends by Month',
				align: 'left'
			},
			colors: getChartColorsArray(chartColors),
			xaxis: {
				categories: ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep']
			}
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#chartLine'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="chartLine" class="apex-charts"></div>
