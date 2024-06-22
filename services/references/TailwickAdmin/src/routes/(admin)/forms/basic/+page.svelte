<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import LucideIcon from '../../../../common/components/LucideIcon.svelte';
</script>

<HeadTitle title="Basic Elements" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Basic Elements" pagetitle="Forms" />

	<div class="grid grid-cols-1 gap-x-5 xl:grid-cols-3">
		<div class="card">
			<div class="card-body">
				<form action="#">
					<div class="mb-3">
						<label for="inputText" class="inline-block mb-2 text-base font-medium">Text <span class="text-red-500">*</span></label>
						<input type="text" id="inputText" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" required>
					</div>
					<button type="submit" class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20">Submit</button>
				</form>
			</div>
		</div><!--end card-->
		<div class="card">
			<div class="card-body">
				<form action="#">
					<div class="mb-3">
						<label for="inputPassword" class="inline-block mb-2 text-base font-medium">Password <span class="text-red-500">*</span></label>
						<input type="password" id="inputPassword" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" required>
					</div>
					<button type="submit" class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20">Submit</button>
				</form>
			</div>
		</div><!--end card-->
		<div class="card">
			<div class="card-body">
				<form action="#">
					<div class="mb-3">
						<label for="inputEmail" class="inline-block mb-2 text-base font-medium">Email Address <span class="text-red-500">*</span></label>
						<input type="email" id="inputEmail" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" required>
					</div>
					<button type="submit" class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20">Submit</button>
				</form>
			</div>
		</div><!--end card-->
		<div class="card">
			<div class="card-body">
				<div>
					<label for="inputPlaceholder" class="inline-block mb-2 text-base font-medium">Input with Placeholder</label>
					<input type="text" id="inputPlaceholder" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Placeholder">
				</div>
			</div>
		</div><!--end card-->
		<div class="card">
			<div class="card-body">
				<div>
					<label for="inputValue" class="inline-block mb-2 text-base font-medium">Input with Value</label>
					<input type="text" id="inputValue" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Enter your value" value="Input Value">
				</div>
			</div>
		</div><!--end card-->
		<div class="card">
			<div class="card-body">
				<div>
					<label for="inputPlainReadOnly" class="inline-block mb-2 text-base font-medium">Readonly Plain Text Input</label>
					<input type="text" id="inputPlainReadOnly" class="border-transparent form-input dark:border-transparent focus:outline-none focus:border-transparent disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-transparent dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-transparent placeholder:text-slate-400 dark:placeholder:text-zink-200" value="Readonly input" readonly>
				</div>
			</div>
		</div><!--end card-->
		<div class="card">
			<div class="card-body">
				<div>
					<label for="inputPlainReadOnly" class="inline-block mb-2 text-base font-medium">Readonly Input</label>
					<input type="text" id="inputPlainReadOnly" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" value="Readonly input" readonly>
				</div>
			</div>
		</div><!--end card-->
		<div class="card">
			<div class="card-body">
				<div>
					<label for="inputPlainReadOnly" class="inline-block mb-2 text-base font-medium">Disabled Input</label>
					<input type="text" id="inputPlainReadOnly" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" value="Disabled input" disabled>
				</div>
			</div>
		</div><!--end card-->
		<div class="card">
			<div class="card-body">
				<div>
					<label for="inputWithIcon" class="inline-block mb-2 text-base font-medium">Input with Icon</label>
					<div class="relative">
						<LucideIcon name="Mail" class="absolute size-4 ltr:left-3 rtl:right-3 top-3 text-slate-500 dark:text-zink-200"/>
						<input type="email" id="inputWithIcon" class="ltr:pl-10 rtl:pr-10 form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Enter your email">
					</div>
				</div>
			</div>
		</div><!--end card-->

		<div class="card">
			<div class="card-body">
				<div>
					<label for="inputIconRight" class="inline-block mb-2 text-base font-medium">Input with Icon Right</label>
					<div class="relative">
						<LucideIcon name="Mail" class="absolute size-4 ltr:right-3 rtl:left-3 top-3 text-slate-500 dark:text-zink-200"/>
						<input type="email" id="inputIconRight" class="ltr:pr-10 rtl:pl-10 form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Enter your email">
					</div>
				</div>
			</div>
		</div><!--end card-->
		<div class="card">
			<div class="card-body">
				<div>
					<label for="inputBorderStyle" class="inline-block mb-2 text-base font-medium">Input Border Style</label>
					<input type="text" id="inputBorderStyle" class="border-dashed form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Enter your text">
				</div>
			</div>
		</div><!--end card-->
		<div class="card">
			<div class="card-body">
				<div>
					<label for="inputRoundedInput" class="inline-block mb-2 text-base font-medium">Rounded Input</label>
					<input type="text" id="inputRoundedInput" class="rounded-full form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Enter your name">
				</div>
			</div>
		</div><!--end card-->
		<div class="card">
			<div class="card-body">
				<div>
					<label for="textArea" class="inline-block mb-2 text-base font-medium">Example Textarea</label>
					<textarea class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" id="textArea" rows="3"></textarea>
				</div>
			</div>
		</div><!--end card-->
		<div class="card">
			<div class="card-body">
				<div>
					<label for="inputFormText" class="inline-block mb-2 text-base font-medium">Form Text</label>
					<input type="password" id="inputFormText" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Enter password">
					<div class="mt-1 text-xs text-slate-500 dark:text-zink-200">
						Must be 8-20 characters long.
					</div>
				</div>
			</div>
		</div><!--end card-->
		<div class="card">
			<div class="card-body">
				<div>
					<label for="inputValid" class="inline-block mb-2 text-base font-medium">Valid Text</label>
					<input type="email" id="inputValid" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200 valid:border-green-500 invalid:border-red-500 dark:valid:border-green-800 dark:invalid:border-red-800" placeholder="Enter email" value="example@gmail.com">
				</div>
			</div>
		</div><!--end card-->
		<div class="card">
			<div class="card-body">
				<div>
					<label for="inputInvalid" class="inline-block mb-2 text-base font-medium">Invalid Text</label>
					<input type="email" id="inputInvalid" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200 valid:border-green-500 invalid:border-red-500 dark:valid:border-green-800 dark:invalid:border-red-800" placeholder="Enter email" value="examplegmail.com">
				</div>
			</div>
		</div><!--end card-->
	</div><!--end grid-->

	<div class="card">
		<div class="card-body">
			<h6 class="mb-4 text-15">Input Sizing</h6>

			<div class="grid items-center grid-cols-1 gap-5 xl:grid-cols-3">
				<div>
					<input type="text" class="px-3 py-1 text-xs form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Enter your name">
				</div>
				<div>
					<input type="text" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Enter your name">
				</div>
				<div>
					<input type="text" class="px-5 py-3 text-15 form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Enter your name">
				</div>
			</div>
		</div>
	</div><!--end card-->

	<div class="card">
		<div class="card-body">
			<h6 class="mb-4 text-15">Range Color Variant</h6>

			<div class="grid items-center grid-cols-1 gap-5 xl:grid-cols-3">
				<div>
					<label for="DefaultRange" class="inline-block mb-2 text-base font-medium">Default Range</label>
					<input type="range" min="0" max="5" step="1" class="w-full h-2 rounded-md appearance-none bg-slate-200 dark:bg-zink-600" id="DefaultRange">
				</div>
				<div>
					<label for="GreenRange" class="inline-block mb-2 text-base font-medium">Green Range</label>
					<input type="range" min="0" max="5" step="1" class="w-full h-2 rounded-md appearance-none range-green bg-slate-200 dark:bg-zink-600" id="GreenRange">
				</div>
				<div>
					<label for="OrangeRange" class="inline-block mb-2 text-base font-medium">Orange Range</label>
					<input type="range" min="0" max="5" step="1" class="w-full h-2 rounded-md appearance-none range-orange bg-slate-200 dark:bg-zink-600" id="OrangeRange">
				</div>
				<div>
					<label for="SkyRange" class="inline-block mb-2 text-base font-medium">Sky Range</label>
					<input type="range" min="0" max="5" step="1" class="w-full h-2 rounded-md appearance-none range-sky bg-slate-200 dark:bg-zink-600" id="SkyRange">
				</div>
				<div>
					<label for="YellowRange" class="inline-block mb-2 text-base font-medium">Yellow Range</label>
					<input type="range" min="0" max="5" step="1" class="w-full h-2 rounded-md appearance-none range-yellow bg-slate-200 dark:bg-zink-600" id="YellowRange">
				</div>
				<div>
					<label for="RedRange" class="inline-block mb-2 text-base font-medium">Red Range</label>
					<input type="range" min="0" max="5" step="1" class="w-full h-2 rounded-md appearance-none range-red bg-slate-200 dark:bg-zink-600" id="RedRange">
				</div>
				<div>
					<label for="PurpleRange" class="inline-block mb-2 text-base font-medium">Purple Range</label>
					<input type="range" min="0" max="5" step="1" class="w-full h-2 rounded-md appearance-none range-purple bg-slate-200 dark:bg-zink-600" id="PurpleRange">
				</div>
				<div>
					<label for="darkRange" class="inline-block mb-2 text-base font-medium">Dark Range</label>
					<input type="range" min="0" max="5" step="1" class="w-full h-2 rounded-md appearance-none range-dark bg-slate-200 dark:bg-zink-600" id="darkRange">
				</div>
			</div>
		</div>
	</div><!--end card-->

	<div class="card">
		<div class="card-body">
			<h6 class="mb-4 text-15">File Sizing</h6>

			<div class="grid items-center grid-cols-1 gap-5 xl:grid-cols-3">
				<div>
					<input type="file" class="cursor-pointer form-file form-file-sm border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500" placeholder="Enter your name">
				</div>
				<div>
					<input type="file" class="cursor-pointer form-file border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500" placeholder="Enter your name">
				</div>
				<div>
					<input type="file" class="cursor-pointer form-file form-file-lg border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500" placeholder="Enter your name">
				</div>
			</div>
		</div>
	</div><!--end card-->

	<div class="card">
		<div class="card-body">
			<h6 class="mb-4 text-15">Input Group</h6>

			<div class="grid items-center grid-cols-1 gap-5 xl:grid-cols-3">
				<div class="flex items-center">
					<span class="inline-block px-3 py-2 border ltr:border-r-0 rtl:border-l-0 border-slate-200 bg-slate-100 dark:border-zink-500 dark:bg-zink-600 ltr:rounded-l-md rtl:rounded-r-md">@</span>
					<input type="text" id="inputText" class="ltr:rounded-l-none rtl:rounded-r-none form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="@name">
				</div>
				<div class="flex items-center">
					<span class="inline-block px-3 py-2 border ltr:border-r-0 rtl:border-l-0 border-slate-200 dark:border-zink-500 ltr:rounded-l-md rtl:rounded-r-md">$</span>
					<input type="number" id="inputText" class="ltr:rounded-l-none rtl:rounded-r-none form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="00.00">
				</div>
				<div class="flex items-center">
					<input type="text" id="inputText" class="ltr:rounded-r-none rtl:rounded-l-none form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="@name">
					<span class="inline-block px-3 py-2 border border-l-0 border-slate-200 bg-slate-100 dark:border-zink-500 dark:bg-zink-600 ltr:rounded-r-md rtl:rounded-l-md">@</span>
				</div>
			</div>
		</div>
	</div>
</div>
