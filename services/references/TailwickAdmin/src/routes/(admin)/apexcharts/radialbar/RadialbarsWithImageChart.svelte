<script>
	import { onMount } from 'svelte';

	onMount(() => {
		var options = {
			series: [67],
			chart: {
				height: 350,
				type: 'radialBar'
			},
			plotOptions: {
				radialBar: {
					hollow: {
						margin: 15,
						size: '70%',
						image: '/assets/images/logo-sm.png',
						imageWidth: 64,
						imageHeight: 64,
						imageClipped: false
					},
					dataLabels: {
						name: {
							show: false,
							color: '#fff'
						},
						value: {
							show: true,
							color: '#333',
							offsetY: 70,
							fontSize: '22px'
						}
					}
				}
			},
			fill: {
				type: 'image',
				image: {
					src: ['/assets/images/small/img-2.jpg']
				}
			},
			stroke: {
				lineCap: 'round'
			},
			labels: ['Volatility']
		};
		setTimeout(() => {
			var chart = new ApexCharts(document.querySelector('#radialbarsWithImageChart'), options);
			chart.render();
		}, 100);
	});
</script>

<div id="radialbarsWithImageChart" class="apex-charts"></div>
