<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	onMount(() => {
		var options = {
			series: [
				{
					name: 'XYZ MOTORS',
					data: [
						{
							x: new Date('2023-01-12').getTime(),
							y: 140
						},
						{
							x: new Date('2023-01-13').getTime(),
							y: 147
						},
						{
							x: new Date('2023-01-14').getTime(),
							y: 150
						},
						{
							x: new Date('2023-01-15').getTime(),
							y: 154
						},
						{
							x: new Date('2023-01-16').getTime(),
							y: 160
						},
						{
							x: new Date('2023-01-17').getTime(),
							y: 165
						},
						{
							x: new Date('2023-01-18').getTime(),
							y: 162
						},
						{
							x: new Date('2023-01-20').getTime(),
							y: 159
						},
						{
							x: new Date('2023-01-21').getTime(),
							y: 164
						},
						{
							x: new Date('2023-01-22').getTime(),
							y: 160
						},
						{
							x: new Date('2023-01-23').getTime(),
							y: 165
						},
						{
							x: new Date('2023-01-24').getTime(),
							y: 169
						},
						{
							x: new Date('2023-01-25').getTime(),
							y: 172
						},
						{
							x: new Date('2023-01-26').getTime(),
							y: 177
						},
						{
							x: new Date('2023-01-27').getTime(),
							y: 173
						},
						{
							x: new Date('2023-01-28').getTime(),
							y: 169
						},
						{
							x: new Date('2023-01-29').getTime(),
							y: 163
						},
						{
							x: new Date('2023-01-30').getTime(),
							y: 158
						},
						{
							x: new Date('2023-02-01').getTime(),
							y: 153
						},
						{
							x: new Date('2023-02-02').getTime(),
							y: 149
						},
						{
							x: new Date('2023-02-03').getTime(),
							y: 144
						},
						{
							x: new Date('2023-02-05').getTime(),
							y: 150
						},
						{
							x: new Date('2023-02-06').getTime(),
							y: 155
						},
						{
							x: new Date('2023-02-07').getTime(),
							y: 159
						},
						{
							x: new Date('2023-02-08').getTime(),
							y: 163
						},
						{
							x: new Date('2023-02-09').getTime(),
							y: 156
						},
						{
							x: new Date('2023-02-11').getTime(),
							y: 151
						},
						{
							x: new Date('2023-02-12').getTime(),
							y: 157
						},
						{
							x: new Date('2023-02-13').getTime(),
							y: 161
						},
						{
							x: new Date('2023-02-14').getTime(),
							y: 150
						},
						{
							x: new Date('2023-02-15').getTime(),
							y: 154
						},
						{
							x: new Date('2023-02-16').getTime(),
							y: 160
						},
						{
							x: new Date('2023-02-17').getTime(),
							y: 165
						},
						{
							x: new Date('2023-02-18').getTime(),
							y: 162
						},
						{
							x: new Date('2023-02-20').getTime(),
							y: 159
						},
						{
							x: new Date('2023-02-21').getTime(),
							y: 164
						},
						{
							x: new Date('2023-02-22').getTime(),
							y: 160
						},
						{
							x: new Date('2023-02-23').getTime(),
							y: 165
						},
						{
							x: new Date('2023-02-24').getTime(),
							y: 169
						},
						{
							x: new Date('2023-02-25').getTime(),
							y: 172
						},
						{
							x: new Date('2023-02-26').getTime(),
							y: 177
						},
						{
							x: new Date('2023-02-27').getTime(),
							y: 173
						},
						{
							x: new Date('2023-02-28').getTime(),
							y: 169
						},
						{
							x: new Date('2023-02-29').getTime(),
							y: 163
						},
						{
							x: new Date('2023-02-30').getTime(),
							y: 162
						},
						{
							x: new Date('2023-03-01').getTime(),
							y: 158
						},
						{
							x: new Date('2023-03-02').getTime(),
							y: 152
						},
						{
							x: new Date('2023-03-03').getTime(),
							y: 147
						},
						{
							x: new Date('2023-03-05').getTime(),
							y: 142
						},
						{
							x: new Date('2023-03-06').getTime(),
							y: 147
						},
						{
							x: new Date('2023-03-07').getTime(),
							y: 151
						},
						{
							x: new Date('2023-03-08').getTime(),
							y: 155
						},
						{
							x: new Date('2023-03-09').getTime(),
							y: 159
						},
						{
							x: new Date('2023-03-11').getTime(),
							y: 162
						},
						{
							x: new Date('2023-03-12').getTime(),
							y: 157
						},
						{
							x: new Date('2023-03-13').getTime(),
							y: 161
						},
						{
							x: new Date('2023-03-14').getTime(),
							y: 166
						},
						{
							x: new Date('2023-03-15').getTime(),
							y: 169
						},
						{
							x: new Date('2023-03-16').getTime(),
							y: 172
						},
						{
							x: new Date('2023-03-17').getTime(),
							y: 177
						},
						{
							x: new Date('2023-03-18').getTime(),
							y: 181
						},
						{
							x: new Date('2023-03-20').getTime(),
							y: 178
						},
						{
							x: new Date('2023-03-21').getTime(),
							y: 173
						},
						{
							x: new Date('2023-03-22').getTime(),
							y: 169
						},
						{
							x: new Date('2023-03-23').getTime(),
							y: 163
						},
						{
							x: new Date('2023-03-24').getTime(),
							y: 159
						},
						{
							x: new Date('2023-03-25').getTime(),
							y: 164
						},
						{
							x: new Date('2023-03-26').getTime(),
							y: 168
						},
						{
							x: new Date('2023-03-27').getTime(),
							y: 172
						},
						{
							x: new Date('2023-03-28').getTime(),
							y: 169
						},
						{
							x: new Date('2023-03-29').getTime(),
							y: 163
						},
						{
							x: new Date('2023-03-30').getTime(),
							y: 162
						},
						{
							x: new Date('2023-04-01').getTime(),
							y: 158
						},
						{
							x: new Date('2023-04-02').getTime(),
							y: 152
						},
						{
							x: new Date('2023-04-03').getTime(),
							y: 147
						},
						{
							x: new Date('2023-04-05').getTime(),
							y: 142
						},
						{
							x: new Date('2023-04-06').getTime(),
							y: 147
						},
						{
							x: new Date('2023-04-07').getTime(),
							y: 151
						},
						{
							x: new Date('2023-04-08').getTime(),
							y: 155
						},
						{
							x: new Date('2023-04-09').getTime(),
							y: 159
						},
						{
							x: new Date('2023-04-11').getTime(),
							y: 162
						},
						{
							x: new Date('2023-04-12').getTime(),
							y: 157
						},
						{
							x: new Date('2023-04-13').getTime(),
							y: 161
						},
						{
							x: new Date('2023-04-14').getTime(),
							y: 166
						},
						{
							x: new Date('2023-04-15').getTime(),
							y: 169
						},
						{
							x: new Date('2023-04-16').getTime(),
							y: 172
						},
						{
							x: new Date('2023-04-17').getTime(),
							y: 177
						},
						{
							x: new Date('2023-04-18').getTime(),
							y: 181
						},
						{
							x: new Date('2023-04-20').getTime(),
							y: 178
						},
						{
							x: new Date('2023-04-21').getTime(),
							y: 173
						},
						{
							x: new Date('2023-04-22').getTime(),
							y: 169
						},
						{
							x: new Date('2023-04-23').getTime(),
							y: 163
						},
						{
							x: new Date('2023-04-24').getTime(),
							y: 159
						},
						{
							x: new Date('2023-04-25').getTime(),
							y: 164
						},
						{
							x: new Date('2023-04-26').getTime(),
							y: 168
						},
						{
							x: new Date('2023-04-27').getTime(),
							y: 172
						},
						{
							x: new Date('2023-04-28').getTime(),
							y: 169
						},
						{
							x: new Date('2023-04-29').getTime(),
							y: 163
						},
						{
							x: new Date('2023-04-30').getTime(),
							y: 162
						},
						{
							x: new Date('2023-05-01').getTime(),
							y: 158
						},
						{
							x: new Date('2023-05-02').getTime(),
							y: 152
						},
						{
							x: new Date('2023-05-03').getTime(),
							y: 147
						},
						{
							x: new Date('2023-05-04').getTime(),
							y: 142
						},
						{
							x: new Date('2023-05-05').getTime(),
							y: 147
						},
						{
							x: new Date('2023-05-07').getTime(),
							y: 151
						},
						{
							x: new Date('2023-05-08').getTime(),
							y: 155
						},
						{
							x: new Date('2023-05-09').getTime(),
							y: 159
						},
						{
							x: new Date('2023-05-11').getTime(),
							y: 162
						},
						{
							x: new Date('2023-05-12').getTime(),
							y: 157
						},
						{
							x: new Date('2023-05-13').getTime(),
							y: 161
						},
						{
							x: new Date('2023-05-14').getTime(),
							y: 166
						},
						{
							x: new Date('2023-05-15').getTime(),
							y: 169
						},
						{
							x: new Date('2023-05-16').getTime(),
							y: 172
						},
						{
							x: new Date('2023-05-17').getTime(),
							y: 177
						},
						{
							x: new Date('2023-05-18').getTime(),
							y: 181
						},
						{
							x: new Date('2023-05-20').getTime(),
							y: 178
						},
						{
							x: new Date('2023-05-21').getTime(),
							y: 173
						},
						{
							x: new Date('2023-05-22').getTime(),
							y: 169
						},
						{
							x: new Date('2023-05-23').getTime(),
							y: 163
						},
						{
							x: new Date('2023-05-24').getTime(),
							y: 159
						},
						{
							x: new Date('2023-05-25').getTime(),
							y: 164
						},
						{
							x: new Date('2023-05-26').getTime(),
							y: 168
						},
						{
							x: new Date('2023-05-27').getTime(),
							y: 172
						},
						{
							x: new Date('2023-05-28').getTime(),
							y: 169
						},
						{
							x: new Date('2023-05-29').getTime(),
							y: 163
						},
						{
							x: new Date('2023-05-30').getTime(),
							y: 162
						}
					]
				}
			],
			chart: {
				type: 'area',
				stacked: false,
				height: 350,
				zoom: {
					type: 'x',
					enabled: true,
					autoScaleYaxis: true
				},
				toolbar: {
					autoSelected: 'zoom'
				}
			},
			dataLabels: {
				enabled: false
			},
			markers: {
				size: 0
			},
			title: {
				text: 'Stock Price Movement',
				align: 'left'
			},
			colors: getChartColorsArray(chartColors),
			fill: {
				type: 'gradient',
				gradient: {
					shadeIntensity: 1,
					inverseColors: false,
					opacityFrom: 0.5,
					opacityTo: 0,
					stops: [0, 90, 100]
				}
			},
			yaxis: {
				labels: {
					formatter: function (val) {
						return (val / 1000000).toFixed(0);
					}
				},
				title: {
					text: 'Price'
				}
			},
			xaxis: {
				type: 'datetime'
			},
			tooltip: {
				shared: false,
				y: {
					formatter: function (val) {
						return (val / 1000000).toFixed(0);
					}
				}
			}
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#zoomableTimeseriesChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="zoomableTimeseriesChart" class="apex-charts"></div>