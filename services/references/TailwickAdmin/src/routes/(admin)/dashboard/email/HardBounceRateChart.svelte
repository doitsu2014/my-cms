<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	var options = {
		series: [{
        name: 'Hard Bounce Rate',
        data: [
            14, 20, 10, 5, 11, 30, 24, 26, 33, 38, 34, 27, 22
        ]
    }],
    chart: {
        id: 'area-datetime',
        type: 'bar',
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
			var chart = new ApexCharts(document.querySelector('#hardBounceRate'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="hardBounceRate" class="grow apex-charts"></div>
