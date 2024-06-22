<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	var options = {
		series: [{
        name: 'Following',
        data: [44, 55, 57, 56, 61, 58, 63, 60, 66, 87, 72]
    }, {
        name: 'Followers',
        data: [76, 85, 101, 98, 87, 105, 91, 114, 94, 105, 91]
    }],
    chart: {
        type: 'bar',
        height: 350,
        toolbar: {
            show: false,
        },
    },
    plotOptions: {
        bar: {
            horizontal: false,
            columnWidth: '45%',
            endingShape: 'rounded'
        },
    },
    dataLabels: {
        enabled: false
    },
    colors: getChartColorsArray(chartColors),
    stroke: {
        show: true,
        width: 1,
        colors: ['transparent']
    },
    xaxis: {
        categories: ['Feb', 'Mar', 'Apr', 'May', 'Jun', 'Jul', 'Aug', 'Sep', 'Oct', 'Nov', 'Dec'],
    },
    fill: {
        opacity: 1
    },
	};

	onMount(() => {
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#recentStatisticsChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="recentStatisticsChart" class="apex-charts"></div>
