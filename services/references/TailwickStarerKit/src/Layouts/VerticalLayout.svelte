<script>
	import { onMount, afterUpdate, onDestroy } from 'svelte';
	import { browser } from '$app/environment';
	import data from '../common/data/Layoutmenudata';
	import { page } from '$app/stores';
	import LucideIcon from '../common/components/LucideIcon.svelte';
	import { _ } from 'svelte-i18n';

	// after routeing complete call afterUpdate function
	afterUpdate(() => {
		document.body.classList.remove('overflow-hidden');
		let pathName = $page.url.pathname;
		const ul = document.getElementById('navbar-nav');

		if (ul) {
			const items = Array.from(ul.querySelectorAll('a'));
			let activeItems = items.filter((x) => x.classList.contains('show'));
			removeActivation(activeItems);

			let matchingMenuItem = items.find((x) => {
				return x.pathname === pathName;
			});
			if (matchingMenuItem) {
				activateParentDropdown(matchingMenuItem);
			}
		}

		if (document.documentElement.clientWidth < 768) {
			var verticalOverlay = document.getElementById('sidebar-overlay');
			verticalOverlay.classList.add('hidden');
			document.getElementById('sidebarSizesm').click();
		}
	});

	onMount(() => {
		let pathName = $page.url.pathname;
		const ul = document.getElementById('navbar-nav');
		if (ul) {
			const items = Array.from(ul.querySelectorAll('a'));
			let activeItems = items.filter((x) => x.classList.contains('show'));
			removeActivation(activeItems);

			let matchingMenuItem = items.find((x) => {
				return x.pathname === pathName;
			});
			if (matchingMenuItem) {
				activateParentDropdown(matchingMenuItem);
			}
		}

		let sidebarOverLay = document.getElementById('sidebar-overlay');
		if (sidebarOverLay) {
			sidebarOverLay.addEventListener('click', function (e) {
				document.querySelector('.app-menu').classList.add('hidden');
				sidebarOverLay.classList.add('hidden');
				document.body.classList.remove('overflow-hidden');
			});
		}
	});

	onDestroy(() => {});

	const toggleItem = (event) => {
		let isCurrentMenuId = event.target.closest('a.dropdown-button');
		let isMenu = isCurrentMenuId.nextElementSibling;
		if (isMenu.classList.contains('opacity-100')) {
			isMenu.classList.add('hidden');
			isMenu.classList.remove('opacity-100');
			// isCurrentMenuId.classList.add("show");
		} else {
			let dropDowns = Array.from(document.querySelectorAll('#navbar-nav .dropdown-content'));
			dropDowns.forEach((node) => {
				node.classList.add('hidden');
				node.classList.remove('opacity-100');
			});
			isMenu ? isMenu.classList.remove('hidden') : null;
			const ul = document.getElementById('navbar-nav');
			if (ul) {
				const iconItems = Array.from(ul.getElementsByTagName('a'));
				let activeIconItems = iconItems.filter(
					(x) => x.classList.contains('active') || x.classList.contains('show')
				);
				activeIconItems.forEach((item) => {
					item.classList.remove('active');
					item.classList.remove('show');
				});
			}
			// isCurrentMenuId.setAttribute('aria-expanded', 'true');
			if (isCurrentMenuId) {
				activateParentDropdown(isCurrentMenuId);
			}
		}
	};

	const activateParentDropdown = (item) => {
		item.classList.add('show');
		item.classList.add('active');
		let parentCollapseDiv = item.closest('.dropdown-content');
		if (parentCollapseDiv) {
			// to set aria expand true remaining
			parentCollapseDiv.classList.remove('hidden');
			parentCollapseDiv.parentElement.children[0].classList.add('active');
			parentCollapseDiv.parentElement.children[0].classList.add('show');
			if (parentCollapseDiv.parentElement.closest('.dropdown-content')) {
				parentCollapseDiv.parentElement.closest('.dropdown-content').classList.remove('hidden');
				if (parentCollapseDiv.parentElement.closest('.dropdown-content').previousElementSibling)
					parentCollapseDiv.parentElement
						.closest('.dropdown-content')
						.previousElementSibling.classList.add('show');
				parentCollapseDiv.parentElement
					.closest('.dropdown-content')
					.previousElementSibling.classList.add('active');
				if (
					parentCollapseDiv.parentElement
						.closest('.dropdown-content')
						.previousElementSibling.closest('.dropdown-content')
				) {
					parentdropdown -
						contentDiv.parentElement
							.closest('.dropdown-content')
							.previousElementSibling.closest('.dropdown-content')
							.classList.add('show');
					parentdropdown -
						contentDiv.parentElement
							.closest('.dropdown-content')
							.previousElementSibling.closest('.dropdown-content')
							.previousElementSibling.classList.add('active');
				}
			}
			return false;
		}
		return false;
	};

	const toggleSubItem = (event) => {
		let isCurrentMenuId = event.target.closest('a.dropdown-button');

		let isMenu = isCurrentMenuId.nextElementSibling;
		isMenu.classList.toggle('hidden');
		isMenu.classList.toggle('opacity-100');
		isCurrentMenuId.classList.toggle('show');

		if (isMenu.classList.contains('hidden')) {
		} else {
			let dropDowns = Array.from(document.querySelectorAll('.sub-menu'));
			dropDowns.forEach((node) => {
				node.classList.remove('show');
			});
			let subDropDowns = Array.from(document.querySelectorAll('.menu-dropdown .nav-link'));
			subDropDowns.forEach((submenu) => {
				submenu.setAttribute('aria-expanded', 'false');
			});

			if (event.target && event.target.nextElementSibling) {
				isCurrentMenuId.setAttribute('aria-expanded', 'true');
				event.target.nextElementSibling.classList.toggle('show');
			}
		}
	};

	const removeActivation = (items) => {
		items.forEach((item) => {
			if (item.classList.contains('dropdown-button')) {
				item.classList.remove('active');
				item.classList.remove('show');
				if (item.nextElementSibling) {
					item.nextElementSibling.classList.remove('opacity-100');
					item.nextElementSibling.classList.add('hidden');
				}
			} else {
				item.classList.remove('active');
				item.classList.remove('show');
			}
		});
	};
