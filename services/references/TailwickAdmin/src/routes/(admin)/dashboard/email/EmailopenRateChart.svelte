<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	var options = {
		series: [{
        name: 'Open Rate',
        data: [
            38, 34, 27, 22, 14, 20, 10, 5, 11, 30, 24, 26, 33
        ]
    }],
    chart: {
        id: 'area-datetime',
        type: 'line',
        height: 80,
        sparkline: {
            enabled: true
        },
        zoom: {
            autoScaleYaxis: true
        }
    },
    colors: getChartColorsArray(chartColors),
    stroke: {
        width: 2,
        curve: 'smooth',
    },
    dataLabels: {
        enabled: false
    },
	};

	onMount(() => {
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#openRate'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="openRate" class="grow apex-charts"></div>
