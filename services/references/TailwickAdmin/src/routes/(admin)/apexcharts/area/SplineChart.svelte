<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
    import {series} from "../../../../common/data/stock-prices"
	export let chartColors;

	var options = {
        series: [{
        name: 'series1',
        data: [31, 40, 28, 51, 42, 109, 100]
    }, {
        name: 'series2',
        data: [11, 32, 45, 32, 34, 52, 41]
    }],
    chart: {
        height: 350,
        type: 'area'
    },
    dataLabels: {
        enabled: false
    },
    grid: {
        padding: {
            top: 0,
            right: 0,
            bottom: 0,
        },
    },
    stroke: {
        curve: 'smooth'
    },
    colors: getChartColorsArray(chartColors),
    xaxis: {
        type: 'datetime',
        categories: ["2018-09-19T00:00:00.000Z", "2018-09-19T01:30:00.000Z", "2018-09-19T02:30:00.000Z", "2018-09-19T03:30:00.000Z", "2018-09-19T04:30:00.000Z", "2018-09-19T05:30:00.000Z", "2018-09-19T06:30:00.000Z"]
    },
    tooltip: {
        x: {
            format: 'dd/MM/yy HH:mm'
        },
    },
	};

	onMount(() => {
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#splineChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="splineChart" class="apex-charts"></div>
