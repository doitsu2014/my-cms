<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import LucideIcon from '../../../../common/components/LucideIcon.svelte';
</script>

<HeadTitle title="Buttons" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Buttons" pagetitle="UI Elements" />

	<div>
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Basic Buttons</h6>
				<div class="flex flex-wrap gap-2">
					<button
						type="button"
						class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
						>Custom Button</button
					>
					<button
						type="button"
						class="text-custom-500 btn bg-custom-100 hover:text-white hover:bg-custom-600 focus:text-white focus:bg-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:ring active:ring-custom-100 dark:bg-custom-500/20 dark:text-custom-500 dark:hover:bg-custom-500 dark:hover:text-white dark:focus:bg-custom-500 dark:focus:text-white dark:active:bg-custom-500 dark:active:text-white dark:ring-custom-400/20"
						>Soft Button</button
					>
					<button
						type="button"
						class="bg-white border-dashed text-custom-500 btn border-custom-500 hover:text-custom-500 hover:bg-custom-50 hover:border-custom-600 focus:text-custom-600 focus:bg-custom-50 focus:border-custom-600 active:text-custom-600 active:bg-custom-50 active:border-custom-600 dark:bg-zink-700 dark:ring-custom-400/20 dark:hover:bg-custom-800/20 dark:focus:bg-custom-800/20 dark:active:bg-custom-800/20"
						>Dashed Buttons</button
					>
					<button
						type="button"
						class="bg-white text-custom-500 btn border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:bg-zink-700 dark:hover:bg-custom-500 dark:ring-custom-400/20 dark:focus:bg-custom-500"
						>Outline Button</button
					>
					<button
						type="button"
						class="bg-white text-custom-500 btn hover:text-custom-500 hover:bg-custom-100 focus:text-custom-500 focus:bg-custom-100 active:text-custom-500 active:bg-custom-100 dark:bg-zink-700 dark:hover:bg-custom-500/10 dark:focus:bg-custom-500/10 dark:active:bg-custom-500/10"
						>Ghost Button</button
					>
					<a
						href="#!"
						class="bg-white border-white text-custom-500 btn hover:text-custom-700 focus:text-custom-700 active:text-custom-700 dark:bg-zink-700 dark:border-zink-700"
						>Btn Link</a
					>
				</div>
			</div>
		</div>
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Basic Buttons</h6>
				<div class="flex flex-wrap gap-2">
					<button
						type="button"
						class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
						>Custom</button
					>
					<button
						type="button"
						class="text-white bg-green-500 border-green-500 btn hover:text-white hover:bg-green-600 hover:border-green-600 focus:text-white focus:bg-green-600 focus:border-green-600 focus:ring focus:ring-green-100 active:text-white active:bg-green-600 active:border-green-600 active:ring active:ring-green-100 dark:ring-green-400/10"
						>Green</button
					>
					<button
						type="button"
						class="text-white bg-orange-500 border-orange-500 btn hover:text-white hover:bg-orange-600 hover:border-orange-600 focus:text-white focus:bg-orange-600 focus:border-orange-600 focus:ring focus:ring-orange-100 active:text-white active:bg-orange-600 active:border-orange-600 active:ring active:ring-orange-100 dark:ring-orange-400/10"
						>Orange</button
					>
					<button
						type="button"
						class="text-white btn bg-sky-500 border-sky-500 hover:text-white hover:bg-sky-600 hover:border-sky-600 focus:text-white focus:bg-sky-600 focus:border-sky-600 focus:ring focus:ring-sky-100 active:text-white active:bg-sky-600 active:border-sky-600 active:ring active:ring-sky-100 dark:ring-sky-400/20"
						>Sky</button
					>
					<button
						type="button"
						class="text-white bg-yellow-500 border-yellow-500 btn hover:text-white hover:bg-yellow-600 hover:border-yellow-600 focus:text-white focus:bg-yellow-600 focus:border-yellow-600 focus:ring focus:ring-yellow-100 active:text-white active:bg-yellow-600 active:border-yellow-600 active:ring active:ring-yellow-100 dark:ring-yellow-400/10"
						>Yellow</button
					>
					<button
						type="button"
						class="text-white bg-red-500 border-red-500 btn hover:text-white hover:bg-red-600 hover:border-red-600 focus:text-white focus:bg-red-600 focus:border-red-600 focus:ring focus:ring-red-100 active:text-white active:bg-red-600 active:border-red-600 active:ring active:ring-red-100 dark:ring-custom-400/20"
						>Red</button
					>
					<button
						type="button"
						class="text-white bg-purple-500 border-purple-500 btn hover:text-white hover:bg-purple-600 hover:border-purple-600 focus:text-white focus:bg-purple-600 focus:border-purple-600 focus:ring focus:ring-purple-100 active:text-white active:bg-purple-600 active:border-purple-600 active:ring active:ring-purple-100 dark:ring-purple-400/10"
						>Purple</button
					>
					<button
						type="button"
						class="text-white btn bg-slate-500 border-slate-500 hover:text-white hover:bg-slate-600 hover:border-slate-600 focus:text-white focus:bg-slate-600 focus:border-slate-600 focus:ring focus:ring-slate-100 active:text-white active:bg-slate-600 active:border-slate-600 active:ring active:ring-slate-100 dark:ring-slate-400/10"
						>Slate</button
					>
					<button
						type="button"
						class="text-slate-500 btn bg-slate-200 border-slate-200 hover:text-slate-600 hover:bg-slate-300 hover:border-slate-300 focus:text-slate-600 focus:bg-slate-300 focus:border-slate-300 focus:ring focus:ring-slate-100 active:text-slate-600 active:bg-slate-300 active:border-slate-300 active:ring active:ring-slate-100 dark:bg-zink-600 dark:hover:bg-zink-500 dark:border-zink-600 dark:hover:border-zink-500 dark:text-zink-200 dark:ring-zink-400/50"
						>Light</button
					>
				</div>
			</div>
		</div>
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Soft Buttons</h6>
				<div class="flex flex-wrap gap-2">
					<button
						type="button"
						class="text-custom-500 btn bg-custom-100 hover:text-white hover:bg-custom-600 focus:text-white focus:bg-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:ring active:ring-custom-100 dark:bg-custom-500/20 dark:text-custom-500 dark:hover:bg-custom-500 dark:hover:text-white dark:focus:bg-custom-500 dark:focus:text-white dark:active:bg-custom-500 dark:active:text-white dark:ring-custom-400/20"
						>Custom</button
					>
					<button
						type="button"
						class="text-green-500 bg-green-100 btn hover:text-white hover:bg-green-600 focus:text-white focus:bg-green-600 focus:ring focus:ring-green-100 active:text-white active:bg-green-600 active:ring active:ring-green-100 dark:bg-green-500/20 dark:text-green-400 dark:hover:bg-green-500 dark:hover:text-white dark:focus:bg-green-500 dark:focus:text-white dark:active:bg-green-500 dark:active:text-white dark:ring-green-400/20"
						>Green</button
					>
					<button
						type="button"
						class="text-orange-500 bg-orange-100 btn hover:text-white hover:bg-orange-600 focus:text-white focus:bg-orange-600 focus:ring focus:ring-orange-100 active:text-white active:bg-orange-600 active:ring active:ring-orange-100 dark:bg-orange-500/20 dark:text-orange-500 dark:hover:bg-orange-500 dark:hover:text-white dark:focus:bg-orange-500 dark:focus:text-white dark:active:bg-orange-500 dark:active:text-white dark:ring-orange-400/20"
						>Orange</button
					>
					<button
						type="button"
						class="text-sky-500 btn bg-sky-100 hover:text-white hover:bg-sky-600 focus:text-white focus:bg-sky-600 focus:ring focus:ring-sky-100 active:text-white active:bg-sky-600 active:ring active:ring-sky-100 dark:bg-sky-500/20 dark:text-sky-400 dark:hover:bg-sky-500 dark:hover:text-white dark:focus:bg-sky-500 dark:focus:text-white dark:active:bg-sky-500 dark:active:text-white dark:ring-sky-400/20"
						>Sky</button
					>
					<button
						type="button"
						class="text-yellow-500 bg-yellow-100 btn hover:text-white hover:bg-yellow-600 focus:text-white focus:bg-yellow-600 focus:ring focus:ring-yellow-100 active:text-white active:bg-yellow-600 active:ring active:ring-yellow-100 dark:bg-yellow-500/20 dark:text-yellow-500 dark:hover:bg-yellow-500 dark:hover:text-white dark:focus:bg-yellow-500 dark:focus:text-white dark:active:bg-yellow-500 dark:active:text-white dark:ring-yellow-400/20"
						>Yellow</button
					>
					<button
						type="button"
						class="text-red-500 bg-red-100 btn hover:text-white hover:bg-red-600 focus:text-white focus:bg-red-600 focus:ring focus:ring-red-100 active:text-white active:bg-red-600 active:ring active:ring-red-100 dark:bg-red-500/20 dark:text-red-500 dark:hover:bg-red-500 dark:hover:text-white dark:focus:bg-red-500 dark:focus:text-white dark:active:bg-red-500 dark:active:text-white dark:ring-red-400/20"
						>Red</button
					>
					<button
						type="button"
						class="text-purple-500 bg-purple-100 btn hover:text-white hover:bg-purple-600 focus:text-white focus:bg-purple-600 focus:ring focus:ring-purple-100 active:text-white active:bg-purple-600 active:ring active:ring-purple-100 dark:bg-purple-500/20 dark:text-purple-500 dark:hover:bg-purple-500 dark:hover:text-white dark:focus:bg-purple-500 dark:focus:text-white dark:active:bg-purple-500 dark:active:text-white dark:ring-purple-400/20"
						>Purple</button
					>
					<button
						type="button"
						class="text-slate-500 btn bg-slate-100 hover:text-white hover:bg-slate-600 focus:text-white focus:bg-slate-600 focus:ring focus:ring-slate-100 active:text-white active:bg-slate-600 active:ring active:ring-slate-100 dark:bg-slate-500/20 dark:text-slate-400 dark:hover:bg-slate-500 dark:hover:text-white dark:focus:bg-slate-500 dark:focus:text-white dark:active:bg-slate-500 dark:active:text-white dark:ring-slate-400/20"
						>Slate</button
					>
				</div>
			</div>
		</div>
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Dashed Buttons</h6>
				<div class="flex flex-wrap gap-2">
					<button
						type="button"
						class="bg-white border-dashed text-custom-500 btn border-custom-500 hover:text-custom-500 hover:bg-custom-50 hover:border-custom-600 focus:text-custom-600 focus:bg-custom-50 focus:border-custom-600 active:text-custom-600 active:bg-custom-50 active:border-custom-600 dark:bg-zink-700 dark:ring-custom-400/20 dark:hover:bg-custom-800/20 dark:focus:bg-custom-800/20 dark:active:bg-custom-800/20"
						>Custom</button
					>
					<button
						type="button"
						class="text-green-500 bg-white border-green-500 border-dashed btn hover:text-green-500 hover:bg-green-50 hover:border-green-600 focus:text-green-600 focus:bg-green-50 focus:border-green-600 active:text-green-600 active:bg-green-50 active:border-green-600 dark:bg-zink-700 dark:ring-green-400/20 dark:hover:bg-green-800/20 dark:focus:bg-green-800/20 dark:active:bg-green-800/20"
						>Green</button
					>
					<button
						type="button"
						class="text-orange-500 bg-white border-orange-500 border-dashed btn hover:text-orange-500 hover:bg-orange-50 hover:border-orange-600 focus:text-orange-600 focus:bg-orange-50 focus:border-orange-600 active:text-orange-600 active:bg-orange-50 active:border-orange-600 dark:bg-zink-700 dark:ring-orange-400/20 dark:hover:bg-orange-800/20 dark:focus:bg-orange-800/20 dark:active:bg-orange-800/20"
						>Orange</button
					>
					<button
						type="button"
						class="bg-white border-dashed text-sky-500 btn border-sky-500 hover:text-sky-500 hover:bg-sky-50 hover:border-sky-600 focus:text-sky-600 focus:bg-sky-50 focus:border-sky-600 active:text-sky-600 active:bg-sky-50 active:border-sky-600 dark:bg-zink-700 dark:ring-sky-400/20 dark:hover:bg-sky-800/20 dark:focus:bg-sky-800/20 dark:active:bg-sky-800/20"
						>Sky</button
					>
					<button
						type="button"
						class="text-yellow-500 bg-white border-yellow-500 border-dashed btn hover:text-yellow-500 hover:bg-yellow-50 hover:border-yellow-600 focus:text-yellow-600 focus:bg-yellow-50 focus:border-yellow-600 active:text-yellow-600 active:bg-yellow-50 active:border-yellow-600 dark:bg-zink-700 dark:ring-yellow-400/20 dark:hover:bg-yellow-800/20 dark:focus:bg-yellow-800/20 dark:active:bg-yellow-800/20"
						>Yellow</button
					>
					<button
						type="button"
						class="text-red-500 bg-white border-red-500 border-dashed btn hover:text-red-500 hover:bg-red-50 hover:border-red-600 focus:text-red-600 focus:bg-red-50 focus:border-red-600 active:text-red-600 active:bg-red-50 active:border-red-600 dark:bg-zink-700 dark:ring-red-400/20 dark:hover:bg-red-800/20 dark:focus:bg-red-800/20 dark:active:bg-red-800/20"
						>Red</button
					>
					<button
						type="button"
						class="text-purple-500 bg-white border-purple-500 border-dashed btn hover:text-purple-500 hover:bg-purple-50 hover:border-purple-600 focus:text-purple-600 focus:bg-purple-50 focus:border-purple-600 active:text-purple-600 active:bg-purple-50 active:border-purple-600 dark:bg-zink-700 dark:ring-purple-400/20 dark:hover:bg-purple-800/20 dark:focus:bg-purple-800/20 dark:active:bg-purple-800/20"
						>Purple</button
					>
					<button
						type="button"
						class="bg-white border-dashed text-slate-500 btn border-slate-500 hover:text-slate-500 hover:bg-slate-50 hover:border-slate-600 focus:text-slate-600 focus:bg-slate-50 focus:border-slate-600 active:text-slate-600 active:bg-slate-50 active:border-slate-600 dark:bg-zink-700 dark:text-zink-200 dark:border-zink-400 dark:ring-zink-400/20 dark:hover:bg-zink-600 dark:hover:text-zink-100 dark:focus:bg-zink-600 dark:focus:text-zink-100 dark:active:bg-zink-600 dark:active:text-zink-100"
						>Slate</button
					>
				</div>
			</div>
		</div>
		<div class="card customMode2:bg-new">
			<div class="card-body">
				<h6 class="mb-4 text-15">Outline Buttons</h6>
				<div class="flex flex-wrap gap-2">
					<button
						type="button"
						class="bg-white text-custom-500 btn border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:bg-zink-700 dark:hover:bg-custom-500 dark:ring-custom-400/20 dark:focus:bg-custom-500"
						>Custom</button
					>
					<button
						type="button"
						class="text-green-500 bg-white border-green-500 btn hover:text-white hover:bg-green-600 hover:border-green-600 focus:text-white focus:bg-green-600 focus:border-green-600 focus:ring focus:ring-green-100 active:text-white active:bg-green-600 active:border-green-600 active:ring active:ring-green-100 dark:bg-zink-700 dark:hover:bg-green-500 dark:ring-green-400/20 dark:focus:bg-green-500"
						>Green</button
					>
					<button
						type="button"
						class="text-orange-500 bg-white border-orange-500 btn hover:text-white hover:bg-orange-600 hover:border-orange-600 focus:text-white focus:bg-orange-600 focus:border-orange-600 focus:ring focus:ring-orange-100 active:text-white active:bg-orange-600 active:border-orange-600 active:ring active:ring-orange-100 dark:bg-zink-700 dark:hover:bg-orange-500 dark:ring-orange-400/20 dark:focus:bg-orange-500"
						>Orange</button
					>
					<button
						type="button"
						class="bg-white text-sky-500 btn border-sky-500 hover:text-white hover:bg-sky-600 hover:border-sky-600 focus:text-white focus:bg-sky-600 focus:border-sky-600 focus:ring focus:ring-sky-100 active:text-white active:bg-sky-600 active:border-sky-600 active:ring active:ring-sky-100 dark:bg-zink-700 dark:hover:bg-sky-500 dark:ring-sky-400/20 dark:focus:bg-sky-500"
						>Sky</button
					>
					<button
						type="button"
						class="text-yellow-500 bg-white border-yellow-500 btn hover:text-white hover:bg-yellow-600 hover:border-yellow-600 focus:text-white focus:bg-yellow-600 focus:border-yellow-600 focus:ring focus:ring-yellow-100 active:text-white active:bg-yellow-600 active:border-yellow-600 active:ring active:ring-yellow-100 dark:bg-zink-700 dark:hover:bg-yellow-500 dark:ring-yellow-400/20 dark:focus:bg-yellow-500"
						>Yellow</button
					>
					<button
						type="button"
						class="text-red-500 bg-white border-red-500 btn hover:text-white hover:bg-red-600 hover:border-red-600 focus:text-white focus:bg-red-600 focus:border-red-600 focus:ring focus:ring-red-100 active:text-white active:bg-red-600 active:border-red-600 active:ring active:ring-red-100 dark:bg-zink-700 dark:hover:bg-red-500 dark:ring-red-400/20 dark:focus:bg-red-500"
						>Red</button
					>
					<button
						type="button"
						class="text-purple-500 bg-white border-purple-500 btn hover:text-white hover:bg-purple-600 hover:border-purple-600 focus:text-white focus:bg-purple-600 focus:border-purple-600 focus:ring focus:ring-purple-100 active:text-white active:bg-purple-600 active:border-purple-600 active:ring active:ring-purple-100 dark:bg-zink-700 dark:hover:bg-purple-500 dark:ring-purple-400/20 dark:focus:bg-purple-500"
						>Purple</button
					>
					<button
						type="button"
						class="bg-white text-slate-500 btn border-slate-500 hover:text-white hover:bg-slate-600 hover:border-slate-600 focus:text-white focus:bg-slate-600 focus:border-slate-600 focus:ring focus:ring-slate-100 active:text-white active:bg-slate-600 active:border-slate-600 active:ring active:ring-slate-100 dark:bg-zink-700 dark:hover:bg-slate-500 dark:ring-slate-400/20 dark:focus:bg-slate-500"
						>Slate</button
					>
				</div>
			</div>
		</div>
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Ghost Buttons</h6>
				<div class="flex flex-wrap gap-2">
					<button
						type="button"
						class="bg-white text-custom-500 btn hover:text-custom-500 hover:bg-custom-100 focus:text-custom-500 focus:bg-custom-100 active:text-custom-500 active:bg-custom-100 dark:bg-zink-700 dark:hover:bg-custom-500/10 dark:focus:bg-custom-500/10 dark:active:bg-custom-500/10"
						>Custom</button
					>
					<button
						type="button"
						class="text-green-500 bg-white btn hover:text-green-500 hover:bg-green-100 focus:text-green-500 focus:bg-green-100 active:text-green-500 active:bg-green-100 dark:bg-zink-700 dark:hover:bg-green-500/10 dark:focus:bg-green-500/10 dark:active:bg-green-500/10"
						>Green</button
					>
					<button
						type="button"
						class="text-orange-500 bg-white btn hover:text-orange-500 hover:bg-orange-100 focus:text-orange-500 focus:bg-orange-100 active:text-orange-500 active:bg-orange-100 dark:bg-zink-700 dark:hover:bg-orange-500/10 dark:focus:bg-orange-500/10 dark:active:bg-orange-500/10"
						>Orange</button
					>
					<button
						type="button"
						class="bg-white text-sky-500 btn hover:text-sky-500 hover:bg-sky-100 focus:text-sky-500 focus:bg-sky-100 active:text-sky-500 active:bg-sky-100 dark:bg-zink-700 dark:hover:bg-sky-500/10 dark:focus:bg-sky-500/10 dark:active:bg-sky-500/10"
						>Sky</button
					>
					<button
						type="button"
						class="text-yellow-500 bg-white btn hover:text-yellow-500 hover:bg-yellow-100 focus:text-yellow-500 focus:bg-yellow-100 active:text-yellow-500 active:bg-yellow-100 dark:bg-zink-700 dark:hover:bg-yellow-500/10 dark:focus:bg-yellow-500/10 dark:active:bg-yellow-500/10"
						>Yellow</button
					>
					<button
						type="button"
						class="text-red-500 bg-white btn hover:text-red-500 hover:bg-red-100 focus:text-red-500 focus:bg-red-100 active:text-red-500 active:bg-red-100 dark:bg-zink-700 dark:hover:bg-red-500/10 dark:focus:bg-red-500/10 dark:active:bg-red-500/10"
						>Red</button
					>
					<button
						type="button"
						class="text-purple-500 bg-white btn hover:text-purple-500 hover:bg-purple-100 focus:text-purple-500 focus:bg-purple-100 active:text-purple-500 active:bg-purple-100 dark:bg-zink-700 dark:hover:bg-purple-500/10 dark:focus:bg-purple-500/10 dark:active:bg-purple-500/10"
						>Purple</button
					>
					<button
						type="button"
						class="bg-white text-slate-500 btn hover:text-slate-500 hover:bg-slate-100 focus:text-slate-500 focus:bg-slate-100 active:text-slate-500 active:bg-slate-100 dark:bg-zink-700 dark:hover:bg-slate-500/10 dark:focus:bg-slate-500/10 dark:active:bg-slate-500/10"
						>Slate</button
					>
				</div>
			</div>
		</div>
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Animation Buttons</h6>
				<div class="flex flex-wrap gap-3">
					<button
						type="button"
						class="flex items-center justify-center size-[37.5px] p-0 text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
					>
						<svg
							class="size-4 animate-spin"
							xmlns="http://www.w3.org/2000/svg"
							fill="none"
							viewBox="0 0 24 24"
						>
							<circle
								class="opacity-25"
								cx="12"
								cy="12"
								r="10"
								stroke="currentColor"
								stroke-width="4"
							></circle>
							<path
								class="opacity-75"
								fill="currentColor"
								d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
							></path>
						</svg>
					</button>
					<button
						type="button"
						class="flex items-center text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
					>
						<svg
							class="size-4 ltr:mr-2 rtl:ml-2 animate-spin"
							xmlns="http://www.w3.org/2000/svg"
							fill="none"
							viewBox="0 0 24 24"
						>
							<circle
								class="opacity-25"
								cx="12"
								cy="12"
								r="10"
								stroke="currentColor"
								stroke-width="4"
							></circle>
							<path
								class="opacity-75"
								fill="currentColor"
								d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
							></path>
						</svg>
						Processing...
					</button>
					<button
						type="button"
						class="relative bg-white border-dashed text-custom-500 btn border-custom-500 hover:text-custom-500 hover:bg-custom-50 hover:border-custom-600 focus:text-custom-600 focus:bg-custom-50 focus:border-custom-600 active:text-custom-600 active:bg-custom-50 active:border-custom-600 dark:bg-zink-700 dark:ring-custom-400/20 dark:hover:bg-custom-800/20 dark:focus:bg-custom-800/20 dark:active:bg-custom-800/20"
					>
						<span class="absolute flex size-2 -top-1 ltr:-right-1 rtl:-left-1">
							<span
								class="absolute inline-flex w-full h-full rounded-full opacity-75 animate-ping bg-sky-400"
							></span>
							<span class="relative inline-flex size-2 rounded-full bg-sky-500"></span>
						</span>
						Dashed Buttons
					</button>
					<button
						type="button"
						class="flex animate-bounce items-center justify-center size-[37.5px] rounded-full p-0 text-green-500 bg-white border-green-500 btn hover:text-white hover:bg-green-600 hover:border-green-600 focus:text-white focus:bg-green-600 focus:border-green-600 focus:ring focus:ring-green-100 active:text-white active:bg-green-600 active:border-green-600 active:ring active:ring-green-100 dark:bg-zink-700 dark:hover:bg-green-500 dark:ring-green-400/20 dark:focus:bg-green-500"
						><i class="ri-download-2-line"></i></button
					>
				</div>
			</div>
		</div>
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Icon Buttons</h6>
				<div class="grid grid-cols-1 gap-5 xl:grid-cols-2">
					<div class="flex flex-wrap gap-2">
						<button
							type="button"
							class="flex items-center justify-center size-[37.5px] p-0 text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
							><i class="ri-download-2-line"></i></button
						>
						<button
							type="button"
							class="flex items-center justify-center size-[37.5px] p-0 text-green-500 bg-white border-green-500 btn hover:text-white hover:bg-green-600 hover:border-green-600 focus:text-white focus:bg-green-600 focus:border-green-600 focus:ring focus:ring-green-100 active:text-white active:bg-green-600 active:border-green-600 active:ring active:ring-green-100 dark:bg-zink-700 dark:hover:bg-green-500 dark:ring-green-400/20 dark:focus:bg-green-500"
							><i class="ri-download-2-line"></i></button
						>
						<button
							type="button"
							class="flex items-center justify-center size-[37.5px] transition-all duration-200 ease-linear p-0 bg-white border-dashed text-yellow-500 btn border-yellow-500 hover:text-yellow-500 hover:bg-yellow-50 hover:border-yellow-600 focus:text-yellow-600 focus:bg-yellow-50 focus:border-yellow-600 active:text-yellow-600 active:bg-yellow-50 active:border-yellow-600 dark:bg-zink-700 dark:ring-yellow-400/20 dark:hover:bg-yellow-800/20 dark:focus:bg-yellow-800/20 dark:active:bg-yellow-800/20"
							><i class="ri-download-2-line"></i></button
						>
						<button
							type="button"
							class="flex items-center justify-center size-[37.5px] transition-all duration-200 ease-linear p-0 text-sky-500 btn bg-sky-100 hover:text-white hover:bg-sky-600 focus:text-white focus:bg-sky-600 focus:ring focus:ring-sky-100 active:text-white active:bg-sky-600 active:ring active:ring-sky-100 dark:bg-sky-500/20 dark:text-sky-400 dark:hover:bg-sky-500 dark:hover:text-white dark:focus:bg-sky-500 dark:focus:text-white dark:active:bg-sky-500 dark:active:text-white dark:ring-sky-400/20"
							><i class="ri-download-2-line"></i></button
						>
						<button
							type="button"
							class="flex items-center justify-center size-[37.5px] transition-all duration-200 ease-linear p-0 text-red-500 bg-white btn hover:text-red-500 hover:bg-red-100 focus:text-red-500 focus:bg-red-100 active:text-red-500 active:bg-red-100 dark:bg-zink-700 dark:hover:bg-red-500/10 dark:focus:bg-red-500/10 dark:active:bg-red-500/10"
							><i class="ri-download-2-line"></i></button
						>
					</div>
					<div class="flex flex-wrap gap-2">
						<button
							type="button"
							class="flex items-center justify-center size-[37.5px] p-0 text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
							><i class="ri-download-2-line"></i></button
						>
						<button
							type="button"
							class="flex items-center justify-center size-[37.5px] p-0 text-white btn bg-green-500 border-green-500 hover:text-white hover:bg-green-600 hover:border-green-600 focus:text-white focus:bg-green-600 focus:border-green-600 focus:ring focus:ring-green-100 active:text-white active:bg-green-600 active:border-green-600 active:ring active:ring-green-100 dark:ring-green-400/20"
							><i class="ri-download-2-line"></i></button
						>
						<button
							type="button"
							class="flex items-center justify-center size-[37.5px] p-0 text-white btn bg-yellow-500 border-yellow-500 hover:text-white hover:bg-yellow-600 hover:border-yellow-600 focus:text-white focus:bg-yellow-600 focus:border-yellow-600 focus:ring focus:ring-yellow-100 active:text-white active:bg-yellow-600 active:border-yellow-600 active:ring active:ring-yellow-100 dark:ring-yellow-400/20"
							><i class="ri-download-2-line"></i></button
						>
						<button
							type="button"
							class="flex items-center justify-center size-[37.5px] p-0 text-white btn bg-sky-500 border-sky-500 hover:text-white hover:bg-sky-600 hover:border-sky-600 focus:text-white focus:bg-sky-600 focus:border-sky-600 focus:ring focus:ring-sky-100 active:text-white active:bg-sky-600 active:border-sky-600 active:ring active:ring-sky-100 dark:ring-sky-400/20"
							><i class="ri-download-2-line"></i></button
						>
						<button
							type="button"
							class="flex items-center justify-center size-[37.5px] p-0 text-white btn bg-red-500 border-red-500 hover:text-white hover:bg-red-600 hover:border-red-600 focus:text-white focus:bg-red-600 focus:border-red-600 focus:ring focus:ring-red-100 active:text-white active:bg-red-600 active:border-red-600 active:ring active:ring-red-100 dark:ring-red-400/20"
							><i class="ri-download-2-line"></i></button
						>
						<button
							type="button"
							class="flex items-center justify-center size-[37.5px] p-0 text-white btn bg-purple-500 border-purple-500 hover:text-white hover:bg-purple-600 hover:border-purple-600 focus:text-white focus:bg-purple-600 focus:border-purple-600 focus:ring focus:ring-purple-100 active:text-white active:bg-purple-600 active:border-purple-600 active:ring active:ring-purple-100 dark:ring-purple-400/20"
							><i class="ri-download-2-line"></i></button
						>
						<button
							type="button"
							class="flex items-center justify-center size-[37.5px] p-0 text-white btn bg-slate-500 border-slate-500 hover:text-white hover:bg-slate-600 hover:border-slate-600 focus:text-white focus:bg-slate-600 focus:border-slate-600 focus:ring focus:ring-slate-100 active:text-white active:bg-slate-600 active:border-slate-600 active:ring active:ring-slate-100 dark:ring-slate-400/20"
							><i class="ri-download-2-line"></i></button
						>
					</div>
				</div>
			</div>
		</div>
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Buttons Sizes</h6>
				<div class="flex flex-wrap items-center gap-2">
					<button
						type="button"
						class="py-1 text-xs px-1.5 text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
						>Custom Button</button
					>
					<button
						type="button"
						class="px-2 py-1.5 text-xs text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
						>Custom Button</button
					>
					<button
						type="button"
						class="px-2.5 py-2 text-xs text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
						>Custom Button</button
					>
					<button
						type="button"
						class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
						>Custom Button</button
					>
					<button
						type="button"
						class="px-4 py-2.5 text-15 text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
						>Custom Button</button
					>
				</div>
			</div>
		</div>
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Rounded Buttons</h6>
				<div class="grid grid-cols-1 gap-5 xl:grid-cols-2">
					<div class="flex flex-wrap gap-2">
						<button
							type="button"
							class="text-white rounded-full btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
							>Custom Button</button
						>
						<button
							type="button"
							class="rounded-full text-custom-500 btn bg-custom-100 hover:text-white hover:bg-custom-600 focus:text-white focus:bg-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:ring active:ring-custom-100 dark:bg-custom-500/20 dark:text-custom-500 dark:hover:bg-custom-500 dark:hover:text-white dark:focus:bg-custom-500 dark:focus:text-white dark:active:bg-custom-500 dark:active:text-white dark:ring-custom-400/20"
							>Soft Button</button
						>
						<button
							type="button"
							class="bg-white border-dashed rounded-full text-custom-500 btn border-custom-500 hover:text-custom-500 hover:bg-custom-50 hover:border-custom-600 focus:text-custom-600 focus:bg-custom-50 focus:border-custom-600 active:text-custom-600 active:bg-custom-50 active:border-custom-600 dark:bg-zink-700 dark:ring-custom-400/20 dark:hover:bg-custom-800/20 dark:focus:bg-custom-800/20 dark:active:bg-custom-800/20"
							>Dashed Buttons</button
						>
						<button
							type="button"
							class="bg-white rounded-full text-custom-500 btn border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:bg-zink-700 dark:hover:bg-custom-500 dark:ring-custom-400/20 dark:focus:bg-custom-500"
							>Outline Button</button
						>
						<button
							type="button"
							class="bg-white rounded-full text-custom-500 btn hover:text-custom-500 hover:bg-custom-100 focus:text-custom-500 focus:bg-custom-100 active:text-custom-500 active:bg-custom-100 dark:bg-zink-700 dark:hover:bg-custom-500/10 dark:focus:bg-custom-500/10 dark:active:bg-custom-500/10"
							>Ghost Button</button
						>
					</div>
					<div class="flex flex-wrap gap-2">
						<button
							type="button"
							class="flex rounded-full items-center justify-center size-[37.5px] p-0 text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
							><i class="ri-download-2-line"></i></button
						>
						<button
							type="button"
							class="flex rounded-full items-center justify-center size-[37.5px] p-0 text-green-500 bg-white border-green-500 btn hover:text-white hover:bg-green-600 hover:border-green-600 focus:text-white focus:bg-green-600 focus:border-green-600 focus:ring focus:ring-green-100 active:text-white active:bg-green-600 active:border-green-600 active:ring active:ring-green-100 dark:bg-zink-700 dark:hover:bg-green-500 dark:ring-green-400/20 dark:focus:bg-green-500"
							><i class="ri-download-2-line"></i></button
						>
						<button
							type="button"
							class="flex rounded-full items-center justify-center size-[37.5px] transition-all duration-200 ease-linear p-0 bg-white border-dashed text-yellow-500 btn border-yellow-500 hover:text-yellow-500 hover:bg-yellow-50 hover:border-yellow-600 focus:text-yellow-600 focus:bg-yellow-50 focus:border-yellow-600 active:text-yellow-600 active:bg-yellow-50 active:border-yellow-600 dark:bg-zink-700 dark:ring-yellow-400/20 dark:hover:bg-yellow-800/20 dark:focus:bg-yellow-800/20 dark:active:bg-yellow-800/20"
							><i class="ri-download-2-line"></i></button
						>
						<button
							type="button"
							class="flex rounded-full items-center justify-center size-[37.5px] transition-all duration-200 ease-linear p-0 text-sky-500 btn bg-sky-100 hover:text-white hover:bg-sky-600 focus:text-white focus:bg-sky-600 focus:ring focus:ring-sky-100 active:text-white active:bg-sky-600 active:ring active:ring-sky-100 dark:bg-sky-500/20 dark:text-sky-400 dark:hover:bg-sky-500 dark:hover:text-white dark:focus:bg-sky-500 dark:focus:text-white dark:active:bg-sky-500 dark:active:text-white dark:ring-sky-400/20"
							><i class="ri-download-2-line"></i></button
						>
						<button
							type="button"
							class="flex rounded-full items-center justify-center size-[37.5px] transition-all duration-200 ease-linear p-0 text-red-500 bg-white btn hover:text-red-500 hover:bg-red-100 focus:text-red-500 focus:bg-red-100 active:text-red-500 active:bg-red-100 dark:bg-zink-700 dark:hover:bg-red-500/10 dark:focus:bg-red-500/10 dark:active:bg-red-500/10"
							><i class="ri-download-2-line"></i></button
						>
					</div>
				</div>
			</div>
		</div>
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Label Buttons</h6>
				<div class="grid grid-cols-1 gap-5 xl:grid-cols-2">
					<div class="flex flex-wrap items-center gap-2">
						<button
							type="button"
							class="py-1 text-xs ltr:pl-[calc(theme('spacing.1')_+_26px)] rtl:pr-[calc(theme('spacing.1')_+_26px)] relative px-1.5 text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
							><i
								class="ri-download-2-line w-[26px] bg-white/10 flex absolute ltr:-left-[1px] rtl:-right-[1px] -bottom-[1px] -top-[1px] items-center justify-center"
							></i> Download</button
						>
						<button
							type="button"
							class="px-2 py-1.5 ltr:pl-[calc(theme('spacing.2')_+_30px)] rtl:pr-[calc(theme('spacing.2')_+_30px)] relative text-xs text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
							><i
								class="ri-download-2-line w-[30px] bg-white/10 flex absolute ltr:-left-[1px] rtl:-right-[1px] -bottom-[1px] -top-[1px] items-center justify-center"
							></i> Download</button
						>
						<button
							type="button"
							class="px-2.5 py-2 ltr:pl-[calc(theme('spacing.2')_*_5.5)] rtl:pr-[calc(theme('spacing.2')_*_5.5)] text-xs relative text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
							><i
								class="ri-download-2-line w-[34px] bg-white/10 flex absolute ltr:-left-[1px] rtl:-right-[1px] -bottom-[1px] -top-[1px] items-center justify-center"
							></i> Download</button
						>
						<button
							type="button"
							class="relative ltr:pl-[calc(theme('spacing.3')_*_4.2)] rtl:pr-[calc(theme('spacing.3')_*_4.2)] text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
							><i
								class="ri-download-2-line w-[37.5px] bg-white/10 flex absolute ltr:-left-[1px] rtl:-right-[1px] -bottom-[1px] -top-[1px] items-center justify-center"
							></i> Download</button
						>
						<button
							type="button"
							class="relative px-4 py-2.5 ltr:pl-[calc(theme('spacing.4')_+_44.5px)] rtl:pr-[calc(theme('spacing.4')_+_44.5px)] text-15 text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
							><i
								class="ri-download-2-line w-[44.5px] bg-white/10 flex absolute ltr:-left-[1px] rtl:-right-[1px] -bottom-[1px] -top-[1px] items-center justify-center"
							></i> Download</button
						>
					</div>
					<div class="flex flex-wrap items-center gap-2">
						<button
							type="button"
							class="py-1 text-xs rtl:pl-[calc(theme('spacing.1')_+_26px)] ltr:pr-[calc(theme('spacing.1')_+_26px)] relative px-1.5 text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
							><i
								class="ri-download-2-line w-[26px] bg-white/10 flex absolute rtl:-left-[1px] ltr:-right-[1px] -bottom-[1px] -top-[1px] items-center justify-center"
							></i> Download</button
						>
						<button
							type="button"
							class="px-2 py-1.5 rtl:pl-[calc(theme('spacing.2')_+_30px)] ltr:pr-[calc(theme('spacing.2')_+_30px)] relative text-xs text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
							><i
								class="ri-download-2-line w-[30px] bg-white/10 flex absolute rtl:-left-[1px] ltr:-right-[1px] -bottom-[1px] -top-[1px] items-center justify-center"
							></i> Download</button
						>
						<button
							type="button"
							class="px-2.5 py-2 rtl:pl-[calc(theme('spacing.2')_*_5.5)] ltr:pr-[calc(theme('spacing.2')_*_5.5)] text-xs relative text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
							><i
								class="ri-download-2-line w-[34px] bg-white/10 flex absolute rtl:-left-[1px] ltr:-right-[1px] -bottom-[1px] -top-[1px] items-center justify-center"
							></i> Download</button
						>
						<button
							type="button"
							class="relative rtl:pl-[calc(theme('spacing.3')_*_4.2)] ltr:pr-[calc(theme('spacing.3')_*_4.2)] text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
							><i
								class="ri-download-2-line w-[37.5px] bg-white/10 flex absolute rtl:-left-[1px] ltr:-right-[1px] -bottom-[1px] -top-[1px] items-center justify-center"
							></i> Download</button
						>
						<button
							type="button"
							class="relative px-4 py-3 rtl:pl-[calc(theme('spacing.4')_+_44.5px)] ltr:pr-[calc(theme('spacing.4')_+_44.5px)] text-15 text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
							><i
								class="ri-download-2-line w-[44.5px] bg-white/10 flex absolute rtl:-left-[1px] ltr:-right-[1px] -bottom-[1px] -top-[1px] items-center justify-center"
							></i> Download</button
						>
					</div>
				</div>
			</div>
		</div>
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Icon with Buttons</h6>
				<div class="flex flex-wrap items-center gap-2">
					<button
						type="button"
						class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
						><i class="align-baseline ltr:pr-1 rtl:pl-1 ri-download-2-line"></i> Download</button
					>
					<button
						type="button"
						class="text-red-500 bg-red-100 btn hover:text-white hover:bg-red-600 focus:text-white focus:bg-red-600 focus:ring focus:ring-red-100 active:text-white active:bg-red-600 active:ring active:ring-red-100 dark:bg-red-500/20 dark:text-red-500 dark:hover:bg-red-500 dark:hover:text-white dark:focus:bg-red-500 dark:focus:text-white dark:active:bg-red-500 dark:active:text-white dark:ring-red-400/20"
						>Close <i class="align-baseline ltr:pl-1 rtl:pr-1 ri-close-line"></i></button
					>
					<button
						type="button"
						class="bg-white border-dashed text-sky-500 btn border-sky-500 hover:text-sky-500 hover:bg-sky-50 hover:border-sky-600 focus:text-sky-600 focus:bg-sky-50 focus:border-sky-600 active:text-sky-600 active:bg-sky-50 active:border-sky-600 dark:bg-zink-700 dark:ring-sky-400/20 dark:hover:bg-sky-800/20 dark:focus:bg-sky-800/20 dark:active:bg-sky-800/20"
						><i class="align-baseline ltr:pr-1 rtl:pl-1 ri-refresh-line"></i> Progress</button
					>
				</div>
			</div>
		</div>
	</div>
</div>
