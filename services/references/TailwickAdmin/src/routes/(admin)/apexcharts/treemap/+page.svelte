<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import BasicTreemapChart from './BasicTreemapChart.svelte';
	import TreemapMultipleSeriesChart from './TreemapMultipleSeriesChart.svelte';
	import ColorRangeChart from './ColorRangeChart.svelte';
	import DistributedChart from './DistributedChart.svelte';
</script>

<HeadTitle title="Treemap Charts" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Treemap Charts" pagetitle="Apexcharts" />

	<div class="grid grid-cols-1 gap-x-5 xl:grid-cols-2">

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Basic</h6>
				<BasicTreemapChart chartColors={["bg-custom-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Treemap Multiple Series</h6>
				<TreemapMultipleSeriesChart />
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Color Range</h6>
				<ColorRangeChart />
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Distributed Chart</h6>
				<DistributedChart />
			</div>
		</div>

	</div>
</div>
