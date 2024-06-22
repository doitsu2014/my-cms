<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import BasicBarChart from './BasicBarChart.svelte';
	import GroupedChart from './GroupedChart.svelte';
	import StackedChart from './StackedChart.svelte';
	import StackedBar100Chart from './StackedBar100Chart.svelte';
	import GroupedStackedChart from './GroupedStackedChart.svelte';
	import NegativeValuesChart from './NegativeValuesChart.svelte';
	import MarkersChart from './MarkersChart.svelte';
	import ReversedChart from './ReversedChart.svelte';
	import CustomDataLabelsChart from './CustomDataLabelsChart.svelte';
	import PatternedChart from './PatternedChart.svelte';
	import BarWithImagesChart from './BarWithImagesChart.svelte';

</script>

<HeadTitle title="Bar Charts" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Bar Charts" pagetitle="Apexcharts" />

	<div class="grid grid-cols-1 gap-x-5 xl:grid-cols-2">
		
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Basic</h6>
				<BasicBarChart chartColors={["bg-custom-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Grouped Chart</h6>
				<GroupedChart chartColors={["bg-custom-500", "bg-green-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Stacked Bar</h6>
				<StackedChart chartColors={["bg-custom-500", "bg-green-500", "bg-orange-500", "bg-yellow-500", "bg-purple-500", "bg-red-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Stacked Bars 100</h6>
				<StackedBar100Chart chartColors={["bg-custom-500", "bg-green-500", "bg-orange-500", "bg-yellow-500", "bg-purple-500", "bg-red-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Grouped Stacked Bars</h6>
				<GroupedStackedChart chartColors={["bg-custom-500", "bg-green-500", "bg-custom-300", "bg-green-300"]} />
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Bar with Negative Values</h6>
				<NegativeValuesChart chartColors={["bg-red-500", "bg-sky-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Bar with Markers</h6>
				<MarkersChart chartColors={["bg-green-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Reversed Bar Chart</h6>
				<ReversedChart chartColors={["bg-custom-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Custom DataLabels Bar</h6>
				<CustomDataLabelsChart  chartColors={["bg-custom-500", "bg-orange-500", "bg-green-500", "bg-sky-500", "bg-yellow-500", "bg-red-500", "bg-purple-500", "bg-slate-300", "bg-slate-800", "bg-emerald-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Patterned Bar</h6>
				<PatternedChart chartColors={["bg-custom-500", "bg-green-500", "bg-orange-500", "bg-yellow-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Bar with Images</h6>
				<BarWithImagesChart />
			</div>
		</div>
	</div>
</div>
