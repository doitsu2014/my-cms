<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
</script>

<HeadTitle title="Switch" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Switch" pagetitle="Forms" />

	<div class="grid grid-cols-1 gap-x-5 xl:grid-cols-3">
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Default Switch</h6>
				<div class="flex flex-col gap-3">
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="customDefaultSwitch" id="customDefaultSwitch" class="absolute block size-5 transition duration-300 ease-linear border-2 rounded-full appearance-none cursor-pointer border-slate-200 dark:border-zink-500 bg-white/80 dark:bg-zink-400 peer/published checked:bg-white dark:checked:bg-white ltr:checked:right-0 rtl:checked:left-0 checked:bg-none checked:border-custom-500 dark:checked:border-custom-500 arrow-none" checked>
							<label for="customDefaultSwitch" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-full cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-custom-500 peer-checked/published:border-custom-500"></label>
						</div>
						<label for="customDefaultSwitch" class="inline-block text-base font-medium cursor-pointer">Custom Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="greenDefaultSwitch" id="greenDefaultSwitch" class="absolute block size-5 transition duration-300 ease-linear border-2 rounded-full appearance-none cursor-pointer border-slate-200 dark:border-zink-500 bg-white/80 dark:bg-zink-400 peer/published checked:bg-white dark:checked:bg-white ltr:checked:right-0 rtl:checked:left-0 checked:bg-none checked:border-green-500 dark:checked:border-green-500 arrow-none" checked>
							<label for="greenDefaultSwitch" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-full cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-green-500 peer-checked/published:border-green-500"></label>
						</div>
						<label for="greenDefaultSwitch" class="inline-block text-base font-medium cursor-pointer">Green Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="orangeDefaultSwitch" id="orangeDefaultSwitch" class="absolute block size-5 transition duration-300 ease-linear border-2 rounded-full appearance-none cursor-pointer border-slate-200 dark:border-zink-500 bg-white/80 dark:bg-zink-400 peer/published checked:bg-white dark:checked:bg-white ltr:checked:right-0 rtl:checked:left-0 checked:bg-none checked:border-orange-500 dark:checked:border-orange-500 arrow-none" checked>
							<label for="orangeDefaultSwitch" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-full cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-orange-500 peer-checked/published:border-orange-500"></label>
						</div>
						<label for="orangeDefaultSwitch" class="inline-block text-base font-medium cursor-pointer">Orange Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="greenDefaultSwitch" id="skyDefaultSwitch" class="absolute block size-5 transition duration-300 ease-linear border-2 rounded-full appearance-none cursor-pointer border-slate-200 dark:border-zink-500 bg-white/80 dark:bg-zink-400 peer/published checked:bg-white dark:checked:bg-white ltr:checked:right-0 rtl:checked:left-0 checked:bg-none checked:border-sky-500 dark:checked:border-sky-500 arrow-none" checked>
							<label for="skyDefaultSwitch" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-full cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-sky-500 peer-checked/published:border-sky-500"></label>
						</div>
						<label for="skyDefaultSwitch" class="inline-block text-base font-medium cursor-pointer">Sky Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="yellowDefaultSwitch" id="yellowDefaultSwitch" class="absolute block size-5 transition duration-300 ease-linear border-2 rounded-full appearance-none cursor-pointer border-slate-200 dark:border-zink-500 bg-white/80 dark:bg-zink-400 peer/published checked:bg-white dark:checked:bg-white ltr:checked:right-0 rtl:checked:left-0 checked:bg-none checked:border-yellow-500 dark:checked:border-yellow-500 arrow-none" checked>
							<label for="yellowDefaultSwitch" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-full cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-yellow-500 peer-checked/published:border-yellow-500"></label>
						</div>
						<label for="yellowDefaultSwitch" class="inline-block text-base font-medium cursor-pointer">Yellow Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="redDefaultSwitch" id="redDefaultSwitch" class="absolute block size-5 transition duration-300 ease-linear border-2 rounded-full appearance-none cursor-pointer border-slate-200 dark:border-zink-500 bg-white/80 dark:bg-zink-400 peer/published checked:bg-white dark:checked:bg-white ltr:checked:right-0 rtl:checked:left-0 checked:bg-none checked:border-red-500 dark:checked:border-red-500 arrow-none" checked>
							<label for="redDefaultSwitch" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-full cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-red-500 peer-checked/published:border-red-500"></label>
						</div>
						<label for="redDefaultSwitch" class="inline-block text-base font-medium cursor-pointer">Red Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="purpleDefaultSwitch" id="purpleDefaultSwitch" class="absolute block size-5 transition duration-300 ease-linear border-2 rounded-full appearance-none cursor-pointer border-slate-200 dark:border-zink-500 bg-white/80 dark:bg-zink-400 peer/published checked:bg-white dark:checked:bg-white ltr:checked:right-0 rtl:checked:left-0 checked:bg-none checked:border-purple-500 dark:checked:border-purple-500 arrow-none" checked>
							<label for="purpleDefaultSwitch" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-full cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-purple-500 peer-checked/published:border-purple-500"></label>
						</div>
						<label for="purpleDefaultSwitch" class="inline-block text-base font-medium cursor-pointer">Purple Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="lightDefaultSwitch" id="lightDefaultSwitch" class="absolute block size-5 transition duration-300 ease-linear border-2 rounded-full appearance-none cursor-pointer border-slate-200 dark:border-zink-500 bg-white/80 dark:bg-zink-400 peer/published checked:bg-white dark:checked:bg-white ltr:checked:right-0 rtl:checked:left-0 checked:bg-none checked:border-slate-400 dark:checked:border-slate-400 arrow-none" checked>
							<label for="lightDefaultSwitch" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-full cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-slate-400 peer-checked/published:border-slate-400"></label>
						</div>
						<label for="lightDefaultSwitch" class="inline-block text-base font-medium cursor-pointer">light Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="darkDefaultSwitch" id="darkDefaultSwitch" class="absolute block size-5 transition duration-300 ease-linear border-2 rounded-full appearance-none cursor-pointer border-slate-200 dark:border-zink-500 bg-white/80 dark:bg-zink-400 peer/published checked:bg-white dark:checked:bg-white ltr:checked:right-0 rtl:checked:left-0 checked:bg-none checked:border-slate-700 dark:checked:border-slate-700 arrow-none" checked>
							<label for="darkDefaultSwitch" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-full cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-slate-700 peer-checked/published:border-slate-700"></label>
						</div>
						<label for="darkDefaultSwitch" class="inline-block text-base font-medium cursor-pointer">Dark Switch</label>
					</div>
				</div>
			</div>
		</div><!--end card-->
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Square Switch</h6>
				<div class="flex flex-col gap-3">
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="customSquareSwitch" id="customSquareSwitch" class="absolute block size-5 transition duration-300 ease-linear border-2 rounded-sm appearance-none cursor-pointer border-slate-200 dark:border-zink-500 bg-white/80 dark:bg-zink-400 peer/published checked:bg-white dark:checked:bg-white ltr:checked:right-0 rtl:checked:left-0 checked:bg-none checked:border-custom-500 dark:checked:border-custom-500 arrow-none" checked>
							<label for="customSquareSwitch" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-sm cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-custom-500 peer-checked/published:border-custom-500"></label>
						</div>
						<label for="customSquareSwitch" class="inline-block text-base font-medium cursor-pointer">Custom Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="greenSquareSwitch" id="greenSquareSwitch" class="absolute block size-5 transition duration-300 ease-linear border-2 rounded-sm appearance-none cursor-pointer border-slate-200 dark:border-zink-500 bg-white/80 dark:bg-zink-400 peer/published checked:bg-white dark:checked:bg-white ltr:checked:right-0 rtl:checked:left-0 checked:bg-none checked:border-green-500 dark:checked:border-green-500 arrow-none" checked>
							<label for="greenSquareSwitch" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-sm cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-green-500 peer-checked/published:border-green-500"></label>
						</div>
						<label for="greenSquareSwitch" class="inline-block text-base font-medium cursor-pointer">Green Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="orangeSquareSwitch" id="orangeSquareSwitch" class="absolute block size-5 transition duration-300 ease-linear border-2 rounded-sm appearance-none cursor-pointer border-slate-200 dark:border-zink-500 bg-white/80 dark:bg-zink-400 peer/published checked:bg-white dark:checked:bg-white ltr:checked:right-0 rtl:checked:left-0 checked:bg-none checked:border-orange-500 dark:checked:border-orange-500 arrow-none" checked>
							<label for="orangeSquareSwitch" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-sm cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-orange-500 peer-checked/published:border-orange-500"></label>
						</div>
						<label for="orangeSquareSwitch" class="inline-block text-base font-medium cursor-pointer">Orange Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="greenSquareSwitch" id="skySquareSwitch" class="absolute block size-5 transition duration-300 ease-linear border-2 rounded-sm appearance-none cursor-pointer border-slate-200 dark:border-zink-500 bg-white/80 dark:bg-zink-400 peer/published checked:bg-white dark:checked:bg-white ltr:checked:right-0 rtl:checked:left-0 checked:bg-none checked:border-sky-500 dark:checked:border-sky-500 arrow-none" checked>
							<label for="skySquareSwitch" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-sm cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-sky-500 peer-checked/published:border-sky-500"></label>
						</div>
						<label for="skySquareSwitch" class="inline-block text-base font-medium cursor-pointer">Sky Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="yellowSquareSwitch" id="yellowSquareSwitch" class="absolute block size-5 transition duration-300 ease-linear border-2 rounded-sm appearance-none cursor-pointer border-slate-200 dark:border-zink-500 bg-white/80 dark:bg-zink-400 peer/published checked:bg-white dark:checked:bg-white ltr:checked:right-0 rtl:checked:left-0 checked:bg-none checked:border-yellow-500 dark:checked:border-yellow-500 arrow-none" checked>
							<label for="yellowSquareSwitch" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-sm cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-yellow-500 peer-checked/published:border-yellow-500"></label>
						</div>
						<label for="yellowSquareSwitch" class="inline-block text-base font-medium cursor-pointer">Yellow Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="redSquareSwitch" id="redSquareSwitch" class="absolute block size-5 transition duration-300 ease-linear border-2 rounded-sm appearance-none cursor-pointer border-slate-200 dark:border-zink-500 bg-white/80 dark:bg-zink-400 peer/published checked:bg-white dark:checked:bg-white ltr:checked:right-0 rtl:checked:left-0 checked:bg-none checked:border-red-500 dark:checked:border-red-500 arrow-none" checked>
							<label for="redSquareSwitch" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-sm cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-red-500 peer-checked/published:border-red-500"></label>
						</div>
						<label for="redSquareSwitch" class="inline-block text-base font-medium cursor-pointer">Red Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="purpleSquareSwitch" id="purpleSquareSwitch" class="absolute block size-5 transition duration-300 ease-linear border-2 rounded-sm appearance-none cursor-pointer border-slate-200 dark:border-zink-500 bg-white/80 dark:bg-zink-400 peer/published checked:bg-white dark:checked:bg-white ltr:checked:right-0 rtl:checked:left-0 checked:bg-none checked:border-purple-500 dark:checked:border-purple-500 arrow-none" checked>
							<label for="purpleSquareSwitch" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-sm cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-purple-500 peer-checked/published:border-purple-500"></label>
						</div>
						<label for="purpleSquareSwitch" class="inline-block text-base font-medium cursor-pointer">Purple Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="lightSquareSwitch" id="lightSquareSwitch" class="absolute block size-5 transition duration-300 ease-linear border-2 rounded-sm appearance-none cursor-pointer border-slate-200 dark:border-zink-500 bg-white/80 dark:bg-zink-400 peer/published checked:bg-white dark:checked:bg-white ltr:checked:right-0 rtl:checked:left-0 checked:bg-none checked:border-slate-400 dark:checked:border-slate-400 arrow-none" checked>
							<label for="lightSquareSwitch" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-sm cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-slate-400 peer-checked/published:border-slate-400"></label>
						</div>
						<label for="lightSquareSwitch" class="inline-block text-base font-medium cursor-pointer">light Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="darkSquareSwitch" id="darkSquareSwitch" class="absolute block size-5 transition duration-300 ease-linear border-2 rounded-sm appearance-none cursor-pointer border-slate-200 dark:border-zink-500 bg-white/80 dark:bg-zink-400 peer/published checked:bg-white dark:checked:bg-white ltr:checked:right-0 rtl:checked:left-0 checked:bg-none checked:border-slate-700 dark:checked:border-slate-700 arrow-none" checked>
							<label for="darkSquareSwitch" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-sm cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-slate-700 peer-checked/published:border-slate-700"></label>
						</div>
						<label for="darkSquareSwitch" class="inline-block text-base font-medium cursor-pointer">Dark Switch</label>
					</div>
				</div>
			</div>
		</div><!--end card-->
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Soft Switch</h6>
				<div class="flex flex-col gap-3">
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="customSoftSwitch" id="customSoftSwitch" class="absolute block size-5 transition duration-300 ease-linear border-2 rounded-full appearance-none cursor-pointer border-slate-200 dark:border-zink-600 bg-white/80 dark:bg-zink-400 peer/published checked:bg-custom-500 dark:checked:bg-custom-500 ltr:checked:right-0 rtl:checked:left-0 checked:border-custom-100 dark:checked:border-custom-900 arrow-none checked:bg-none" checked>
							<label for="customSoftSwitch" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-full cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-custom-100 dark:peer-checked/published:bg-custom-900 peer-checked/published:border-custom-100 dark:peer-checked/published:border-custom-900"></label>
						</div>
						<label for="customSoftSwitch" class="inline-block text-base font-medium cursor-pointer">Custom Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="greenSoftSwitch" id="greenSoftSwitch" class="absolute block size-5 transition duration-300 ease-linear border-2 rounded-full appearance-none cursor-pointer border-slate-200 dark:border-zink-600 bg-white/80 dark:bg-zink-400 peer/published checked:bg-green-500 dark:checked:bg-green-500 ltr:checked:right-0 rtl:checked:left-0 checked:border-green-100 dark:checked:border-green-900 arrow-none checked:bg-none" checked>
							<label for="greenSoftSwitch" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-full cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-green-100 peer-checked/published:border-green-100 dark:peer-checked/published:bg-green-900 dark:peer-checked/published:border-green-900"></label>
						</div>
						<label for="greenSoftSwitch" class="inline-block text-base font-medium cursor-pointer">Green Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="orangeSoftSwitch" id="orangeSoftSwitch" class="absolute block size-5 transition duration-300 ease-linear border-2 rounded-full appearance-none cursor-pointer border-slate-200 dark:border-zink-600 bg-white/80 dark:bg-zink-400 peer/published checked:bg-orange-500 dark:checked:bg-orange-500 ltr:checked:right-0 rtl:checked:left-0 checked:border-orange-100 dark:checked:border-orange-900 arrow-none checked:bg-none" checked>
							<label for="orangeSoftSwitch" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-full cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-orange-100 peer-checked/published:border-orange-100 dark:peer-checked/published:bg-orange-900 dark:peer-checked/published:border-orange-900"></label>
						</div>
						<label for="orangeSoftSwitch" class="inline-block text-base font-medium cursor-pointer">Orange Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="greenSoftSwitch" id="skySoftSwitch" class="absolute block size-5 transition duration-300 ease-linear border-2 rounded-full appearance-none cursor-pointer border-slate-200 dark:border-zink-600 bg-white/80 dark:bg-zink-400 peer/published checked:bg-sky-500 dark:checked:bg-sky-500 ltr:checked:right-0 rtl:checked:left-0 checked:border-sky-100 arrow-none dark:checked:border-sky-900 checked:bg-none" checked>
							<label for="skySoftSwitch" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-full cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-sky-100 peer-checked/published:border-sky-100 dark:peer-checked/published:bg-sky-900 dark:peer-checked/published:border-sky-900"></label>
						</div>
						<label for="skySoftSwitch" class="inline-block text-base font-medium cursor-pointer">Sky Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="yellowSoftSwitch" id="yellowSoftSwitch" class="absolute block size-5 transition duration-300 ease-linear border-2 rounded-full appearance-none cursor-pointer border-slate-200 dark:border-zink-600 bg-white/80 dark:bg-zink-400 peer/published checked:bg-yellow-500 dark:checked:bg-yellow-500 ltr:checked:right-0 rtl:checked:left-0 checked:border-yellow-100 dark:checked:border-yellow-900 arrow-none checked:bg-none" checked>
							<label for="yellowSoftSwitch" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-full cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-yellow-100 peer-checked/published:border-yellow-100 dark:peer-checked/published:bg-yellow-900 dark:peer-checked/published:border-yellow-900"></label>
						</div>
						<label for="yellowSoftSwitch" class="inline-block text-base font-medium cursor-pointer">Yellow Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="redSoftSwitch" id="redSoftSwitch" class="absolute block size-5 transition duration-300 ease-linear border-2 rounded-full appearance-none cursor-pointer border-slate-200 dark:border-zink-600 bg-white/80 dark:bg-zink-400 peer/published checked:bg-red-500 dark:checked:bg-red-500 ltr:checked:right-0 rtl:checked:left-0 checked:border-red-100 dark:checked:border-red-900 arrow-none checked:bg-none" checked>
							<label for="redSoftSwitch" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-full cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-red-100 peer-checked/published:border-red-100 dark:peer-checked/published:bg-red-900 dark:peer-checked/published:border-red-900"></label>
						</div>
						<label for="redSoftSwitch" class="inline-block text-base font-medium cursor-pointer">Red Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="purpleSoftSwitch" id="purpleSoftSwitch" class="absolute block size-5 transition duration-300 ease-linear border-2 rounded-full appearance-none cursor-pointer border-slate-200 dark:border-zink-600 bg-white/80 dark:bg-zink-400 peer/published checked:bg-purple-500 dark:checked:bg-purple-500 ltr:checked:right-0 rtl:checked:left-0 checked:border-purple-100 dark:checked:border-purple-900 arrow-none checked:bg-none" checked>
							<label for="purpleSoftSwitch" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-full cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-purple-100 peer-checked/published:border-purple-100 dark:peer-checked/published:bg-purple-900 dark:peer-checked/published:border-purple-900"></label>
						</div>
						<label for="purpleSoftSwitch" class="inline-block text-base font-medium cursor-pointer">Purple Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="lightSoftSwitch" id="lightSoftSwitch" class="absolute block size-5 transition duration-300 ease-linear border-2 rounded-full appearance-none cursor-pointer border-slate-200 dark:border-zink-600 bg-white/80 dark:bg-zink-400 peer/published checked:bg-slate-500 dark:checked:bg-zink-500 ltr:checked:right-0 rtl:checked:left-0 checked:border-slate-200 dark:checked:border-zink-200 arrow-none checked:bg-none" checked>
							<label for="lightSoftSwitch" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-full cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-slate-300 peer-checked/published:border-slate-100 dark:peer-checked/published:bg-zink-200 dark:peer-checked/published:border-zink-200"></label>
						</div>
						<label for="lightSoftSwitch" class="inline-block text-base font-medium cursor-pointer">light Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="darkSoftSwitch" id="darkSoftSwitch" class="absolute block size-5 transition duration-300 ease-linear border-2 rounded-full appearance-none cursor-pointer border-slate-200 dark:border-zink-600 bg-white/80 dark:bg-zink-400 peer/published checked:bg-slate-500 dark:checked:bg-zink-300 ltr:checked:right-0 rtl:checked:left-0 checked:border-slate-400 dark:checked:border-zink-500 arrow-none checked:bg-none" checked>
							<label for="darkSoftSwitch" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-full cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-slate-400 peer-checked/published:border-slate-400 dark:peer-checked/published:bg-zink-500 dark:peer-checked/published:border-zink-500"></label>
						</div>
						<label for="darkSoftSwitch" class="inline-block text-base font-medium cursor-pointer">Dark Switch</label>
					</div>
				</div>
			</div>
		</div><!--end card-->
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Outline Switch</h6>
				<div class="flex flex-col gap-3">
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="customOutlineSwitch" id="customOutlineSwitch" class="absolute block size-4 top-0.5 transition duration-300 ease-linear border-2 border-white rounded-full dark:border-zink-700 appearance-none cursor-pointer bg-slate-200 peer/published dark:bg-zink-500 checked:bg-custom-500 dark:checked:bg-custom-500 ltr:checked:right-0.5 rtl:checked:left-0.5 checked:left-auto checked:border-white dark:checked:border-zink-700 arrow-none" checked>
							<label for="customOutlineSwitch" class="block h-5 overflow-hidden duration-300 ease-linear bg-white border rounded-full cursor-pointer cursor-pointertransition dark:bg-zink-700 border-slate-200 dark:border-zink-500 peer-checked/published:bg-white dark:peer-checked/published:bg-zink-700 peer-checked/published:border-custom-500 dark:peer-checked/published:border-custom-800"></label>
						</div>
						<label for="customOutlineSwitch" class="inline-block text-base font-medium cursor-pointer">Custom Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="greenOutlineSwitch" id="greenOutlineSwitch" class="absolute block size-4 top-0.5 transition duration-300 ease-linear border-2 border-white rounded-full dark:border-zink-700 appearance-none cursor-pointer bg-slate-200 peer/published dark:bg-zink-500 checked:bg-green-500  dark:checked:bg-green-500 ltr:checked:right-0.5 rtl:checked:left-0.5 checked:left-auto checked:border-white dark:checked:border-zink-700 arrow-none" checked>
							<label for="greenOutlineSwitch" class="block h-5 overflow-hidden duration-300 ease-linear bg-white border rounded-full cursor-pointer cursor-pointertransition dark:bg-zink-700 border-slate-200 dark:border-zink-500 peer-checked/published:bg-white dark:peer-checked/published:bg-zink-700 peer-checked/published:border-green-500 dark:peer-checked/published:border-green-800"></label>
						</div>
						<label for="greenOutlineSwitch" class="inline-block text-base font-medium cursor-pointer">Green Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="orangeOutlineSwitch" id="orangeOutlineSwitch" class="absolute block size-4 top-0.5 transition duration-300 ease-linear border-2 border-white rounded-full dark:border-zink-700 appearance-none cursor-pointer bg-slate-200 peer/published dark:bg-zink-500 checked:bg-orange-500 dark:checked:bg-orange-500 ltr:checked:right-0.5 rtl:checked:left-0.5 checked:left-auto checked:border-white dark:checked:border-zink-700 arrow-none" checked>
							<label for="orangeOutlineSwitch" class="block h-5 overflow-hidden duration-300 ease-linear bg-white border rounded-full cursor-pointer cursor-pointertransition dark:bg-zink-700 border-slate-200 dark:border-zink-500 peer-checked/published:bg-white dark:peer-checked/published:bg-zink-700 peer-checked/published:border-orange-500 dark:peer-checked/published:border-orange-800"></label>
						</div>
						<label for="orangeOutlineSwitch" class="inline-block text-base font-medium cursor-pointer">Orange Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="skyOutlineSwitch" id="skyOutlineSwitch" class="absolute block size-4 top-0.5 transition duration-300 ease-linear border-2 border-white rounded-full dark:border-zink-700 appearance-none cursor-pointer bg-slate-200 peer/published dark:bg-zink-500 checked:bg-sky-500 dark:checked:bg-sky-500 ltr:checked:right-0.5 rtl:checked:left-0.5 checked:left-auto checked:border-white dark:checked:border-zink-700 arrow-none" checked>
							<label for="skyOutlineSwitch" class="block h-5 overflow-hidden duration-300 ease-linear bg-white border rounded-full cursor-pointer cursor-pointertransition dark:bg-zink-700 border-slate-200 dark:border-zink-500 peer-checked/published:bg-white dark:peer-checked/published:bg-zink-700 peer-checked/published:border-sky-500 dark:peer-checked/published:border-sky-800"></label>
						</div>
						<label for="skyOutlineSwitch" class="inline-block text-base font-medium cursor-pointer">Sky Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="yellowOutlineSwitch" id="yellowOutlineSwitch" class="absolute block size-4 top-0.5 transition duration-300 ease-linear border-2 border-white rounded-full dark:border-zink-700 appearance-none cursor-pointer bg-slate-200 peer/published dark:bg-zink-500 checked:bg-yellow-500 dark:checked:bg-yellow-500 ltr:checked:right-0.5 rtl:checked:left-0.5 checked:left-auto checked:border-white dark:checked:border-zink-700 arrow-none" checked>
							<label for="yellowOutlineSwitch" class="block h-5 overflow-hidden duration-300 ease-linear bg-white border rounded-full cursor-pointer cursor-pointertransition dark:bg-zink-700 border-slate-200 dark:border-zink-500 peer-checked/published:bg-white dark:peer-checked/published:bg-zink-700 peer-checked/published:border-yellow-500 dark:peer-checked/published:border-yellow-800"></label>
						</div>
						<label for="yellowOutlineSwitch" class="inline-block text-base font-medium cursor-pointer">Yellow Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="redOutlineSwitch" id="redOutlineSwitch" class="absolute block size-4 top-0.5 transition duration-300 ease-linear border-2 border-white rounded-full dark:border-zink-700 appearance-none cursor-pointer bg-slate-200 peer/published dark:bg-zink-500 checked:bg-red-500 dark:checked:bg-red-500 ltr:checked:right-0.5 rtl:checked:left-0.5 checked:left-auto checked:border-white dark:checked:border-zink-700 arrow-none" checked>
							<label for="redOutlineSwitch" class="block h-5 overflow-hidden duration-300 ease-linear bg-white border rounded-full cursor-pointer cursor-pointertransition dark:bg-zink-700 border-slate-200 dark:border-zink-500 peer-checked/published:bg-white dark:peer-checked/published:bg-zink-700 peer-checked/published:border-red-500 dark:peer-checked/published:border-red-800"></label>
						</div>
						<label for="redOutlineSwitch" class="inline-block text-base font-medium cursor-pointer">Red Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="purpleOutlineSwitch" id="purpleOutlineSwitch" class="absolute block size-4 top-0.5 transition duration-300 ease-linear border-2 border-white rounded-full dark:border-zink-700 appearance-none cursor-pointer bg-slate-200 peer/published dark:bg-zink-500 checked:bg-purple-500 dark:checked:bg-purple-500 ltr:checked:right-0.5 rtl:checked:left-0.5 checked:left-auto checked:border-white dark:checked:border-zink-700 arrow-none" checked>
							<label for="purpleOutlineSwitch" class="block h-5 overflow-hidden duration-300 ease-linear bg-white border rounded-full cursor-pointer cursor-pointertransition dark:bg-zink-700 border-slate-200 dark:border-zink-500 peer-checked/published:bg-white dark:peer-checked/published:bg-zink-700 peer-checked/published:border-purple-500 dark:peer-checked/published:border-purple-800"></label>
						</div>
						<label for="purpleOutlineSwitch" class="inline-block text-base font-medium cursor-pointer">Red Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="lightOutlineSwitch" id="lightOutlineSwitch" class="absolute block size-4 top-0.5 transition duration-300 ease-linear border-2 border-white rounded-full dark:border-zink-700 appearance-none cursor-pointer bg-slate-200 peer/published dark:bg-zink-500 checked:bg-slate-400 dark:checked:bg-zink-400 ltr:checked:right-0.5 rtl:checked:left-0.5 checked:left-auto checked:border-white dark:checked:border-zink-700 arrow-none" checked>
							<label for="lightOutlineSwitch" class="block h-5 overflow-hidden duration-300 ease-linear bg-white border rounded-full cursor-pointer cursor-pointertransition dark:bg-zink-700 border-slate-200 dark:border-zink-500 peer-checked/published:bg-white dark:peer-checked/published:bg-zink-700 peer-checked/published:border-slate-200 dark:peer-checked/published:border-slate-300"></label>
						</div>
						<label for="lightOutlineSwitch" class="inline-block text-base font-medium cursor-pointer">Light Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="darkOutlineSwitch" id="darkOutlineSwitch" class="absolute block size-4 top-0.5 transition duration-300 ease-linear border-2 border-white rounded-full dark:border-zink-700 appearance-none cursor-pointer bg-slate-200 peer/published dark:bg-zink-500 checked:bg-slate-700 dark:checked:bg-zink-500 ltr:checked:right-0.5 rtl:checked:left-0.5 checked:left-auto checked:border-white dark:checked:border-zink-700 arrow-none" checked>
							<label for="darkOutlineSwitch" class="block h-5 overflow-hidden duration-300 ease-linear bg-white border rounded-full cursor-pointer cursor-pointertransition dark:bg-zink-700 border-slate-200 dark:border-zink-500 peer-checked/published:bg-white dark:peer-checked/published:bg-zink-700 peer-checked/published:border-slate-700 dark:peer-checked/published:border-slate-500"></label>
						</div>
						<label for="darkOutlineSwitch" class="inline-block text-base font-medium cursor-pointer">Dark Switch</label>
					</div>
				</div>
			</div>
		</div><!--end card-->
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Icon with Switch</h6>
				<div class="flex flex-col gap-3">
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="customIconSwitch" id="customIconSwitch" class="absolute block size-5 transition duration-300 ease-linear border-2 border-slate-200 dark:border-zink-500 rounded-full appearance-none cursor-pointer bg-white/80 dark:bg-zink-600 peer/published checked:bg-white dark:checked:bg-white ltr:checked:right-0 rtl:checked:left-0 checked:bg-none checked:border-custom-500 dark:checked:border-custom-500 arrow-none after:absolute after:text-slate-500 dark:after:text-zink-200 after:content-['\eb99'] after:text-xs after:inset-0 after:flex after:items-center after:justify-center after:font-remix after:leading-none checked:after:text-custom-500 dark:checked:after:text-custom-500 checked:after:content-['\eb7b']" checked>
							<label for="customIconSwitch" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-full cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-custom-500 peer-checked/published:border-custom-500"></label>
						</div>
						<label for="customIconSwitch" class="inline-block text-base font-medium cursor-pointer">Custom Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="greenIconSwitch" id="greenIconSwitch" class="absolute block size-5 transition duration-300 ease-linear border-2 border-slate-200 dark:border-zink-500 rounded-full appearance-none cursor-pointer bg-white/80 dark:bg-zink-600 peer/published checked:bg-white dark:checked:bg-white ltr:checked:right-0 rtl:checked:left-0 checked:bg-none checked:border-green-500 dark:checked:border-green-500 arrow-none after:absolute after:text-slate-500 dark:after:text-zink-200 after:content-['\eb99'] after:text-xs after:inset-0 after:flex after:items-center after:justify-center after:font-remix after:leading-none checked:after:text-green-500 dark:checked:after:text-green-500 checked:after:content-['\eb7b']" checked>
							<label for="greenIconSwitch" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-full cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-green-500 peer-checked/published:border-green-500"></label>
						</div>
						<label for="greenIconSwitch" class="inline-block text-base font-medium cursor-pointer">Green Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="orangeIconSwitch" id="orangeIconSwitch" class="absolute block size-5 transition duration-300 ease-linear border-2 border-slate-200 dark:border-zink-500 rounded-full appearance-none cursor-pointer bg-white/80 dark:bg-zink-600 peer/published checked:bg-white dark:checked:bg-white ltr:checked:right-0 rtl:checked:left-0 checked:bg-none checked:border-orange-500 dark:checked:border-orange-500 arrow-none after:absolute after:text-slate-500 dark:after:text-zink-200 after:content-['\eb99'] after:text-xs after:inset-0 after:flex after:items-center after:justify-center after:font-remix after:leading-none checked:after:text-orange-500 dark:checked:after:text-orange-500 checked:after:content-['\eb7b']" checked>
							<label for="orangeIconSwitch" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-full cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-orange-500 peer-checked/published:border-orange-500"></label>
						</div>
						<label for="orangeIconSwitch" class="inline-block text-base font-medium cursor-pointer">Orange Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="skyIconSwitch" id="skyIconSwitch" class="absolute block size-5 transition duration-300 ease-linear border-2 border-slate-200 dark:border-zink-500 rounded-full appearance-none cursor-pointer bg-white/80 dark:bg-zink-600 peer/published checked:bg-white dark:checked:bg-white ltr:checked:right-0 rtl:checked:left-0 checked:bg-none checked:border-sky-500 dark:checked:border-sky-500 arrow-none after:absolute after:text-slate-500 dark:after:text-zink-200 after:content-['\eb99'] after:text-xs after:inset-0 after:flex after:items-center after:justify-center after:font-remix after:leading-none checked:after:text-sky-500 dark:checked:after:text-sky-500 checked:after:content-['\eb7b']" checked>
							<label for="skyIconSwitch" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-full cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-sky-500 peer-checked/published:border-sky-500"></label>
						</div>
						<label for="skyIconSwitch" class="inline-block text-base font-medium cursor-pointer">Sky Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="yellowIconSwitch" id="yellowIconSwitch" class="absolute block size-5 transition duration-300 ease-linear border-2 border-slate-200 dark:border-zink-500 rounded-full appearance-none cursor-pointer bg-white/80 dark:bg-zink-600 peer/published checked:bg-white dark:checked:bg-white ltr:checked:right-0 rtl:checked:left-0 checked:bg-none checked:border-yellow-500 dark:checked:border-yellow-500 arrow-none after:absolute after:text-slate-500 dark:after:text-zink-200 after:content-['\eb99'] after:text-xs after:inset-0 after:flex after:items-center after:justify-center after:font-remix after:leading-none checked:after:text-yellow-500 dark:checked:after:text-yellow-500 checked:after:content-['\eb7b']" checked>
							<label for="yellowIconSwitch" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-full cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-yellow-500 peer-checked/published:border-yellow-500"></label>
						</div>
						<label for="yellowIconSwitch" class="inline-block text-base font-medium cursor-pointer">Yellow Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="redIconSwitch" id="redIconSwitch" class="absolute block size-5 transition duration-300 ease-linear border-2 border-slate-200 dark:border-zink-500 rounded-full appearance-none cursor-pointer bg-white/80 dark:bg-zink-600 peer/published checked:bg-white dark:checked:bg-white ltr:checked:right-0 rtl:checked:left-0 checked:bg-none checked:border-red-500 dark:checked:border-red-500 arrow-none after:absolute after:text-slate-500 dark:after:text-zink-200 after:content-['\eb99'] after:text-xs after:inset-0 after:flex after:items-center after:justify-center after:font-remix after:leading-none checked:after:text-red-500 dark:checked:after:text-red-500 checked:after:content-['\eb7b']" checked>
							<label for="redIconSwitch" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-full cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-red-500 peer-checked/published:border-red-500"></label>    
						</div>
						<label for="redIconSwitch" class="inline-block text-base font-medium cursor-pointer">Red Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="purpleIconSwitch" id="purpleIconSwitch" class="absolute block size-5 transition duration-300 ease-linear border-2 border-slate-200 dark:border-zink-500 rounded-full appearance-none cursor-pointer bg-white/80 dark:bg-zink-600 peer/published checked:bg-white dark:checked:bg-white ltr:checked:right-0 rtl:checked:left-0 checked:bg-none checked:border-purple-500 dark:checked:border-purple-500 arrow-none after:absolute after:text-slate-500 dark:after:text-zink-200 after:content-['\eb99'] after:text-xs after:inset-0 after:flex after:items-center after:justify-center after:font-remix after:leading-none checked:after:text-purple-500 dark:checked:after:text-purple-500 checked:after:content-['\eb7b']" checked>
							<label for="purpleIconSwitch" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-full cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-purple-500 peer-checked/published:border-purple-500"></label>
						</div>
						<label for="purpleIconSwitch" class="inline-block text-base font-medium cursor-pointer">Purple Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="lightIconSwitch" id="lightIconSwitch" class="absolute block size-5 transition duration-300 ease-linear border-2 border-slate-200 dark:border-zink-500 rounded-full appearance-none cursor-pointer bg-white/80 dark:bg-zink-600 peer/published checked:bg-white dark:checked:bg-white ltr:checked:right-0 rtl:checked:left-0 checked:bg-none checked:border-slate-400 dark:checked:border-slate-400 arrow-none after:absolute after:text-slate-500 dark:after:text-zink-200 after:content-['\eb99'] after:text-xs after:inset-0 after:flex after:items-center after:justify-center after:font-remix after:leading-none checked:after:text-slate-400 dark:checked:after:text-slate-400 checked:after:content-['\eb7b']" checked>
							<label for="lightIconSwitch" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-full cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-slate-400 peer-checked/published:border-slate-400"></label>
						</div>
						<label for="lightIconSwitch" class="inline-block text-base font-medium cursor-pointer">Light Switch</label>
					</div>
					<div class="flex items-center">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
							<input type="checkbox" name="darkIconSwitch" id="darkIconSwitch" class="absolute block size-5 transition duration-300 ease-linear border-2 border-slate-200 dark:border-zink-500 rounded-full appearance-none cursor-pointer bg-white/80 dark:bg-zink-600 peer/published checked:bg-white dark:checked:bg-white ltr:checked:right-0 rtl:checked:left-0 checked:bg-none checked:border-slate-700 dark:checked:border-slate-700 arrow-none after:absolute after:text-slate-500 dark:after:text-zink-200 after:content-['\eb99'] after:text-xs after:inset-0 after:flex after:items-center after:justify-center after:font-remix after:leading-none checked:after:text-slate-700 dark:checked:after:text-slate-700 checked:after:content-['\eb7b']" checked>
							<label for="darkIconSwitch" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-full cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-slate-700 peer-checked/published:border-slate-700"></label>
						</div>
						<label for="darkIconSwitch" class="inline-block text-base font-medium cursor-pointer">Dark Switch</label>
					</div>
				</div>
			</div>
		</div><!--end card-->
		<div>
			<div class="card">
				<div class="card-body">
					<h6 class="mb-4 text-15">Sizes Switch</h6>
					<div class="flex flex-col gap-3">
						<div class="flex items-center">
							<div class="relative inline-block w-8 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
								<input type="checkbox" name="sizeSwitch" id="sizeSwitch" class="absolute block size-4 transition duration-300 ease-linear border-2 rounded-full appearance-none cursor-pointer border-slate-200 dark:border-zink-500 bg-white/80 dark:bg-zink-400 peer/published checked:bg-white dark:checked:bg-white ltr:checked:right-0 rtl:checked:left-0 checked:bg-none checked:border-custom-500 dark:checked:border-custom-500 arrow-none" checked>
								<label for="sizeSwitch" class="block h-4 overflow-hidden transition duration-300 ease-linear border rounded-full cursor-pointer border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-custom-500 peer-checked/published:border-custom-500"></label>
							</div>
							<label for="sizeSwitch" class="inline-block text-sm font-medium">Custom Switch</label>
						</div>
						<div class="flex items-center">
							<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
								<input type="checkbox" name="sizeSwitch2" id="sizeSwitch2" class="absolute block size-5 transition duration-300 ease-linear border-2 rounded-full appearance-none cursor-pointer border-slate-200 dark:border-zink-500 bg-white/80 dark:bg-zink-400 peer/published checked:bg-white dark:checked:bg-white ltr:checked:right-0 rtl:checked:left-0 checked:bg-none checked:border-green-500 dark:checked:border-green-500 arrow-none" checked>
								<label for="sizeSwitch2" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-full cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-green-500 peer-checked/published:border-green-500"></label>
							</div>
							<label for="sizeSwitch2" class="inline-block text-base font-medium cursor-pointer">Green Switch</label>
						</div>
						<div class="flex items-center">
							<div class="relative inline-block w-12 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
								<input type="checkbox" name="sizeSwitch3" id="sizeSwitch3" class="absolute block size-6 transition duration-300 ease-linear border-2 rounded-full appearance-none cursor-pointer border-slate-200 dark:border-zink-500 bg-white/80 dark:bg-zink-400 peer/published checked:bg-white dark:checked:bg-white ltr:checked:right-0 rtl:checked:left-0 checked:bg-none checked:border-orange-500 dark:checked:border-orange-500 arrow-none" checked>
								<label for="sizeSwitch3" class="block h-6 overflow-hidden transition duration-300 ease-linear border rounded-full cursor-pointer border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-orange-500 peer-checked/published:border-orange-500"></label>
							</div>
							<label for="sizeSwitch3" class="inline-block font-medium text-15">Orange Switch</label>
						</div>
					</div>
				</div>
			</div><!--end card-->
			<div class="mb-0 card">
				<div class="card-body">
					<h6 class="mb-4 text-15">Disabled Switch</h6>
					<div class="flex flex-col gap-3">
						<div class="flex items-center">
							<div class="relative inline-block w-8 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
								<input type="checkbox" name="disabledSwitch" id="disabledSwitch" class="absolute block size-4 transition duration-300 ease-linear border-2 rounded-full appearance-none cursor-auto border-slate-200 dark:border-zink-500 bg-white/80 dark:bg-zink-400 peer/published checked:bg-white dark:checked:bg-white ltr:checked:right-0 rtl:checked:left-0 checked:bg-none checked:border-custom-500 dark:checked:border-custom-500 arrow-none" disabled>
								<label for="disabledSwitch" class="block h-4 overflow-hidden transition duration-300 ease-linear border rounded-full cursor-auto border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-custom-500 peer-checked/published:border-custom-500"></label>
							</div>
							<label for="disabledSwitch" class="inline-block text-sm font-medium">Custom Switch</label>
						</div>
						<div class="flex items-center">
							<div class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
								<input type="checkbox" name="disabledSwitch2" id="disabledSwitch2" class="absolute block size-5 transition duration-300 ease-linear border-2 rounded-full appearance-none cursor-auto border-slate-200 dark:border-zink-500 bg-white/80 dark:bg-zink-400 peer/published checked:bg-white dark:checked:bg-white ltr:checked:right-0 rtl:checked:left-0 checked:bg-none checked:border-green-500 dark:checked:border-green-500 arrow-none" checked disabled>
								<label for="disabledSwitch2" class="block h-5 overflow-hidden duration-300 ease-linear border rounded-full cursor-auto cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-green-400 peer-checked/published:border-green-400"></label>
							</div>
							<label for="disabledSwitch2" class="inline-block text-base font-medium">Green Switch</label>
						</div>
						<div class="flex items-center">
							<div class="relative inline-block w-12 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2">
								<input type="checkbox" name="disabledSwitch3" id="disabledSwitch3" class="absolute block size-6 transition duration-300 ease-linear border-2 rounded-full appearance-none cursor-auto border-slate-200 dark:border-zink-500 bg-white/80 dark:bg-zink-400 peer/published checked:bg-white dark:checked:bg-white ltr:checked:right-0 rtl:checked:left-0 checked:bg-none checked:border-orange-500 dark:checked:border-orange-500 arrow-none" checked disabled>
								<label for="disabledSwitch3" class="block h-6 overflow-hidden transition duration-300 ease-linear border rounded-full cursor-auto border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-orange-400 peer-checked/published:border-orange-400"></label>
							</div>
							<label for="disabledSwitch3" class="inline-block font-medium text-15">Orange Switch</label>
						</div>
					</div>
				</div>
			</div><!--end card-->
		</div>
	</div>
</div>