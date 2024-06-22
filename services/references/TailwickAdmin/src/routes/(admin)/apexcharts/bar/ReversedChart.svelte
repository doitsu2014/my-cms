<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	var options = {
		series: [{
        data: [400, 430, 448, 470, 540, 580, 690]
    }],
    chart: {
        type: 'bar',
        height: 350
    },
    annotations: {
        xaxis: [{
            x: 500,
            borderColor: '#00E396',
            label: {
                borderColor: '#00E396',
                style: {
                    color: '#fff',
                    background: '#00E396',
                },
                text: 'X annotation',
            }
        }],
        yaxis: [{
            y: 'July',
            y2: 'September',
            label: {
                text: 'Y annotation'
            }
        }]
    },
    plotOptions: {
        bar: {
            horizontal: true,
        }
    },
    dataLabels: {
        enabled: true
    },
    colors: getChartColorsArray(chartColors),
    xaxis: {
        categories: ['June', 'July', 'August', 'September', 'October', 'November', 'December'],
    },
    grid: {
        xaxis: {
            lines: {
                show: true
            }
        }
    },
    yaxis: {
        reversed: true,
        axisTicks: {
            show: true
        }
    }
	};

	onMount(() => {
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#reversedChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="reversedChart" class="apex-charts"></div>
