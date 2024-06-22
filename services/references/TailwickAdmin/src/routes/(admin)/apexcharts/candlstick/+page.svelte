<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import BasicChart from './BasicChart.svelte';
	import ComboCandlestick from './ComboCandlestick.svelte';
	import ComboCandlestick2 from './ComboCandlestick2.svelte';
	import CategoryCandlestick from './CategoryCandlestick.svelte';
	import CandlestickWithLine from './CandlestickWithLine.svelte';

</script>

<HeadTitle title="Candlstick Charts" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Candlstick Charts" pagetitle="Apexcharts" />

	<div class="grid grid-cols-1 gap-x-5 xl:grid-cols-2">
		
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-gray-800 text-15 dark:text-white">Simple</h6>
				<BasicChart chartColors={["bg-green-500", "bg-red-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-gray-800 text-15 dark:text-white">Candlestick Synced with Brush Chart (Combo)</h6>
				<div>
					<ComboCandlestick chartColors={["bg-sky-500", "bg-orange-500"]}/>
					<ComboCandlestick2 chartColors={["bg-yellow-500", "bg-red-500"]}/>
				</div>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-gray-800 text-15 dark:text-white">Category x-axis</h6>
				<CategoryCandlestick chartColors={["bg-green-500", "bg-red-500", "bg-sky-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-gray-800 text-15 dark:text-white">Candlestick with Line</h6>
				<CandlestickWithLine chartColors={["bg-green-500", "bg-red-500"]}/>
			</div>
		</div>
	</div>
</div>
