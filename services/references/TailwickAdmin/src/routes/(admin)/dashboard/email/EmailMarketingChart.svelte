<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	var options = {
		series: [44, 55, 67],
		chart: {
			height: 410,
			type: 'radialBar'
		},
		legend: {
			show: true,
			position: 'bottom',
			horizontalAlign: 'center'
		},
		plotOptions: {
			radialBar: {
				dataLabels: {
					name: {
						fontSize: '22px'
					},
					value: {
						fontSize: '16px'
					},
					total: {
						show: true,
						label: 'Total',
						formatter: function (w) {
							return 249;
						}
					}
				},
				track: {
					margin: 14
				}
			}
		},
		colors: getChartColorsArray(chartColors),
		labels: ['Sent', 'Opened', 'Not Opened']
	};

	onMount(() => {
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#emailMarketingChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="emailMarketingChart" class="apex-charts"></div>
