<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	var options = {
        series: [{
        name: 'Click Through',
        data: [
            11, 30, 24, 26, 33, 38, 14, 20, 10, 5, 34, 27, 22
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
			var chart = new ApexCharts(document.querySelector('#clickThrough'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="clickThrough" class="grow apex-charts"></div>
