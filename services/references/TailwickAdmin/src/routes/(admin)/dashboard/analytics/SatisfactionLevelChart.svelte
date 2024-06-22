<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	let options = {
		series: [95.33],
    chart: {
        type: 'radialBar',
        height: 450,
        offsetY: -20,
        sparkline: {
            enabled: true
        }
    },
    plotOptions: {
        radialBar: {
            startAngle: -90,
            endAngle: 90,
            track: {
                strokeWidth: '97%',
                margin: 5, // margin is in pixels
            },
            hollow: {
                size: '60%',
                margin: 0,
                image: '../../assets/images/logo-sm.png',
                imageWidth: 36,
                imageHeight: 36,
                imageClipped: false,
                imageOffsetY: -60,
            },
            dataLabels: {
                name: {
                    show: false
                },
                value: {
                    offsetY: -5,
                    fontSize: '22px',
                    fontWeight: '600'
                }
            }
        }
    },
    grid: {
        padding: {
            top: -10
        }
    },
    colors: getChartColorsArray(chartColors),
    fill: {
        type: 'gradient',
        gradient: {
            shade: 'light',
            shadeIntensity: 0.4,
            inverseColors: false,
            opacityFrom: 1,
            opacityTo: 1,
            stops: [0, 50, 53, 91]
        },
    },
    stroke: {
        dashArray: 4
    },
    labels: ['Average Results'],
	};

	onMount(() => {
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#satisfactionLevelChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="satisfactionLevelChart" class="apex-charts" dir="ltr"></div>
