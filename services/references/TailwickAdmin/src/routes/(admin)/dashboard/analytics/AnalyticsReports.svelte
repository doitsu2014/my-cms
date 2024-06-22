<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;


	onMount(() => {
		setTimeout(() => {
			let options = {
				series: [
					{
						name: 'Income - 2023',
						data: [28, 29, 33, 36, 32]
					},
					{
						name: 'Expense - 2023',
						data: [20, 17, 21, 29, 23]
					}
				],
				chart: {
					height: 235,
					type: 'line',
					dropShadow: {
						enabled: true,
						color: '#000',
						top: 18,
						left: 7,
						blur: 10,
						opacity: 0.2
					},
					toolbar: {
						show: false
					}
				},
				colors: getChartColorsArray(chartColors),
				dataLabels: {
					enabled: true
				},
				stroke: {
					curve: 'smooth',
					size: 2
				},
				markers: {
					size: 1
				},
				yaxis: {
					show: false
				},
				xaxis: {
					categories: ['Mar', 'Apr', 'May', 'Jun', 'Jul']
				},
				legend: {
					position: 'top',
					horizontalAlign: 'right',
					floating: true,
					offsetY: -25,
					offsetX: -5
				},
				grid: {
					xaxis: {
						lines: {
							show: false
						}
					},
					yaxis: {
						lines: {
							show: false
						}
					},
					padding: {
						top: -25
					}
				}
			};

			var chart = new ApexCharts(document.querySelector('#lineWithDataLabel'), options);
			chart.render();
		}, 100);
	});
</script>

<div class="order-12 col-span-12 lg:col-span-6 2xl:order-1 card 2xl:col-span-3">
	<div class="card-body">
		<h6 class="mb-3 text-15">Analytics Reports</h6>
		<div id="lineWithDataLabel" class="apex-charts" dir="ltr"></div>
	</div>
</div>
