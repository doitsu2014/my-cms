<script>
	import { browser } from '$app/environment';
	import Header from '../../Layouts/Header.svelte';
	import RightSidebar from '../../Layouts/RightSidebar.svelte';
	import Sidebar from '../../Layouts/Sidebar.svelte';
	import Footer from '../../Layouts/Footer.svelte';
	import { onMount } from 'svelte';
	import { goto } from '$app/navigation';
	import { init, getLocaleFromNavigator } from "svelte-i18n";

	let layout = 'vertical';
	let isOpen = false;

	//  i18n translation
	init({
		fallbackLocale: "en",
		initialLocale: getLocaleFromNavigator(),
	});

	function toggleRightSidebar() {
		isOpen = !isOpen;
	}

	if (browser) {
		document.body.setAttribute('class', '');
		document.body.setAttribute(
			'class',
			'text-base bg-body-bg text-body font-public dark:text-zink-100 dark:bg-zink-800 group-data-[skin=bordered]:bg-body-bordered group-data-[skin=bordered]:dark:bg-zink-700'
		);
		document.documentElement.classList.remove("overflow-x-hidden")

	}

	onMount(() => {
		if (browser) {
			var user = sessionStorage.getItem('authuser');
			if (!user || user == null) {
				goto('/account/login'); // Redirect to login if not authenticated
				return false;
			}
		}

		//  Window scroll sticky class add
		function windowScroll() {
			var navbar = document.getElementById('page-topbar');
			if (navbar) {
				if (document.body.scrollTop >= 50 || document.documentElement.scrollTop >= 50) {
					navbar.classList.add('is-sticky');
				} else {
					navbar.classList.remove('is-sticky');
				}
			}
		}

		document.addEventListener('scroll', function () {
			windowScroll();
		});
	});
</script>

<Sidebar {layout} />
<Header on:rightsidebartoggle={toggleRightSidebar} />
<div class="relative min-h-screen group-data-[sidebar-size=sm]:min-h-sm">
	<div
		class="group-data-[sidebar-size=lg]:ltr:md:ml-vertical-menu group-data-[sidebar-size=lg]:rtl:md:mr-vertical-menu group-data-[sidebar-size=md]:ltr:ml-vertical-menu-md group-data-[sidebar-size=md]:rtl:mr-vertical-menu-md group-data-[sidebar-size=sm]:ltr:ml-vertical-menu-sm group-data-[sidebar-size=sm]:rtl:mr-vertical-menu-sm pt-[calc(theme('spacing.header')_*_1)] pb-[calc(theme('spacing.header')_*_0.8)] px-4 group-data-[navbar=bordered]:pt-[calc(theme('spacing.header')_*_1.3)] group-data-[navbar=hidden]:pt-0 group-data-[layout=horizontal]:mx-auto group-data-[layout=horizontal]:max-w-screen-2xl group-data-[layout=horizontal]:px-0 group-data-[layout=horizontal]:group-data-[sidebar-size=lg]:ltr:md:ml-auto group-data-[layout=horizontal]:group-data-[sidebar-size=lg]:rtl:md:mr-auto group-data-[layout=horizontal]:md:pt-[calc(theme('spacing.header')_*_1.8)] group-data-[layout=horizontal]:px-3 group-data-[layout=horizontal]:group-data-[navbar=hidden]:pt-[calc(theme('spacing.header')_*_0.9)]"
	>
		<slot />
	</div>
	<Footer />
</div>

<RightSidebar {layout} isRightsidebarOpen={isOpen} toggle={toggleRightSidebar} />
