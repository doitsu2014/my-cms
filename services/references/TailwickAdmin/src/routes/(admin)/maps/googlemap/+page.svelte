<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import Gmapmarker from './Gmapmarker.svelte';
	import Gmapoverlay from './Gmapoverlay.svelte';
	import GmapStreetViewPanorama from './GmapStreetViewPanorama.svelte';
	import GmapType from './GmapType.svelte';
</script>

<HeadTitle title="Google Maps" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Google Maps" pagetitle="Icons" />

	<div class="grid grid-cols-1 gap-x-5 xl:grid-cols-2">
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-gray-800 text-15 dark:text-white">Markers</h6>
				<Gmapmarker />
			</div>
		</div>
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-gray-800 text-15 dark:text-white">Overlays</h6>
				<Gmapoverlay />
			</div>
		</div>
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-gray-800 text-15 dark:text-white">Street View Panoramas</h6>
				<GmapStreetViewPanorama />
			</div>
		</div>
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-gray-800 text-15 dark:text-white">Map Types</h6>
				<GmapType />
			</div>
		</div>
	</div>
</div>
