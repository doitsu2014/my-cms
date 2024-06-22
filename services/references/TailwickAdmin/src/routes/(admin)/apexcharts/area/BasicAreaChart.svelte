<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
    import {series} from "../../../../common/data/stock-prices"
	export let chartColors;

	var options = {
		series: [{
        name: "STOCK ABC",
        data: series.monthDataSeries1.prices
    }],
    chart: {
        type: 'area',
        height: 350,
        zoom: {
            enabled: false
        }
    },
    dataLabels: {
        enabled: false
    },
    stroke: {
        curve: 'straight'
    },

    title: {
        text: 'Fundamental Analysis of Stocks',
        align: 'left'
    },
    grid: {
        padding: {
            top: 0,
            right: 0,
            bottom: 0,
            left: 0
        },
    },
    subtitle: {
        text: 'Price Movements',
        align: 'left'
    },
    colors: getChartColorsArray(chartColors),
    labels: series.monthDataSeries1.dates,
    xaxis: {
        type: 'datetime',
    },
    yaxis: {
        opposite: true
    },
    legend: {
        horizontalAlign: 'left'
    }
	};

	onMount(() => {
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#basicAreaChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="basicAreaChart" class="apex-charts"></div>
