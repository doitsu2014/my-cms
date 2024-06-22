<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	onMount(() => {
		var options = {
			series: [
				{
					name: 'High - 2013',
					data: [28, 29, 33, 36, 32, 32, 33]
				},
				{
					name: 'Low - 2013',
					data: [12, 11, 14, 18, 17, 13, 13]
				}
			],
			chart: {
				height: 350,
				type: 'line',
				dropShadow: {
					enabled: true,
					color: '#000',
					top: 18,
					left: 7,
					blur: 10,
					opacity: 0.2
				},
				toolbar: {
					show: false
				}
			},
			colors: getChartColorsArray(chartColors),
			dataLabels: {
				enabled: true
			},
			stroke: {
				curve: 'smooth'
			},
			markers: {
				size: 1
			},
			xaxis: {
				categories: ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul'],
				title: {
					text: 'Month'
				}
			},
			yaxis: {
				title: {
					text: 'Temperature'
				},
				min: 5,
				max: 40
			},
			legend: {
				position: 'top',
				horizontalAlign: 'right',
				floating: true,
				offsetY: -25,
				offsetX: -5
			}
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#lineWithDataLabelChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="lineWithDataLabelChart" class="apex-charts"></div>
