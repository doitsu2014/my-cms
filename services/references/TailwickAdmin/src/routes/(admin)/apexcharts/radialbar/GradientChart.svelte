<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	onMount(() => {
		var options = {
			series: [75],
			chart: {
				height: 350,
				type: 'radialBar',
				toolbar: {
					show: true
				}
			},
			colors: getChartColorsArray(chartColors),
			plotOptions: {
				radialBar: {
					startAngle: -135,
					endAngle: 225,
					hollow: {
						margin: 0,
						size: '70%',
						image: undefined,
						imageOffsetX: 0,
						imageOffsetY: 0,
						position: 'front'
					},
					track: {
						strokeWidth: '67%',
						margin: 0 // margin is in pixels
					},

					dataLabels: {
						show: true,
						name: {
							offsetY: -10,
							show: true,
							fontSize: '17px'
						},
						value: {
							formatter: function (val) {
								return parseInt(val);
							},
							fontSize: '36px',
							show: true
						}
					}
				}
			},
			fill: {
				type: 'gradient',
				gradient: {
					shade: 'dark',
					type: 'horizontal',
					shadeIntensity: 0.5,
					gradientToColors: [getChartColorsArray(chartColors)[1]],
					inverseColors: true,
					opacityFrom: 1,
					opacityTo: 1,
					stops: [0, 100]
				}
			},
			stroke: {
				lineCap: 'round'
			},
			labels: ['Percent']
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#gradientChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="gradientChart" class="apex-charts"></div>
