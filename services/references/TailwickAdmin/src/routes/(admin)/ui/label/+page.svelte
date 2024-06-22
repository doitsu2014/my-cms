<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import LucideIcon from '../../../../common/components/LucideIcon.svelte';
</script>

<HeadTitle title="Label" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Label" pagetitle="UI Elements" />

	<div>
		<div class="grid grid-cols-1 gap-x-5 xl:grid-cols-2">
			<div class="card">
				<div class="card-body">
					<h6 class="mb-4 text-15">Default Badge</h6>
					<div class="flex flex-wrap items-center gap-2">
						<span
							class="px-2.5 py-0.5 inline-block text-xs font-medium rounded border bg-custom-100 border-transparent text-custom-500 dark:bg-custom-500/20 dark:border-transparent"
							>Custom</span
						>
						<span
							class="px-2.5 py-0.5 inline-block text-xs font-medium rounded border bg-green-100 border-transparent text-green-500 dark:bg-green-500/20 dark:border-transparent"
							>Green</span
						>
						<span
							class="px-2.5 py-0.5 inline-block text-xs font-medium rounded border bg-orange-100 border-transparent text-orange-500 dark:bg-orange-500/20 dark:border-transparent"
							>Orange</span
						>
						<span
							class="px-2.5 py-0.5 inline-block text-xs font-medium rounded border bg-sky-100 border-transparent text-sky-500 dark:bg-sky-500/20 dark:border-transparent"
							>Sky</span
						>
						<span
							class="px-2.5 py-0.5 inline-block text-xs font-medium rounded border bg-yellow-100 border-transparent text-yellow-500 dark:bg-yellow-500/20 dark:border-transparent"
							>Yellow</span
						>
						<span
							class="px-2.5 py-0.5 inline-block text-xs font-medium rounded border bg-red-100 border-transparent text-red-500 dark:bg-red-500/20 dark:border-transparent"
							>Red</span
						>
						<span
							class="px-2.5 py-0.5 inline-block text-xs font-medium rounded border bg-purple-100 border-transparent text-purple-500 dark:bg-purple-500/20 dark:border-transparent"
							>Purple</span
						>
						<span
							class="px-2.5 py-0.5 inline-block text-xs font-medium rounded border bg-slate-100 border-transparent text-slate-500 dark:bg-slate-500/20 dark:text-zink-200 dark:border-transparent"
							>Slate</span
						>
					</div>
				</div>
			</div>
			<!--end card-->
			<div class="card">
				<div class="card-body">
					<h6 class="mb-4 text-15">Outline Badge</h6>
					<div class="flex flex-wrap items-center gap-2">
						<span
							class="px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-white border-custom-400 text-custom-500 dark:bg-zink-700 dark:border-custom-700"
							>Custom</span
						>
						<span
							class="px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-white border-green-400 text-green-500 dark:bg-zink-700 dark:border-green-700"
							>Green</span
						>
						<span
							class="px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-white border-orange-400 text-orange-500 dark:bg-zink-700 dark:border-orange-700"
							>Orange</span
						>
						<span
							class="px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-white border-sky-400 text-sky-500 dark:bg-zink-700 dark:border-sky-700"
							>Sky</span
						>
						<span
							class="px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-white border-yellow-400 text-yellow-500 dark:bg-zink-700 dark:border-yellow-700"
							>Yellow</span
						>
						<span
							class="px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-white border-red-400 text-red-500 dark:bg-zink-700 dark:border-red-700"
							>Red</span
						>
						<span
							class="px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-white border-purple-400 text-purple-500 dark:bg-zink-700 dark:border-purple-700"
							>Purple</span
						>
						<span
							class="px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-white border-slate-400 text-slate-500 dark:bg-zink-700 dark:border-zink-400 dark:text-zink-200"
							>Slate</span
						>
					</div>
				</div>
			</div>
			<!--end card-->
		</div>
		<!--end grid-->
		<div class="grid grid-cols-1 gap-x-5 xl:grid-cols-2">
			<div class="card">
				<div class="card-body">
					<h6 class="mb-4 text-15">Bordered Badge</h6>
					<div class="flex flex-wrap items-center gap-2">
						<span
							class="px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-custom-100 border-custom-200 text-custom-500 dark:bg-custom-500/20 dark:border-custom-500/20"
							>Custom</span
						>
						<span
							class="px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-green-100 border-green-200 text-green-500 dark:bg-green-500/20 dark:border-green-500/20"
							>Green</span
						>
						<span
							class="px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-orange-100 border-orange-200 text-orange-500 dark:bg-orange-500/20 dark:border-orange-500/20"
							>Orange</span
						>
						<span
							class="px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-sky-100 border-sky-200 text-sky-500 dark:bg-sky-500/20 dark:border-sky-500/20"
							>Sky</span
						>
						<span
							class="px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-yellow-100 border-yellow-200 text-yellow-500 dark:bg-yellow-500/20 dark:border-yellow-500/20"
							>Yellow</span
						>
						<span
							class="px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-red-100 border-red-200 text-red-500 dark:bg-red-500/20 dark:border-red-500/20"
							>Red</span
						>
						<span
							class="px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-purple-100 border-purple-200 text-purple-500 dark:bg-purple-500/20 dark:border-purple-500/20"
							>Purple</span
						>
						<span
							class="px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-slate-100 border-slate-200 text-slate-500 dark:bg-slate-500/20 dark:border-slate-500/20 dark:text-zink-200"
							>Slate</span
						>
					</div>
				</div>
			</div>
			<!--end card-->
			<div class="card">
				<div class="card-body">
					<h6 class="mb-4 text-15">Soft Outline Badge</h6>
					<div class="flex flex-wrap items-center gap-2">
						<span
							class="px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-white border-custom-100 text-custom-500 dark:bg-zink-700 dark:border-custom-900"
							>Custom</span
						>
						<span
							class="px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-white border-green-100 text-green-500 dark:bg-zink-700 dark:border-green-900"
							>Green</span
						>
						<span
							class="px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-white border-orange-100 text-orange-500 dark:bg-zink-700 dark:border-orange-900"
							>Orange</span
						>
						<span
							class="px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-white border-sky-100 text-sky-500 dark:bg-zink-700 dark:border-sky-900"
							>Sky</span
						>
						<span
							class="px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-white border-yellow-100 text-yellow-500 dark:bg-zink-700 dark:border-yellow-900"
							>Yellow</span
						>
						<span
							class="px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-white border-red-100 text-red-500 dark:bg-zink-700 dark:border-red-900"
							>Red</span
						>
						<span
							class="px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-white border-purple-100 text-purple-500 dark:bg-zink-700 dark:border-purple-900"
							>Purple</span
						>
						<span
							class="px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-white border-slate-100 text-slate-500 dark:bg-zink-700 dark:border-zink-500 dark:text-zink-200"
							>Slate</span
						>
					</div>
				</div>
			</div>
			<!--end card-->
		</div>
		<!--end grid-->
		<div class="grid grid-cols-1 gap-x-5 xl:grid-cols-2">
			<div class="card">
				<div class="card-body">
					<h6 class="mb-4 text-15">Pills Badge</h6>
					<div class="flex flex-wrap items-center gap-2">
						<span
							class="px-2.5 py-0.5 inline-block text-xs font-medium rounded-full border bg-custom-100 border-transparent text-custom-500 dark:bg-custom-500/20 dark:border-transparent"
							>Custom</span
						>
						<span
							class="px-2.5 py-0.5 inline-block text-xs font-medium rounded-full border bg-green-100 border-transparent text-green-500 dark:bg-green-500/20 dark:border-transparent"
							>Green</span
						>
						<span
							class="px-2.5 py-0.5 inline-block text-xs font-medium rounded-full border bg-orange-100 border-transparent text-orange-500 dark:bg-orange-500/20 dark:border-transparent"
							>Orange</span
						>
						<span
							class="px-2.5 py-0.5 inline-block text-xs font-medium rounded-full border bg-sky-100 border-transparent text-sky-500 dark:bg-sky-500/20 dark:border-transparent"
							>Sky</span
						>
						<span
							class="px-2.5 py-0.5 inline-block text-xs font-medium rounded-full border bg-yellow-100 border-transparent text-yellow-500 dark:bg-yellow-500/20 dark:border-transparent"
							>Yellow</span
						>
						<span
							class="px-2.5 py-0.5 inline-block text-xs font-medium rounded-full border bg-red-100 border-transparent text-red-500 dark:bg-red-500/20 dark:border-transparent"
							>Red</span
						>
						<span
							class="px-2.5 py-0.5 inline-block text-xs font-medium rounded-full border bg-purple-100 border-transparent text-purple-500 dark:bg-purple-500/20 dark:border-transparent"
							>Purple</span
						>
						<span
							class="px-2.5 py-0.5 inline-block text-xs font-medium rounded-full border bg-slate-100 border-transparent text-slate-500 dark:bg-slate-500/20 dark:text-zink-200 dark:border-transparent"
							>Slate</span
						>
					</div>
				</div>
			</div>
			<!--end card-->
			<div class="card">
				<div class="card-body">
					<h6 class="mb-4 text-15">Pills Outline Badge</h6>
					<div class="flex flex-wrap items-center gap-2">
						<span
							class="px-2.5 py-0.5 text-xs inline-block font-medium rounded-full border bg-white border-custom-400 text-custom-500 dark:bg-zink-700 dark:border-custom-700"
							>Custom</span
						>
						<span
							class="px-2.5 py-0.5 text-xs inline-block font-medium rounded-full border bg-white border-green-400 text-green-500 dark:bg-zink-700 dark:border-green-700"
							>Green</span
						>
						<span
							class="px-2.5 py-0.5 text-xs inline-block font-medium rounded-full border bg-white border-orange-400 text-orange-500 dark:bg-zink-700 dark:border-orange-700"
							>Orange</span
						>
						<span
							class="px-2.5 py-0.5 text-xs inline-block font-medium rounded-full border bg-white border-sky-400 text-sky-500 dark:bg-zink-700 dark:border-sky-700"
							>Sky</span
						>
						<span
							class="px-2.5 py-0.5 text-xs inline-block font-medium rounded-full border bg-white border-yellow-400 text-yellow-500 dark:bg-zink-700 dark:border-yellow-700"
							>Yellow</span
						>
						<span
							class="px-2.5 py-0.5 text-xs inline-block font-medium rounded-full border bg-white border-red-400 text-red-500 dark:bg-zink-700 dark:border-red-700"
							>Red</span
						>
						<span
							class="px-2.5 py-0.5 text-xs inline-block font-medium rounded-full border bg-white border-purple-400 text-purple-500 dark:bg-zink-700 dark:border-purple-700"
							>Purple</span
						>
						<span
							class="px-2.5 py-0.5 text-xs inline-block font-medium rounded-full border bg-white border-slate-400 text-slate-500 dark:bg-zink-700 dark:border-zink-400 dark:text-zink-200"
							>Slate</span
						>
					</div>
				</div>
			</div>
			<!--end card-->
		</div>
		<!--end grid-->
		<div class="grid grid-cols-1 gap-x-5 xl:grid-cols-2">
			<div class="card">
				<div class="card-body">
					<h6 class="mb-4 text-15">Dismissible Badges</h6>
					<div class="flex flex-wrap items-center gap-2">
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded border bg-custom-100 border-transparent text-custom-500 dark:bg-custom-500/20 dark:border-transparent"
							>Custom <a href="#!" class="transition text-custom-400 hover:text-custom-600"
								><LucideIcon name="X" class="size-3 ltr:ml-1 rtl:mr-1" /></a
							></span
						>
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded border bg-green-100 border-transparent text-green-500 dark:bg-green-500/20 dark:border-transparent"
							>Green <a href="#!" class="text-green-400 transition hover:text-green-600"
								><LucideIcon name="X" class="size-3 ltr:ml-1 rtl:mr-1" /></a
							></span
						>
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded border bg-orange-100 border-transparent text-orange-500 dark:bg-orange-500/20 dark:border-transparent"
							>Orange <a href="#!" class="text-orange-400 transition hover:text-orange-600"
								><LucideIcon name="X" class="size-3 ltr:ml-1 rtl:mr-1" /></a
							></span
						>
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded border bg-sky-100 border-transparent text-sky-500 dark:bg-sky-500/20 dark:border-transparent"
							>Sky <a href="#!" class="transition text-sky-400 hover:text-sky-600"
								><LucideIcon name="X" class="size-3 ltr:ml-1 rtl:mr-1" /></a
							></span
						>
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded border bg-yellow-100 border-transparent text-yellow-500 dark:bg-yellow-500/20 dark:border-transparent"
							>Yellow <a href="#!" class="text-yellow-400 transition hover:text-yellow-600"
								><LucideIcon name="X" class="size-3 ltr:ml-1 rtl:mr-1" /></a
							></span
						>
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded border bg-red-100 border-transparent text-red-500 dark:bg-red-500/20 dark:border-transparent"
							>Red <a href="#!" class="text-red-400 transition hover:text-red-600"
								><LucideIcon name="X" class="size-3 ltr:ml-1 rtl:mr-1" /></a
							></span
						>
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded border bg-purple-100 border-transparent text-purple-500 dark:bg-purple-500/20 dark:border-transparent"
							>Purple <a href="#!" class="text-purple-400 transition hover:text-purple-600"
								><LucideIcon name="X" class="size-3 ltr:ml-1 rtl:mr-1" /></a
							></span
						>
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded border bg-slate-100 border-transparent text-slate-500 dark:bg-slate-500/20 dark:text-zink-200 dark:border-transparent"
							>Slate <a href="#!" class="transition text-slate-400 hover:text-slate-600"
								><LucideIcon name="X" class="size-3 ltr:ml-1 rtl:mr-1" /></a
							></span
						>
					</div>
				</div>
			</div>
			<!--end card-->
			<div class="card">
				<div class="card-body">
					<h6 class="mb-4 text-15">Dismissible Outline Badge</h6>
					<div class="flex flex-wrap items-center gap-2">
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded border bg-white border-custom-400 text-custom-500 dark:bg-zink-700 dark:border-custom-700"
							>Custom <a href="#!" class="transition text-custom-400 hover:text-custom-600"
								><LucideIcon name="X" class="size-3 ltr:ml-1 rtl:mr-1" /></a
							></span
						>
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded border bg-white border-green-400 text-green-500 dark:bg-zink-700 dark:border-green-700"
							>Green<a href="#!" class="text-green-400 transition hover:text-green-600"
								><LucideIcon name="X" class="size-3 ltr:ml-1 rtl:mr-1" /></a
							></span
						>
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded border bg-white border-orange-400 text-orange-500 dark:bg-zink-700 dark:border-orange-700"
							>Orange <a href="#!" class="text-orange-400 transition hover:text-orange-600"
								><LucideIcon name="X" class="size-3 ltr:ml-1 rtl:mr-1" /></a
							></span
						>
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded border bg-white border-sky-400 text-sky-500 dark:bg-zink-700 dark:border-sky-700"
							>Sky <a href="#!" class="transition text-sky-400 hover:text-sky-600"
								><LucideIcon name="X" class="size-3 ltr:ml-1 rtl:mr-1" /></a
							></span
						>
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded border bg-white border-yellow-400 text-yellow-500 dark:bg-zink-700 dark:border-yellow-700"
							>Yellow <a href="#!" class="text-yellow-400 transition hover:text-yellow-600"
								><LucideIcon name="X" class="size-3 ltr:ml-1 rtl:mr-1" /></a
							></span
						>
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded border bg-white border-red-400 text-red-500 dark:bg-zink-700 dark:border-red-700"
							>Red <a href="#!" class="text-red-400 transition hover:text-red-600"
								><LucideIcon name="X" class="size-3 ltr:ml-1 rtl:mr-1" /></a
							></span
						>
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded border bg-white border-purple-400 text-purple-500 dark:bg-zink-700 dark:border-purple-700"
							>Purple <a href="#!" class="text-purple-400 transition hover:text-purple-600"
								><LucideIcon name="X" class="size-3 ltr:ml-1 rtl:mr-1" /></a
							></span
						>
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded border bg-white border-slate-400 text-slate-500 dark:bg-zink-700 dark:border-zink-400 dark:text-zink-200"
							>Slate <a href="#!" class="transition text-slate-400 hover:text-slate-600"
								><LucideIcon name="X" class="size-3 ltr:ml-1 rtl:mr-1" /></a
							></span
						>
					</div>
				</div>
			</div>
			<!--end card-->
		</div>
		<!--end grid-->
		<div class="grid grid-cols-1 gap-x-5 xl:grid-cols-2">
			<div class="card">
				<div class="card-body">
					<h6 class="mb-4 text-15">Fill Badges</h6>
					<div class="flex flex-wrap items-center gap-2">
						<span
							class="px-2.5 py-0.5 text-xs font-medium inline-block rounded border bg-custom-500 border-custom-500 text-custom-50"
							>Custom</span
						>
						<span
							class="px-2.5 py-0.5 text-xs font-medium inline-block rounded border bg-green-500 border-green-500 text-green-50"
							>Green</span
						>
						<span
							class="px-2.5 py-0.5 text-xs font-medium inline-block rounded border bg-orange-500 border-orange-500 text-orange-50"
							>Orange</span
						>
						<span
							class="px-2.5 py-0.5 text-xs font-medium inline-block rounded border bg-sky-500 border-sky-500 text-sky-50"
							>Sky</span
						>
						<span
							class="px-2.5 py-0.5 text-xs font-medium inline-block rounded border bg-yellow-500 border-yellow-500 text-yellow-50"
							>Yellow</span
						>
						<span
							class="px-2.5 py-0.5 text-xs font-medium inline-block rounded border bg-red-500 border-red-500 text-red-50"
							>Red</span
						>
						<span
							class="px-2.5 py-0.5 text-xs font-medium inline-block rounded border bg-purple-500 border-purple-500 text-purple-50"
							>Purple</span
						>
						<span
							class="px-2.5 py-0.5 text-xs font-medium inline-block rounded border bg-slate-500 border-slate-500 text-slate-50"
							>Slate</span
						>
					</div>
				</div>
			</div>
			<!--end card-->
			<div class="card">
				<div class="card-body">
					<h6 class="mb-4 text-15">Dismissible Outline Badge</h6>
					<div class="flex flex-wrap items-center gap-2">
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded-full border bg-white border-custom-400 text-custom-500 dark:bg-zink-700 dark:border-custom-700"
							>Custom <a href="#!" class="transition text-custom-400 hover:text-custom-600"
								><LucideIcon name="X" class="size-3 ltr:ml-1 rtl:mr-1" /></a
							></span
						>
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded-full border bg-white border-green-400 text-green-500 dark:bg-zink-700 dark:border-green-700"
							>Green<a href="#!" class="text-green-400 transition hover:text-green-600"
								><LucideIcon name="X" class="size-3 ltr:ml-1 rtl:mr-1" /></a
							></span
						>
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded-full border bg-white border-orange-400 text-orange-500 dark:bg-zink-700 dark:border-orange-700"
							>Orange <a href="#!" class="text-orange-400 transition hover:text-orange-600"
								><LucideIcon name="X" class="size-3 ltr:ml-1 rtl:mr-1" /></a
							></span
						>
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded-full border bg-white border-sky-400 text-sky-500 dark:bg-zink-700 dark:border-sky-700"
							>Sky <a href="#!" class="transition text-sky-400 hover:text-sky-600"
								><LucideIcon name="X" class="size-3 ltr:ml-1 rtl:mr-1" /></a
							></span
						>
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded-full border bg-white border-yellow-400 text-yellow-500 dark:bg-zink-700 dark:border-yellow-700"
							>Yellow <a href="#!" class="text-yellow-400 transition hover:text-yellow-600"
								><LucideIcon name="X" class="size-3 ltr:ml-1 rtl:mr-1" /></a
							></span
						>
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded-full border bg-white border-red-400 text-red-500 dark:bg-zink-700 dark:border-red-700"
							>Red <a href="#!" class="text-red-400 transition hover:text-red-600"
								><LucideIcon name="X" class="size-3 ltr:ml-1 rtl:mr-1" /></a
							></span
						>
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded-full border bg-white border-purple-400 text-purple-500 dark:bg-zink-700 dark:border-purple-700"
							>Purple <a href="#!" class="text-purple-400 transition hover:text-purple-600"
								><LucideIcon name="X" class="size-3 ltr:ml-1 rtl:mr-1" /></a
							></span
						>
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded-full border bg-white border-slate-400 text-slate-500 dark:bg-zink-700 dark:border-zink-400 dark:text-zink-200"
							>Slate <a href="#!" class="transition text-slate-400 hover:text-slate-600"
								><LucideIcon name="X" class="size-3 ltr:ml-1 rtl:mr-1" /></a
							></span
						>
					</div>
				</div>
			</div>
			<!--end card-->
		</div>
		<!--end grid-->
		<div class="grid grid-cols-1 gap-x-5 xl:grid-cols-2">
			<div class="card">
				<div class="card-body">
					<h6 class="mb-4 text-15">Badge with Icon</h6>
					<div class="flex flex-wrap items-center gap-2">
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded border bg-custom-100 border-transparent text-custom-500 dark:bg-custom-500/20 dark:border-transparent"
							><LucideIcon name="Frown" class="size-3 ltr:mr-1 rtl:ml-1" /> Custom</span
						>
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded border bg-green-100 border-transparent text-green-500 dark:bg-green-500/20 dark:border-transparent"
							><LucideIcon name="Frown" class="size-3 ltr:mr-1 rtl:ml-1" /> Green</span
						>
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded border bg-orange-100 border-transparent text-orange-500 dark:bg-orange-500/20 dark:border-transparent"
							><LucideIcon name="Frown" class="size-3 ltr:mr-1 rtl:ml-1" /> Orange</span
						>
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded border bg-sky-100 border-transparent text-sky-500 dark:bg-sky-500/20 dark:border-transparent"
							><LucideIcon name="Frown" class="size-3 ltr:mr-1 rtl:ml-1" /> Sky</span
						>
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded border bg-yellow-100 border-transparent text-yellow-500 dark:bg-yellow-500/20 dark:border-transparent"
							><LucideIcon name="Frown" class="size-3 ltr:mr-1 rtl:ml-1" /> Yellow</span
						>
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded border bg-red-100 border-transparent text-red-500 dark:bg-red-500/20 dark:border-transparent"
							><LucideIcon name="Frown" class="size-3 ltr:mr-1 rtl:ml-1" /> Red</span
						>
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded border bg-purple-100 border-transparent text-purple-500 dark:bg-purple-500/20 dark:border-transparent"
							><LucideIcon name="Frown" class="size-3 ltr:mr-1 rtl:ml-1" /> Purple</span
						>
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded border bg-slate-100 border-transparent text-slate-500 dark:bg-slate-500/20 dark:text-zink-200 dark:border-transparent"
							><LucideIcon name="Frown" class="size-3 ltr:mr-1 rtl:ml-1" /> Slate</span
						>
					</div>
				</div>
			</div>
			<!--end card-->
			<div class="card">
				<div class="card-body">
					<h6 class="mb-4 text-15">Badge with Indicator</h6>
					<div class="flex flex-wrap items-center gap-2">
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded border bg-white border-custom-400 text-custom-500 dark:bg-zink-700 dark:border-custom-700"
						>
							<span class=" size-1.5 ltr:mr-1 rtl:ml-1 rounded-full inline-block bg-custom-500"
							></span> Custom
						</span>
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded border bg-white border-green-400 text-green-500 dark:bg-zink-700 dark:border-green-700"
						>
							<span class=" size-1.5 ltr:mr-1 rtl:ml-1 rounded-full inline-block bg-green-500"
							></span> Green
						</span>
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded-full border bg-white border-orange-400 text-orange-500 dark:bg-zink-700 dark:border-orange-700"
						>
							<span class=" size-1.5 ltr:mr-1 rtl:ml-1 rounded-full inline-block bg-orange-500"
							></span> Orange
						</span>
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded border bg-white border-sky-400 text-sky-500 dark:bg-zink-700 dark:border-sky-700"
						>
							<span class=" size-1.5 ltr:mr-1 rtl:ml-1 rounded-full inline-block bg-sky-500"></span>
							Sky
						</span>
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded border bg-white border-yellow-400 text-yellow-500 dark:bg-zink-700 dark:border-yellow-700"
						>
							<span class=" size-1.5 ltr:mr-1 rtl:ml-1 rounded-full inline-block bg-yellow-500"
							></span> Yellow
						</span>
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded border bg-white border-red-400 text-red-500 dark:bg-zink-700 dark:border-red-700"
						>
							<span class=" size-1.5 ltr:mr-1 rtl:ml-1 rounded-full inline-block bg-red-500"></span>
							Red
						</span>
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded border bg-white border-purple-400 text-purple-500 dark:bg-zink-700 dark:border-purple-700"
						>
							<span class=" size-1.5 ltr:mr-1 rtl:ml-1 rounded-full inline-block bg-purple-500"
							></span> Purple
						</span>
						<span
							class="flex items-center px-2.5 py-0.5 text-xs font-medium rounded border bg-white border-slate-400 text-slate-500 dark:bg-zink-700 dark:border-zink-400 dark:text-zink-200"
						>
							<span class=" size-1.5 ltr:mr-1 rtl:ml-1 rounded-full inline-block bg-slate-500"
							></span> Slate
						</span>
					</div>
				</div>
			</div>
			<!--end card-->
		</div>
		<!--end grid-->
		<div class="grid grid-cols-1 gap-x-5 xl:grid-cols-2">
			<div class="card">
				<div class="card-body">
					<h6 class="mb-4 text-15">Badge with Icon Only</h6>
					<div class="flex flex-wrap items-center gap-2">
						<span
							class="flex items-center justify-center size-8 text-xs font-medium border border-transparent rounded bg-custom-100 text-custom-500 dark:bg-custom-500/20 dark:border-transparent"
							><LucideIcon name="Gift" class="size-3" /></span
						>
						<span
							class="flex items-center justify-center size-8 text-xs font-medium text-green-500 bg-green-100 border border-transparent rounded-full dark:bg-green-500/20 dark:border-transparent"
							><LucideIcon name="Gift" class="size-3" /></span
						>
						<span
							class="flex items-center justify-center size-8 text-xs font-medium text-orange-500 bg-white border border-transparent rounded dark:bg-orange-500/20 dark:border-transparent"
							><LucideIcon name="Gift" class="size-3" /></span
						>
					</div>
				</div>
			</div>
			<!--end card-->
			<div class="card">
				<div class="card-body">
					<h6 class="mb-4 text-15">Badge with Button</h6>
					<div class="flex flex-wrap items-center gap-2">
						<button
							type="button"
							class="relative flex items-center justify-center text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
						>
							Custom Button
							<span
								class="flex items-center justify-center size-5 ltr:ml-2 rtl:mr-2 text-[10px] shrink-0 font-medium border rounded-full bg-custom-400 border-custom-400"
								>15</span
							>
						</button>
						<button
							type="button"
							class="flex items-center justify-center size-[37.5px] relative p-0 text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
						>
							<span
								class="absolute flex items-center justify-center size-5 text-[11px] font-medium bg-red-400 border border-red-400 rounded-full ltr:-right-2 rtl:-left-4 -top-2"
								>8</span
							>
							<LucideIcon name="Mail" class="size-4" />
						</button>
					</div>
				</div>
			</div>
			<!--end card-->
		</div>
		<!--end grid-->
		<div class="grid grid-cols-1 gap-x-5 xl:grid-cols-2">
			<div class="card">
				<div class="card-body">
					<h6 class="mb-4 text-15">Badges as Links</h6>
					<div class="flex flex-wrap items-center gap-2">
						<a
							href="#!"
							class="px-2.5 py-0.5 text-xs font-medium inline-block rounded border transition-all duration-200 ease-linear bg-custom-100 border-transparent text-custom-500 hover:bg-custom-200 dark:bg-custom-400/20 dark:hover:bg-custom-400/30 dark:border-transparent"
							>Custom</a
						>
						<a
							href="#!"
							class="px-2.5 py-0.5 text-xs font-medium inline-block rounded border transition-all duration-200 ease-linear bg-green-100 border-transparent text-green-500 hover:bg-green-200 dark:bg-green-400/20 dark:hover:bg-green-400/30 dark:border-transparent"
							>Green</a
						>
						<a
							href="#!"
							class="px-2.5 py-0.5 text-xs font-medium inline-block rounded border transition-all duration-200 ease-linear bg-orange-100 border-transparent text-orange-500 hover:bg-orange-200 dark:bg-orange-400/20 dark:hover:bg-orange-400/30 dark:border-transparent"
							>Orange</a
						>
						<a
							href="#!"
							class="px-2.5 py-0.5 text-xs font-medium inline-block rounded border transition-all duration-200 ease-linear bg-sky-100 border-transparent text-sky-500 hover:bg-sky-200 dark:bg-sky-400/20 dark:hover:bg-sky-400/30 dark:border-transparent"
							>Sky</a
						>
						<a
							href="#!"
							class="px-2.5 py-0.5 text-xs font-medium inline-block rounded border transition-all duration-200 ease-linear bg-yellow-100 border-transparent text-yellow-500 hover:bg-yellow-200 dark:bg-yellow-400/20 dark:hover:bg-yellow-400/30 dark:border-transparent"
							>Yellow</a
						>
						<a
							href="#!"
							class="px-2.5 py-0.5 text-xs font-medium inline-block rounded border transition-all duration-200 ease-linear bg-red-100 border-transparent text-red-500 hover:bg-red-200 dark:bg-red-400/20 dark:hover:bg-red-400/30 dark:border-transparent"
							>Red</a
						>
						<a
							href="#!"
							class="px-2.5 py-0.5 text-xs font-medium inline-block rounded border transition-all duration-200 ease-linear bg-purple-100 border-transparent text-purple-500 hover:bg-purple-200 dark:bg-purple-400/20 dark:hover:bg-purple-400/30 dark:border-transparent"
							>Purple</a
						>
						<a
							href="#!"
							class="px-2.5 py-0.5 text-xs font-medium inline-block rounded border transition-all duration-200 ease-linear bg-slate-100 border-transparent text-slate-500 hover:bg-slate-200 dark:text-zink-200 dark:bg-slate-400/20 dark:hover:bg-slate-400/30 dark:border-transparent"
							>Slate</a
						>
					</div>
				</div>
			</div>
			<!--end card-->
		</div>
		<!--end grid-->
	</div>
</div>
