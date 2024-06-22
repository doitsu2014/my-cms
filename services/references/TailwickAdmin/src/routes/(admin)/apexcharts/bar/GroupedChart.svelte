<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	var options = {
		series: [
			{
				data: [44, 55, 41, 64, 22, 43, 21]
			},
			{
				data: [53, 32, 33, 52, 13, 44, 32]
			}
		],
		chart: {
			type: 'bar',
			height: 350
		},
		plotOptions: {
			bar: {
				horizontal: true,
				dataLabels: {
					position: 'top'
				}
			}
		},
		dataLabels: {
			enabled: true,
			offsetX: -6,
			style: {
				fontSize: '12px',
				colors: ['#fff']
			}
		},
		colors: getChartColorsArray(chartColors),
		stroke: {
			show: true,
			width: 1,
			colors: ['#fff']
		},
		tooltip: {
			shared: true,
			intersect: false
		},
		xaxis: {
			categories: [2001, 2002, 2003, 2004, 2005, 2006, 2007]
		}
	};

	onMount(() => {
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#groupedChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="groupedChart" class="apex-charts"></div>
