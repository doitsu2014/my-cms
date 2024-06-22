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
							x: 'Code',
							y: [new Date('2019-03-02').getTime(), new Date('2019-03-04').getTime()]
						},
						{
							x: 'Test',
							y: [new Date('2019-03-04').getTime(), new Date('2019-03-08').getTime()]
						},
						{
							x: 'Validation',
							y: [new Date('2019-03-08').getTime(), new Date('2019-03-12').getTime()]
						},
						{
							x: 'Deployment',
							y: [new Date('2019-03-12').getTime(), new Date('2019-03-18').getTime()]
						}
					]
				}
			],
			chart: {
				height: 350,
				type: 'rangeBar'
			},
			plotOptions: {
				bar: {
					horizontal: true
				}
			},
			colors: getChartColorsArray(chartColors),
			xaxis: {
				type: 'datetime'
			}
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#basicTimelineChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="basicTimelineChart" class="apex-charts"></div>
