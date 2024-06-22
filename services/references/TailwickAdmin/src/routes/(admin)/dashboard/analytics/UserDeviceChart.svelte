<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	let options = {
		series: [
			{
				name: 'Desktop',
				data: [80, 50, 30, 40, 100, 20]
			},
			{
				name: 'Mobile',
				data: [20, 30, 40, 80, 20, 80]
			},
			{
				name: 'Others',
				data: [44, 76, 78, 13, 43, 10]
			}
		],
		chart: {
			height: 240,
			type: 'radar',
			dropShadow: {
				enabled: true,
				blur: 1,
				left: 1,
				top: 1
			},
			toolbar: {
				show: false
			}
		},
		stroke: {
			width: 2
		},
		fill: {
			opacity: 0.1
		},
		markers: {
			size: 0
		},
		responsive: [
			{
				breakpoint: 480,
				options: {
					legend: {
						show: false
					}
				}
			}
		],
		colors: getChartColorsArray(chartColors),
		legend: {
			position: 'right',
			offsetY: 0,
			height: 230
		},
		xaxis: {
			categories: ['2018', '2019', '2020', '2021', '2022', '2023']
		}
	};

	onMount(() => {
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#userDeviceCharts'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="userDeviceCharts" class="apex-charts" dir="ltr"></div>
