<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	var options = {
		series: [10],
            chart: {
                height: 110,
                type: 'radialBar',
                sparkline: {
                    enabled: true
                }
            },
            plotOptions: {
                radialBar: {
                    hollow: {
                        margin: 0,
                        size: '50%',
                    },
                    track: {
                        margin: 2,
                    },
                    dataLabels: {
                        show: false
                    }
                }
            },
            grid: {
                padding: {
                    top: -15,
                    bottom: -15
                }
            },
            stroke: {
                lineCap: 'round'
            },
            labels: ['Total Employee'],
            colors: getChartColorsArray(chartColors)
	};

	onMount(() => {
		var chart = new ApexCharts(document.querySelector('#totalEmployeeChart'), options);
		chart.render();
	});
</script>

<div id="totalEmployeeChart" class="grow apex-charts"></div>
