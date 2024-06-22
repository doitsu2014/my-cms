<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	
</script>

<HeadTitle title="Video Player" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Video Player" pagetitle="UI Elements" />

	<div>
		<div class="grid grid-cols-1 gap-x-5 xl:grid-cols-2">
			<div class="card">
				<div class="card-body">
					<h6 class="mb-4 text-gray-800 text-15 dark:text-white">Preview Video Player</h6>

					<div class="plyr__video-embed" id="player">
						<!-- svelte-ignore a11y-missing-attribute -->
						<iframe src="https://www.youtube.com/embed/qYgogv4R8zg?si=_YxUDmc2fDgHyPae" allowfullscreen allowtransparency allow="autoplay" width="100%" height="400px"></iframe>
					</div>
				</div>
			</div>
			<!--end card-->
		</div>
		<!--end grid-->
	</div>
</div>
