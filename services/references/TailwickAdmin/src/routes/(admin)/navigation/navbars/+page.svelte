<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import LucideIcon from '../../../../common/components/LucideIcon.svelte';
	let isActive = false;
</script>

<HeadTitle title="Navbar" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Navbar" pagetitle="Navigation" />

	<h6 class="mb-4 text-15">Basic Navbar</h6>

	<nav
		class="group-data-[skin=bordered]:shadow-sm group-data-[skin=bordered]:border group-data-[skin=bordered]:border-slate-200 group-data-[skin=bordered]:dark:border-zink-500 relative flex items-center h-16 px-4 py-2 bg-white rounded-md shadow-md navbar dark:bg-zink-700"
	>
		<div class="shrink-0">
			<a href="#!"
				><img src="/assets/images/logo-dark.png" alt="" class="block h-5 dark:hidden" /></a
			>
			<a href="#!"
				><img src="/assets/images/logo-light.png" alt="" class="hidden h-5 dark:block" /></a
			>
		</div>
		<ul
			id="navbar7"
			class="absolute inset-x-0 z-20 items-center hidden py-3 bg-white shadow-lg dark:bg-zink-600 dark:md:bg-transparent md:z-0 navbar-menu rounded-b-md md:shadow-none md:flex top-full ltr:ml-auto rtl:mr-auto md:relative md:bg-transparent md:rounded-none md:top-auto md:py-0"
		>
			<li>
				<a
					href="#!"
					class="block md:inline-block px-4 md:px-3 py-2.5 md:py-0.5 text-15 font-medium text-slate-800 transition-all duration-300 ease-linear hover:text-custom-500 [&.active]:text-custom-500 dark:text-zink-100 dark:hover:text-custom-500 dark:[&.active]:text-custom-500 active"
					>Home</a
				>
			</li>
			<li>
				<a
					href="#!"
					class="block md:inline-block px-4 md:px-3 py-2.5 md:py-0.5 text-15 font-medium text-slate-800 transition-all duration-300 ease-linear hover:text-custom-500 [&.active]:text-custom-500 dark:text-zink-100 dark:hover:text-custom-500 dark:[&.active]:text-custom-500"
					>About Us</a
				>
			</li>
			<li>
				<a
					href="#!"
					class="block md:inline-block px-4 md:px-3 py-2.5 md:py-0.5 text-15 font-medium text-slate-800 transition-all duration-300 ease-linear hover:text-custom-500 [&.active]:text-custom-500 dark:text-zink-100 dark:hover:text-custom-500 dark:[&.active]:text-custom-500"
					>Service</a
				>
			</li>
			<li>
				<a
					href="#!"
					class="block md:inline-block px-4 md:px-3 py-2.5 md:py-0.5 text-15 font-medium text-slate-800 transition-all duration-300 ease-linear hover:text-custom-500 [&.active]:text-custom-500 dark:text-zink-100 dark:hover:text-custom-500 dark:[&.active]:text-custom-500"
					>Blog</a
				>
			</li>
			<li>
				<a
					href="#!"
					class="block md:inline-block px-4 md:px-3 py-2.5 md:py-0.5 text-15 font-medium text-slate-800 transition-all duration-300 ease-linear hover:text-custom-500 [&.active]:text-custom-500 dark:text-zink-100 dark:hover:text-custom-500 dark:[&.active]:text-custom-500"
					>Contact</a
				>
			</li>
		</ul>
		<div class="ltr:ml-auto rtl:mr-auto md:hidden navbar-toggale-button">
			<button
				type="button"
				class="flex items-center justify-center size-[37.5px] p-0 text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
				><LucideIcon name="Menu" /></button
			>
		</div>
	</nav>

	<h6 class="mb-4 mt-7 text-16">Hamburger menu</h6>

	<nav
		class="group-data-[skin=bordered]:shadow-sm group-data-[skin=bordered]:border group-data-[skin=bordered]:border-slate-200 group-data-[skin=bordered]:dark:border-zink-500 relative flex items-center h-16 px-4 py-2 bg-white rounded-md shadow-md navbar dark:bg-zink-700"
	>
		<div class="shrink-0">
			<a href="#!"
				><img src="/assets/images/logo-dark.png" alt="" class="block h-5 dark:hidden" /></a
			>
			<a href="#!"
				><img src="/assets/images/logo-light.png" alt="" class="hidden h-5 dark:block" /></a
			>
		</div>
		<ul
			id="navbar8"
			class="absolute inset-x-0 z-20 items-center py-3 bg-white shadow-lg dark:bg-zink-600 navbar-menu rounded-b-md top-full ltr:ml-auto rtl:mr-auto {!isActive
				? 'hidden'
				: ''}"
		>
			<li>
				<a
					href="#!"
					class="block px-4 py-2.5 text-15 font-medium text-slate-800 transition-all duration-300 ease-linear hover:text-custom-500 [&.active]:text-custom-500 dark:text-zink-100 dark:hover:text-custom-500 dark:[&.active]:text-custom-500 active"
					>Home</a
				>
			</li>
			<li>
				<a
					href="#!"
					class="block px-4 py-2.5 text-15 font-medium text-slate-800 transition-all duration-300 ease-linear hover:text-custom-500 [&.active]:text-custom-500 dark:text-zink-100 dark:hover:text-custom-500 dark:[&.active]:text-custom-500"
					>About Us</a
				>
			</li>
			<li>
				<a
					href="#!"
					class="block px-4 py-2.5 text-15 font-medium text-slate-800 transition-all duration-300 ease-linear hover:text-custom-500 [&.active]:text-custom-500 dark:text-zink-100 dark:hover:text-custom-500 dark:[&.active]:text-custom-500"
					>Service</a
				>
			</li>
			<li>
				<a
					href="#!"
					class="block px-4 py-2.5 text-15 font-medium text-slate-800 transition-all duration-300 ease-linear hover:text-custom-500 [&.active]:text-custom-500 dark:text-zink-100 dark:hover:text-custom-500 dark:[&.active]:text-custom-500"
					>Blog</a
				>
			</li>
			<li>
				<a
					href="#!"
					class="block px-4 py-2.5 text-15 font-medium text-slate-800 transition-all duration-300 ease-linear hover:text-custom-500 [&.active]:text-custom-500 dark:text-zink-100 dark:hover:text-custom-500 dark:[&.active]:text-custom-500"
					>Contact</a
				>
			</li>
		</ul>
		<div class="ltr:ml-auto rtl:mr-auto navbar-toggale-button">
			<button
				type="button"
				class="flex items-center justify-center size-[37.5px] p-0 text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
				on:click={() => (isActive = !isActive)}><LucideIcon name="Menu" /></button
			>
		</div>
	</nav>

	<h6 class="mb-4 mt-7 text-16">Navbar with button</h6>

	<nav
		class="group-data-[skin=bordered]:shadow-sm group-data-[skin=bordered]:border group-data-[skin=bordered]:border-slate-200 group-data-[skin=bordered]:dark:border-zink-500 flex items-center h-16 px-4 py-2 bg-white rounded-md shadow-md dark:bg-zink-700 navbar"
	>
		<div class="shrink-0">
			<a href="#!"
				><img src="/assets/images/logo-dark.png" alt="" class="block h-5 dark:hidden" /></a
			>
			<a href="#!"
				><img src="/assets/images/logo-light.png" alt="" class="hidden h-5 dark:block" /></a
			>
		</div>
		<ul
			id="navbar5"
			class="absolute inset-x-0 z-20 items-center hidden py-3 mx-auto bg-white shadow-lg dark:bg-zink-600 dark:md:bg-transparent md:z-0 navbar-menu rounded-b-md md:shadow-none md:flex top-full md:relative md:bg-transparent md:rounded-none md:top-auto md:py-0"
		>
			<li>
				<a
					href="#!"
					class="block md:inline-block px-4 md:px-3 py-2.5 md:py-0.5 text-15 font-medium text-slate-800 transition-all duration-300 ease-linear hover:text-custom-500 [&.active]:text-custom-500 dark:text-zink-100 dark:hover:text-custom-500 dark:[&.active]:text-custom-500 active"
					>Home</a
				>
			</li>
			<li>
				<a
					href="#!"
					class="block md:inline-block px-4 md:px-3 py-2.5 md:py-0.5 text-15 font-medium text-slate-800 transition-all duration-300 ease-linear hover:text-custom-500 [&.active]:text-custom-500 dark:text-zink-100 dark:hover:text-custom-500 dark:[&.active]:text-custom-500"
					>About Us</a
				>
			</li>
			<li>
				<a
					href="#!"
					class="block md:inline-block px-4 md:px-3 py-2.5 md:py-0.5 text-15 font-medium text-slate-800 transition-all duration-300 ease-linear hover:text-custom-500 [&.active]:text-custom-500 dark:text-zink-100 dark:hover:text-custom-500 dark:[&.active]:text-custom-500"
					>Service</a
				>
			</li>
			<li>
				<a
					href="#!"
					class="block md:inline-block px-4 md:px-3 py-2.5 md:py-0.5 text-15 font-medium text-slate-800 transition-all duration-300 ease-linear hover:text-custom-500 [&.active]:text-custom-500 dark:text-zink-100 dark:hover:text-custom-500 dark:[&.active]:text-custom-500"
					>Blog</a
				>
			</li>
			<li>
				<a
					href="#!"
					class="block md:inline-block px-4 md:px-3 py-2.5 md:py-0.5 text-15 font-medium text-slate-800 transition-all duration-300 ease-linear hover:text-custom-500 [&.active]:text-custom-500 dark:text-zink-100 dark:hover:text-custom-500 dark:[&.active]:text-custom-500"
					>Contact</a
				>
			</li>
		</ul>
		<ul class="flex items-center ltr:ml-auto rtl:mr-auto">
			<li>
				<a
					href="#!"
					class="inline-flex items-center justify-center size-10 transition-all duration-300 ease-linear tems-center text-slate-500 hover:text-custom-500 active:text-custom-500 dark:text-zink-200 dark:hover:text-custom-500 dark:active:text-custom-500 group/item"
					><LucideIcon
						name="ShoppingBag"
						class="size-4 fill-slate-50 dark:fill-zink-500 group-hover/item:fill-custom-50 dark:group-hover/item:fill-custom-500/20"
					/></a
				>
			</li>
			<li>
				<a
					href="#!"
					class="inline-flex items-center justify-center size-10 transition-all duration-300 ease-linear tems-center text-slate-500 hover:text-custom-500 active:text-custom-500 dark:text-zink-200 dark:hover:text-custom-500 dark:active:text-custom-500 group/item"
					><LucideIcon
						name="Bell"
						class="size-4 fill-slate-50 dark:fill-zink-500 group-hover/item:fill-custom-50 dark:group-hover/item:fill-custom-500/20"
					/></a
				>
			</li>
			<li>
				<a
					href="#!"
					class="inline-flex items-center justify-center size-10 transition-all duration-300 ease-linear tems-center text-slate-500 hover:text-custom-500 active:text-custom-500 dark:text-zink-200 dark:hover:text-custom-500 dark:active:text-custom-500 group/item"
					><LucideIcon
						name="UserCircle"
						class="size-4 fill-slate-50 dark:fill-zink-500 group-hover/item:fill-custom-50 dark:group-hover/item:fill-custom-500/20"
					/></a
				>
			</li>
		</ul>
		<div class="md:hidden navbar-toggale-button">
			<button
				type="button"
				class="flex items-center justify-center size-[37.5px] p-0 text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
				><LucideIcon name="Menu" /></button
			>
		</div>
	</nav>

	<h6 class="mb-4 mt-7 text-16">Navbar with Search</h6>

	<nav
		class="group-data-[skin=bordered]:shadow-sm group-data-[skin=bordered]:border group-data-[skin=bordered]:border-slate-200 group-data-[skin=bordered]:dark:border-zink-500 flex items-center h-16 px-4 py-2 bg-white rounded-md shadow-md dark:bg-zink-700 navbar"
	>
		<div class="shrink-0">
			<a href="#!"
				><img src="/assets/images/logo-dark.png" alt="" class="block h-5 dark:hidden" /></a
			>
			<a href="#!"
				><img src="/assets/images/logo-light.png" alt="" class="hidden h-5 dark:block" /></a
			>
		</div>
		<div class="mx-1 xl:mx-4">
			<input
				type="text"
				class="py-2 px-4 text-sm text-slate-700 bg-white border border-slate-200 rounded placeholder:text-gray-400 form-input focus-visible:outline-0 max-w-[300px] hidden lg:block focus:border-custom-400 dark:bg-zink-700 dark:border-zink-500 dark:placeholder:text-zink-200 dark:text-zink-50 dark:focus:border-custom-500"
				placeholder="Search..."
				autocomplete="off"
			/>
		</div>
		<ul
			id="navbar1"
			class="absolute inset-x-0 z-20 items-center hidden py-3 mx-auto bg-white shadow-lg dark:bg-zink-600 dark:md:bg-transparent md:z-0 grow navbar-menu rounded-b-md md:shadow-none md:flex top-full md:relative md:bg-transparent md:rounded-none md:top-auto md:py-0"
		>
			<li>
				<a
					href="#!"
					class="block md:inline-block px-4 md:px-3 py-2.5 md:py-0.5 text-15 font-medium text-slate-800 transition-all duration-300 ease-linear hover:text-custom-500 [&.active]:text-custom-500 dark:text-zink-100 dark:hover:text-custom-500 dark:[&.active]:text-custom-500 active"
					>Home</a
				>
			</li>
			<li>
				<a
					href="#!"
					class="block md:inline-block px-4 md:px-3 py-2.5 md:py-0.5 text-15 font-medium text-slate-800 transition-all duration-300 ease-linear hover:text-custom-500 [&.active]:text-custom-500 dark:text-zink-100 dark:hover:text-custom-500 dark:[&.active]:text-custom-500"
					>About Us</a
				>
			</li>
			<li>
				<a
					href="#!"
					class="block md:inline-block px-4 md:px-3 py-2.5 md:py-0.5 text-15 font-medium text-slate-800 transition-all duration-300 ease-linear hover:text-custom-500 [&.active]:text-custom-500 dark:text-zink-100 dark:hover:text-custom-500 dark:[&.active]:text-custom-500"
					>Blog</a
				>
			</li>
			<li>
				<a
					href="#!"
					class="block md:inline-block px-4 md:px-3 py-2.5 md:py-0.5 text-15 font-medium text-slate-800 transition-all duration-300 ease-linear hover:text-custom-500 [&.active]:text-custom-500 dark:text-zink-100 dark:hover:text-custom-500 dark:[&.active]:text-custom-500"
					>Contact</a
				>
			</li>
		</ul>
		<ul id="navbar2" class="flex items-center ltr:ml-auto rtl:mr-auto">
			<li>
				<a
					href="#!"
					class="inline-flex items-center justify-center size-10 transition-all duration-300 ease-linear tems-center text-slate-500 hover:text-custom-500 active:text-custom-500 dark:text-zink-200 dark:hover:text-custom-500 dark:active:text-custom-500 group/item"
					><LucideIcon
						name="ShoppingBag"
						class="size-4 fill-slate-50 dark:fill-zink-500 group-hover/item:fill-custom-50 dark:group-hover/item:fill-custom-500/20"
					/></a
				>
			</li>
			<li>
				<a
					href="#!"
					class="inline-flex items-center justify-center size-10 transition-all duration-300 ease-linear tems-center text-slate-500 hover:text-custom-500 active:text-custom-500 dark:text-zink-200 dark:hover:text-custom-500 dark:active:text-custom-500 group/item"
					><LucideIcon
						name="Bell"
						class="size-4 fill-slate-50 dark:fill-zink-500 group-hover/item:fill-custom-50 dark:group-hover/item:fill-custom-500/20"
					/></a
				>
			</li>
			<li>
				<a
					href="#!"
					class="inline-flex items-center justify-center size-10 transition-all duration-300 ease-linear tems-center text-slate-500 hover:text-custom-500 active:text-custom-500 dark:text-zink-200 dark:hover:text-custom-500 dark:active:text-custom-500 group/item"
					><LucideIcon
						name="UserCircle"
						class="size-4 fill-slate-50 dark:fill-zink-500 group-hover/item:fill-custom-50 dark:group-hover/item:fill-custom-500/20"
					/></a
				>
			</li>
		</ul>
		<div class="md:hidden navbar-toggale-button">
			<button
				type="button"
				class="flex items-center justify-center size-[37.5px] p-0 text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
				><LucideIcon name="Menu" /></button
			>
		</div>
	</nav>

	<h6 class="mb-4 mt-7 text-16">Dark Navbar</h6>

	<nav
		class="group-data-[skin=bordered]:shadow-sm group-data-[skin=bordered]:border group-data-[skin=bordered]:border-slate-200 group-data-[skin=bordered]:dark:border-zink-500 flex items-center h-16 px-4 py-2 mb-4 rounded-md shadow-md bg-slate-900 dark:bg-zink-600 navbar"
	>
		<div class="shrink-0">
			<a href="#!"><img src="/assets/images/logo-light.png" alt="" class="h-5" /></a>
		</div>
		<ul
			id="navbar3"
			class="absolute inset-x-0 z-20 items-center hidden py-3 mx-auto bg-white shadow-lg dark:bg-zink-600 dark:md:bg-transparent md:z-0 navbar-menu rounded-b-md md:shadow-none md:flex top-full md:relative md:bg-transparent md:rounded-none md:top-auto md:py-0"
		>
			<li>
				<a
					href="#!"
					class="block md:inline-block px-4 md:px-3 py-2.5 md:py-0.5 text-15 font-medium text-slate-400 transition-all duration-300 ease-linear hover:text-custom-500 [&.active]:text-custom-500 dark:text-zink-200 dark:hover:text-custom-500 dark:[&.active]:text-custom-500 active"
					>Home</a
				>
			</li>
			<li>
				<a
					href="#!"
					class="block md:inline-block px-4 md:px-3 py-2.5 md:py-0.5 text-15 font-medium text-slate-400 transition-all duration-300 ease-linear hover:text-custom-500 [&.active]:text-custom-500 dark:text-zink-200 dark:hover:text-custom-500 dark:[&.active]:text-custom-500"
					>About Us</a
				>
			</li>
			<li>
				<a
					href="#!"
					class="block md:inline-block px-4 md:px-3 py-2.5 md:py-0.5 text-15 font-medium text-slate-400 transition-all duration-300 ease-linear hover:text-custom-500 [&.active]:text-custom-500 dark:text-zink-200 dark:hover:text-custom-500 dark:[&.active]:text-custom-500"
					>Service</a
				>
			</li>
			<li>
				<a
					href="#!"
					class="block md:inline-block px-4 md:px-3 py-2.5 md:py-0.5 text-15 font-medium text-slate-400 transition-all duration-300 ease-linear hover:text-custom-500 [&.active]:text-custom-500 dark:text-zink-200 dark:hover:text-custom-500 dark:[&.active]:text-custom-500"
					>Blog</a
				>
			</li>
			<li>
				<a
					href="#!"
					class="block md:inline-block px-4 md:px-3 py-2.5 md:py-0.5 text-15 font-medium text-slate-400 transition-all duration-300 ease-linear hover:text-custom-500 [&.active]:text-custom-500 dark:text-zink-200 dark:hover:text-custom-500 dark:[&.active]:text-custom-500"
					>Contact</a
				>
			</li>
		</ul>
		<ul id="navbar4" class="flex items-center ltr:ml-auto rtl:mr-auto">
			<li>
				<a
					href="#!"
					class="inline-flex items-center justify-center size-10 transition-all duration-300 ease-linear tems-center text-slate-400 hover:text-custom-500 active:text-custom-500 group/item dark:text-zink-200 dark:hover:text-custom-500 dark:active:text-custom-500"
					><LucideIcon
						name="Search"
						class="size-4 fill-slate-50/10 group-hover/item:fill-custom-400/20 dark:fill-zink-500"
					/></a
				>
			</li>
			<li>
				<a
					href="#!"
					class="inline-flex items-center justify-center size-10 transition-all duration-300 ease-linear tems-center text-slate-400 hover:text-custom-500 active:text-custom-500 group/item dark:text-zink-200 dark:hover:text-custom-500 dark:active:text-custom-500"
					><LucideIcon
						name="ShoppingBag"
						class="size-4 fill-slate-50/10 group-hover/item:fill-custom-400/20 dark:fill-zink-500"
					/></a
				>
			</li>
			<li>
				<a
					href="#!"
					class="inline-flex items-center justify-center size-10 transition-all duration-300 ease-linear tems-center text-slate-400 hover:text-custom-500 active:text-custom-500 group/item dark:text-zink-200 dark:hover:text-custom-500 dark:active:text-custom-500"
					><LucideIcon
						name="Bell"
						class="size-4 fill-slate-50/10 group-hover/item:fill-custom-400/20 dark:fill-zink-500"
					/></a
				>
			</li>
			<li>
				<a
					href="#!"
					class="inline-flex items-center justify-center size-10 transition-all duration-300 ease-linear tems-center text-slate-400 hover:text-custom-500 active:text-custom-500 group/item dark:text-zink-200 dark:hover:text-custom-500 dark:active:text-custom-500"
					><LucideIcon
						name="UserCircle"
						class="size-4 fill-slate-50/10 group-hover/item:fill-custom-400/20 dark:fill-zink-500"
					/></a
				>
			</li>
		</ul>
		<div class="md:hidden navbar-toggale-button">
			<button
				type="button"
				class="flex items-center justify-center size-[37.5px] p-0 text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
				><LucideIcon name="Menu" /></button
			>
		</div>
	</nav>
</div>
