<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import BasicRangeAreaChart from './BasicRangeAreaChart.svelte';
	import ComboRangeAreaChart from './ComboRangeAreaChart.svelte';

</script>

<HeadTitle title="Range Area Charts" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Range Area Charts" pagetitle="Apexcharts" />

	<div class="grid grid-cols-1 gap-x-5 xl:grid-cols-2">

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Basic Range Area</h6>
				<BasicRangeAreaChart chartColors={["bg-custom-500"]}/>
			</div>
		</div><!--end card-->
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Combo Range Area</h6>
				<ComboRangeAreaChart chartColors={["bg-sky-100", "bg-orange-100", "bg-sky-500", "bg-orange-500"]}/>
			</div>
		</div>
	</div>
</div>
