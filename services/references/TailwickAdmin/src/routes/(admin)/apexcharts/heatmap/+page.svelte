<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import BasicHeatmapChart from './BasicHeatmapChart.svelte';
	import MultipleColorsHeatmap from './MultipleColorsHeatmap.svelte';
	import ColorsRangeHeatmapChart from './ColorsRangeHeatmapChart.svelte';
	import RoundedRangeHeatmapChart from './RoundedRangeHeatmapChart.svelte';

</script>

<HeadTitle title="Heatmap Chart" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Heatmap Chart" pagetitle="Apexcharts" />

	<div class="grid grid-cols-1 gap-x-5 xl:grid-cols-2">
		
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Basic</h6>
				<BasicHeatmapChart chartColors={["bg-custom-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Multiple Colors</h6>
				<MultipleColorsHeatmap chartColors={["bg-custom-500", "bg-purple-500", "bg-orange-500", "bg-yellow-500", "bg-green-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Color Range</h6>
				<ColorsRangeHeatmapChart chartColors={["bg-custom-500", "bg-purple-500", "bg-orange-500", "bg-yellow-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Rounded (Range without Shades)</h6>
				<RoundedRangeHeatmapChart chartColors={["bg-sky-500", "bg-green-500"]}/>
			</div>
		</div>
	</div>
</div>