</script>

{#each data.Navdata as item}
	{#if item.isHeader}
		<li
			class="px-4 py-1 text-vertical-menu-item group-data-[sidebar=dark]:text-vertical-menu-item-dark group-data-[sidebar=brand]:text-vertical-menu-item-brand group-data-[sidebar=modern]:text-vertical-menu-item-modern uppercase font-medium text-[11px] cursor-default tracking-wider group-data-[sidebar-size=sm]:hidden group-data-[layout=horizontal]:hidden inline-block group-data-[sidebar-size=md]:block group-data-[sidebar-size=md]:underline group-data-[sidebar-size=md]:text-center group-data-[sidebar=dark]:dark:text-zink-200"
		>
			<span data-key="t-menu">{$_(item.label)}</span>
		</li>
	{:else if item.subMenu}
		<li class="relative group-data-[layout=horizontal]:shrink-0 group/sm">
			<a
				on:click={toggleItem}
				class="relative dropdown-button flex items-center ltr:pl-3 rtl:pr-3 ltr:pr-5 rtl:pl-5 mx-3 my-1 group/menu-link text-vertical-menu-item-font-size font-normal transition-all duration-75 ease-linear rounded-md py-2.5 text-vertical-menu-item hover:text-vertical-menu-item-hover hover:bg-vertical-menu-item-bg-hover [&.active]:text-vertical-menu-item-active [&.active]:bg-vertical-menu-item-bg-active group-data-[sidebar=dark]:text-vertical-menu-item-dark group-data-[sidebar=dark]:hover:text-vertical-menu-item-hover-dark group-data-[sidebar=dark]:dark:hover:text-custom-500 group-data-[layout=horizontal]:dark:hover:text-custom-500 group-data-[sidebar=dark]:hover:bg-vertical-menu-item-bg-hover-dark group-data-[sidebar=dark]:dark:hover:bg-zink-600 group-data-[sidebar=dark]:[&.active]:text-vertical-menu-item-active-dark group-data-[sidebar=dark]:[&.active]:bg-vertical-menu-item-bg-active-dark group-data-[sidebar=brand]:text-vertical-menu-item-brand group-data-[sidebar=brand]:hover:text-vertical-menu-item-hover-brand group-data-[sidebar=brand]:hover:bg-vertical-menu-item-bg-hover-brand group-data-[sidebar=brand]:[&.active]:bg-vertical-menu-item-bg-active-brand group-data-[sidebar=brand]:[&.active]:text-vertical-menu-item-active-brand group-data-[sidebar=modern]:text-vertical-menu-item-modern group-data-[sidebar=modern]:hover:bg-vertical-menu-item-bg-hover-modern group-data-[sidebar=modern]:hover:text-vertical-menu-item-hover-modern group-data-[sidebar=modern]:[&.active]:bg-vertical-menu-item-bg-active-modern group-data-[sidebar=modern]:[&.active]:text-vertical-menu-item-active-modern group-data-[sidebar-size=md]:block group-data-[sidebar-size=md]:text-center group-data-[sidebar-size=sm]:group-hover/sm:w-[calc(theme('spacing.vertical-menu-sm')_*_3.63)] group-data-[sidebar-size=sm]:group-hover/sm:bg-vertical-menu group-data-[sidebar-size=sm]:group-data-[sidebar=dark]:group-hover/sm:bg-vertical-menu-dark group-data-[sidebar-size=sm]:group-data-[sidebar=modern]:group-hover/sm:bg-vertical-menu-border-modern group-data-[sidebar-size=sm]:group-data-[sidebar=brand]:group-hover/sm:bg-vertical-menu-brand group-data-[sidebar-size=sm]:my-0 group-data-[sidebar-size=sm]:rounded-b-none group-data-[layout=horizontal]:m-0 group-data-[layout=horizontal]:ltr:pr-8 group-data-[layout=horizontal]:rtl:pl-8 group-data-[layout=horizontal]:hover:bg-transparent group-data-[layout=horizontal]:[&.active]:bg-transparent [&.dropdown-button]:before:absolute [&.dropdown-button]:[&.show]:before:content-['\ea4e'] [&.dropdown-button]:before:content-['\ea6e'] [&.dropdown-button]:before:font-remix ltr:[&.dropdown-button]:before:right-2 rtl:[&.dropdown-button]:before:left-2 [&.dropdown-button]:before:text-16 group-data-[sidebar-size=sm]:[&.dropdown-button]:before:hidden group-data-[sidebar-size=md]:[&.dropdown-button]:before:hidden group-data-[sidebar=dark]:dark:text-zink-200 group-data-[layout=horizontal]:dark:text-zink-200 group-data-[sidebar=dark]:[&.active]:dark:bg-zink-600 group-data-[layout=horizontal]:dark:[&.active]:text-custom-500 rtl:[&.dropdown-button]:before:rotate-180 group-data-[layout=horizontal]:[&.dropdown-button]:before:rotate-90 group-data-[layout=horizontal]:[&.dropdown-button]:[&.show]:before:rotate-0 rtl:[&.dropdown-button]:[&.show]:before:rotate-0"
				href={'javascript:void(0)'}
			>
				<span
					class="min-w-[1.75rem] group-data-[sidebar-size=sm]:h-[1.75rem] inline-block text-start text-[16px] group-data-[sidebar-size=md]:block group-data-[sidebar-size=sm]:flex group-data-[sidebar-size=sm]:items-center"
				>
					<LucideIcon
						name={item.icon}
						class="h-4 group-data-[sidebar-size=sm]:h-5 group-data-[sidebar-size=sm]:w-5 transition group-hover/menu-link:animate-icons fill-slate-100 group-hover/menu-link:fill-blue-200 group-data-[sidebar=dark]:fill-vertical-menu-item-bg-active-dark group-data-[sidebar=dark]:dark:fill-zink-600 group-data-[layout=horizontal]:dark:fill-zink-600 group-data-[sidebar=brand]:fill-vertical-menu-item-bg-active-brand group-data-[sidebar=modern]:fill-vertical-menu-item-bg-active-modern group-data-[sidebar=dark]:group-hover/menu-link:fill-vertical-menu-item-bg-active-dark group-data-[sidebar=dark]:group-hover/menu-link:dark:fill-custom-500/20 group-data-[layout=horizontal]:dark:group-hover/menu-link:fill-custom-500/20 group-data-[sidebar=brand]:group-hover/menu-link:fill-vertical-menu-item-bg-active-brand group-data-[sidebar=modern]:group-hover/menu-link:fill-vertical-menu-item-bg-active-modern group-data-[sidebar-size=md]:block group-data-[sidebar-size=md]:mx-auto group-data-[sidebar-size=md]:mb-2"
					/>
				</span>
				<span
					class="group-data-[sidebar-size=sm]:ltr:pl-10 group-data-[sidebar-size=sm]:rtl:pr-10 align-middle group-data-[sidebar-size=sm]:group-hover/sm:block group-data-[sidebar-size=sm]:hidden"
					data-key="t-dashboards">{$_(item.label)}</span
				>
			</a>
			<div
				class="dropdown-content group-data-[sidebar-size=sm]:ltr:left-vertical-menu-sm group-data-[sidebar-size=sm]:rtl:right-vertical-menu-sm group-data-[sidebar-size=sm]:w-[calc(theme('spacing.vertical-menu-sm')_*_2.8)] group-data-[sidebar-size=sm]:absolute group-data-[sidebar-size=sm]:rounded-b-sm bg-vertical-menu group-data-[sidebar=dark]:bg-vertical-menu-dark group-data-[sidebar=dark]:dark:bg-zink-700 group-data-[sidebar=brand]:bg-vertical-menu-brand group-data-[sidebar=modern]:bg-transparent group-data-[layout=horizontal]:md:absolute group-data-[layout=horizontal]:top-full group-data-[layout=horizontal]:md:w-44 group-data-[layout=horizontal]:py-2 group-data-[layout=horizontal]:rounded-b-md group-data-[layout=horizontal]:md:shadow-lg group-data-[layout=horizontal]:md:shadow-slate-500/10 group-data-[layout=horizontal]:dark:bg-zink-700 group-data-[layout=horizontal]:dark:md:shadow-zink-600/20 hidden group-data-[sidebar-size=sm]:hidden group-data-[sidebar-size=sm]:group-hover/sm:block group-data-[sidebar-size=sm]:ltr:rounded-br-md group-data-[sidebar-size=sm]:rtl:rounded-br-md group-data-[sidebar-size=sm]:shadow-lg group-data-[sidebar-size=sm]:shadow-slate-700/10 group-data-[sidebar-size=sm]:group-data-[sidebar=modern]:group-hover/sm:to-vertical-menu-to-modern group-data-[sidebar-size=sm]:group-data-[sidebar=modern]:group-hover/sm:from-vertical-menu-from-modern group-data-[sidebar-size=sm]:group-data-[sidebar=modern]:group-hover/sm:bg-gradient-to-tr"
			>
				<ul
					class="ltr:pl-[1.75rem] rtl:pr-[1.75rem] group-data-[sidebar-size=md]:ltr:pl-0 group-data-[sidebar-size=md]:rtl:pr-0 group-data-[sidebar-size=sm]:ltr:pl-0 group-data-[sidebar-size=sm]:rtl:pr-0 group-data-[sidebar-size=sm]:py-2 group-data-[layout=horizontal]:ltr:pl-0 group-data-[layout=horizontal]:rtl:pr-0"
				>
					{#each item.subMenu as subMenu}
						{#if subMenu.isChildItem}
							<li class="group-data-[layout=horizontal]:relative group/sub">
								<a
									href={'javascript:void(0);'}
									class="dropdown-button relative flex items-center px-6 py-2 text-vertical-menu-item-font-size transition-all duration-75 ease-linear text-vertical-menu-sub-item hover:text-vertical-menu-sub-item-hover [&.active]:text-vertical-menu-sub-item-active before:absolute ltr:before:left-1.5 rtl:before:right-1.5 before:top-4 before:w-1 before:h-1 before:rounded before:transition-all before:duration-75 before:ease-linear before:bg-vertical-menu-sub-item hover:before:bg-vertical-menu-sub-item-hover [&.active]:before:bg-vertical-menu-sub-item-active group-data-[sidebar=dark]:text-vertical-menu-sub-item-dark group-data-[sidebar=dark]:dark:text-zink-200 group-data-[sidebar=dark]:hover:text-vertical-menu-sub-item-hover-dark group-data-[sidebar=dark]:[&.active]:text-vertical-menu-sub-item-active-dark group-data-[sidebar=dark]:before:bg-vertical-menu-sub-item-dark/50 group-data-[sidebar=dark]:hover:before:bg-vertical-menu-sub-item-hover-dark group-data-[sidebar=dark]:[&.active]:before:text-vertical-menu-sub-item-active-dark group-data-[sidebar=brand]:text-vertical-menu-sub-item-brand group-data-[sidebar=brand]:hover:text-vertical-menu-sub-item-hover-brand group-data-[sidebar=brand]:before:bg-vertical-menu-sub-item-brand/80 group-data-[sidebar=brand]:hover:before:bg-vertical-menu-sub-item-hover-brand/80 group-data-[sidebar=brand]:[&.active]:before:bg-vertical-menu-sub-item-active-brand/80 group-data-[sidebar=brand]:[&.active]:text-vertical-menu-sub-item-active-brand group-data-[sidebar=modern]:text-vertical-menu-sub-item-modern group-data-[sidebar=modern]:before:bg-vertical-menu-sub-item-modern/70 group-data-[sidebar=modern]:hover:text-vertical-menu-sub-item-hover-modern group-data-[sidebar=modern]:before:hover:bg-vertical-menu-sub-item-hover-modern group-data-[sidebar=modern]:[&.active]:text-vertical-menu-sub-item-active-modern group-data-[sidebar=modern]:before:[&.active]:text-vertical-menu-sub-item-active-modern group-data-[sidebar-size=md]:before:hidden group-data-[sidebar-size=md]:text-center group-data-[sidebar-size=md]:block group-data-[sidebar-size=sm]:before:hidden group-data-[layout=horizontal]:px-5 [&.dropdown-button]:after:absolute [&.dropdown-button]:[&.show]:after:content-['\ea4e'] [&.dropdown-button]:after:content-['\ea6e'] [&.dropdown-button]:after:font-remix ltr:[&.dropdown-button]:after:right-5 rtl:[&.dropdown-button]:after:left-5 [&.dropdown-button]:after:text-16 group-data-[sidebar-size=sm]:[&.dropdown-button]:after:hidden group-data-[sidebar-size=md]:[&.dropdown-button]:after:hidden group-data-[layout=horizontal]:before:left-[1.4rem] group-data-[layout=horizontal]:md:before:hidden group-data-[layout=horizontal]:ltr:pl-10 group-data-[layout=horizontal]:rtl:pr-10 group-data-[layout=horizontal]:ltr:pr-5 group-data-[layout=horizontal]:rtl:pl-5 group-data-[layout=horizontal]:md:!px-5"
									on:click={toggleSubItem}
								>
									{$_(subMenu.label)}
								</a>
								<div
									class="dropdown-content group-data-[sidebar-size=sm]:w-[calc(theme('spacing.vertical-menu-sm')_*_2.8)] group-data-[sidebar-size=sm]:absolute group-data-[sidebar-size=sm]:rounded-b-sm bg-vertical-menu group-data-[sidebar=dark]:bg-vertical-menu-dark group-data-[sidebar=dark]:dark:bg-zink-700 group-data-[sidebar=brand]:bg-vertical-menu-brand group-data-[sidebar=modern]:bg-transparent group-data-[layout=horizontal]:md:absolute group-data-[layout=horizontal]:top-full group-data-[layout=horizontal]:lg:w-44 group-data-[layout=horizontal]:py-2 group-data-[layout=horizontal]:rounded-b-md group-data-[layout=horizontal]:md:shadow-lg group-data-[layout=horizontal]:shadow-slate-500/10 group-data-[layout=horizontal]:left-full group-data-[sidebar-size=sm]:hidden group-data-[sidebar-size=sm]:group-hover/sub:block group-data-[sidebar-size=sm]:left-full group-data-[sidebar-size=sm]:top-0 group-data-[sidebar-size=sm]:shadow-lg group-data-[sidebar-size=sm]:shadow-slate-700/5 group-data-[layout=horizontal]:dark:bg-zink-700 group-data-[layout=horizontal]:dark:shadow-zink-600/20 group-data-[layout=horizontal]:z-10 hidden"
								>
									<ul
										class="ltr:pl-[1.75rem] rtl:pr-[1.75rem] group-data-[sidebar-size=md]:ltr:pl-0 group-data-[sidebar-size=md]:rtl:pr-0 group-data-[sidebar-size=sm]:ltr:pl-0 group-data-[sidebar-size=sm]:rtl:pr-0 group-data-[sidebar-size=sm]:py-2 group-data-[layout=horizontal]:ltr:md:pl-0 group-data-[layout=horizontal]:rtl:md:pr-0"
									>
										{#each subMenu.childItems as childItem}
											<li>
												<a
													class="relative flex items-center px-6 py-2 text-vertical-menu-item-font-size transition-all duration-75 ease-linear text-vertical-menu-sub-item hover:text-vertical-menu-sub-item-hover [&.active]:text-vertical-menu-sub-item-active before:absolute ltr:before:left-1.5 rtl:before:right-1.5 before:top-4 before:w-1 before:h-1 before:rounded before:transition-all before:duration-75 before:ease-linear before:bg-vertical-menu-sub-item hover:before:bg-vertical-menu-sub-item-hover [&.active]:before:bg-vertical-menu-sub-item-active group-data-[sidebar=dark]:text-vertical-menu-sub-item-dark group-data-[sidebar=dark]:dark:text-zink-200 group-data-[layout=horizontal]:dark:text-zink-200 group-data-[layout=horizontal]:dark:hover:text-custom-500 group-data-[sidebar=dark]:hover:text-vertical-menu-sub-item-hover-dark group-data-[sidebar=dark]:dark:hover:text-custom-500 group-data-[sidebar=dark]:[&.active]:text-vertical-menu-sub-item-active-dark group-data-[sidebar=dark]:dark:[&.active]:text-custom-500 group-data-[layout=horizontal]:dark:[&.active]:text-custom-500 group-data-[sidebar=dark]:before:bg-vertical-menu-sub-item-dark/50 group-data-[sidebar=dark]:hover:before:bg-vertical-menu-sub-item-hover-dark group-data-[sidebar=dark]:dark:hover:before:bg-custom-500 group-data-[sidebar=dark]:[&.active]:before:bg-vertical-menu-sub-item-active-dark group-data-[sidebar=dark]:dark:[&.active]:before:bg-custom-500 group-data-[sidebar=brand]:text-vertical-menu-sub-item-brand group-data-[sidebar=brand]:hover:text-vertical-menu-sub-item-hover-brand group-data-[sidebar=brand]:before:bg-vertical-menu-sub-item-brand/80 group-data-[sidebar=brand]:hover:before:bg-vertical-menu-sub-item-hover-brand/80 group-data-[sidebar=brand]:[&.active]:before:bg-vertical-menu-sub-item-active-brand/80 group-data-[sidebar=brand]:[&.active]:text-vertical-menu-sub-item-active-brand group-data-[sidebar=modern]:text-vertical-menu-sub-item-modern group-data-[sidebar=modern]:before:bg-vertical-menu-sub-item-modern/70 group-data-[sidebar=modern]:hover:text-vertical-menu-sub-item-hover-modern group-data-[sidebar=modern]:before:hover:bg-vertical-menu-sub-item-hover-modern group-data-[sidebar=modern]:[&.active]:text-vertical-menu-sub-item-active-modern group-data-[sidebar=modern]:before:[&.active]:text-vertical-menu-sub-item-active-modern group-data-[sidebar-size=md]:before:hidden group-data-[sidebar-size=md]:text-center group-data-[sidebar-size=md]:block group-data-[sidebar-size=sm]:before:hidden group-data-[layout=horizontal]:before:left-[1.4rem] group-data-[layout=horizontal]:md:before:hidden group-data-[layout=horizontal]:ltr:pl-10 group-data-[layout=horizontal]:rtl:pr-10 group-data-[layout=horizontal]:ltr:pr-5 group-data-[layout=horizontal]:rtl:pl-5 group-data-[layout=horizontal]:md:!px-5"
													href={childItem.link}>{$_(childItem.label)}</a
												>
											</li>
										{/each}
									</ul>
								</div>
							</li>
						{:else}
							<li>
								<a
									class="relative flex items-center px-6 py-2 text-vertical-menu-item-font-size transition-all duration-75 ease-linear text-vertical-menu-sub-item hover:text-vertical-menu-sub-item-hover [&.active]:text-vertical-menu-sub-item-active before:absolute ltr:before:left-1.5 rtl:before:right-1.5 before:top-4 before:w-1 before:h-1 before:rounded before:transition-all before:duration-75 before:ease-linear before:bg-vertical-menu-sub-item hover:before:bg-vertical-menu-sub-item-hover [&.active]:before:bg-vertical-menu-sub-item-active group-data-[sidebar=dark]:text-vertical-menu-sub-item-dark group-data-[sidebar=dark]:dark:text-zink-200 group-data-[layout=horizontal]:dark:text-zink-200 group-data-[layout=horizontal]:dark:hover:text-custom-500 group-data-[sidebar=dark]:hover:text-vertical-menu-sub-item-hover-dark group-data-[sidebar=dark]:dark:hover:text-custom-500 group-data-[sidebar=dark]:[&.active]:text-vertical-menu-sub-item-active-dark group-data-[sidebar=dark]:dark:[&.active]:text-custom-500 group-data-[layout=horizontal]:dark:[&.active]:text-custom-500 group-data-[sidebar=dark]:before:bg-vertical-menu-sub-item-dark/50 group-data-[sidebar=dark]:hover:before:bg-vertical-menu-sub-item-hover-dark group-data-[sidebar=dark]:dark:hover:before:bg-custom-500 group-data-[sidebar=dark]:[&.active]:before:bg-vertical-menu-sub-item-active-dark group-data-[sidebar=dark]:dark:[&.active]:before:bg-custom-500 group-data-[sidebar=brand]:text-vertical-menu-sub-item-brand group-data-[sidebar=brand]:hover:text-vertical-menu-sub-item-hover-brand group-data-[sidebar=brand]:before:bg-vertical-menu-sub-item-brand/80 group-data-[sidebar=brand]:hover:before:bg-vertical-menu-sub-item-hover-brand/80 group-data-[sidebar=brand]:[&.active]:before:bg-vertical-menu-sub-item-active-brand/80 group-data-[sidebar=brand]:[&.active]:text-vertical-menu-sub-item-active-brand group-data-[sidebar=modern]:text-vertical-menu-sub-item-modern group-data-[sidebar=modern]:before:bg-vertical-menu-sub-item-modern/70 group-data-[sidebar=modern]:hover:text-vertical-menu-sub-item-hover-modern group-data-[sidebar=modern]:before:hover:bg-vertical-menu-sub-item-hover-modern group-data-[sidebar=modern]:[&.active]:text-vertical-menu-sub-item-active-modern group-data-[sidebar=modern]:before:[&.active]:text-vertical-menu-sub-item-active-modern group-data-[sidebar-size=md]:before:hidden group-data-[sidebar-size=md]:text-center group-data-[sidebar-size=md]:block group-data-[sidebar-size=sm]:before:hidden group-data-[layout=horizontal]:before:left-[1.4rem] group-data-[layout=horizontal]:md:before:hidden group-data-[layout=horizontal]:ltr:pl-10 group-data-[layout=horizontal]:rtl:pr-10 group-data-[layout=horizontal]:ltr:pr-5 group-data-[layout=horizontal]:rtl:pl-5 group-data-[layout=horizontal]:md:!px-5"
									href={subMenu.link}
								>
									{$_(subMenu.label)}</a
								>
							</li>
						{/if}
					{/each}
				</ul>
			</div>
		</li>
	{:else}
		<li class="relative group-data-[layout=horizontal]:shrink-0 group/sm">
			<a
				href={item.link}
				class="relative flex items-center ltr:pl-3 rtl:pr-3 ltr:pr-5 rtl:pl-5 mx-3 my-1 group/menu-link text-vertical-menu-item-font-size font-normal transition-all duration-75 ease-linear rounded-md py-2.5 text-vertical-menu-item hover:text-vertical-menu-item-hover hover:bg-vertical-menu-item-bg-hover [&.active]:text-vertical-menu-item-active [&.active]:bg-vertical-menu-item-bg-active group-data-[sidebar=dark]:text-vertical-menu-item-dark group-data-[sidebar=dark]:hover:text-vertical-menu-item-hover-dark group-data-[sidebar=dark]:dark:hover:text-custom-500 group-data-[layout=horizontal]:dark:hover:text-custom-500 group-data-[sidebar=dark]:hover:bg-vertical-menu-item-bg-hover-dark group-data-[sidebar=dark]:dark:hover:bg-zink-600 group-data-[sidebar=dark]:[&.active]:text-vertical-menu-item-active-dark group-data-[sidebar=dark]:[&.active]:bg-vertical-menu-item-bg-active-dark group-data-[sidebar=brand]:text-vertical-menu-item-brand group-data-[sidebar=brand]:hover:text-vertical-menu-item-hover-brand group-data-[sidebar=brand]:hover:bg-vertical-menu-item-bg-hover-brand group-data-[sidebar=brand]:[&.active]:bg-vertical-menu-item-bg-active-brand group-data-[sidebar=brand]:[&.active]:text-vertical-menu-item-active-brand group-data-[sidebar=modern]:text-vertical-menu-item-modern group-data-[sidebar=modern]:hover:bg-vertical-menu-item-bg-hover-modern group-data-[sidebar=modern]:hover:text-vertical-menu-item-hover-modern group-data-[sidebar=modern]:[&.active]:bg-vertical-menu-item-bg-active-modern group-data-[sidebar=modern]:[&.active]:text-vertical-menu-item-active-modern group-data-[sidebar-size=md]:block group-data-[sidebar-size=md]:text-center group-data-[sidebar-size=sm]:group-hover/sm:w-[calc(theme('spacing.vertical-menu-sm')_*_3.63)] group-data-[sidebar-size=sm]:group-hover/sm:bg-vertical-menu group-data-[sidebar-size=sm]:group-data-[sidebar=dark]:group-hover/sm:bg-vertical-menu-dark group-data-[sidebar-size=sm]:group-data-[sidebar=modern]:group-hover/sm:bg-vertical-menu-border-modern group-data-[sidebar-size=sm]:group-data-[sidebar=brand]:group-hover/sm:bg-vertical-menu-brand group-data-[sidebar-size=sm]:my-0 group-data-[sidebar-size=sm]:rounded-b-none group-data-[layout=horizontal]:m-0 group-data-[layout=horizontal]:ltr:pr-8 group-data-[layout=horizontal]:rtl:pl-8 group-data-[layout=horizontal]:hover:bg-transparent group-data-[layout=horizontal]:[&.active]:bg-transparent [&.dropdown-button]:before:absolute [&.dropdown-button]:[&.show]:before:content-['\ea4e'] [&.dropdown-button]:before:content-['\ea6e'] [&.dropdown-button]:before:font-remix ltr:[&.dropdown-button]:before:right-2 rtl:[&.dropdown-button]:before:left-2 [&.dropdown-button]:before:text-16 group-data-[sidebar-size=sm]:[&.dropdown-button]:before:hidden group-data-[sidebar-size=md]:[&.dropdown-button]:before:hidden group-data-[sidebar=dark]:dark:text-zink-200 group-data-[layout=horizontal]:dark:text-zink-200 group-data-[sidebar=dark]:[&.active]:dark:bg-zink-600 group-data-[layout=horizontal]:dark:[&.active]:text-custom-500 rtl:[&.dropdown-button]:before:rotate-180 group-data-[layout=horizontal]:[&.dropdown-button]:before:rotate-90 group-data-[layout=horizontal]:[&.dropdown-button]:[&.show]:before:rotate-0 rtl:[&.dropdown-button]:[&.show]:before:rotate-0"
			>
				<span
					class="min-w-[1.75rem] group-data-[sidebar-size=sm]:h-[1.75rem] inline-block text-start text-[16px] group-data-[sidebar-size=md]:block group-data-[sidebar-size=sm]:flex group-data-[sidebar-size=sm]:items-center"
				>
					<LucideIcon
						name={item.icon}
						class="h-4 group-data-[sidebar-size=sm]:h-5 group-data-[sidebar-size=sm]:w-5 transition group-hover/menu-link:animate-icons fill-slate-100 group-hover/menu-link:fill-blue-200 group-data-[sidebar=dark]:fill-vertical-menu-item-bg-active-dark group-data-[sidebar=dark]:dark:fill-zink-600 group-data-[layout=horizontal]:dark:fill-zink-600 group-data-[sidebar=brand]:fill-vertical-menu-item-bg-active-brand group-data-[sidebar=modern]:fill-vertical-menu-item-bg-active-modern group-data-[sidebar=dark]:group-hover/menu-link:fill-vertical-menu-item-bg-active-dark group-data-[sidebar=dark]:group-hover/menu-link:dark:fill-custom-500/20 group-data-[layout=horizontal]:dark:group-hover/menu-link:fill-custom-500/20 group-data-[sidebar=brand]:group-hover/menu-link:fill-vertical-menu-item-bg-active-brand group-data-[sidebar=modern]:group-hover/menu-link:fill-vertical-menu-item-bg-active-modern group-data-[sidebar-size=md]:block group-data-[sidebar-size=md]:mx-auto group-data-[sidebar-size=md]:mb-2"
					/>
				</span>
				<span
					class="group-data-[sidebar-size=sm]:ltr:pl-10 group-data-[sidebar-size=sm]:rtl:pr-10 align-middle group-data-[sidebar-size=sm]:group-hover/sm:block group-data-[sidebar-size=sm]:hidden"
					data-key="t-email">{$_(item.label)}</span
				>
			</a>
		</li>
	{/if}
{/each}
