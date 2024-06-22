<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	onMount(() => {
		var options = {
			series: [44, 55, 67, 83],
			chart: {
				height: 350,
				type: 'radialBar'
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
								// By default this function returns the average of all series. The below is just an example to show the use of custom formatter function
								return 249;
							}
						}
					}
				}
			},
			colors: getChartColorsArray(chartColors),
			labels: ['Apples', 'Oranges', 'Bananas', 'Berries']
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#multipleRadialbarChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="multipleRadialbarChart" class="apex-charts"></div>
