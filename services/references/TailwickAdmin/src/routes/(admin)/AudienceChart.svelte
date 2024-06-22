<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../common/components/ChartColorsArray.svelte';
	let chartColors = ['bg-sky-500', 'bg-orange-400', 'bg-green-500', 'bg-yellow-500'];

	let options = {
		series: [
			{
				name: 'Male',
				data: [44, 55, 41, 67, 22, 43, 26]
			},
			{
				name: 'Female',
				data: [13, 23, 20, 8, 13, 27, 41]
			}
		],
		chart: {
			type: 'bar',
			height: 390,
			stacked: true,
			toolbar: {
				show: false
			},
			zoom: {
				enabled: true
			}
		},
		plotOptions: {
			bar: {
				horizontal: false,
				borderRadius: 6,
				columnWidth: '44%',
				dataLabels: {
					total: {
						enabled: true,
						style: {
							fontSize: '13px',
							fontWeight: 600
						}
					}
				}
			}
		},
		xaxis: {
			type: 'datetime',
			categories: [
				'01/01/2023 GMT',
				'01/02/2023 GMT',
				'01/03/2023 GMT',
				'01/04/2023 GMT',
				'01/05/2023 GMT',
				'01/06/2023 GMT',
				'01/07/2023 GMT'
			]
		},
		colors: getChartColorsArray(chartColors),
		legend: {
			position: 'top',
			horizontalAlign: 'right'
		},
		fill: {
			opacity: 1
		}
	};

	onMount(() => {
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#audienceChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div class="col-span-12 card lg:col-span-6 2xl:col-span-3">
	<div class="card-body">
		<h6 class="relative mb-3 grow text-15">Audience</h6>
		<div id="audienceChart" class="-mt-9 apex-charts" dir="ltr"></div>
	</div>
</div>
