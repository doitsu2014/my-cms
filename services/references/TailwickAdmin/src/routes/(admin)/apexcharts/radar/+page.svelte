<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import BasicRadarChart from './BasicRadarChart.svelte';
	import RadarMultipleSeriesChart from './RadarMultipleSeriesChart.svelte';
	import RadarWithPolygonfillChart from './RadarWithPolygonfillChart.svelte';
</script>

<HeadTitle title="Radar Charts" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Radar Charts" pagetitle="Apexcharts" />

	<div class="grid grid-cols-1 gap-x-5 xl:grid-cols-2">

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Basic</h6>
				<BasicRadarChart chartColors={["bg-custom-300", "bg-custom-300"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Radar – Multiple Series</h6>
				<RadarMultipleSeriesChart chartColors={["bg-custom-500", "bg-green-500", "bg-yellow-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Radar with Polygon-fill</h6>
				<RadarWithPolygonfillChart chartColors={[]}/>
			</div>
		</div>
	</div>
</div>
