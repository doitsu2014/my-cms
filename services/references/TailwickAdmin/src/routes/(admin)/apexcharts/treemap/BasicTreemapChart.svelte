<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	onMount(() => {
		var options = {
			series: [
				{
					data: [
						{
							x: 'New Delhi',
							y: 218
						},
						{
							x: 'Kolkata',
							y: 149
						},
						{
							x: 'Mumbai',
							y: 184
						},
						{
							x: 'Ahmedabad',
							y: 55
						},
						{
							x: 'Bangaluru',
							y: 84
						},
						{
							x: 'Pune',
							y: 31
						},
						{
							x: 'Chennai',
							y: 70
						},
						{
							x: 'Jaipur',
							y: 30
						},
						{
							x: 'Surat',
							y: 44
						},
						{
							x: 'Hyderabad',
							y: 68
						},
						{
							x: 'Lucknow',
							y: 28
						},
						{
							x: 'Indore',
							y: 19
						},
						{
							x: 'Kanpur',
							y: 29
						}
					]
				}
			],
			legend: {
				show: false
			},
			chart: {
				height: 350,
				type: 'treemap'
			},
			colors: getChartColorsArray(chartColors)
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#basicTreemapChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="basicTreemapChart" class="apex-charts"></div>
