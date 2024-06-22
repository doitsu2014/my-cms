<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	let options = {
        series: [{
        name: "Response Times",
        data: [10, 41, 35, 51, 49, 62, 69, 91, 148]
    }],
    chart: {
        height: 350,
        type: 'line',
        zoom: {
            enabled: false
        },
        margin: {
            left: 0,
            right: 0,
            top: 0,
            bottom: 0
        },
        toolbar: {
            show: false,
        },
    },
    stroke: {
        show: true,
        curve: 'smooth',
        lineCap: 'butt',
        width: 2,
        dashArray: 0,
    },
    dataLabels: {
        enabled: false
    },
    colors: getChartColorsArray(chartColors),
    xaxis: {
        categories: ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep'],
    }
	};

	onMount(() => {
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#responseTimesChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="responseTimesChart" class="apex-charts" dir="ltr"></div>
