<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
    import {githubdata} from "../../../../common/data/github-data"
	export let chartColors;

	var options = {
		series: [{
        name: 'commits',
        data: githubdata.series
    }],
    chart: {
        height: 150,
        type: 'area',
        toolbar: {
            autoSelected: 'selection',
        },
        brush: {
            enabled: true,
            target: 'chartyear'
        },
        selection: {
            enabled: true,
            xaxis: {
                min: new Date('26 Jan 2014').getTime(),
                max: new Date('29 Mar 2015').getTime()
            }
        },
    },
    colors: getChartColorsArray(chartColors),
    dataLabels: {
        enabled: false
    },
    stroke: {
        width: 0,
        curve: 'smooth'
    },
    fill: {
        opacity: 1,
        type: 'solid'
    },
    legend: {
        position: 'top',
        horizontalAlign: 'left'
    },
    xaxis: {
        type: 'datetime'
    },
	};

	onMount(() => {
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#areaYearsChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="areaYearsChart" class="apex-charts"></div>
