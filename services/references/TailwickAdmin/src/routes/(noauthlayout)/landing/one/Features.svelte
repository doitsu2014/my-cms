<script>
	import LucideIcon from '../../../../common/components/LucideIcon.svelte';
</script>

<section class="relative py-32 bg-slate-900 dark:bg-zink-700" id="features">
	<div class="container 2xl:max-w-[87.5rem] px-4 mx-auto">
		<div class="grid grid-cols-1 gap-5 xl:grid-cols-12">
			<div class="order-2 xl:col-span-5 xl:order-1">
				<div class="relative z-10 grid grid-cols-3 gap-5 xl:grid-cols-12">
					<div class="xl:col-span-12">
						<img
							src="/assets/images/landing/widgets.jpg"
							alt=""
							class="shadow-xl rtl:mr-auto ltr:ml-auto rounded-xl"
						/>
					</div>
					<div class="xl:col-span-6">
						<img
							src="/assets/images/landing/widgets-3.jpg"
							alt=""
							class="shadow-xl xl:mt-5 rtl:mr-auto ltr:ml-auto rounded-xl"
						/>
					</div>
					<div class="xl:col-span-6">
						<img
							src="/assets/images/landing/widgets-2.jpg"
							alt=""
							class="shadow-xl xl:mt-5 rtl:mr-auto ltr:ml-auto rounded-xl"
						/>
					</div>
				</div>
			</div>
			<div class="relative z-20 order-1 xl:-mt-72 xl:col-span-7 xl:order-last">
				<img src="/assets/images/landing/home.jpg" alt="" class="shadow-2xl rounded-xl" />
			</div>
		</div>
		<div class="mt-24">
			<h1 class="mb-16 leading-normal capitalize text-slate-100 dark:text-zink-50">
				Build for anyone who works with Tailwick
			</h1>
		</div>
	</div>
	<div
		class="absolute rotate-45 border border-dashed size-[500px] border-t-slate-700 border-l-slate-700 border-r-slate-700 border-b-slate-700 bottom-48 rounded-full ltr:-left-80 rtl:-right-80 hidden md:block"
	></div>
	<div
		class="absolute rotate-45 border border-dashed size-[700px] border-t-slate-700 border-l-slate-700 border-r-slate-700 border-b-slate-700 bottom-24 rounded-full ltr:-left-96 rtl:-right-96 hidden md:block"
	></div>
</section>

