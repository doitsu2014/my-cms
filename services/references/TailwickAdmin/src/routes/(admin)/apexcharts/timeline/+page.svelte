<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import BasicTimelineChart from './BasicTimelineChart.svelte';
	import ColorTimelineChart from './ColorTimelineChart.svelte';
	import MultiSeriesChart from './MultiSeriesChart.svelte';
	import AdvancedMultipleRanges from './AdvancedMultipleRanges.svelte';
	import MultipleSeriesGroupRows from './MultipleSeriesGroupRows.svelte';
	import DumbbellChart from './DumbbellChart.svelte';
</script>

<HeadTitle title="Timeline Charts" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Timeline Charts" pagetitle="Apexcharts" />

	<div class="grid grid-cols-1 gap-x-5 xl:grid-cols-2">

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Basic</h6>
				<BasicTimelineChart chartColors={["bg-custom-500"]} />
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Different Color For Each Bar</h6>
				<ColorTimelineChart chartColors={["bg-custom-500", "bg-yellow-500", "bg-green-500", "bg-sky-500", "bg-purple-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Multi Series</h6>
				<MultiSeriesChart chartColors={["bg-custom-500", "bg-green-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Advanced (Multiple Ranges)</h6>
				<AdvancedMultipleRanges chartColors={["bg-custom-500", "bg-orange-500", "bg-green-500"]} />
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Multiple Series – Group Rows</h6>
				<MultipleSeriesGroupRows chartColors={["bg-custom-500", "bg-yellow-500", "bg-green-500", "bg-sky-500", "bg-purple-500", "bg-red-500", "bg-slate-500", "bg-orange-500", "bg-green-200","bg-pink-500", "bg-custom-200", "bg-red-300"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Dumbbell Chart (Horizontal)</h6>
				<DumbbellChart chartColors={["bg-custom-500", "bg-green-500"]}/>
			</div>
		</div>

	</div>
</div>
