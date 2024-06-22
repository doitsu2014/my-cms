<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	onMount(() => {
		var options = {
			series: [44, 55, 41, 17, 15],
			chart: {
				height: 350,
				type: 'donut',
				dropShadow: {
					enabled: true,
					color: '#111',
					top: -1,
					left: 3,
					blur: 3,
					opacity: 0.2
				}
			},
			stroke: {
				width: 0
			},
			plotOptions: {
				pie: {
					donut: {
						labels: {
							show: true,
							total: {
								showAlways: true,
								show: true
							}
						}
					}
				}
			},
			labels: ['Comedy', 'Action', 'SciFi', 'Drama', 'Horror'],
			dataLabels: {
				dropShadow: {
					blur: 3,
					opacity: 0.8
				}
			},
			fill: {
				type: 'pattern',
				opacity: 1,
				pattern: {
					enabled: true,
					style: ['verticalLines', 'squares', 'horizontalLines', 'circles', 'slantedLines']
				}
			},
			states: {
				hover: {
					filter: 'none'
				}
			},
			colors: getChartColorsArray(chartColors),
			theme: {
				palette: 'palette2'
			},
			legend: {
				position: 'bottom'
			}
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#donutWithPattern'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="donutWithPattern" class="apex-charts"></div>
