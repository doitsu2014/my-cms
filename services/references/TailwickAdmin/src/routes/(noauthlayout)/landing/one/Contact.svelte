<script>
	import LucideIcon from '../../../../common/components/LucideIcon.svelte';
</script>

<section class="relative py-20 bg-custom-600 dark:bg-custom-800" id="contact">
	<div
		class="absolute rotate-45 border border-dashed size-[500px] border-t-custom-500 border-l-custom-500 border-r-slate-700 border-b-slate-700 -bottom-[250px] rounded-full ltr:right-40 rtl:left-40 z-10 hidden lg:block"
	></div>
	<div class="container 2xl:max-w-[87.5rem] px-4 mx-auto">
		<div class="grid items-center grid-cols-1 gap-5 lg:grid-cols-12">
			<div class="lg:col-span-9">
				<h1 class="mb-4 leading-normal capitalize text-custom-50">
					Ready to get started with Tailwick
				</h1>
				<p class="text-lg text-custom-200">
					Tell us which describes you, and we'll get in touch with next steps.
				</p>
			</div>
			<div class="ltr:lg:text-right rtl:lg:text-left lg:col-span-3">
				<button
					type="button"
					class="py-2.5 px-6 text-base transition-all duration-200 ease-linear bg-white text-custom-500 btn border-white hover:text-custom-600 hover:bg-white hover:border-white z-10 relative"
				>
					<LucideIcon
						name="ShoppingCart"
						class="inline-block size-4 align-middle ltr:mr-1 rtl:ml-1"
					/>
					<span class="align-middle">Purchase Now</span>
				</button>
			</div>
		</div>
	</div>
	<!--end container-->
</section>
