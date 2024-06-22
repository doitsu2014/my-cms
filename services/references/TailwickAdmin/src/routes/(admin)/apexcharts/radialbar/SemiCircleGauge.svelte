<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	onMount(() => {
		var options = {
			series: [76],
			chart: {
				type: 'radialBar',
				height: 350,
				offsetY: -20,
				sparkline: {
					enabled: true
				}
			},
			plotOptions: {
				radialBar: {
					startAngle: -90,
					endAngle: 90,
					track: {
						strokeWidth: '97%',
						margin: 5 // margin is in pixels
					},
					dataLabels: {
						name: {
							show: false
						},
						value: {
							offsetY: -2,
							fontSize: '22px'
						}
					}
				}
			},
			grid: {
				padding: {
					top: -10
				}
			},
			colors: getChartColorsArray(chartColors),
			fill: {
				type: 'gradient',
				gradient: {
					shade: 'light',
					shadeIntensity: 0.4,
					inverseColors: false,
					opacityFrom: 1,
					opacityTo: 1,
					stops: [0, 50, 53, 91]
				}
			},
			labels: ['Average Results']
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#semiCircleGauge'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="semiCircleGauge" class="apex-charts"></div>
