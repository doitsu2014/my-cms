<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	onMount(() => {
		var options = {
			series: [67],
			chart: {
				height: 350,
				type: 'radialBar',
				offsetY: -10
			},
			plotOptions: {
				radialBar: {
					startAngle: -135,
					endAngle: 135,
					dataLabels: {
						name: {
							fontSize: '16px',
							color: undefined,
							offsetY: 120
						},
						value: {
							offsetY: 76,
							fontSize: '22px',
							color: undefined,
							formatter: function (val) {
								return val + '%';
							}
						}
					}
				}
			},
			colors: getChartColorsArray(chartColors),
			fill: {
				type: 'gradient',
				gradient: {
					shade: 'dark',
					shadeIntensity: 0.15,
					inverseColors: false,
					opacityFrom: 1,
					opacityTo: 1,
					stops: [0, 50, 65, 91]
				}
			},
			stroke: {
				dashArray: 4
			},
			labels: ['Median Ratio']
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#strokedGauge'), options);
			chart.render();
		}, 100);
	});
</script>


<div id="strokedGauge" class="apex-charts"></div>
