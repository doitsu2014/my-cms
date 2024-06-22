<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;
	var chart;
	var options;

	onMount(() => {
		 options = {
			series: [44, 55, 13, 33],
			chart: {
				width: 380,
				type: 'donut'
			},
			dataLabels: {
				enabled: false
			},
			responsive: [
				{
					breakpoint: 480,
					options: {
						chart: {
							width: 200
						},
						legend: {
							show: false
						}
					}
				}
			],
			colors: getChartColorsArray(chartColors),
			legend: {
				position: 'right',
				offsetY: 0,
				height: 230
			}
		};

		setTimeout(() => {
			chart = new ApexCharts(document.querySelector('#updatingDonutChart'), options);
			chart.render();
		}, 100);
	});

	function appendData() {
		var arr = chart.w.globals.series.slice();
		arr.push(Math.floor(Math.random() * (100 - 1 + 1)) + 1);
		chart.updateSeries(arr)
	}

	function removeData() {
		var arr = chart.w.globals.series.slice();
		arr.pop();
		chart.updateSeries(arr)
	}

	function randomize() {
		chart.updateSeries(chart.w.globals.series.map(function () {
			return Math.floor(Math.random() * (100 - 1 + 1)) + 1;
		})
		)
	}

	function reset() {
		chart.updateSeries(options.series);
	}
</script>

<div id="updatingDonutChart" class="apex-charts"></div>

<div class="flex flex-wrap items-start justify-center gap-2 mt-4">
	<button
		id="add"
		class="px-2 py-1.5 text-xs bg-white border-dashed text-custom-500 btn border-custom-500 hover:text-custom-500 hover:bg-custom-50 hover:border-custom-600 focus:text-custom-600 focus:bg-custom-50 focus:border-custom-600 active:text-custom-600 active:bg-custom-50 active:border-custom-600 dark:bg-zink-700 dark:ring-custom-400/20 dark:hover:bg-custom-800/20 dark:focus:bg-custom-800/20 dark:active:bg-custom-800/20"
		on:click={appendData}
	>
		+ ADD
	</button>

	<button
		id="remove"
		class="px-2 py-1.5 text-xs bg-white border-dashed text-custom-500 btn border-custom-500 hover:text-custom-500 hover:bg-custom-50 hover:border-custom-600 focus:text-custom-600 focus:bg-custom-50 focus:border-custom-600 active:text-custom-600 active:bg-custom-50 active:border-custom-600 dark:bg-zink-700 dark:ring-custom-400/20 dark:hover:bg-custom-800/20 dark:focus:bg-custom-800/20 dark:active:bg-custom-800/20"
		on:click={removeData}
	>
		- REMOVE
	</button>

	<button
		id="randomize"
		class="px-2 py-1.5 text-xs bg-white border-dashed text-custom-500 btn border-custom-500 hover:text-custom-500 hover:bg-custom-50 hover:border-custom-600 focus:text-custom-600 focus:bg-custom-50 focus:border-custom-600 active:text-custom-600 active:bg-custom-50 active:border-custom-600 dark:bg-zink-700 dark:ring-custom-400/20 dark:hover:bg-custom-800/20 dark:focus:bg-custom-800/20 dark:active:bg-custom-800/20"
	on:click={randomize}>
		RANDOMIZE
	</button>

	<button
		id="reset"
		class="px-2 py-1.5 text-xs bg-white border-dashed text-custom-500 btn border-custom-500 hover:text-custom-500 hover:bg-custom-50 hover:border-custom-600 focus:text-custom-600 focus:bg-custom-50 focus:border-custom-600 active:text-custom-600 active:bg-custom-50 active:border-custom-600 dark:bg-zink-700 dark:ring-custom-400/20 dark:hover:bg-custom-800/20 dark:focus:bg-custom-800/20 dark:active:bg-custom-800/20"
	on:click={reset}>
		RESET
	</button>
</div>
