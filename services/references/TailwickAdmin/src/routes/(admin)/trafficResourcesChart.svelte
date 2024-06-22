<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	let options = {
		series: [44, 34, 22],
		chart: {
			height: 222,
			type: 'radialBar',
		},
		plotOptions: {
			radialBar: {
				dataLabels: {
					total: {
						show: true,
						label: 'Total',
						formatter: function (w) {
							// By default this function returns the average of all series. The below is just an example to show the use of custom formatter function
							return 875;
						}
					}
				}
			}
		},
		grid: {
			show: true,
			padding: {
				top: -8,
				bottom: -15,
				left: 0,
				right: 0
			}
		},
		colors: getChartColorsArray(chartColors),
		labels: ['Direct', 'Referrals', 'Search Engine']
	};

	onMount(() => {
		var chart = new ApexCharts(document.querySelector('#trafficResourcesChart'), options);
		chart.render();
	});
</script>

<div id="trafficResourcesChart" class="apex-charts"></div>
