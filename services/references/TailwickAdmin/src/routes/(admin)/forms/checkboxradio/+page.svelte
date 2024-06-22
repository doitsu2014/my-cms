<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
</script>

<HeadTitle title="Checkbox & Radio" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Checkbox & Radio" pagetitle="Forms" />

		<div class="grid grid-cols-1 gap-x-5 xl:grid-cols-3">
			<div class="card">
				<div class="card-body">
					<h6 class="mb-4 text-16">Default Checkbox</h6>

					<div class="flex flex-col gap-2">
						<div class="flex items-center gap-2">
							<input id="checkboxDefault1" class="size-4 border rounded-sm appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-custom-500 checked:border-custom-500 dark:checked:bg-custom-500 dark:checked:border-custom-500 checked:disabled:bg-custom-400 checked:disabled:border-custom-400" type="checkbox" value="" checked>
							<label for="checkboxDefault1" class="align-middle">
								Custom
							</label>
						</div>

						<div class="flex items-center gap-2">
							<input id="checkboxDefault2" class="size-4 border rounded-sm appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-green-500 checked:border-green-500 dark:checked:bg-green-500 dark:checked:border-green-500 checked:disabled:bg-green-400 checked:disabled:border-green-400" type="checkbox" value="">
							<label for="checkboxDefault2" class="align-middle">
								Green
							</label>
						</div>

						<div class="flex items-center gap-2">
							<input id="checkboxDefault3" class="size-4 border rounded-sm appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-orange-500 checked:border-orange-500 dark:checked:bg-orange-500 dark:checked:border-orange-500 checked:disabled:bg-orange-400 checked:disabled:border-orange-400" type="checkbox" value="" checked>
							<label for="checkboxDefault3" class="align-middle">
								Orange
							</label>
						</div>

						<div class="flex items-center gap-2">
							<input id="checkboxDefault4" class="size-4 border rounded-sm appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-sky-500 checked:border-sky-500 dark:checked:bg-sky-500 dark:checked:border-sky-500 checked:disabled:bg-sky-400 checked:disabled:border-sky-400" type="checkbox" value="">
							<label for="checkboxDefault4" class="align-middle">
								Sky
							</label>
						</div>

						<div class="flex items-center gap-2">
							<input id="checkboxDefault5" class="size-4 border rounded-sm appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-yellow-500 checked:border-yellow-500 dark:checked:bg-yellow-500 dark:checked:border-yellow-500 checked:disabled:bg-yellow-400 checked:disabled:border-yellow-400" type="checkbox" value="">
							<label for="checkboxDefault5" class="align-middle">
								Yellow
							</label>
						</div>

						<div class="flex items-center gap-2">
							<input id="checkboxDefault6" class="size-4 border rounded-sm appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-red-500 checked:border-red-500 dark:checked:bg-red-500 dark:checked:border-red-500 checked:disabled:bg-red-400 checked:disabled:border-red-400" type="checkbox" value="" checked>
							<label for="checkboxDefault6" class="align-middle">
								Red
							</label>
						</div>

						<div class="flex items-center gap-2">
							<input id="checkboxDefault7" class="size-4 border rounded-sm appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-purple-500 checked:border-purple-500 dark:checked:bg-purple-500 dark:checked:border-purple-500 checked:disabled:bg-purple-400 checked:disabled:border-purple-400" type="checkbox" value="" checked>
							<label for="checkboxDefault7" class="align-middle">
								Purple
							</label>
						</div>

						<div class="flex items-center gap-2">
							<input id="checkboxDefault8" class="size-4 border rounded-sm appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-slate-300 checked:border-slate-300 dark:checked:bg-zink-200 dark:checked:border-zink-200 checked:disabled:bg-zink-400 checked:disabled:border-zink-400" type="checkbox" value="">
							<label for="checkboxDefault8" class="align-middle">
								Light
							</label>
						</div>

						<div class="flex items-center gap-2">
							<input id="checkboxDefault9" class="size-4 border rounded-sm appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-slate-700 checked:border-slate-700 dark:checked:bg-zink-400 dark:checked:border-zink-400 checked:disabled:bg-zink-500 checked:disabled:border-zink-500" type="checkbox" value="" checked>
							<label for="checkboxDefault9" class="align-middle">
								Dark
							</label>
						</div>
					</div>
				</div>
			</div><!--end card-->
			<div class="card">
				<div class="card-body">
					<h6 class="mb-4 text-16">Circle Checkbox</h6>
			
					<div class="flex flex-col gap-2">
						<div class="flex items-center gap-2">
							<input id="checkboxCircle1" class="size-4 border rounded-full appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-custom-500 checked:border-custom-500 dark:checked:bg-custom-500 dark:checked:border-custom-500" type="checkbox" value="">
							<label for="checkboxCircle1" class="align-middle">
								Custom
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="checkboxCircle2" class="size-4 border rounded-full appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-green-500 checked:border-green-500 dark:checked:bg-green-500 dark:checked:border-green-500 checked:disabled:bg-green-400 checked:disabled:border-green-400" type="checkbox" value="" checked>
							<label for="checkboxCircle2" class="align-middle">
								Green
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="checkboxCircle3" class="size-4 border rounded-full appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-orange-500 checked:border-orange-500 dark:checked:bg-orange-500 dark:checked:border-orange-500 checked:disabled:bg-orange-400 checked:disabled:border-orange-400" type="checkbox" value="">
							<label for="checkboxCircle3" class="align-middle">
								Orange
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="checkboxCircle4" class="size-4 border rounded-full appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-sky-500 checked:border-sky-500 dark:checked:bg-sky-500 dark:checked:border-sky-500 checked:disabled:bg-sky-400 checked:disabled:border-sky-400" type="checkbox" value="" checked>
							<label for="checkboxCircle4" class="align-middle">
								Sky
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="checkboxCircle5" class="size-4 border rounded-full appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-yellow-500 checked:border-yellow-500 dark:checked:bg-yellow-500 dark:checked:border-yellow-500 checked:disabled:bg-yellow-400 checked:disabled:border-yellow-400" type="checkbox" value="" checked>
							<label for="checkboxCircle5" class="align-middle">
								Yellow
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="checkboxCircle6" class="size-4 border rounded-full appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-red-500 checked:border-red-500 dark:checked:bg-red-500 dark:checked:border-red-500 checked:disabled:bg-red-400 checked:disabled:border-red-400" type="checkbox" value="">
							<label for="checkboxCircle6" class="align-middle">
								Red
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="checkboxCircle7" class="size-4 border rounded-full appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-purple-500 checked:border-purple-500 dark:checked:bg-purple-500 dark:checked:border-purple-500 checked:disabled:bg-purple-400 checked:disabled:border-purple-400" type="checkbox" value="">
							<label for="checkboxCircle7" class="align-middle">
								Purple
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="checkboxCircle8" class="size-4 border rounded-full appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-slate-300 checked:border-slate-300 dark:checked:bg-zink-200 dark:checked:border-zink-200 checked:disabled:bg-zink-400 checked:disabled:border-zink-400" type="checkbox" value="">
							<label for="checkboxCircle8" class="align-middle">
								Light
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="checkboxCircle9" class="size-4 border rounded-full appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-slate-700 checked:border-slate-700 dark:checked:bg-zink-400 dark:checked:border-zink-400 checked:disabled:bg-zink-500 checked:disabled:border-zink-500" type="checkbox" value="">
							<label for="checkboxCircle9" class="align-middle">
								Dark
							</label>
						</div>
					</div>
				</div>
			</div><!--end card-->
			<div class="card">
				<div class="card-body">
					<h6 class="mb-4 text-16">Bordered Checkbox</h6>
			
					<div class="flex flex-col gap-2">
						<div class="flex items-center gap-2">
							<input id="checkboxOutline1" class="size-4 bg-white border rounded-sm appearance-none cursor-pointer dark:bg-zink-700 border-slate-200 dark:border-zink-500 checked:bg-custom-500 checked:border-custom-500 dark:checked:bg-custom-500 dark:checked:border-custom-500 checked:disabled:bg-custom-400 checked:disabled:border-custom-400" type="checkbox" value="">
							<label for="checkboxOutline1" class="align-middle">
								Custom
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="checkboxOutline2" class="size-4 bg-white border rounded-sm appearance-none cursor-pointer dark:bg-zink-700 border-slate-200 dark:border-zink-500 checked:bg-green-500 checked:border-green-500 dark:checked:bg-green-500 dark:checked:border-green-500 checked:disabled:bg-green-400 checked:disabled:border-green-400" type="checkbox" value="" checked>
							<label for="checkboxOutline2" class="align-middle">
								Green
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="checkboxOutline3" class="size-4 bg-white border rounded-sm appearance-none cursor-pointer dark:bg-zink-700 border-slate-200 dark:border-zink-500 checked:bg-orange-500 checked:border-orange-500 dark:checked:bg-orange-500 dark:checked:border-orange-500 checked:disabled:bg-orange-400 checked:disabled:border-orange-400" type="checkbox" value="">
							<label for="checkboxOutline3" class="align-middle">
								Orange
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="checkboxOutline4" class="size-4 bg-white border rounded-sm appearance-none cursor-pointer dark:bg-zink-700 border-slate-200 dark:border-zink-500 checked:bg-sky-500 checked:border-sky-500 dark:checked:bg-sky-500 dark:checked:border-sky-500 checked:disabled:bg-sky-400 checked:disabled:border-sky-400" type="checkbox" value="" checked>
							<label for="checkboxOutline4" class="align-middle">
								Sky
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="checkboxOutline5" class="size-4 bg-white border rounded-sm appearance-none cursor-pointer dark:bg-zink-700 border-slate-200 dark:border-zink-500 checked:bg-yellow-500 checked:border-yellow-500 dark:checked:bg-yellow-500 dark:checked:border-yellow-500 checked:disabled:bg-yellow-400 checked:disabled:border-yellow-400" type="checkbox" value="">
							<label for="checkboxOutline5" class="align-middle">
								Yellow
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="checkboxOutline6" class="size-4 bg-white border rounded-full appearance-none cursor-pointer dark:bg-zink-700 border-slate-200 dark:border-zink-500 checked:bg-red-500 checked:border-red-500 dark:checked:bg-red-500 dark:checked:border-red-500 checked:disabled:bg-sky-400 checked:disabled:border-sky-400" type="checkbox" value="">
							<label for="checkboxOutline6" class="align-middle">
								Red
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="checkboxOutline7" class="size-4 bg-white border rounded-full appearance-none cursor-pointer dark:bg-zink-700 border-slate-200 dark:border-zink-500 checked:bg-purple-500 checked:border-purple-500 dark:checked:bg-purple-500 dark:checked:border-purple-500 checked:disabled:bg-purple-400 checked:disabled:border-purple-400" type="checkbox" value="" checked>
							<label for="checkboxOutline7" class="align-middle">
								Purple
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="checkboxOutline8" class="size-4 bg-white border rounded-full appearance-none cursor-pointer dark:bg-zink-700 border-slate-200 dark:border-zink-500 checked:bg-slate-300 checked:border-slate-300 dark:checked:bg-zink-200 dark:checked:border-zink-200 checked:disabled:bg-zink-400 checked:disabled:border-zink-400" type="checkbox" value="">
							<label for="checkboxOutline8" class="align-middle">
								Light
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="checkboxOutline9" class="size-4 bg-white border rounded-full appearance-none cursor-pointer dark:bg-zink-700 border-slate-200 dark:border-zink-500 checked:bg-slate-700 checked:border-slate-700 dark:checked:bg-zink-400 dark:checked:border-zink-400 checked:disabled:bg-zink-500 checked:disabled:border-zink-500" type="checkbox" value="" checked>
							<label for="checkboxOutline9" class="align-middle">
								Dark
							</label>
						</div>
					</div>
				</div>
			</div><!--end card-->
			<div class="card">
				<div class="card-body">
					<h6 class="mb-4 text-16">Soft Checkbox</h6>
			
					<div class="flex flex-col gap-2">
						<div class="flex items-center gap-2">
							<input id="checkboxSoft1" class="size-4 border rounded-sm appearance-none cursor-pointer border-custom-200 bg-custom-50 checked:bg-custom-500 checked:border-custom-500 dark:border-custom-800 dark:bg-custom-500/20 dark:checked:bg-custom-500 dark:checked:border-custom-500" type="checkbox" value="">
							<label for="checkboxSoft1" class="align-middle">
								Custom
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="checkboxSoft2" class="size-4 border border-green-200 rounded-sm appearance-none bg-green-50 checked:bg-green-500 checked:border-green-500 dark:border-green-800 dark:bg-green-500/20 dark:checked:bg-green-500 dark:checked:border-green-500" type="checkbox" value="" checked>
							<label for="checkboxSoft2" class="align-middle">
								Green
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="checkboxSoft3" class="size-4 border border-orange-200 rounded-sm appearance-none bg-orange-50 checked:bg-orange-500 checked:border-orange-500 dark:border-orange-800 dark:bg-orange-500/10 dark:checked:bg-orange-500 dark:checked:border-orange-500" type="checkbox" value="">
							<label for="checkboxSoft3" class="align-middle">
								Orange
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="checkboxSoft4" class="size-4 border rounded-sm appearance-none cursor-pointer border-sky-200 bg-sky-50 checked:bg-sky-500 checked:border-sky-500 dark:border-sky-800 dark:bg-sky-500/10 dark:checked:bg-sky-500 dark:checked:border-sky-500" type="checkbox" value="" checked>
							<label for="checkboxSoft4" class="align-middle">
								Sky
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="checkboxSoft5" class="size-4 border border-yellow-200 rounded-sm appearance-none cursor-pointer bg-yellow-50 checked:bg-yellow-500 checked:border-yellow-500 dark:border-yellow-800 dark:bg-yellow-500/10 dark:checked:bg-yellow-500 dark:checked:border-yellow-500" type="checkbox" value="">
							<label for="checkboxSoft5" class="align-middle">
								Yellow
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="checkboxSoft6" class="size-4 border border-red-200 rounded-full appearance-none cursor-pointer bg-red-50 checked:bg-red-500 checked:border-red-500 dark:border-red-800 dark:bg-red-500/10 dark:checked:bg-red-500 dark:checked:border-red-500" type="checkbox" value="">
							<label for="checkboxSoft6" class="align-middle">
								Red
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="checkboxSoft7" class="size-4 border border-purple-200 rounded-full appearance-none cursor-pointer bg-purple-50 checked:bg-purple-500 checked:border-purple-500 dark:border-purple-800 dark:bg-purple-500/10 dark:checked:bg-purple-500 dark:checked:border-purple-500" type="checkbox" value="" checked>
							<label for="checkboxSoft7" class="align-middle">
								Purple
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="checkboxSoft8" class="size-4 border rounded-full appearance-none cursor-pointer border-slate-200 bg-slate-50 checked:bg-slate-300 checked:border-slate-300 dark:border-zink-500 dark:bg-zink-600 dark:checked:bg-zink-200 dark:checked:border-zink-200" type="checkbox" value="">
							<label for="checkboxSoft8" class="align-middle">
								Light
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="checkboxSoft9" class="size-4 border rounded-full appearance-none cursor-pointer border-slate-300 bg-slate-200 checked:bg-slate-700 checked:border-slate-700 dark:border-zink-500 dark:bg-zink-600 dark:checked:bg-zink-400 dark:checked:border-zink-400" type="checkbox" value="" checked>
							<label for="checkboxSoft9" class="align-middle">
								Dark
							</label>
						</div>
					</div>
				</div>
			</div><!--end card-->
			<div class="card">
				<div class="card-body">
					<h6 class="mb-4 text-16">Outline Checkbox</h6>
			
					<div class="flex flex-col gap-2">
						<div class="flex items-center gap-2">
							<input id="checkboxOutline11" class="size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800" type="checkbox" value="">
							<label for="checkboxOutline11" class="align-middle">
								Custom
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="checkboxOutline12" class="size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-green-500 checked:border-green-500 dark:after:text-green-500 dark:checked:border-green-800" type="checkbox" value="" checked>
							<label for="checkboxOutline12" class="align-middle">
								Green
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="checkboxOutline13" class="size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-orange-500 checked:border-orange-500 dark:after:text-orange-500 dark:checked:border-orange-800" type="checkbox" value="">
							<label for="checkboxOutline13" class="align-middle">
								Orange
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="checkboxOutline14" class="size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-sky-500 checked:border-sky-500 dark:after:text-sky-500 dark:checked:border-sky-800" type="checkbox" value="" checked>
							<label for="checkboxOutline14" class="align-middle">
								Sky
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="checkboxOutline15" class="size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-yellow-500 checked:border-yellow-500 dark:after:text-sky-500 dark:checked:border-sky-800" type="checkbox" value="">
							<label for="checkboxOutline15" class="align-middle">
								Yellow
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="checkboxOutline16" class="size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-red-500 checked:border-red-500 dark:after:text-red-500 dark:checked:border-red-800" type="checkbox" value="">
							<label for="checkboxOutline16" class="align-middle">
								Red
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="checkboxOutline17" class="size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-purple-500 checked:border-purple-500 dark:after:text-purple-500 dark:checked:border-purple-800" type="checkbox" value="" checked>
							<label for="checkboxOutline17" class="align-middle">
								Purple
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="checkboxOutline18" class="size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-slate-400 checked:border-slate-400 dark:after:text-zink-200 dark:checked:border-zink-200" type="checkbox" value="">
							<label for="checkboxOutline18" class="align-middle">
								Light
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="checkboxOutline19" class="size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-slate-700 checked:border-slate-700 dark:after:text-zink-400 dark:checked:border-zink-400" type="checkbox" value="" checked>
							<label for="checkboxOutline19" class="align-middle">
								Dark
							</label>
						</div>
					</div>
				</div>
			</div><!--end card-->

			<div class="card">
				<div class="card-body">
					<h6 class="mb-4 text-16">Default Radio</h6>
			
					<div class="flex flex-col gap-2">
						<div class="flex items-center gap-2">
							<input id="radioDefault1" class="size-4 border rounded-full appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-custom-500 checked:border-custom-500 dark:checked:bg-custom-500 dark:checked:border-custom-500" type="radio" value="" checked>
							<label for="radioDefault1" class="align-middle">
								Custom
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioDefault2" class="size-4 border rounded-full appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-green-500 checked:border-green-500 dark:checked:bg-green-500 dark:checked:border-green-500" type="radio" value="">
							<label for="radioDefault2" class="align-middle">
								Green
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioDefault3" class="size-4 border rounded-full appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-orange-500 checked:border-orange-500 dark:checked:bg-orange-500 dark:checked:border-orange-500" type="radio" value="" checked>
							<label for="radioDefault3" class="align-middle">
								Orange
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioDefault4" class="size-4 border rounded-full appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-sky-500 checked:border-sky-500 dark:checked:bg-sky-500 dark:checked:border-sky-500" type="radio" value="">
							<label for="radioDefault4" class="align-middle">
								Sky
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioDefault5" class="size-4 border rounded-full appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-yellow-500 checked:border-yellow-500 dark:checked:bg-yellow-500 dark:checked:border-yellow-500" type="radio" value="">
							<label for="radioDefault5" class="align-middle">
								Yellow
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioDefault6" class="size-4 border rounded-full appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-red-500 checked:border-red-500 dark:checked:bg-red-500 dark:checked:border-red-500" type="radio" value="" checked>
							<label for="radioDefault6" class="align-middle">
								Red
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioDefault7" class="size-4 border rounded-full appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-purple-500 checked:border-purple-500 dark:checked:bg-purple-500 dark:checked:border-purple-500" type="radio" value="" checked>
							<label for="radioDefault7" class="align-middle">
								Purple
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioDefault8" class="size-4 border rounded-full appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-slate-300 checked:border-slate-300 dark:checked:bg-zink-200 dark:checked:border-zink-200" type="radio" value="">
							<label for="radioDefault8" class="align-middle">
								Light
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioDefault9" class="size-4 border rounded-full appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-slate-700 checked:border-slate-700 dark:checked:bg-zink-400 dark:checked:border-zink-400" type="radio" value="" checked>
							<label for="radioDefault9" class="align-middle">
								Dark
							</label>
						</div>
					</div>
				</div>
			</div><!--end card-->
			<div class="card">
				<div class="card-body">
					<h6 class="mb-4 text-16">Square Radio</h6>
			
					<div class="flex flex-col gap-2">
						<div class="flex items-center gap-2">
							<input id="radioSquare1" class="size-4 border rounded-sm appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-custom-500 checked:border-custom-500 dark:checked:bg-custom-400 dark:checked:border-custom-400" type="radio" value="" checked>
							<label for="radioSquare1" class="align-middle">
								Custom
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioSquare2" class="size-4 border rounded-sm appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-green-500 checked:border-green-500 dark:checked:bg-green-400 dark:checked:border-green-400" type="radio" value="">
							<label for="radioSquare2" class="align-middle">
								Green
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioSquare3" class="size-4 border rounded-sm appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-orange-500 checked:border-orange-500 dark:checked:bg-orange-400 dark:checked:border-orange-400" type="radio" value="" checked>
							<label for="radioSquare3" class="align-middle">
								Orange
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioSquare4" class="size-4 border rounded-sm appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-sky-500 checked:border-sky-500 dark:checked:bg-sky-400 dark:checked:border-sky-400" type="radio" value="">
							<label for="radioSquare4" class="align-middle">
								Sky
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioSquare5" class="size-4 border rounded-sm appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-yellow-500 checked:border-yellow-500 dark:checked:bg-yellow-400 dark:checked:border-yellow-400" type="radio" value="">
							<label for="radioSquare5" class="align-middle">
								Yellow
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioSquare6" class="size-4 border rounded-sm appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-red-500 checked:border-red-500 dark:checked:bg-red-400 dark:checked:border-red-400" type="radio" value="" checked>
							<label for="radioSquare6" class="align-middle">
								Red
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioSquare7" class="size-4 border rounded-sm appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-purple-500 checked:border-purple-500 dark:checked:bg-purple-400 dark:checked:border-purple-400" type="radio" value="" checked>
							<label for="radioSquare7" class="align-middle">
								Purple
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioSquare8" class="size-4 border rounded-sm appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-slate-300 checked:border-slate-300 dark:checked:bg-zink-200 dark:checked:border-zink-200" type="radio" value="">
							<label for="radioSquare8" class="align-middle">
								Light
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioSquare9" class="size-4 border rounded-sm appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-slate-700 checked:border-slate-700 dark:checked:bg-zink-400 dark:checked:border-zink-400" type="radio" value="" checked>
							<label for="radioSquare9" class="align-middle">
								Dark
							</label>
						</div>
					</div>
				</div>
			</div><!--end card-->

			<div class="card">
				<div class="card-body">
					<h6 class="mb-4 text-16">Bordered Radio</h6>
			
					<div class="flex flex-col gap-2">
						<div class="flex items-center gap-2">
							<input id="radioBordered1" class="size-4 bg-white border rounded-full appearance-none cursor-pointer dark:bg-zink-700 border-slate-200 dark:border-zink-500 checked:bg-custom-500 checked:border-custom-500 dark:checked:bg-custom-400 dark:checked:border-custom-400" type="radio" value="" checked>
							<label for="radioBordered1" class="align-middle">
								Custom
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioBordered2" class="size-4 bg-white border rounded-full appearance-none cursor-pointer dark:bg-zink-700 border-slate-200 dark:border-zink-500 checked:bg-green-500 checked:border-green-500 dark:checked:bg-green-400 dark:checked:border-green-400" type="radio" value="">
							<label for="radioBordered2" class="align-middle">
								Green
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioBordered3" class="size-4 bg-white border rounded-full appearance-none cursor-pointer dark:bg-zink-700 border-slate-200 dark:border-zink-500 checked:bg-orange-500 checked:border-orange-500 dark:checked:bg-orange-400 dark:checked:border-orange-400" type="radio" value="" checked>
							<label for="radioBordered3" class="align-middle">
								Orange
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioBordered4" class="size-4 bg-white border rounded-full appearance-none cursor-pointer dark:bg-zink-700 border-slate-200 dark:border-zink-500 checked:bg-sky-500 checked:border-sky-500 dark:checked:bg-sky-400 dark:checked:border-sky-400" type="radio" value="">
							<label for="radioBordered4" class="align-middle">
								Sky
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioBordered5" class="size-4 bg-white border rounded-full appearance-none cursor-pointer dark:bg-zink-700 border-slate-200 dark:border-zink-500 checked:bg-yellow-500 checked:border-yellow-500 dark:checked:bg-yellow-400 dark:checked:border-yellow-400" type="radio" value="">
							<label for="radioBordered5" class="align-middle">
								Yellow
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioBordered6" class="size-4 bg-white border rounded-full appearance-none cursor-pointer dark:bg-zink-700 border-slate-200 dark:border-zink-500 checked:bg-red-500 checked:border-red-500 dark:checked:bg-red-400 dark:checked:border-red-400" type="radio" value="" checked>
							<label for="radioBordered6" class="align-middle">
								Red
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioBordered7" class="size-4 bg-white border rounded-full appearance-none cursor-pointer dark:bg-zink-700 border-slate-200 dark:border-zink-500 checked:bg-purple-500 checked:border-purple-500 dark:checked:bg-purple-400 dark:checked:border-purple-400" type="radio" value="" checked>
							<label for="radioBordered7" class="align-middle">
								Purple
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioBordered8" class="size-4 bg-white border rounded-full appearance-none cursor-pointer dark:bg-zink-700 border-slate-200 dark:border-zink-500 checked:bg-slate-300 checked:border-slate-300 dark:checked:bg-zink-200 dark:checked:border-zink-200" type="radio" value="">
							<label for="radioBordered8" class="align-middle">
								Light
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioBordered9" class="size-4 bg-white border rounded-full appearance-none cursor-pointer dark:bg-zink-700 border-slate-200 dark:border-zink-500 checked:bg-slate-700 checked:border-slate-700 dark:checked:bg-zink-400 dark:checked:border-zink-400" type="radio" value="" checked>
							<label for="radioBordered9" class="align-middle">
								Dark
							</label>
						</div>
					</div>
				</div>
			</div><!--end card-->

			<div class="card">
				<div class="card-body">
					<h6 class="mb-4 text-16">Soft Radio</h6>
			
					<div class="flex flex-col gap-2">
						<div class="flex items-center gap-2">
							<input id="radioSoft1" class="size-4 border rounded-sm appearance-none cursor-pointer border-custom-200 bg-custom-50 checked:bg-custom-500 checked:border-custom-500 dark:border-custom-800 dark:bg-custom-500/20 dark:checked:bg-custom-500 dark:checked:border-custom-500" type="radio" value="">
							<label for="radioSoft1" class="align-middle">
								Custom
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioSoft2" class="size-4 border border-green-200 rounded-sm appearance-none cursor-pointer bg-green-50 checked:bg-green-500 checked:border-green-500 dark:border-green-800 dark:bg-green-500/20 dark:checked:bg-green-500 dark:checked:border-green-500" type="radio" value="" checked>
							<label for="radioSoft2" class="align-middle">
								Green
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioSoft3" class="size-4 border border-orange-200 rounded-sm appearance-none cursor-pointer bg-orange-50 checked:bg-orange-500 checked:border-orange-500 dark:border-orange-800 dark:bg-orange-500/20 dark:checked:bg-orange-500 dark:checked:border-orange-500" type="radio" value="">
							<label for="radioSoft3" class="align-middle">
								Orange
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioSoft4" class="size-4 border rounded-sm appearance-none cursor-pointer border-sky-200 bg-sky-50 checked:bg-sky-500 checked:border-sky-500 dark:border-sky-800 dark:bg-sky-500/20 dark:checked:bg-sky-500 dark:checked:border-sky-500" type="radio" value="" checked>
							<label for="radioSoft4" class="align-middle">
								Sky
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioSoft5" class="size-4 border border-yellow-200 rounded-sm appearance-none cursor-pointer bg-yellow-50 checked:bg-yellow-500 checked:border-yellow-500 dark:border-yellow-800 dark:bg-yellow-500/20 dark:checked:bg-yellow-500 dark:checked:border-yellow-500" type="radio" value="">
							<label for="radioSoft5" class="align-middle">
								Yellow
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioSoft6" class="size-4 border border-red-200 rounded-full appearance-none cursor-pointer bg-red-50 checked:bg-red-500 checked:border-red-500 dark:border-red-800 dark:bg-red-500/20 dark:checked:bg-red-500 dark:checked:border-red-500" type="radio" value="">
							<label for="radioSoft6" class="align-middle">
								Red
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioSoft7" class="size-4 border border-purple-200 rounded-sm appearance-none cursor-pointer bg-purple-50 checked:bg-purple-500 checked:border-purple-500 dark:border-purple-800 dark:bg-purple-500/20 dark:checked:bg-purple-500 dark:checked:border-purple-500" type="radio" value="" checked>
							<label for="radioSoft7" class="align-middle">
								Purple
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioSoft8" class="size-4 border rounded-full appearance-none cursor-pointer border-slate-200 bg-slate-50 checked:bg-slate-300 checked:border-slate-300 dark:border-zink-500 dark:bg-zink-600 dark:checked:bg-zink-200 dark:checked:border-zink-200" type="radio" value="">
							<label for="radioSoft8" class="align-middle">
								Light
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioSoft9" class="size-4 border rounded-full appearance-none cursor-pointer border-slate-300 bg-slate-200 checked:bg-slate-700 checked:border-slate-700 dark:border-zink-500 dark:bg-zink-600 dark:checked:bg-zink-400 dark:checked:border-zink-400" type="radio" value="" checked>
							<label for="radioSoft9" class="align-middle">
								Dark
							</label>
						</div>
					</div>
				</div>
			</div><!--end card-->

			<div class="card">
				<div class="card-body">
					<h6 class="mb-4 text-16">Outline Radio</h6>
			
					<div class="flex flex-col gap-2">
						<div class="flex items-center gap-2">
							<input id="radioOutline11" class="size-4 cursor-pointer bg-white border border-slate-200 dark:bg-zink-700 dark:border-zink-500 rounded-full appearance-none arrow-none relative after:absolute after:content-['\eb7c'] after:text-[8px] after:top-[3px] after:left-[3px] after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800" type="radio" value="">
							<label for="radioOutline11" class="align-middle">
								Custom
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioOutline12" class="size-4 cursor-pointer bg-white border border-slate-200 dark:bg-zink-700 dark:border-zink-500 rounded-full appearance-none arrow-none relative after:absolute after:content-['\eb7c'] after:text-[8px] after:top-[3px] after:left-[3px] after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-green-500 checked:border-green-500 dark:after:text-green-500 dark:checked:border-green-800" type="radio" value="" checked>
							<label for="radioOutline12" class="align-middle">
								Green
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioOutline13" class="size-4 cursor-pointer bg-white border border-slate-200 dark:bg-zink-700 dark:border-zink-500 rounded-full appearance-none arrow-none relative after:absolute after:content-['\eb7c'] after:text-[8px] after:top-[3px] after:left-[3px] after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-orange-500 checked:border-orange-500 dark:after:text-orange-500 dark:checked:border-orange-800" type="radio" value="">
							<label for="radioOutline13" class="align-middle">
								Orange
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioOutline14" class="size-4 cursor-pointer bg-white border border-slate-200 dark:bg-zink-700 dark:border-zink-500 rounded-full appearance-none arrow-none relative after:absolute after:content-['\eb7c'] after:text-[8px] after:top-[3px] after:left-[3px] after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-sky-500 checked:border-sky-500 dark:after:text-sky-500 dark:checked:border-sky-800" type="radio" value="" checked>
							<label for="radioOutline14" class="align-middle">
								Sky
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioOutline15" class="size-4 cursor-pointer bg-white border border-slate-200 dark:bg-zink-700 dark:border-zink-500 rounded-full appearance-none arrow-none relative after:absolute after:content-['\eb7c'] after:text-[8px] after:top-[3px] after:left-[3px] after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-yellow-500 checked:border-yellow-500 dark:after:text-yellow-500 dark:checked:border-yellow-800" type="radio" value="">
							<label for="radioOutline15" class="align-middle">
								Yellow
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioOutline16" class="size-4 cursor-pointer bg-white border border-slate-200 dark:bg-zink-700 dark:border-zink-500 rounded-full appearance-none arrow-none relative after:absolute after:content-['\eb7c'] after:text-[8px] after:top-[3px] after:left-[3px] after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-red-500 checked:border-red-500 dark:after:text-red-500 dark:checked:border-red-800" type="radio" value="">
							<label for="radioOutline16" class="align-middle">
								Red
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioOutline17" class="size-4 cursor-pointer bg-white border border-slate-200 dark:bg-zink-700 dark:border-zink-500 rounded-full appearance-none arrow-none relative after:absolute after:content-['\eb7c'] after:text-[8px] after:top-[3px] after:left-[3px] after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-purple-500 checked:border-purple-500 dark:after:text-purple-500 dark:checked:border-purple-800" type="radio" value="" checked>
							<label for="radioOutline17" class="align-middle">
								Purple
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioOutline18" class="size-4 cursor-pointer bg-white border border-slate-200 dark:bg-zink-700 dark:border-zink-500 rounded-full appearance-none arrow-none relative after:absolute after:content-['\eb7c'] after:text-[8px] after:top-[3px] after:left-[3px] after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-slate-400 checked:border-slate-400 dark:after:text-zink-200 dark:checked:border-zink-200" type="radio" value="">
							<label for="radioOutline18" class="align-middle">
								Light
							</label>
						</div>
			
						<div class="flex items-center gap-2">
							<input id="radioOutline19" class="size-4 cursor-pointer bg-white border border-slate-200 dark:bg-zink-700 dark:border-zink-500 rounded-full appearance-none arrow-none relative after:absolute after:content-['\eb7c'] after:text-[8px] after:top-[3px] after:left-[3px] after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-slate-700 checked:border-slate-700 dark:after:text-zink-400 dark:checked:border-zink-400" type="radio" value="" checked>
							<label for="radioOutline19" class="align-middle">
								Dark
							</label>
						</div>
					</div>
				</div>
			</div><!--end card-->
			<div>
				<div class="card">
					<div class="card-body">
						<h6 class="mb-4 text-16">Inline Checkbox</h6>
				
						<div class="flex flex-wrap gap-2">
							<div class="flex items-center gap-2">
								<input id="checkboxDefault21" class="size-4 border rounded-sm appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-custom-500 checked:border-custom-500 dark:checked:bg-custom-500 dark:checked:border-custom-500 checked:disabled:bg-custom-400 checked:disabled:border-custom-400" type="checkbox" value="" checked>
								<label for="checkboxDefault21" class="align-middle">
									Custom
								</label>
							</div>
				
							<div class="flex items-center gap-2">
								<input id="checkboxDefault22" class="size-4 border rounded-sm appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-green-500 checked:border-green-500 dark:checked:bg-green-500 dark:checked:border-green-500 checked:disabled:bg-green-400 checked:disabled:border-green-400" type="checkbox" value="">
								<label for="checkboxDefault22" class="align-middle">
									Green
								</label>
							</div>
				
							<div class="flex items-center gap-2">
								<input id="checkboxDefault23" class="size-4 border rounded-sm appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-orange-500 checked:border-orange-500 dark:checked:bg-orange-500 dark:checked:border-orange-500 checked:disabled:bg-orange-400 checked:disabled:border-orange-400" type="checkbox" value="" checked>
								<label for="checkboxDefault23" class="align-middle">
									Orange
								</label>
							</div>
				
							<div class="flex items-center gap-2">
								<input id="checkboxDefault24" class="size-4 border rounded-sm appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-sky-500 checked:border-sky-500 dark:checked:bg-sky-500 dark:checked:border-sky-500 checked:disabled:bg-sky-400 checked:disabled:border-sky-400" type="checkbox" value="">
								<label for="checkboxDefault24" class="align-middle">
									Sky
								</label>
							</div>
				
							<div class="flex items-center gap-2">
								<input id="checkboxDefault25" class="size-4 border rounded-sm appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-yellow-500 checked:border-yellow-500 dark:checked:bg-yellow-500 dark:checked:border-yellow-500 checked:disabled:bg-yellow-400 checked:disabled:border-yellow-400" type="checkbox" value="">
								<label for="checkboxDefault25" class="align-middle">
									Yellow
								</label>
							</div>
				
							<div class="flex items-center gap-2">
								<input id="checkboxDefault26" class="size-4 border rounded-sm appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-red-500 checked:border-red-500 dark:checked:bg-red-500 dark:checked:border-red-500 checked:disabled:bg-red-400 checked:disabled:border-red-400" type="checkbox" value="" checked>
								<label for="checkboxDefault26" class="align-middle">
									Red
								</label>
							</div>
						</div>
					</div>
				</div><!--end card-->
				<div class="card">
					<div class="card-body">
						<h6 class="mb-4 text-16">Without Label Checkbox</h6>
				
						<div class="flex flex-wrap gap-2">
							<div class="flex items-center gap-2">
								<input id="checkboxDefault11" class="size-4 border rounded-sm appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-custom-500 checked:border-custom-500 dark:checked:bg-custom-500 dark:checked:border-custom-500 checked:disabled:bg-custom-400 checked:disabled:border-custom-400" type="checkbox" value="" checked>
								<label for="checkboxDefault11" class="hidden align-middle">
									Custom
								</label>
							</div>
				
							<div class="flex items-center gap-2">
								<input id="checkboxDefault12" class="size-4 border rounded-sm appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-green-500 checked:border-green-500 dark:checked:bg-green-500 dark:checked:border-green-500 checked:disabled:bg-green-400 checked:disabled:border-green-400" type="checkbox" value="">
								<label for="checkboxDefault12" class="hidden align-middle">
									Green
								</label>
							</div>
				
							<div class="flex items-center gap-2">
								<input id="checkboxDefault13" class="size-4 border rounded-sm appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-orange-500 checked:border-orange-500 dark:checked:bg-orange-500 dark:checked:border-orange-500 checked:disabled:bg-orange-400 checked:disabled:border-orange-400" type="checkbox" value="" checked>
								<label for="checkboxDefault13" class="hidden align-middle">
									Orange
								</label>
							</div>
				
							<div class="flex items-center gap-2">
								<input id="checkboxDefault14" class="size-4 border rounded-sm appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-sky-500 checked:border-sky-500 dark:checked:bg-sky-500 dark:checked:border-sky-500 checked:disabled:bg-sky-400 checked:disabled:border-sky-400" type="checkbox" value="">
								<label for="checkboxDefault14" class="hidden align-middle">
									Sky
								</label>
							</div>
				
							<div class="flex items-center gap-2">
								<input id="checkboxDefault15" class="size-4 border rounded-sm appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-yellow-500 checked:border-yellow-500 dark:checked:bg-yellow-500 dark:checked:border-yellow-500 checked:disabled:bg-yellow-400 checked:disabled:border-yellow-400" type="checkbox" value="">
								<label for="checkboxDefault15" class="hidden align-middle">
									Yellow
								</label>
							</div>
				
							<div class="flex items-center gap-2">
								<input id="checkboxDefault16" class="size-4 border rounded-sm appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-red-500 checked:border-red-500 dark:checked:bg-red-500 dark:checked:border-red-500 checked:disabled:bg-red-400 checked:disabled:border-red-400" type="checkbox" value="" checked>
								<label for="checkboxDefault16" class="hidden align-middle">
									Red
								</label>
							</div>
				
							<div class="flex items-center gap-2">
								<input id="checkboxDefault17" class="size-4 border rounded-sm appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-purple-500 checked:border-purple-500 dark:checked:bg-purple-500 dark:checked:border-purple-500 checked:disabled:bg-purple-400 checked:disabled:border-purple-400" type="checkbox" value="" checked>
								<label for="checkboxDefault17" class="hidden align-middle">
									Purple
								</label>
							</div>
				
							<div class="flex items-center gap-2">
								<input id="checkboxDefault18" class="size-4 border rounded-sm appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-slate-300 checked:border-slate-300 dark:checked:bg-zink-200 dark:checked:border-zink-200 checked:disabled:bg-zink-400 checked:disabled:border-zink-400" type="checkbox" value="">
								<label for="checkboxDefault18" class="hidden align-middle">
									Light
								</label>
							</div>
				
							<div class="flex items-center gap-2">
								<input id="checkboxDefault19" class="size-4 border rounded-sm appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-slate-700 checked:border-slate-700 dark:checked:bg-zink-400 dark:checked:border-zink-400 checked:disabled:bg-zink-500 checked:disabled:border-zink-500" type="checkbox" value="" checked>
								<label for="checkboxDefault19" class="hidden align-middle">
									Dark
								</label>
							</div>
						</div>
					</div>
				</div><!--end card-->
				<div class="card">
					<div class="card-body">
						<h6 class="mb-4 text-16">Disabled Checkbox</h6>
				
						<div class="flex flex-wrap gap-2">
							<div class="flex items-center gap-2">
								<input id="checkboxDefault31" class="size-4 border rounded-sm appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-custom-500 checked:border-custom-500 dark:checked:bg-custom-500 dark:checked:border-custom-500 disabled:bg-custom-400 checked:disabled:border-custom-400" type="checkbox" value="" checked disabled>
								<label for="checkboxDefault31" class="align-middle">
									Disabled Custom
								</label>
							</div>
				
							<div class="flex items-center gap-2">
								<input id="checkboxDefault32" class="size-4 border rounded-sm appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-green-500 checked:border-green-500 dark:checked:bg-green-500 dark:checked:border-green-500 checked:disabled:bg-green-400 checked:disabled:border-green-400" type="checkbox" value="" disabled>
								<label for="checkboxDefault32" class="align-middle">
									Disabled Green
								</label>
							</div>
				
							<div class="flex items-center gap-2">
								<input id="checkboxDefault33" class="size-4 border rounded-sm appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-orange-500 checked:border-orange-500 dark:checked:bg-orange-500 dark:checked:border-orange-500 checked:disabled:bg-orange-400 checked:disabled:border-orange-400" type="checkbox" value="" checked disabled>
								<label for="checkboxDefault33" class="align-middle">
									Disabled Orange
								</label>
							</div>
						</div>
					</div>
				</div><!--end card-->
			</div>
			<div>
				<div class="card">
					<div class="card-body">
						<h6 class="mb-4 text-16">Inline Radio</h6>
				
						<div class="flex flex-wrap gap-2">
							<div class="flex items-center gap-2">
								<input id="radioInline1" name="InlineRadio" class="size-4 border rounded-full appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-custom-500 checked:border-custom-500 dark:checked:bg-custom-500 dark:checked:border-custom-500" type="radio" value="" checked>
								<label for="radioInline1" class="align-middle">
									Custom
								</label>
							</div>
				
							<div class="flex items-center gap-2">
								<input id="radioInline2" name="InlineRadio" class="size-4 border rounded-full appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-green-500 checked:border-green-500 dark:checked:bg-green-500 dark:checked:border-green-500" type="radio" value="">
								<label for="radioInline2" class="align-middle">
									Green
								</label>
							</div>
				
							<div class="flex items-center gap-2">
								<input id="radioInline3" name="InlineRadio" class="size-4 border rounded-full appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-orange-500 checked:border-orange-500 dark:checked:bg-orange-500 dark:checked:border-orange-500" type="radio" value="">
								<label for="radioInline3" class="align-middle">
									Orange
								</label>
							</div>
				
							<div class="flex items-center gap-2">
								<input id="radioInline4" name="InlineRadio" class="size-4 border rounded-full appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-sky-500 checked:border-sky-500 dark:checked:bg-sky-500 dark:checked:border-sky-500" type="radio" value="">
								<label for="radioInline4" class="align-middle">
									Sky
								</label>
							</div>
						</div>
					</div>
				</div><!--end card-->
				<div class="card">
					<div class="card-body">
						<h6 class="mb-4 text-16">Without Label Radio</h6>
				
						<div class="flex flex-wrap gap-2">
							<div class="flex items-center gap-2">
								<input id="radioInlineWithoutLabel1" name="withoutInlineRadio" class="size-4 border rounded-full appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-custom-500 checked:border-custom-500 dark:checked:bg-custom-500 dark:checked:border-custom-500" type="radio" value="">
								<label for="radioInlineWithoutLabel1" class="hidden align-middle">
									Custom
								</label>
							</div>
				
							<div class="flex items-center gap-2">
								<input id="radioInlineWithoutLabel2" name="withoutInlineRadio" class="size-4 border rounded-full appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-green-500 checked:border-green-500 dark:checked:bg-green-500 dark:checked:border-green-500" type="radio" value="" checked>
								<label for="radioInlineWithoutLabel2" class="hidden align-middle">
									Green
								</label>
							</div>
				
							<div class="flex items-center gap-2">
								<input id="radioInlineWithoutLabel3" name="withoutInlineRadio" class="size-4 border rounded-full appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-orange-500 checked:border-orange-500 dark:checked:bg-orange-500 dark:checked:border-orange-500" type="radio" value="">
								<label for="radioInlineWithoutLabel3" class="hidden align-middle">
									Orange
								</label>
							</div>
				
							<div class="flex items-center gap-2">
								<input id="radioInlineWithoutLabel4" name="withoutInlineRadio" class="size-4 border rounded-full appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-sky-500 checked:border-sky-500 dark:checked:bg-sky-500 dark:checked:border-sky-500" type="radio" value="">
								<label for="radioInlineWithoutLabel4" class="hidden align-middle">
									Sky
								</label>
							</div>
						</div>
					</div>
				</div><!--end card-->
				<div class="card">
					<div class="card-body">
						<h6 class="mb-4 text-16">Disabled Radio</h6>
				
						<div class="flex flex-wrap gap-2">
							<div class="flex items-center gap-2">
								<input id="radioDisabled1" name="disabledRadio" class="size-4 border rounded-full appearance-none bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-custom-500 checked:border-custom-500 dark:checked:bg-custom-500 dark:checked:border-custom-500 checked:disabled:border-custom-400 checked:disabled:bg-custom-400" type="radio" value="" disabled>
								<label for="radioDisabled1" class="align-middle">
									Disabled Custom
								</label>
							</div>
				
							<div class="flex items-center gap-2">
								<input id="radioDisabled2" name="disabledRadio" class="size-4 border rounded-full appearance-none bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-green-500 checked:border-green-500 dark:checked:bg-green-500 dark:checked:border-green-500 checked:disabled:border-green-400 checked:disabled:bg-green-400" type="radio" value="" disabled>
								<label for="radioDisabled2" class="align-middle">
									Disabled Green
								</label>
							</div>
				
							<div class="flex items-center gap-2">
								<input id="radioDisabled3" name="disabledRadio" class="size-4 border rounded-full appearance-none bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-orange-500 checked:border-orange-500 dark:checked:bg-orange-500 dark:checked:border-orange-500 checked:disabled:border-orange-400 checked:disabled:bg-orange-400" type="radio" value="" checked disabled>
								<label for="radioDisabled3" class="align-middle">
									Disabled Orange
								</label>
							</div>
						</div>
					</div>
				</div><!--end card-->
			</div>
		</div><!--end grid-->
</div>