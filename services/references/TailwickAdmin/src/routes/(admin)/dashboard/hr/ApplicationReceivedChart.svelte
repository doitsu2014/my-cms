<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	var options = {
		series: [
			{
				name: 'Total Application',
				type: 'area',
				data: [44, 55, 41, 67, 22, 43, 21, 41, 56, 27, 43]
			},
			{
				name: 'Hired Candidates',
				type: 'line',
				data: [30, 25, 36, 30, 45, 35, 64, 52, 59, 36, 39]
			}
		],
		chart: {
			height: 300,
			type: 'line',
			stacked: false,
			margin: {
				left: 0,
				right: 0,
				top: 0,
				bottom: 0
			},
			toolbar: {
				show: false
			}
		},
		stroke: {
			width: [2, 2],
			curve: 'smooth'
		},
		plotOptions: {
			bar: {
				columnWidth: '50%'
			}
		},

		fill: {
			opacity: [0.03, 1],
			gradient: {
				inverseColors: false,
				shade: 'light',
				type: 'vertical',
				opacityFrom: 0.85,
				opacityTo: 0.55,
				stops: [0, 100, 100, 100]
			}
		},
		labels: ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'],
		colors: getChartColorsArray(chartColors),
		markers: {
			size: 0
		},
		grid: {
			padding: {
				top: -15,
				right: 0
			}
		},
		tooltip: {
			shared: true,
			intersect: false,
			y: {
				formatter: function (y) {
					if (typeof y !== 'undefined') {
						return y.toFixed(0) + ' points';
					}
					return y;
				}
			}
		}
	};

	onMount(() => {
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#applicationReceivedChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="applicationReceivedChart" class="apex-charts"></div>
