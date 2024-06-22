<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	onMount(() => {
		var options = {
			series: [
				{
					data: [21, 22, 10, 28, 16, 21, 13, 30]
				}
			],
			chart: {
				height: 350,
				type: 'bar',
				events: {
					click: function (chart, w, e) {
						// console.log(chart, w, e)
					}
				}
			},
			colors: getChartColorsArray(chartColors),
			plotOptions: {
				bar: {
					columnWidth: '45%',
					distributed: true
				}
			},
			dataLabels: {
				enabled: false
			},
			legend: {
				show: false
			},
			xaxis: {
				categories: [
					['John', 'Doe'],
					['Joe', 'Smith'],
					['Jake', 'Williams'],
					'Amber',
					['Peter', 'Brown'],
					['Mary', 'Evans'],
					['David', 'Wilson'],
					['Lily', 'Roberts']
				],
				labels: {
					style: {
						colors: getChartColorsArray(chartColors),
						fontSize: '12px'
					}
				}
			}
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#columnDistributedChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="columnDistributedChart" class="apex-charts"></div>
