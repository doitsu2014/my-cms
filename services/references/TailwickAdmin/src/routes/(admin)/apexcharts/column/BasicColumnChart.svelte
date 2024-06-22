<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	onMount(() => {
		
		var options = {
			series: [
				{
					name: 'Net Profit',
					data: [44, 55, 57, 56, 61, 58, 63, 60, 66]
				},
				{
					name: 'Revenue',
					data: [76, 85, 101, 98, 87, 105, 91, 114, 94]
				},
				{
					name: 'Free Cash Flow',
					data: [35, 41, 36, 26, 45, 48, 52, 53, 41]
				}
			],
			chart: {
				type: 'bar',
				height: 350
			},
			plotOptions: {
				bar: {
					horizontal: false,
					columnWidth: '55%',
					endingShape: 'rounded'
				}
			},
			dataLabels: {
				enabled: false
			},
			colors: getChartColorsArray(chartColors),
			stroke: {
				show: true,
				width: 2,
				colors: ['transparent']
			},
			xaxis: {
				categories: ['Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct']
			},
			yaxis: {
				title: {
					text: '$ (thousands)'
				}
			},
			fill: {
				opacity: 1
			},
			tooltip: {
				y: {
					formatter: function (val) {
						return '$ ' + val + ' thousands';
					}
				}
			}
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#basicColumnChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="basicColumnChart" class="apex-charts"></div>
