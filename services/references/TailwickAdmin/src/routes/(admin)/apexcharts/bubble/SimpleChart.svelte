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
					name: 'Bubble1',
					data: generateData(new Date('11 Feb 2017 GMT').getTime(), 20, {
						min: 10,
						max: 60
					})
				},
				{
					name: 'Bubble2',
					data: generateData(new Date('11 Feb 2017 GMT').getTime(), 20, {
						min: 10,
						max: 60
					})
				},
				{
					name: 'Bubble3',
					data: generateData(new Date('11 Feb 2017 GMT').getTime(), 20, {
						min: 10,
						max: 60
					})
				},
				{
					name: 'Bubble4',
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
			colors: getChartColorsArray(chartColors),
			fill: {
				opacity: 0.8
			},
			xaxis: {
				tickAmount: 12,
				type: 'category'
			},
			yaxis: {
				max: 70
			}
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#simpleChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="simpleChart" class="apex-charts"></div>
