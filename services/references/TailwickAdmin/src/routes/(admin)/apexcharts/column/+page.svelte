<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import BasicColumnChart from './BasicColumnChart.svelte';
	import ColumnWithDatalabelChart from './ColumnWithDatalabelChart.svelte';
	import StackedColumnChart from './StackedColumnChart.svelte';
	import StackedColumn100Chart from './StackedColumn100Chart.svelte';
	import GroupedStackedColumnChart from './GroupedStackedColumnChart.svelte';
	import DumbbellChart from './DumbbellChart.svelte';
	import ColumnMarkersChart from './ColumnMarkersChart.svelte';
	import ColumnGroupLabelChart from './ColumnGroupLabelChart.svelte';
	import ColumnRotatedLabelChart from './ColumnRotatedLabelChart.svelte';
	import ColumnNegativeValueChart from './ColumnNegativeValueChart.svelte';
	import ColumnDistributedChart from './ColumnDistributedChart.svelte';
</script>

<HeadTitle title="Column Charts" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Column Charts" pagetitle="Apexcharts" />

	<div class="grid grid-cols-1 gap-x-5 xl:grid-cols-2">
		
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Basic</h6>
				<BasicColumnChart chartColors={["bg-custom-500", "bg-yellow-500", "bg-purple-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Column with Data Labels</h6>
				<ColumnWithDatalabelChart chartColors={["bg-custom-500"]} />
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Stacked Columns</h6>
				<StackedColumnChart chartColors={["bg-custom-500", "bg-orange-500", "bg-green-500", "bg-yellow-500"]} />
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Stacked Columns 100</h6>
				<StackedColumn100Chart chartColors={["bg-sky-500", "bg-orange-500", "bg-yellow-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Grouped Stacked Columns</h6>
				<GroupedStackedColumnChart chartColors={["bg-custom-500", "bg-green-500", "bg-custom-300", "bg-green-300"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Dumbbell Chart</h6>
				<DumbbellChart chartColors={["bg-custom-500", "bg-green-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Column with Markers</h6>
				<ColumnMarkersChart chartColors={["bg-sky-500", "bg-purple-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Column with Group Label</h6>
				<ColumnGroupLabelChart chartColors={["bg-custom-500"]} />
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Column with Rotated Labels</h6>
				<ColumnRotatedLabelChart chartColors={["bg-red-500", "bg-custom-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Column with Negative Values</h6>
				<ColumnNegativeValueChart chartColors={["bg-custom-500", "bg-yellow-500", "bg-orange-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Distributed Columns</h6>
				<ColumnDistributedChart chartColors={["bg-custom-500", "bg-yellow-500", "bg-green-500", "bg-orange-500", "bg-sky-500", "bg-purple-500", "bg-red-500", "bg-emerald-500"]}/>
			</div>
		</div>
	</div>
</div>
