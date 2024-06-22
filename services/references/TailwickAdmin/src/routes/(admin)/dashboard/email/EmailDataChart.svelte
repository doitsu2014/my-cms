<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	var options = {
		series: [
        {
            name: "Open Rate",
            data: [28, 29, 31, 36, 32, 32, 33, 40, 37]
        },
        {
            name: "Click Rate",
            data: [26, 41, 40, 51, 49, 62, 69, 52, 58]
        }
    ],
    chart: {
        height: 350,
        type: 'line',
        dropShadow: {
            enabled: true,
            color: '#000',
            top: 18,
            left: 7,
            blur: 10,
            opacity: 0.2
        },
        toolbar: {
            show: false
        },
    },
    colors: getChartColorsArray(chartColors),
    stroke: {
        curve: 'smooth',
        width: 3
    },
    legend: {
        show: true,
    },
    markers: {
        size: 5,
        hover: {
            sizeOffset: 1
        }
    },
    xaxis: {
        categories: ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep'],
    },
    legend: {
        position: 'top',
        horizontalAlign: 'right',
        floating: true,
        offsetY: -25,
        offsetX: -5
    }
	};

	onMount(() => {
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#emailDataChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="emailDataChart" class="apex-charts"></div>
