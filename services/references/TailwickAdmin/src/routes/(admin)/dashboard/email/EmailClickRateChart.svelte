<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	var options = {
        series: [{
        name: 'Click Rate',
        data: [
            30, 24, 14, 20, 10, 5, 11, 26, 33, 38, 34, 27, 22
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
			var chart = new ApexCharts(document.querySelector('#emailClickRateChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="emailClickRateChart" class="grow apex-charts"></div>
