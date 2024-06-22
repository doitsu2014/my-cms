<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors

	let options = {
		series: [
			{
				data: [
					{
						x: 'React',
						y: 218
					},
					{
						x: 'TailwindCSS',
						y: 187
					},
					{
						x: 'Angular',
						y: 134
					},
					{
						x: 'Vue Js',
						y: 55
					},
					{
						x: 'Laravel',
						y: 99
					},
					{
						x: 'PHP',
						y: 34
					},
					{
						x: 'ASP.Net',
						y: 86
					},
					{
						x: 'Django',
						y: 30
					},
					{
						x: 'CI',
						y: 44
					}
				]
			}
		],
		legend: {
			show: false
		},
		chart: {
			height: 270,
			type: 'treemap',
			toolbar: {
				show: false
			}
		},
		grid: {
			show: false,
			padding: {
				top: -15,
				bottom: 0,
				right: -20
			}
		},
		colors: getChartColorsArray(chartColors)
	};

	onMount(() => {
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#platformPerspectiveChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="platformPerspectiveChart" class="apex-charts" dir="ltr"></div>
