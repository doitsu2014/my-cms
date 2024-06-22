<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	onMount(() => {
		var options = {
			series: [
				{
					name: '',
					data: [200, 330, 548, 740, 880, 990, 1100, 1380]
				}
			],
			chart: {
				type: 'bar',
				height: 350
			},
			plotOptions: {
				bar: {
					borderRadius: 0,
					horizontal: true,
					distributed: true,
					barHeight: '80%',
					isFunnel: true
				}
			},
			colors: getChartColorsArray(chartColors),
			dataLabels: {
				enabled: true,
				formatter: function (val, opt) {
					return opt.w.globals.labels[opt.dataPointIndex];
				},
				dropShadow: {
					enabled: true
				}
			},
			xaxis: {
				categories: [
					'Sweets',
					'Processed Foods',
					'Healthy Fats',
					'Meat',
					'Beans & Legumes',
					'Dairy',
					'Fruits & Vegetables',
					'Grains'
				]
			},
			legend: {
				show: false
			}
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#pyramidChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="pyramidChart" class="apex-charts"></div>
