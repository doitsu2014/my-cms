<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	onMount(() => {
		var options = {
			series: [44, 33, 54, 45],
			chart: {
				height: 350,
				type: 'pie'
			},
			colors: getChartColorsArray(chartColors),
			fill: {
				type: 'image',
				opacity: 0.85,
				image: {
					src: [
						'/assets/images/small/img-1.jpg',
						'/assets/images/small/img-2.jpg',
						'/assets/images/small/img-6.jpg',
						'/assets/images/small/img-4.jpg'
					],
					width: 25,
					imagedHeight: 25
				}
			},
			stroke: {
				width: 4
			},
			dataLabels: {
				enabled: true,
				style: {
					colors: ['#111']
				},
				background: {
					enabled: true,
					foreColor: '#fff',
					borderWidth: 0
				}
			},
			legend: {
				position: 'bottom'
			}
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#pieWithImage'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="pieWithImage" class="apex-charts"></div>
