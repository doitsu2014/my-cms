<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import SimplePieChart from './SimplePieChart.svelte';
	import SimpleDonutChart from './SimpleDonutChart.svelte';
	import UpdatingDonutChart from './UpdatingDonutChart.svelte';
	import MonochromePieChart from './MonochromePieChart.svelte';
	import GradientDonut from './GradientDonut.svelte';
	import SemiDonutChart from './semiDonutChart.svelte';
	import DonutWithPatternChart from './DonutWithPatternChart.svelte';
	import PieWithImageChart from './PieWithImageChart.svelte';

</script>

<HeadTitle title="Pie / Donut Charts" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Pie / Donut Charts" pagetitle="Apexcharts" />

	<div class="grid grid-cols-1 gap-x-5 xl:grid-cols-2">
		
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Simple Pie</h6>
				<SimplePieChart chartColors={["bg-custom-500", "bg-orange-500", "bg-green-500", "bg-yellow-500", "bg-purple-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Simple Donut</h6>
				<SimpleDonutChart chartColors={["bg-custom-500", "bg-orange-500", "bg-green-500", "bg-sky-500", "bg-yellow-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Donut Update</h6>
				<div>
					<UpdatingDonutChart chartColors={["bg-custom-400", "bg-orange-400", "bg-green-400", "bg-yellow-400"]}/>
				
					
				</div>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Monochrome Pie</h6>
				<MonochromePieChart chartColors={["bg-custom-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Gradient Donut</h6>
				<GradientDonut chartColors={["bg-custom-500", "bg-orange-500", "bg-green-500", "bg-yellow-500", "bg-purple-500"]}/>
			</div>
		</div>
		
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Semi Donut</h6>
				<SemiDonutChart chartColors={["bg-custom-500", "bg-orange-500", "bg-green-500", "bg-yellow-500", "bg-purple-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Donut with Pattern</h6>
				<DonutWithPatternChart chartColors={["bg-custom-500", "bg-orange-500", "bg-green-500", "bg-yellow-500", "bg-purple-500"]} />
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Pie with Image</h6>
				<PieWithImageChart chartColors={["bg-custom-500", "bg-orange-500", "bg-green-500", "bg-yellow-500", "bg-purple-500"]}/>
			</div>
		</div>

	</div>
</div>
