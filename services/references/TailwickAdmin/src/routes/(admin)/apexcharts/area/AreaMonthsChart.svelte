<script>
	import { onDestroy, onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
    import {githubdata} from "../../../../common/data/github-data"
	export let chartColors;
    var chart;

	var options = {
        series: [{
        name: 'commits',
        data: githubdata.series
    }],
    chart: {
        id: 'chartyear',
        type: 'area',
        height: 130,
        toolbar: {
            show: false,
            autoSelected: 'pan'
        },
        events: {
            mounted: function (chart) {
                var commitsEl = document.querySelector('.cmeta span.commits');
                var commits = chart.getSeriesTotalXRange(chart.w.globals.minX, chart.w.globals.maxX)

                commitsEl.innerHTML = commits
            },
            updated: function (chart) {
                var commitsEl = document.querySelector('.cmeta span.commits');
                var commits = chart.getSeriesTotalXRange(chart.w.globals.minX, chart.w.globals.maxX)

                commitsEl.innerHTML = commits
            }
        }
    },
    colors: getChartColorsArray(chartColors),
    stroke: {
        width: 0,
        curve: 'smooth'
    },
    dataLabels: {
        enabled: false
    },
    fill: {
        opacity: 1,
        type: 'solid'
    },
    yaxis: {
        show: false,
        tickAmount: 3,
    },
    xaxis: {
        type: 'datetime',
    }
	};

	onMount(() => {
		setTimeout(() => {
			 chart = new ApexCharts(document.querySelector('#areaMonthsChart'), options);
			chart.render();
		}, 100);
	});

    onDestroy(() => {
        if(chart){
            chart.destroy();
        }
    })
</script>

<div id="areaMonthsChart" class="apex-charts"></div>
