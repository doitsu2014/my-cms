<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	onMount(() => {
		var options = {
			series: [
				{
					name: 'PRODUCT A',
					data: [44, 55, 41, 67, 22, 43]
				},
				{
					name: 'PRODUCT B',
					data: [13, 23, 20, 8, 13, 27]
				},
				{
					name: 'PRODUCT C',
					data: [11, 17, 15, 15, 21, 14]
				},
				{
					name: 'PRODUCT D',
					data: [21, 7, 25, 13, 22, 8]
				}
			],
			chart: {
				type: 'bar',
				height: 350,
				stacked: true,
				toolbar: {
					show: true
				},
				zoom: {
					enabled: true
				}
			},
			plotOptions: {
				bar: {
					horizontal: false,
					borderRadius: 10,
					dataLabels: {
						total: {
							enabled: true,
							style: {
								fontSize: '13px',
								fontWeight: 900
							}
						}
					}
				}
			},
			xaxis: {
				type: 'datetime',
				categories: [
					'01/01/2011 GMT',
					'01/02/2011 GMT',
					'01/03/2011 GMT',
					'01/04/2011 GMT',
					'01/05/2011 GMT',
					'01/06/2011 GMT'
				]
			},
			colors: getChartColorsArray(chartColors),
			legend: {
				position: 'bottom'
			},
			fill: {
				opacity: 1
			}
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#stackedColumnChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="stackedColumnChart" class="apex-charts"></div>
