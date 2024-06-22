<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	let dailyVisitInsightsChartColors = getChartColorsArray(chartColors);

	onMount(() => {
		setTimeout(() => {
			let options = {
				series: [
					{
						name: 'Male',
						data: [55, 41, 67, 22, 43, 21, 33, 45]
					},
					{
						name: 'Female',
						data: [55, 41, 67, 22, 43, 21, 33, 45]
					}
				],
				annotations: {
					points: [
						{
							x: 'Bananas',
							seriesIndex: 0,
							label: {
								borderColor: dailyVisitInsightsChartColors[1],
								offsetY: 0,
								style: {
									color: '#fff',
									background: dailyVisitInsightsChartColors[1]
								},
								text: 'Bananas are good'
							}
						}
					]
				},
				colors: dailyVisitInsightsChartColors,
				chart: {
					height: 238,
					type: 'bar',
					toolbar: {
						show: false
					}
				},
				plotOptions: {
					bar: {
						borderRadius: 10,
						columnWidth: '70%'
					}
				},
				dataLabels: {
					enabled: false
				},
				stroke: {
					width: 2
				},
				xaxis: {
					labels: {
						rotate: -45
					},
					categories: ['May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'],
					tickPlacement: 'on'
				},
				fill: {
					type: 'gradient',
					gradient: {
						shade: 'light',
						type: 'horizontal',
						shadeIntensity: 0.25,
						gradientToColors: undefined,
						inverseColors: true,
						opacityFrom: 0.85,
						opacityTo: 0.85,
						stops: [50, 0, 100]
					}
				},
				grid: {
					xaxis: {
						lines: {
							show: true
						}
					},
					yaxis: {
						lines: {
							show: true
						}
					},
					padding: {
						top: -10,
						right: -10,
						left: -10
					}
				},
				yaxis: {
					show: false
				}
			};

			var chart = new ApexCharts(document.querySelector('#dailyVisitInsightsChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="dailyVisitInsightsChart" class="apex-charts" dir="ltr"></div>
