<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	onMount(() => {
		var options = {
			series: [25, 15, 44, 55, 41, 17],
			chart: {
				height: 300,
				width: '100%',
				type: 'pie'
			},
			labels: ['Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday'],
			theme: {
				monochrome: {
					enabled: true
				}
			},
			plotOptions: {
				pie: {
					dataLabels: {
						offset: -5
					}
				}
			},
			dataLabels: {
				formatter(val, opts) {
					const name = opts.w.globals.labels[opts.seriesIndex];
					return [name, val.toFixed(1) + '%'];
				}
			},
			theme: {
				monochrome: {
					enabled: true,
					color: getChartColorsArray(chartColors)[0],
					shadeTo: 'light',
					shadeIntensity: 0.6
				}
			},
			legend: {
				show: false
			}
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#monochromePieChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="monochromePieChart" class="apex-charts"></div>
