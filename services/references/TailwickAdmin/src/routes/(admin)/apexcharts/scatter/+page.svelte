<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import BasicScatterChart from './BasicScatterChart.svelte';
	import ScatterDatetimeChart from './ScatterDatetimeChart.svelte';
	import ScatterImagesChart from './ScatterImagesChart.svelte';

</script>

<HeadTitle title="Scatter Charts" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Scatter Charts" pagetitle="Apexcharts" />

	<div class="grid grid-cols-1 gap-x-5 xl:grid-cols-2">

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Basic</h6>
				<BasicScatterChart  chartColors={["bg-custom-500","bg-green-500", "bg-yellow-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Scatter – Datetime</h6>
				<ScatterDatetimeChart chartColors={["bg-custom-500","bg-green-500", "bg-yellow-500", "bg-purple-500", "bg-red-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Scatter – Images</h6>
				<ScatterImagesChart chartColors={["bg-custom-500","bg-green-500"]}/>
			</div>
		</div>

	</div>
</div>
