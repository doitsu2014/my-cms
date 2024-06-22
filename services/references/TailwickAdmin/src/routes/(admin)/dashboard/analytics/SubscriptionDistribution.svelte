<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	let options = {
		series: [44, 55, 41, 17, 15],
		labels: ['Beginner', 'Intermediate', 'Enterprise', 'VIP', 'Professional'],
		chart: {
			height: 270,
			type: 'donut'
		},
		plotOptions: {
			pie: {
				startAngle: -90,
				donut: {
					size: '75%'
				}
			}
		},
		dataLabels: {
			enabled: false
		},
		fill: {
			type: 'gradient'
		},
		colors: getChartColorsArray(chartColors),
		legend: {
			formatter: function (val, opts) {
				return val + ' - ' + opts.w.globals.series[opts.seriesIndex];
			}
		},
		legend: {
			position: 'bottom'
		}
	};

	onMount(() => {
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#subscriptionDistribution'), options);
			chart.render();
		}, 100);
	});
</script>

<div class="col-span-12 lg:col-span-6 order-[14] 2xl:order-1 card 2xl:col-span-3">
	<div class="card-body">
		<h6 class="mb-3 text-15">Subscription Distribution</h6>
		<div id="subscriptionDistribution" class="apex-charts" dir="ltr"></div>
	</div>
</div>
