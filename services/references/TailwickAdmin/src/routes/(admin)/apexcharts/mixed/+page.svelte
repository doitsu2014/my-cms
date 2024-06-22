<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import LineColumnChart from './LineColumnChart.svelte';
	import MultipleYaxisChart from './MultipleYaxisChart.svelte';
	import LineAreaChart from './LineAreaChart.svelte';
	import LineColumnAreaChart from './LineColumnAreaChart.svelte';
	import LineScatterChart from './LineScatterChart.svelte';


</script>

<HeadTitle title="Mixed Charts" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Mixed Charts" pagetitle="Apexcharts" />

	<div class="grid grid-cols-1 gap-x-5 xl:grid-cols-2">
		
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Line Column</h6>
				<LineColumnChart chartColors={["bg-custom-500", "bg-orange-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Multiple Y-Axis</h6>
				<MultipleYaxisChart chartColors={["bg-custom-500", "bg-green-500", "bg-yellow-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Line & Area</h6>
				<LineAreaChart chartColors={["bg-custom-200", "bg-green-500"]} />
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Line Column Area</h6>
				<LineColumnAreaChart chartColors={["bg-sky-500", "bg-orange-500","bg-purple-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Line Scatter</h6>
				<LineScatterChart chartColors={["bg-custom-500", "bg-yellow-500"]}/>
			</div>
		</div>
	</div>
</div>
