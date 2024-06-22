<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	onMount(() => {
		var options = {
			series: [
				{
					name: 'PRODUCT A',
					data: [44, 55, 41, 67, 22, 43, 21, 49]
				},
				{
					name: 'PRODUCT B',
					data: [13, 23, 20, 8, 13, 27, 33, 12]
				},
				{
					name: 'PRODUCT C',
					data: [11, 17, 15, 15, 21, 14, 15, 13]
				}
			],
			chart: {
				type: 'bar',
				height: 350,
				stacked: true,
				stackType: '100%'
			},
			xaxis: {
				categories: [
					'2011 Q1',
					'2011 Q2',
					'2011 Q3',
					'2011 Q4',
					'2012 Q1',
					'2012 Q2',
					'2012 Q3',
					'2012 Q4'
				]
			},
			colors: getChartColorsArray(chartColors),
			fill: {
				opacity: 1
			},
			legend: {
				position: 'bottom'
			}
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#stackedColumn100Chart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="stackedColumn100Chart" class="apex-charts"></div>
