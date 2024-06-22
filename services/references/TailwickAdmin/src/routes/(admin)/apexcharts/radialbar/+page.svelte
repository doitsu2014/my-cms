<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import SimpleRadialbarChart from './SimpleRadialbarChart.svelte';
	import MultipleRadialbarChart from './MultipleRadialbarChart.svelte';
	import CustomAngleCircleChart from './CustomAngleCircleChart.svelte';
	import GradientChart from './GradientChart.svelte';
	import RadialbarsWithImageChart from './RadialbarsWithImageChart.svelte';
	import StrokedGauge from './StrokedGauge.svelte';
	import SemiCircleGauge from './SemiCircleGauge.svelte';

</script>

<HeadTitle title="Radialbar Charts" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Radialbar Charts" pagetitle="Apexcharts" />

	<div class="grid grid-cols-1 gap-x-5 xl:grid-cols-2">

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Simple Radialbar Chart</h6>
				<SimpleRadialbarChart chartColors={["bg-custom-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Multiple Radialbar</h6>
				<MultipleRadialbarChart chartColors={["bg-custom-500", "bg-orange-500", "bg-green-500", "bg-yellow-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Custom Angle Circle</h6>
				<CustomAngleCircleChart chartColors={["bg-custom-500", "bg-sky-500", "bg-green-500", "bg-purple-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Gradient</h6>
				<GradientChart chartColors={["bg-custom-500", "bg-green-400"]} />
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Radialbar with Image</h6>
				<RadialbarsWithImageChart />
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Stroked Gauge</h6>
				<StrokedGauge chartColors={["bg-purple-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Semi Circle Gauge</h6>
				<SemiCircleGauge chartColors={["bg-custom-500"]}/>
			</div>
		</div>

	</div>
</div>
