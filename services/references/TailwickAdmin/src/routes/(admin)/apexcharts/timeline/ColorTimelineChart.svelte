<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	import moment from "moment";
	export let chartColors;

	onMount(() => {
		var options = {
			series: [
				{
					data: [
						{
							x: 'Analysis',
							y: [new Date('2019-02-27').getTime(), new Date('2019-03-04').getTime()]
						},
						{
							x: 'Design',
							y: [new Date('2019-03-04').getTime(), new Date('2019-03-08').getTime()]
						},
						{
							x: 'Coding',
							y: [new Date('2019-03-07').getTime(), new Date('2019-03-10').getTime()]
						},
						{
							x: 'Testing',
							y: [new Date('2019-03-08').getTime(), new Date('2019-03-12').getTime()]
						},
						{
							x: 'Deployment',
							y: [new Date('2019-03-12').getTime(), new Date('2019-03-17').getTime()]
						}
					]
				}
			],
			chart: {
				height: 350,
				type: 'rangeBar'
			},
			colors: getChartColorsArray(chartColors),
			plotOptions: {
				bar: {
					horizontal: true,
					distributed: true,
					dataLabels: {
						hideOverflowingLabels: false
					}
				}
			},
			dataLabels: {
				enabled: true,
				formatter: function (val, opts) {
					var label = opts.w.globals.labels[opts.dataPointIndex];
					var a = moment(val[0]);
					var b = moment(val[1]);
					var diff = b.diff(a, 'days');
					return label + ': ' + diff + (diff > 1 ? ' days' : ' day');
				}
			},
			xaxis: {
				type: 'datetime'
			},
			yaxis: {
				show: false
			}
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#colorTimelineChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="colorTimelineChart" class="apex-charts"></div>
