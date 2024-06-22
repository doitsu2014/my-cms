<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	onMount(() => {
		var options = {
			series: [
				{
					name: 'Funnel Series',
					data: [1380, 1100, 990, 880, 740, 548, 330, 200]
				}
			],
			chart: {
				type: 'bar',
				height: 350
			},
			plotOptions: {
				bar: {
					borderRadius: 0,
					horizontal: true,
					barHeight: '80%',
					isFunnel: true
				}
			},
			colors: getChartColorsArray(chartColors),
			dataLabels: {
				enabled: true,
				formatter: function (val, opt) {
					return opt.w.globals.labels[opt.dataPointIndex] + ':  ' + val;
				},
				dropShadow: {
					enabled: true
				}
			},
			xaxis: {
				categories: [
					'Sourced',
					'Screened',
					'Assessed',
					'HR Interview',
					'Technical',
					'Verify',
					'Offered',
					'Hired'
				]
			},
			legend: {
				show: false
			}
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#basicFunnelChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="basicFunnelChart" class="apex-charts"></div>
