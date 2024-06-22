<script>
	import { onMount } from 'svelte';
	import { getChartColorsArray } from '../../../../common/components/ChartColorsArray.svelte';
	import moment from "moment";
	export let chartColors;

	onMount(() => {

		var options = {
			series: [
				{
					name: 'sales',
					data: [
						{
							x: '2019/01/01',
							y: 400
						},
						{
							x: '2019/04/01',
							y: 430
						},
						{
							x: '2019/07/01',
							y: 448
						},
						{
							x: '2019/10/01',
							y: 470
						},
						{
							x: '2020/01/01',
							y: 540
						},
						{
							x: '2020/04/01',
							y: 580
						},
						{
							x: '2020/07/01',
							y: 690
						},
						{
							x: '2020/10/01',
							y: 690
						}
					]
				}
			],
			chart: {
				type: 'bar',
				height: 350
			},
			xaxis: {
				type: 'category',
				labels: {
					formatter: function (val) {
						return 'Q' + moment(val).quarter();
					}
				},
				group: {
					style: {
						fontSize: '10px',
						fontWeight: 700
					},
					groups: [
						{ title: '2019', cols: 4 },
						{ title: '2020', cols: 4 }
					]
				}
			},
			colors: getChartColorsArray(chartColors),
			tooltip: {
				x: {
					formatter: function (val) {
						return 'Q' + dayjs(val).quarter() + ' ' + dayjs(val).format('YYYY');
					}
				}
			}
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#columnGroupLabelChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="columnGroupLabelChart" class="apex-charts"></div>
