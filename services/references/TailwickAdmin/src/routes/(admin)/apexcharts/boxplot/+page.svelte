<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import BasicBoxplotChart from './BasicBoxplotChart.svelte';
	import BoxplotScatterChart from './BoxplotScatterChart.svelte';
	import BoxplotHorizontalChart from './BoxplotHorizontalChart.svelte';

</script>

<HeadTitle title="Boxplot Charts" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Boxplot Charts" pagetitle="Apexcharts" />

	<div class="grid grid-cols-1 gap-x-5 xl:grid-cols-2">
		
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Basic</h6>
				<BasicBoxplotChart chartColors={["bg-custom-500", "bg-slate-100"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Boxplot-Scatter</h6>
				<BoxplotScatterChart chartColors={["bg-custom-500", "bg-green-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Horizontal BoxPlot</h6>
				<BoxplotHorizontalChart chartColors={["bg-slate-50", "bg-slate-200"]}/>
			</div>
		</div>
	</div>
</div>
