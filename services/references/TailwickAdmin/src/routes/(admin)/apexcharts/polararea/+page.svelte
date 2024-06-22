<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import BasicPolarChart from './BasicPolarChart.svelte';
	import MonochromeChart from './MonochromeChart.svelte';


</script>

<HeadTitle title="Polar Area Charts" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Polar Area Charts" pagetitle="Apexcharts" />

	<div class="grid grid-cols-1 gap-x-5 xl:grid-cols-2">

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Basic</h6>
				<BasicPolarChart chartColors={["bg-custom-300", "bg-orange-300", "bg-green-300", "bg-yellow-300", "bg-purple-300", "bg-slate-300", "bg-red-300", "bg-emerald-200","bg-pink-200"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Monochrome</h6>
				<MonochromeChart chartColors={["bg-custom-500"]}/>
			</div>
		</div>
	</div>
</div>
