<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import LucideIcon from '../../../../common/components/LucideIcon.svelte';
	import { cleave } from 'svelte-cleavejs';
</script>

<HeadTitle title="Input Mask" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Input Mask" pagetitle="Forms" />

	<div class="card">
		<div class="card-body">
			<h6 class="mb-4 text-15">Date Formatting</h6>
			<div class="grid grid-cols-1 gap-5 md:grid-cols-2 xl:grid-cols-2">
				<div>
					<label for="cleave-date" class="inline-block mb-2 text-base font-medium">Date</label>
					<input type="text" id="cleave-date" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="DD-MM-YYYY" use:cleave={{
						date: true,
						delimiter: '-',
						datePattern: ['d', 'm', 'Y']
					}}>
				</div>
				<div>
					<label for="cleaveDateFormat" class="inline-block mb-2 text-base font-medium">Date Formatting</label>
					<input type="text" id="cleaveDateFormat" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="DD/MM" use:cleave={{
						date: true,
						datePattern: ['m', 'y']
					}}>
				</div>
			</div><!--END GRID-->
		</div>
	</div><!--end card-->

	<div class="card">
		<div class="card-body">
			<h6 class="mb-4 text-15">Time Formatting</h6>
			<div class="grid grid-cols-1 gap-5 md:grid-cols-2 xl:grid-cols-2">
				<div>
					<label for="cleaveTime" class="inline-block mb-2 text-base font-medium">Time</label>
					<input type="text" id="cleaveTime" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="hh:mm:ss"use:cleave={{
						time: true,
						timePattern: ['h', 'm', 's']
					}}>
				</div>
				<div>
					<label for="cleaveTimeFormat" class="inline-block mb-2 text-base font-medium">Time Formatting</label>
					<input type="text" id="cleaveTimeFormat" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="hh:mm"use:cleave={{
						time: true,
						timePattern: ['h', 'm']
					}}>
				</div>
			</div><!--END GRID-->
		</div>
	</div><!--end card-->

	<div class="card">
		<div class="card-body">
			<h6 class="mb-4 text-15">Custom Options</h6>
			<div class="grid grid-cols-1 gap-5 md:grid-cols-2 xl:grid-cols-2">
				<div>
					<label for="cleaveCreditCard" class="inline-block mb-2 text-base font-medium">Credit Card</label>
					<input type="text" id="cleaveCreditCard" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="xxxx xxxx xxxx xxxx"use:cleave={{
						creditCard: true
					}}>
				</div>
				<div>
					<label for="cleaveDelimiter" class="inline-block mb-2 text-base font-medium">Delimiter</label>
					<input type="text" id="cleaveDelimiter" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="xxx·xxx·xxx"use:cleave={{
						delimiter: '·',
						blocks: [3, 3, 3],
						uppercase: true
					}}>
				</div>
				<div>
					<label for="cleaveDelimiters" class="inline-block mb-2 text-base font-medium">Delimiters</label>
					<input type="text" id="cleaveDelimiters" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="xxx.xxx.xxx-xx"use:cleave={{
						delimiters: ['.', '.', '-'],
						blocks: [3, 3, 3, 2],
						uppercase: true
					}}>
				</div>
				<div>
					<label for="cleavePrefix" class="inline-block mb-2 text-base font-medium">Prefix</label>
					<input type="text" id="cleavePrefix" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="xxx.xxx.xxx-xx"use:cleave={{
						prefix: 'PREFIX',
						delimiter: '-',
						blocks: [6, 4, 4, 4],
						uppercase: true
					}}>
				</div>
				<div>
					<label for="cleavePhone" class="inline-block mb-2 text-base font-medium">Phone</label>
					<input type="text" id="cleavePhone" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="(xxx)xxx-xxxx"use:cleave={{
						delimiters: ['(', ')', '-'],
						blocks: [0, 3, 3, 4],
					}}>
				</div>
				<div>
					<label for="cleaveNumeral" class="inline-block mb-2 text-base font-medium">Numeral Formatting</label>
					<input type="text" id="cleaveNumeral" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Enter numeral"use:cleave={{
						numeral: true,
						numeralThousandsGroupStyle: 'thousand'
					}}>
				</div>
			</div><!--END GRID-->
		</div>
	</div>
</div>