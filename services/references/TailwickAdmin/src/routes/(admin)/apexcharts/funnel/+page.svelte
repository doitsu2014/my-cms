<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import BasicFunnelChart from './BasicFunnelChart.svelte';
	import PyramidChart from './PyramidChart.svelte';

</script>

<HeadTitle title="Funnel Charts" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Funnel Charts" pagetitle="Apexcharts" />

	<div class="grid grid-cols-1 gap-x-5 xl:grid-cols-2">
		
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Basic</h6>
				<BasicFunnelChart chartColors={["bg-custom-500"]}/>
			</div>
		</div><!--end card-->
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Pyramid</h6>
				<PyramidChart chartColors={["bg-custom-500", "bg-custom-400", "bg-custom-300", "bg-custom-200", "bg-purple-200", "bg-purple-300", "bg-purple-400", "bg-purple-500"]}/>
			</div>
		</div>
	</div>
</div>
