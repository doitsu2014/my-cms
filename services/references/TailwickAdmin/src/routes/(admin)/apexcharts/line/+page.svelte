<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import BasicLineChart from './BasicLineChart.svelte';
	import LineWithDataLabelChart from './LineWithDataLabelChart.svelte';
	import ZoomableTimeseriesChart from './ZoomableTimeseriesChart.svelte';
	import LineWithAnnotationsChart from './LineWithAnnotationsChart.svelte';
	import BrushChartLine from './BrushChartLine.svelte';
	import ChartStepline from './ChartStepline.svelte';
	import GradientCharts from './GradientCharts.svelte';
	import MissingCharts from './MissingCharts.svelte';
	import DashedCharts from './DashedCharts.svelte';
	import RealtimeCharts from './RealtimeCharts.svelte';
	// import BrushChartLine2 from './BrushChartLine2.svelte';

</script>

<HeadTitle title="Line Chart" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Line Chart" pagetitle="Apexcharts" />

	<div class="grid grid-cols-1 gap-x-5 xl:grid-cols-2">
		
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Basic</h6>
				<BasicLineChart chartColors={["bg-custom-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Line with Data Labels</h6>
				<LineWithDataLabelChart chartColors={["bg-custom-500", "bg-green-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Zoomable Timeseries</h6>
				<ZoomableTimeseriesChart chartColors={["bg-custom-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Line Chart with Annotations</h6>
				<LineWithAnnotationsChart chartColors={["bg-custom-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Brush Chart</h6>
				<div>
					<!-- <div id="brushChartLine" class="apex-charts" data-chart-colors='["bg-slate-600"]' dir="ltr"></div> -->
					<!-- <div id="brushChartLine2" class="apex-charts" data-chart-colors='["bg-custom-500"]' dir="ltr"></div> -->
					<!-- <BrushChartLine chartColors={["bg-slate-600"]}/> -->
					<!-- <BrushChartLine2 chartColors={["bg-slate-500"]}/> -->
				</div>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Stepline</h6>
				<ChartStepline chartColors={["bg-yellow-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Gradient Charts</h6>
				<GradientCharts chartColors={["bg-purple-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Missing / Null Values</h6>
				<MissingCharts chartColors={["bg-green-500", "bg-sky-500", "bg-yellow-500"]} />
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Dashed Charts</h6>
				<DashedCharts chartColors={["bg-custom-500", "bg-orange-500", "bg-green-500"]}/>
			</div>
		</div>

		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Realtime Charts</h6>
				<!-- <div id="realtimeCharts" class="apex-charts" data-chart-colors='["bg-custom-500"]' dir="ltr"></div> -->
				<RealtimeCharts chartColors={["bg-custom-500"]}/>
			</div>
		</div>
	</div>
</div>
