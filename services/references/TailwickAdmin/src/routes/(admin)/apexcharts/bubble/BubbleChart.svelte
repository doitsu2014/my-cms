<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	function generateData(baseval, count, yrange) {
		var i = 0;
		var series = [];
		while (i < count) {
			var x = Math.floor(Math.random() * (750 - 1 + 1)) + 1;
			var y = Math.floor(Math.random() * (yrange.max - yrange.min + 1)) + yrange.min;
			var z = Math.floor(Math.random() * (75 - 15 + 1)) + 15;

			series.push([x, y, z]);
			baseval += 86400000;
			i++;
		}
		return series;
	}

	onMount(() => {
		var options = {
			series: [
				{
					name: 'Product1',
					data: generateData(new Date('11 Feb 2017 GMT').getTime(), 20, {
						min: 10,
						max: 60
					})
				},
				{
					name: 'Product2',
					data: generateData(new Date('11 Feb 2017 GMT').getTime(), 20, {
						min: 10,
						max: 60
					})
				},
				{
					name: 'Product3',
					data: generateData(new Date('11 Feb 2017 GMT').getTime(), 20, {
						min: 10,
						max: 60
					})
				},
				{
					name: 'Product4',
					data: generateData(new Date('11 Feb 2017 GMT').getTime(), 20, {
						min: 10,
						max: 60
					})
				}
			],
			chart: {
				height: 350,
				type: 'bubble'
			},
			dataLabels: {
				enabled: false
			},
			fill: {
				type: 'gradient'
			},
			colors: getChartColorsArray(chartColors),
			xaxis: {
				tickAmount: 12,
				type: 'datetime',
				labels: {
					rotate: 0
				}
			},
			yaxis: {
				max: 70
			},
			theme: {
				palette: 'palette2'
			}
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#bubbleChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="bubbleChart" class="apex-charts"></div>
