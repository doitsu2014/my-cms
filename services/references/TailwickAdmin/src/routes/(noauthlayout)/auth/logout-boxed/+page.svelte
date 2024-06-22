<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import LucideIcon from '../../../../common/components/LucideIcon.svelte';

	import { classList } from 'svelte-body';
	import { browser } from '$app/environment';
	import Dropdown from '../../../../common/components/Dropdown.svelte';
	import DropdownToggle from '../../../../common/components/DropdownToggle.svelte';

	if (browser) {
		document.body.setAttribute('class', '');
	}
</script>

<svelte:body
	use:classList={'flex items-center justify-center min-h-screen px-4 py-16 bg-cover bg-auth-pattern dark:bg-auth-pattern-dark dark:text-zink-100 font-public'}
/>

<HeadTitle title="Log Out" />

<div class="mb-0 border-none shadow-none xl:w-2/3 card bg-white/70 dark:bg-zink-500/70">
	<div class="grid grid-cols-1 gap-0 lg:grid-cols-12">
		<div class="lg:col-span-5">
			<div class="!px-12 !py-12 card-body h-full flex flex-col">
				<div class="my-auto">
					<div class="mt-8 text-center">
						<div class="mb-4 text-center">
							<LucideIcon name="LogOut" class="size-6 mx-auto text-purple-500 fill-purple-100" />
						</div>
						<h4 class="mb-2 text-custom-500 dark:text-custom-500">You are Logged Out</h4>
						<p class="mb-8 text-slate-500 dark:text-zink-200">
							Thank you for using tailwick admin template
						</p>
					</div>

					<a
						href="/auth/signin-boxed"
						class="w-full text-white transition-all duration-200 ease-linear btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
						>Sign In</a
					>
				</div>
			</div>
		</div>
		<div
			class="mx-2 mt-2 mb-2 border-none shadow-none lg:col-span-7 card bg-white/60 dark:bg-zink-500/60"
		>
			<div class="!px-10 !pt-10 h-full !pb-0 card-body flex flex-col">
				<div class="flex items-center justify-between gap-3">
					<div class="grow">
						<a href="/">
							<img src="/assets/images/logo-light.png" alt="" class="hidden h-6 dark:block" />
							<img src="/assets/images/logo-dark.png" alt="" class="block h-6 dark:hidden" />
						</a>
					</div>
					<div class="shrink-0">
						<!-- <div class="relative dropdown text-end">
                            <button type="button" class="inline-flex items-center gap-3 transition-all duration-200 ease-linear dropdown-toggle btn border-slate-200 dark:border-zink-400/60 group/items focus:border-custom-500 dark:focus:border-custom-500" id="dropdownMenuButton" data-bs-toggle="dropdown">
                                <img src="/assets/images/flags/us.svg" alt="" class="object-cover h-5 rounded-full">
                                <h6 class="text-base font-medium transition-all duration-200 ease-linear text-slate-600 group-hover/items:text-custom-500 dark:text-zink-200 dark:group-hover/items:text-custom-500">English</h6>
                            </button>
        
                            <div class="absolute z-50 hidden p-3 mt-1 text-left list-none bg-white rounded-md shadow-md dropdown-menu min-w-[9rem] flex flex-col gap-3 dark:bg-zink-600" aria-labelledby="dropdownMenuButton">
                                <a href="#!" class="flex items-center gap-3 group/items">
                                    <img src="/assets/images/flags/us.svg" alt="" class="object-cover h-4 rounded-full">
                                    <h6 class="text-sm font-medium transition-all duration-200 ease-linear text-slate-600 group-hover/items:text-custom-500 dark:text-zink-200 dark:group-hover/items:text-custom-500">English</h6>
                                </a>
                                <a href="#!" class="flex items-center gap-3 group/items">
                                    <img src="/assets/images/flags/es.svg" alt="" class="object-cover h-4 rounded-full">
                                    <h6 class="text-sm font-medium transition-all duration-200 ease-linear text-slate-600 group-hover/items:text-custom-500 dark:text-zink-200 dark:group-hover/items:text-custom-500">Spanish</h6>
                                </a>
                                <a href="#!" class="flex items-center gap-3 group/items">
                                    <img src="/assets/images/flags/de.svg" alt="" class="object-cover h-4 rounded-full">
                                    <h6 class="text-sm font-medium transition-all duration-200 ease-linear text-slate-600 group-hover/items:text-custom-500 dark:text-zink-200 dark:group-hover/items:text-custom-500">German</h6>
                                </a>
                                <a href="#!" class="flex items-center gap-3 group/items">
                                    <img src="/assets/images/flags/fr.svg" alt="" class="object-cover h-4 rounded-full">
                                    <h6 class="text-sm font-medium transition-all duration-200 ease-linear text-slate-600 group-hover/items:text-custom-500 dark:text-zink-200 dark:group-hover/items:text-custom-500">French</h6>
                                </a>
                                <a href="#!" class="flex items-center gap-3 group/items">
                                    <img src="/assets/images/flags/jp.svg" alt="" class="object-cover h-4 rounded-full">
                                    <h6 class="text-sm font-medium transition-all duration-200 ease-linear text-slate-600 group-hover/items:text-custom-500 dark:text-zink-200 dark:group-hover/items:text-custom-500">Japanese</h6>
                                </a>
                                <a href="#!" class="flex items-center gap-3 group/items">
                                    <img src="/assets/images/flags/it.svg" alt="" class="object-cover h-4 rounded-full">
                                    <h6 class="text-sm font-medium transition-all duration-200 ease-linear text-slate-600 group-hover/items:text-custom-500 dark:text-zink-200 dark:group-hover/items:text-custom-500">Italian</h6>
                                </a>
                                <a href="#!" class="flex items-center gap-3 group/items">
                                    <img src="/assets/images/flags/ru.svg" alt="" class="object-cover h-4 rounded-full">
                                    <h6 class="text-sm font-medium transition-all duration-200 ease-linear text-slate-600 group-hover/items:text-custom-500 dark:text-zink-200 dark:group-hover/items:text-custom-500">Russian</h6>
                                </a>
                                <a href="#!" class="flex items-center gap-3 group/items">
                                    <img src="/assets/images/flags/ae.svg" alt="" class="object-cover h-4 rounded-full">
                                    <h6 class="text-sm font-medium transition-all duration-200 ease-linear text-slate-600 group-hover/items:text-custom-500 dark:text-zink-200 dark:group-hover/items:text-custom-500">Arabic</h6>
                                </a>
                            </div>
                        </div> -->
						<Dropdown className="relative text-end">
							<DropdownToggle
								className="inline-flex items-center gap-3 transition-all duration-200 ease-linear dropdown-toggle btn border-slate-200 dark:border-zink-400/60 group/items focus:border-custom-500 dark:focus:border-custom-500"
							>
								<img
									src="/assets/images/flags/us.svg"
									alt=""
									class="object-cover h-5 rounded-full"
								/>
								<h6
									class="text-base font-medium transition-all duration-200 ease-linear text-slate-600 group-hover/items:text-custom-500 dark:text-zink-200 dark:group-hover/items:text-custom-500"
								>
									English
								</h6>
							</DropdownToggle>
						</Dropdown>
					</div>
				</div>
				<div class="mt-auto">
					<img src="/assets/images/auth/img-01.png" alt="" class="md:max-w-[32rem] mx-auto" />
				</div>
			</div>
		</div>
	</div>
</div>
