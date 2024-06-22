<script>
	import { goto } from '$app/navigation';
	import { browser } from '$app/environment';
	import Dropdown from './Dropdown.svelte';
	import DropdownMenu from './DropdownMenu.svelte';
	import DropdownToggle from './DropdownToggle.svelte';
	import LucideIcon from './LucideIcon.svelte';

	function handleLogout() {
		if (browser) {
			sessionStorage.removeItem('authuser');
		}
		goto('/account/login'); // Redirect to login after logout
	}
</script>

<Dropdown className="relative flex items-center h-header" direction="bottom-start">
	<DropdownToggle
		className="inline-block p-0 transition-all duration-200 ease-linear bg-topbar rounded-full text-topbar-item dropdown-toggle btn hover:bg-topbar-item-bg-hover hover:text-topbar-item-hover group-data-[topbar=dark]:text-topbar-item-dark group-data-[topbar=dark]:bg-topbar-dark group-data-[topbar=dark]:hover:bg-topbar-item-bg-hover-dark group-data-[topbar=dark]:hover:text-topbar-item-hover-dark group-data-[topbar=brand]:bg-topbar-brand group-data-[topbar=brand]:hover:bg-topbar-item-bg-hover-brand group-data-[topbar=brand]:hover:text-topbar-item-hover-brand group-data-[topbar=dark]:dark:bg-zink-700 group-data-[topbar=dark]:dark:hover:bg-zink-600 group-data-[topbar=brand]:text-topbar-item-brand group-data-[topbar=dark]:dark:hover:text-zink-50 group-data-[topbar=dark]:dark:text-zink-200"
	>
		<div class="bg-pink-100 rounded-full">
			<img src="/assets/images/users/user-profile.png" alt="" class="size-[37.5px] rounded-full" />
		</div>
	</DropdownToggle>
	<DropdownMenu
		class="absolute z-50 p-4 ltr:text-left rtl:text-right bg-white rounded-md shadow-md !top-4 dropdown-menu min-w-[14rem] dark:bg-zink-600 block"
	>
		<h6 class="mb-2 text-sm font-normal text-slate-500 dark:text-zink-300">Welcome to Tailwick</h6>
		<a href="#!" class="flex gap-3 mb-3">
			<div class="relative inline-block shrink-0">
				<div class="rounded bg-slate-100 dark:bg-zink-500">
					<img src="/assets/images/users/user-profile.png" alt="" class="size-12 rounded" />
				</div>
				<span
					class="-top-1 ltr:-right-1 rtl:-left-1 absolute w-2.5 h-2.5 bg-green-400 border-2 border-white rounded-full dark:border-zink-600"
				></span>
			</div>
			<div>
				<h6 class="mb-1 text-15">Paula Keenan</h6>
				<p class="text-slate-500 dark:text-zink-300">CEO & Founder</p>
			</div>
		</a>
		<ul>
			<li>
				<a
					class="block ltr:pr-4 rtl:pl-4 py-1.5 text-base font-medium transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:text-custom-500 focus:text-custom-500 dark:text-zink-200 dark:hover:text-custom-500 dark:focus:text-custom-500"
					href="#!"
				>
					<LucideIcon name="user2" class="inline-block size-4 ltr:mr-2 rtl:ml-2" />
					Profile</a
				>
			</li>
			<li>
				<a
					class="block ltr:pr-4 rtl:pl-4 py-1.5 text-base font-medium transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:text-custom-500 focus:text-custom-500 dark:text-zink-200 dark:hover:text-custom-500 dark:focus:text-custom-500"
					href="#!"
					><LucideIcon
						name="Mail"
						data-lucide="mail"
						class="inline-block size-4 ltr:mr-2 rtl:ml-2"
					/> Inbox
					<span
						class="inline-flex items-center justify-center size-5 ltr:ml-2 rtl:mr-2 text-[11px] font-medium border rounded-full text-white bg-red-500 border-red-500"
						>15</span
					></a
				>
			</li>
			<li>
				<a
					class="block ltr:pr-4 rtl:pl-4 py-1.5 text-base font-medium transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:text-custom-500 focus:text-custom-500 dark:text-zink-200 dark:hover:text-custom-500 dark:focus:text-custom-500"
					href="#!"
					><LucideIcon name="MessageSquare" class="inline-block size-4 ltr:mr-2 rtl:ml-2" /> Chat</a
				>
			</li>
			<li>
				<a
					class="block ltr:pr-4 rtl:pl-4 py-1.5 text-base font-medium transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:text-custom-500 focus:text-custom-500 dark:text-zink-200 dark:hover:text-custom-500 dark:focus:text-custom-500"
					href="#!"
					><LucideIcon name="GemIcon" class="inline-block size-4 ltr:mr-2 rtl:ml-2" /> Upgrade
					<span
						class="inline-flex items-center justify-center w-auto h-5 ltr:ml-2 rtl:mr-2 px-1 text-[12px] font-medium border rounded text-white bg-sky-500 border-sky-500"
						>Pro</span
					></a
				>
			</li>
			<li class="pt-2 mt-2 border-t border-slate-200 dark:border-zink-500">
				<a
					class="block ltr:pr-4 rtl:pl-4 py-1.5 text-base font-medium transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:text-custom-500 focus:text-custom-500 dark:text-zink-200 dark:hover:text-custom-500 dark:focus:text-custom-500"
					href="#!"
					on:click={handleLogout}
					><LucideIcon name="LogOutIcon" class="inline-block size-4 ltr:mr-2 rtl:ml-2" /> Sign Out</a
				>
			</li>
		</ul>
	</DropdownMenu>
</Dropdown>
