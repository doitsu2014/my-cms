<script>
	import { onMount } from 'svelte';
	import LucideIcon from '../../../../common/components/LucideIcon.svelte';
	import { browser } from '$app/environment';

	let navClass = '';
	onMount(() => {
		if (browser) {
			window.addEventListener('scroll', scrollNavigation, true);

			const menuItems = document.querySelectorAll('.navbar-menu a');

			menuItems.forEach((item) => {
				item.addEventListener('click', function () {
					const parentId = item.parentElement.parentElement.getAttribute('id');

					if (parentId) {
						const parentContainer = document.getElementById(parentId);

						if (parentContainer) {
							const linksInsideContainer = parentContainer.querySelectorAll('li a.active');

							linksInsideContainer.forEach((link) => {
								link.classList.remove('active');
							});
						}
					}

					// Add 'active' class to the clicked link
					this.classList.add('active');
					const navbarMenu = document.querySelector('.navbar-menu');
					navbarMenu.classList.add('hidden');
				});
			});
		}
	});

	function scrollNavigation() {
		var scrollup = document.documentElement.scrollTop;
		if (scrollup > 50) {
			navClass = 'is-sticky';
		} else {
			navClass = '';
		}
	}

	const togglenavbar = () => {
		if (browser) {
			const navbarMenu = document.querySelector('.navbar-menu');
			navbarMenu.classList.toggle('hidden');
		}
	};
</script>

<nav
	class="fixed inset-x-0 top-0 z-50 flex items-center justify-center h-20 py-3 [&.is-sticky]:bg-white dark:[&.is-sticky]:bg-zink-700 border-b border-slate-200 dark:border-zink-500 [&.is-sticky]:shadow-lg [&.is-sticky]:shadow-slate-200/25 dark:[&.is-sticky]:shadow-zink-500/30 navbar {navClass}"
	id="navbar"
>
	<div class="container 2xl:max-w-[87.5rem] px-4 mx-auto flex items-center self-center w-full">
		<div class="shrink-0">
			<a href="#!">
				<img src="/assets/images/logo-dark.png" alt="" class="block h-6 dark:hidden" />
				<img src="/assets/images/logo-light.png" alt="" class="hidden h-6 dark:block" />
			</a>
		</div>
		<div class="mx-auto">
			<ul
				id="navbar7"
				class="absolute inset-x-0 z-20 items-center hidden py-3 bg-white shadow-lg dark:bg-zink-600 dark:md:bg-transparent md:z-0 navbar-menu rounded-b-md md:shadow-none md:flex top-full ltr:ml-auto rtl:mr-auto md:relative md:bg-transparent md:rounded-none md:top-auto md:py-0"
			>
				<li>
					<a
						href="#home"
						class="block md:inline-block px-4 md:px-3 py-2.5 md:py-0.5 text-15 font-medium text-slate-800 transition-all duration-300 ease-linear hover:text-custom-500 [&.active]:text-custom-500 dark:text-zink-100 dark:hover:text-custom-500 dark:[&.active]:text-custom-500 active"
						>Home</a
					>
				</li>
				<li>
					<a
						href="#features"
						class="block md:inline-block px-4 md:px-3 py-2.5 md:py-0.5 text-15 font-medium text-slate-800 transition-all duration-300 ease-linear hover:text-custom-500 [&.active]:text-custom-500 dark:text-zink-100 dark:hover:text-custom-500 dark:[&.active]:text-custom-500"
						>Our Features</a
					>
				</li>
				<li>
					<a
						href="#about"
						class="block md:inline-block px-4 md:px-3 py-2.5 md:py-0.5 text-15 font-medium text-slate-800 transition-all duration-300 ease-linear hover:text-custom-500 [&.active]:text-custom-500 dark:text-zink-100 dark:hover:text-custom-500 dark:[&.active]:text-custom-500"
						>About Us</a
					>
				</li>
				<li>
					<a
						href="#pricing"
						class="block md:inline-block px-4 md:px-3 py-2.5 md:py-0.5 text-15 font-medium text-slate-800 transition-all duration-300 ease-linear hover:text-custom-500 [&.active]:text-custom-500 dark:text-zink-100 dark:hover:text-custom-500 dark:[&.active]:text-custom-500"
						>Pricing</a
					>
				</li>
				<li>
					<a
						href="#contact"
						class="block md:inline-block px-4 md:px-3 py-2.5 md:py-0.5 text-15 font-medium text-slate-800 transition-all duration-300 ease-linear hover:text-custom-500 [&.active]:text-custom-500 dark:text-zink-100 dark:hover:text-custom-500 dark:[&.active]:text-custom-500"
						>Contact</a
					>
				</li>
			</ul>
		</div>
		<div class="flex gap-2">
			<div class="ltr:ml-auto rtl:mr-auto md:hidden navbar-toggale-button">
				<button
					type="button"
					class="flex items-center justify-center size-[37.5px] p-0 text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
					on:click={togglenavbar}
				>
					<LucideIcon name="Menu" />
				</button>
			</div>
			<button
				type="button"
				class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
				><span class="align-middle">Sign In</span>
				<LucideIcon name="LogIn" class="inline-block size-4 ltr:ml-1 rtl:mr-1" />
			</button>
		</div>
	</div>
</nav>
