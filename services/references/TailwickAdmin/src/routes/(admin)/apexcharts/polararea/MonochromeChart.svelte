<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	onMount(() => {
		var options = {
			series: [42, 47, 52, 58, 65],
			chart: {
				height: 350,
				type: 'polarArea'
			},
			labels: ['Rose A', 'Rose B', 'Rose C', 'Rose D', 'Rose E'],
			fill: {
				opacity: 1
			},
			stroke: {
				width: 1,
				colors: undefined
			},
			yaxis: {
				show: false
			},
			legend: {
				position: 'bottom'
			},
			plotOptions: {
				polarArea: {
					rings: {
						strokeWidth: 0
					},
					spokes: {
						strokeWidth: 0
					}
				}
			},
			theme: {
				monochrome: {
					enabled: true,
					color: getChartColorsArray(chartColors)[0],
					shadeTo: 'light',
					shadeIntensity: 0.6
				}
			}
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#monochromeChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="monochromeChart" class="apex-charts"></div>
