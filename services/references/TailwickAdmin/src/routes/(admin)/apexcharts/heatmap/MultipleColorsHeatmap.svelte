<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	import { generateData } from './ChartDataGenerate';
	export let chartColors;

	onMount(() => {
		var data = [
			{
				name: 'W1',
				data: generateData(8, {
					min: 0,
					max: 90
				})
			},
			{
				name: 'W2',
				data: generateData(8, {
					min: 0,
					max: 90
				})
			},
			{
				name: 'W3',
				data: generateData(8, {
					min: 0,
					max: 90
				})
			},
			{
				name: 'W4',
				data: generateData(8, {
					min: 0,
					max: 90
				})
			},
			{
				name: 'W5',
				data: generateData(8, {
					min: 0,
					max: 90
				})
			},
			{
				name: 'W6',
				data: generateData(8, {
					min: 0,
					max: 90
				})
			},
			{
				name: 'W7',
				data: generateData(8, {
					min: 0,
					max: 90
				})
			},
			{
				name: 'W8',
				data: generateData(8, {
					min: 0,
					max: 90
				})
			},
			{
				name: 'W9',
				data: generateData(8, {
					min: 0,
					max: 90
				})
			},
			{
				name: 'W10',
				data: generateData(8, {
					min: 0,
					max: 90
				})
			},
			{
				name: 'W11',
				data: generateData(8, {
					min: 0,
					max: 90
				})
			},
			{
				name: 'W12',
				data: generateData(8, {
					min: 0,
					max: 90
				})
			},
			{
				name: 'W13',
				data: generateData(8, {
					min: 0,
					max: 90
				})
			},
			{
				name: 'W14',
				data: generateData(8, {
					min: 0,
					max: 90
				})
			},
			{
				name: 'W15',
				data: generateData(8, {
					min: 0,
					max: 90
				})
			}
		];

		data.reverse();

		var options = {
			series: data,
			chart: {
				height: 350,
				type: 'heatmap'
			},
			dataLabels: {
				enabled: false
			},
			colors: getChartColorsArray(chartColors),
			_xaxis: {
				type: 'category',
				categories: ['10:00', '10:30', '11:00', '11:30', '12:00', '12:30', '01:00', '01:30']
			},
			get xaxis() {
				return this._xaxis;
			},
			set xaxis(value) {
				this._xaxis = value;
			},
			grid: {
				padding: {
					right: 20
				}
			}
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#multipleColorsHeatmap'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="multipleColorsHeatmap" class="apex-charts"></div>
