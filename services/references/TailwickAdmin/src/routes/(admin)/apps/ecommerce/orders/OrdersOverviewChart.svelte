<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../../common/components/ChartColorsArray.svelte';
	export let chartColors;

	let options = {
        series: [{
        name: 'Orders',
        data: [2.3, 3.1, 4.0, 10.1, 4.0, 3.6, 3.2, 2.3, 1.4, 3.4, 5.5, 8.8]
    }],
    chart: {
        height: 238,
        type: 'bar',
        toolbar: {
            show: false,
        }
    },
    plotOptions: {
        bar: {
            borderRadius: 5,
            dataLabels: {
                position: 'top', // top, center, bottom
            },
        }
    },
    dataLabels: {
        enabled: true,
        formatter: function (val) {
            return val + "%";
        },
        style: {
            fontSize: '12px',
        }
    },
    grid: {
        padding: {
            bottom: -10,
        }
    },
    xaxis: {
        categories: ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"],
        position: 'top',
        axisBorder: {
            show: false
        },
        axisTicks: {
            show: false
        },
        tooltip: {
            enabled: true,
        }
    },
    yaxis: {
        axisBorder: {
            show: false
        },
        axisTicks: {
            show: false,
        },
        labels: {
            show: false,
            formatter: function (val) {
                return val + "%";
            }
        }

    },
    colors: getChartColorsArray(chartColors),
	};

	onMount(() => {
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#ordersOverviewChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="ordersOverviewChart" class=" apex-charts" dir="ltr"></div>
