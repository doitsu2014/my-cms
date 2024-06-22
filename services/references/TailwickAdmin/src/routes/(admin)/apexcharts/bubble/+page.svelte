<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import SimpleChart from './SimpleChart.svelte';
	import BubbleChart from './BubbleChart.svelte';

</script>

<HeadTitle title="Bubble Charts" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Bubble Charts" pagetitle="Apexcharts" />

	<div class="grid grid-cols-1 gap-x-5 xl:grid-cols-2">
		
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Simple</h6>
				<SimpleChart chartColors={["bg-custom-500", "bg-green-500", "bg-orange-500", "bg-yellow-500"]} />
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">3D Bubble</h6>
				<BubbleChart chartColors={["bg-sky-500", "bg-yellow-500", "bg-red-500", "bg-purple-500"]}/>
			</div>
		</div>

	</div>
</div>
