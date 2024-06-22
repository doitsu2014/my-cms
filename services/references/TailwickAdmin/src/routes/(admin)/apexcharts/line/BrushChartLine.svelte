<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	// import BrushChartLine2 from './BrushChartLine2.svelte';
	export let chartColors;
	

	function generateDayWiseTimeSeries(baseval, count, yrange) {
		var i = 0;
		var series = [];
		while (i < count) {
			var x = baseval;
			var y = Math.floor(Math.random() * (yrange.max - yrange.min + 1)) + yrange.min;

			series.push([x, y]);
			baseval += 86400000;
			i++;
		}
		return series;
	}

	onMount(() => {
		var data = generateDayWiseTimeSeries(new Date('11 Feb 2017').getTime(), 185, {
			min: 30,
			max: 90
		});

		var options = {
			series: [
				{
					data: data
				}
			],
			chart: {
				// id: 'chart2',
				type: 'line',
				height: 230,
				toolbar: {
					autoSelected: 'pan',
					show: false
				}
			},
			colors: getChartColorsArray(chartColors),
			stroke: {
				width: 3
			},
			dataLabels: {
				enabled: false
			},
			fill: {
				opacity: 1
			},
			markers: {
				size: 0
			},
			xaxis: {
				type: 'datetime'
			}
		};

		// second line Chart
		var options2 = {
			series: [
				{
					data: data
				}
			],
			chart: {
				id: 'chart1',
				height: 130,
				type: 'area',
				brush: {
					target: 'chart2',
					enabled: true
				},
				selection: {
					enabled: true,
					xaxis: {
						min: new Date('19 Jun 2017').getTime(),
						max: new Date('14 Aug 2017').getTime()
					}
				}
			},
			colors: getChartColorsArray(chartColors),
			fill: {
				type: 'gradient',
				gradient: {
					opacityFrom: 0.91,
					opacityTo: 0.1
				}
			},
			xaxis: {
				type: 'datetime',
				tooltip: {
					enabled: false
				}
			},
			yaxis: {
				tickAmount: 2
			}
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#brushChartLine'), options);
			chart.render();
			// var chart2 = new ApexCharts(document.querySelector('#brushChartLine2'), options2);
			// chart2.render();
		}, 100);
	});
</script>

<div id="brushChartLine" class="apex-charts"></div>
<!-- <div id="brushChartLine2" class="apex-charts"></div> -->

<!-- <BrushChartLine2 {data} chartColors={["bg-slate-500"]}/> -->
