<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	function generateDayWiseTimeSeries(baseval, count, yrange) {
		var i = 0;
		var series = [];
		while (i < count) {
			var y = Math.floor(Math.random() * (yrange.max - yrange.min + 1)) + yrange.min;

			series.push([baseval, y]);
			baseval += 86400000;
			i++;
		}
		return series;
	}

	onMount(() => {
		var options = {
			series: [
				{
					name: 'TEAM 1',
					data: generateDayWiseTimeSeries(new Date('11 Feb 2017 GMT').getTime(), 20, {
						min: 10,
						max: 60
					})
				},
				{
					name: 'TEAM 2',
					data: generateDayWiseTimeSeries(new Date('11 Feb 2017 GMT').getTime(), 20, {
						min: 10,
						max: 60
					})
				},
				{
					name: 'TEAM 3',
					data: generateDayWiseTimeSeries(new Date('11 Feb 2017 GMT').getTime(), 30, {
						min: 10,
						max: 60
					})
				},
				{
					name: 'TEAM 4',
					data: generateDayWiseTimeSeries(new Date('11 Feb 2017 GMT').getTime(), 10, {
						min: 10,
						max: 60
					})
				},
				{
					name: 'TEAM 5',
					data: generateDayWiseTimeSeries(new Date('11 Feb 2017 GMT').getTime(), 30, {
						min: 10,
						max: 60
					})
				}
			],
			chart: {
				height: 350,
				type: 'scatter',
				zoom: {
					type: 'xy'
				}
			},
			dataLabels: {
				enabled: false
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
				}
			},
			xaxis: {
				type: 'datetime'
			},
			colors: getChartColorsArray(chartColors),
			yaxis: {
				max: 70
			}
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#scatterDatetimeChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="scatterDatetimeChart" class="apex-charts"></div>