<section class="relative pb-32 bg-slate-50 dark:bg-zink-700/40">
	<div class="container 2xl:max-w-[87.5rem] px-4 mx-auto">
		<div class="grid grid-cols-1 gap-6 lg:grid-cols-2 xl:grid-cols-12 gap-x-5">
			<div class="xl:col-span-4">
				<div
					class="transition-all duration-300 ease-linear -mt-36 card hover:-translate-y-2 dark:bg-zink-600"
				>
					<div class="p-6">
						<img src="/assets/images/landing/chat.jpg" alt="" class="rounded-md shadow" />
						<div class="mt-6">
							<span
								class="px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-sky-100 border-sky-200 text-sky-500 dark:bg-sky-500/20 dark:border-sky-500/20"
								>Popular Apps</span
							>
							<h6 class="mt-3 mb-2 text-lg truncate">
								<a href="#!">Chat with Client & Employee</a>
							</h6>
							<p class="mb-3 text-slate-500 dark:text-zink-200 text-16">
								A chat UI, or user interface, is one of the first things users see when using a web
								or mobile application to communicate.
							</p>
							<a href="#!" class="text-custom-500 text-16"
								>Read More <LucideIcon
									name="MoveRight"
									class="inline-block size-4 align-middle ltr:ml-1 rtl:mr-1 rtl:rotate-180"
								/></a
							>
						</div>
					</div>
				</div>
			</div>
			<!--end col-->
			<div class="xl:col-span-4">
				<div
					class="transition-all duration-300 ease-linear md:-mt-36 card hover:-translate-y-2 dark:bg-zink-600"
				>
					<div class="p-6">
						<img src="/assets/images/landing/email.jpg" alt="" class="rounded-md shadow" />
						<div class="mt-6">
							<span
								class="px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-purple-100 border-purple-200 text-purple-500 dark:bg-purple-500/20 dark:border-purple-500/20"
								>Email Apps</span
							>
							<h6 class="mt-3 mb-2 text-lg truncate">
								<a href="#!">Exciting New Features Await You</a>
							</h6>
							<p class="mb-3 text-slate-500 dark:text-zink-200 text-16">
								A chat UI, or user interface, is one of the first things users see when using a web
								or mobile application to communicate.
							</p>
							<a href="#!" class="text-custom-500 text-16"
								>Read More <LucideIcon
									name="MoveRight"
									class="inline-block size-4 align-middle ltr:ml-1 rtl:mr-1 rtl:rotate-180"
								/></a
							>
						</div>
					</div>
				</div>
			</div>
			<!--end col-->
			<div class="xl:col-span-4">
				<div
					class="transition-all duration-300 ease-linear xl:-mt-36 card hover:-translate-y-2 dark:bg-zink-600"
				>
					<div class="p-6">
						<img src="/assets/images/landing/order-overview.jpg" alt="" class="rounded-md shadow" />
						<div class="mt-6">
							<span
								class="px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-red-100 border-red-200 text-red-500 dark:bg-red-500/20 dark:border-red-500/20"
								>HR Management</span
							>
							<h6 class="mt-3 mb-2 text-lg truncate">
								<a href="#!">Unleash Your Creativity with Our Latest Tools</a>
							</h6>
							<p class="mb-3 text-slate-500 dark:text-zink-200 text-16">
								A chat UI, or user interface, is one of the first things users see when using a web
								or mobile application to communicate.
							</p>
							<a href="#!" class="text-custom-500 text-16"
								>Read More <LucideIcon
									name="MoveRight"
									class="inline-block size-4 align-middle ltr:ml-1 rtl:mr-1 rtl:rotate-180"
								/></a
							>
						</div>
					</div>
				</div>
			</div>
			<!--end col-->
		</div>
		<!--end grid-->

		<div class="p-10 mt-20 rounded-md bg-slate-900 dark:bg-zink-700">
			<div class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-5">
				<div class="text-center">
					<h3 class="mb-2 text-slate-50 dark:text-zink-50">
						<span class="counter-value" data-target="4">0</span> +
					</h3>
					<p class="text-16 text-slate-400 dark:text-zink-200">Multiple Layouts</p>
				</div>
				<!--end col-->
				<div class="text-center">
					<h3 class="mb-2 text-slate-50 dark:text-zink-50">
						<span class="counter-value" data-target="12">0</span> +
					</h3>
					<p class="text-16 text-slate-400 dark:text-zink-200">Multi Languages</p>
				</div>
				<!--end col-->
				<div class="text-center">
					<h3 class="mb-2 text-slate-50 dark:text-zink-50">
						<span class="counter-value" data-target="7">0</span> +
					</h3>
					<p class="text-16 text-slate-400 dark:text-zink-200">Apps & Dashboard</p>
				</div>
				<!--end col-->
				<div class="text-center">
					<h3 class="mb-2 text-slate-50 dark:text-zink-50">
						<span class="counter-value" data-target="4">0</span> +
					</h3>
					<p class="text-16 text-slate-400 dark:text-zink-200">Multiple Modes</p>
				</div>
				<!--end col-->
				<div class="text-center">
					<h3 class="mb-2 text-slate-50 dark:text-zink-50">
						<span class="counter-value" data-target="36500">0</span> +
					</h3>
					<p class="text-16 text-slate-400 dark:text-zink-200">Happy Customers</p>
				</div>
				<!--end col-->
			</div>
			<!--end grid-->
		</div>
	</div>
</section>
