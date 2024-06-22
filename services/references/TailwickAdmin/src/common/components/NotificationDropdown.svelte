<script>
	import Dropdown from './Dropdown.svelte';
	import DropdownMenu from './DropdownMenu.svelte';
	import DropdownToggle from './DropdownToggle.svelte';
	import LucideIcon from './LucideIcon.svelte';
	let type = 'all';
	const Filter = (e, category) => {
		type = category;
	};
</script>

<Dropdown className="relative flex items-center dropdown h-header" direction="bottom-end">
	<DropdownToggle
		className="inline-flex justify-center relative items-center p-0 text-topbar-item transition-all size-[37.5px] duration-200 ease-linear bg-topbar rounded-md dropdown-toggle btn hover:bg-topbar-item-bg-hover hover:text-topbar-item-hover group-data-[topbar=dark]:bg-topbar-dark group-data-[topbar=dark]:hover:bg-topbar-item-bg-hover-dark group-data-[topbar=dark]:hover:text-topbar-item-hover-dark group-data-[topbar=brand]:bg-topbar-brand group-data-[topbar=brand]:hover:bg-topbar-item-bg-hover-brand group-data-[topbar=brand]:hover:text-topbar-item-hover-brand group-data-[topbar=dark]:dark:bg-zink-700 group-data-[topbar=dark]:dark:hover:bg-zink-600 group-data-[topbar=brand]:text-topbar-item-brand group-data-[topbar=dark]:dark:hover:text-zink-50 group-data-[topbar=dark]:dark:text-zink-200 group-data-[topbar=dark]:text-topbar-item-dark"
	>
		<LucideIcon
			name="BellRing"
			class="inline-block size-5 stroke-1 fill-slate-100 group-data-[topbar=dark]:fill-topbar-item-bg-hover-dark group-data-[topbar=brand]:fill-topbar-item-bg-hover-brand"
		/>
		<span class="absolute top-0 right-0 flex size-1.5">
			<span
				class="absolute inline-flex w-full h-full rounded-full opacity-75 animate-ping bg-sky-400"
			></span>
			<span class="relative inline-flex size-1.5 rounded-full bg-sky-500"></span>
		</span>
	</DropdownToggle>
	<DropdownMenu
		class="absolute z-50 ltr:text-left rtl:text-right bg-white rounded-md shadow-md !top-4 dropdown-menu min-w-[20rem] lg:min-w-[26rem] dark:bg-zink-600 block"
	>
		<div class="p-4">
			<h6 class="mb-4 text-16">
				Notifications <span
					class="inline-flex items-center justify-center size-5 ml-1 text-[11px] font-medium border rounded-full text-white bg-orange-500 border-orange-500"
					>15</span
				>
			</h6>
			<ul
				class="flex flex-wrap w-full p-1 mb-2 text-sm font-medium text-center rounded-md filter-btns text-slate-500 bg-slate-100 nav-tabs dark:bg-zink-500 dark:text-zink-200"
				data-filter-target="notification-list"
			>
				<li class="grow">
					<a
						href={'javascript:void(0);'}
						data-filter="all"
						on:click={(e) => {
							Filter(e, 'all');
						}}
						class="inline-block nav-link px-1.5 w-full py-1 text-xs transition-all duration-300 ease-linear
                        rounded-md text-slate-500 border border-transparent [&.active]:bg-white
                        [&.active]:text-custom-500 hover:text-custom-500 active:text-custom-500 dark:text-zink-200
                        dark:hover:text-custom-500 dark:[&.active]:bg-zink-600 -mb-[1px] {type ==
						'all'
							? 'active'
							: ''}">View All</a
					>
				</li>
				<li class="grow">
					<a
						href={'javascript:void(0);'}
						data-filter="mention"
						on:click={(e) => {
							Filter(e, 'mention');
						}}
						class="inline-block nav-link px-1.5 w-full py-1 text-xs transition-all duration-300 ease-linear
                        rounded-md text-slate-500 border border-transparent [&.active]:bg-white
                        [&.active]:text-custom-500 hover:text-custom-500 active:text-custom-500 dark:text-zink-200
                        dark:hover:text-custom-500 dark:[&.active]:bg-zink-600 -mb-[1px] {type ==
						'mention'
							? 'active'
							: ''}">Mentions</a
					>
				</li>
				<li class="grow">
					<a
						href={'javascript:void(0);'}
						data-filter="follower"
						on:click={(e) => {
							Filter(e, 'follower');
						}}
						class="inline-block nav-link px-1.5 w-full py-1 text-xs transition-all duration-300 ease-linear rounded-md text-slate-500 border border-transparent [&.active]:bg-white [&.active]:text-custom-500 hover:text-custom-500 active:text-custom-500 dark:text-zink-200 dark:hover:text-custom-500 dark:[&.active]:bg-zink-600 -mb-[1px] {type ===
						'follower'
							? 'active'
							: ''}">Followers</a
					>
				</li>
				<li class="grow">
					<a
						href={'javascript:void(0);'}
						data-filter="invite"
						on:click={(e) => {
							Filter(e, 'invite');
						}}
						class="inline-block nav-link px-1.5 w-full py-1 text-xs transition-all duration-300 ease-linear rounded-md text-slate-500 border border-transparent [&.active]:bg-white [&.active]:text-custom-500 hover:text-custom-500 active:text-custom-500 dark:text-zink-200 dark:hover:text-custom-500 dark:[&.active]:bg-zink-600 -mb-[1px] {type ===
						'invite'
							? 'active'
							: ''}">Invites</a
					>
				</li>
			</ul>
		</div>
		<div data-overlayscrollbars-initialize class="max-h-[350px]">
			<div class="flex flex-col gap-1" id="notification-list">
				{#if type == 'all'}
					<a
						href="#!"
						class="flex gap-3 p-4 product-item hover:bg-slate-50 dark:hover:bg-zink-500 follower"
					>
						<div class="size-10 rounded-md shrink-0 bg-slate-100">
							<img src="/assets/images/users/avatar-3.png" alt="" class="rounded-md" />
						</div>
						<div class="grow">
							<h6 class="mb-1 font-medium"><b>@willie_passem</b> followed you</h6>
							<p class="mb-0 text-sm text-slate-500 dark:text-zink-300">
								<LucideIcon name="Clock" class="inline-block size-3.5 mr-1" />
								<span class="align-middle">Wednesday 03:42 PM</span>
							</p>
						</div>
						<div
							class="flex items-center self-start gap-2 text-xs text-slate-500 shrink-0 dark:text-zink-300"
						>
							<div class=" size-1.5 bg-custom-500 rounded-full"></div>
							4 sec
						</div>
					</a>
					<a
						href="#!"
						class="flex gap-3 p-4 product-item hover:bg-slate-50 dark:hover:bg-zink-500 mention"
					>
						<div class="size-10 bg-yellow-100 rounded-md shrink-0">
							<img src="/assets/images/users/avatar-5.png" alt="" class="rounded-md" />
						</div>
						<div class="grow">
							<h6 class="mb-1 font-medium"><b>@caroline_jessica</b> commented on your post</h6>
							<p class="mb-3 text-sm text-slate-500 dark:text-zink-300">
								<!-- <LucideIcon name="clock" class="inline-block size-3.5 mr-1"></i> -->
								<LucideIcon name="Clock" class="inline-block size-3.5 mr-1" />

								<span class="align-middle">Wednesday 03:42 PM</span>
							</p>
							<div
								class="p-2 rounded bg-slate-100 text-slate-500 dark:bg-zink-500 dark:text-zink-300"
							>
								Amazing! Fast, to the point, professional and really amazing to work with them!!!
							</div>
						</div>
						<div
							class="flex items-center self-start gap-2 text-xs text-slate-500 shrink-0 dark:text-zink-300"
						>
							<div class=" size-1.5 bg-custom-500 rounded-full"></div>
							15 min
						</div>
					</a>
					<a
						href="#!"
						class="flex gap-3 p-4 product-item hover:bg-slate-50 dark:hover:bg-zink-500 invite"
					>
						<div class="flex items-center justify-center size-10 bg-red-100 rounded-md shrink-0">
							<!-- <LucideIcon name="shopping-bag" class="size-5 text-red-500 fill-red-200"></i> -->
							<LucideIcon name="ShoppingBag" class="size-5 text-red-500 fill-red-200" />
						</div>
						<div class="grow">
							<h6 class="mb-1 font-medium">
								Successfully purchased a business plan for <span class="text-red-500">$199.99</span>
							</h6>
							<p class="mb-0 text-sm text-slate-500 dark:text-zink-300">
								<!-- <LucideIcon name="clock" class="inline-block size-3.5 mr-1"></i> -->
								<LucideIcon name="Clock" class="inline-block size-3.5 mr-1" />
								<span class="align-middle">Monday 11:26 AM</span>
							</p>
						</div>
						<div
							class="flex items-center self-start gap-2 text-xs text-slate-500 shrink-0 dark:text-zink-300"
						>
							<div class=" size-1.5 bg-custom-500 rounded-full"></div>
							Yesterday
						</div>
					</a>
					<a
						href="#!"
						class="flex gap-3 p-4 product-item hover:bg-slate-50 dark:hover:bg-zink-500 mention"
					>
						<div class="relative shrink-0">
							<div class="size-10 bg-pink-100 rounded-md">
								<img src="/assets/images/users/avatar-7.png" alt="" class="rounded-md" />
							</div>
							<div class="absolute text-orange-500 -bottom-0.5 -right-0.5 text-16">
								<i class="ri-heart-fill"></i>
							</div>
						</div>
						<div class="grow">
							<h6 class="mb-1 font-medium"><b>@scott</b> liked your post</h6>
							<p class="mb-0 text-sm text-slate-500 dark:text-zink-300">
								<LucideIcon name="Clock" class="inline-block size-3.5 mr-1" />
								<span class="align-middle">Thursday 06:59 AM</span>
							</p>
						</div>
						<div
							class="flex items-center self-start gap-2 text-xs text-slate-500 shrink-0 dark:text-zink-300"
						>
							<div class=" size-1.5 bg-custom-500 rounded-full"></div>
							1 Week
						</div>
					</a>
				{:else if type == 'follower'}
					<a
						href="#!"
						class="flex gap-3 p-4 product-item hover:bg-slate-50 dark:hover:bg-zink-500 follower"
					>
						<div class="size-10 rounded-md shrink-0 bg-slate-100">
							<img src="/assets/images/users/avatar-3.png" alt="" class="rounded-md" />
						</div>
						<div class="grow">
							<h6 class="mb-1 font-medium"><b>@willie_passem</b> followed you</h6>
							<p class="mb-0 text-sm text-slate-500 dark:text-zink-300">
								<!-- <LucideIcon name="clock" class="inline-block size-3.5 mr-1"></i>  -->
								<LucideIcon name="Clock" class="inline-block size-3.5 mr-1" />
								<span class="align-middle">Wednesday 03:42 PM</span>
							</p>
						</div>
						<div
							class="flex items-center self-start gap-2 text-xs text-slate-500 shrink-0 dark:text-zink-300"
						>
							<div class=" size-1.5 bg-custom-500 rounded-full"></div>
							4 sec
						</div>
					</a>
				{:else if type == 'mention'}
					<a
						href="#!"
						class="flex gap-3 p-4 product-item hover:bg-slate-50 dark:hover:bg-zink-500 mention"
					>
						<div class="size-10 bg-yellow-100 rounded-md shrink-0">
							<img src="/assets/images/users/avatar-5.png" alt="" class="rounded-md" />
						</div>
						<div class="grow">
							<h6 class="mb-1 font-medium"><b>@caroline_jessica</b> commented on your post</h6>
							<p class="mb-3 text-sm text-slate-500 dark:text-zink-300">
								<!-- <LucideIcon name="clock" class="inline-block size-3.5 mr-1"></i> -->
								<LucideIcon name="Clock" class="inline-block size-3.5 mr-1" />

								<span class="align-middle">Wednesday 03:42 PM</span>
							</p>
							<div
								class="p-2 rounded bg-slate-100 text-slate-500 dark:bg-zink-500 dark:text-zink-300"
							>
								Amazing! Fast, to the point, professional and really amazing to work with them!!!
							</div>
						</div>
						<div
							class="flex items-center self-start gap-2 text-xs text-slate-500 shrink-0 dark:text-zink-300"
						>
							<div class=" size-1.5 bg-custom-500 rounded-full"></div>
							15 min
						</div>
					</a>
					<a
						href="#!"
						class="flex gap-3 p-4 product-item hover:bg-slate-50 dark:hover:bg-zink-500 mention"
					>
						<div class="relative shrink-0">
							<div class="size-10 bg-pink-100 rounded-md">
								<img src="/assets/images/users/avatar-7.png" alt="" class="rounded-md" />
							</div>
							<div class="absolute text-orange-500 -bottom-0.5 -right-0.5 text-16">
								<i class="ri-heart-fill"></i>
							</div>
						</div>
						<div class="grow">
							<h6 class="mb-1 font-medium"><b>@scott</b> liked your post</h6>
							<p class="mb-0 text-sm text-slate-500 dark:text-zink-300">
								<!-- <LucideIcon name="clock" class="inline-block size-3.5 mr-1"></i>  -->
								<LucideIcon name="Clock" class="inline-block size-3.5 mr-1" />
								<span class="align-middle">Thursday 06:59 AM</span>
							</p>
						</div>
						<div
							class="flex items-center self-start gap-2 text-xs text-slate-500 shrink-0 dark:text-zink-300"
						>
							<div class=" size-1.5 bg-custom-500 rounded-full"></div>
							1 Week
						</div>
					</a>
				{:else if type == 'invite'}
					<a
						href="#!"
						class="flex gap-3 p-4 product-item hover:bg-slate-50 dark:hover:bg-zink-500 invite"
					>
						<div class="flex items-center justify-center size-10 bg-red-100 rounded-md shrink-0">
							<!-- <LucideIcon name="shopping-bag" class="size-5 text-red-500 fill-red-200"></i> -->
							<LucideIcon name="ShoppingBag" class="size-5 text-red-500 fill-red-200" />
						</div>
						<div class="grow">
							<h6 class="mb-1 font-medium">
								Successfully purchased a business plan for <span class="text-red-500">$199.99</span>
							</h6>
							<p class="mb-0 text-sm text-slate-500 dark:text-zink-300">
								<!-- <LucideIcon name="clock" class="inline-block size-3.5 mr-1"></i> -->
								<LucideIcon name="Clock" class="inline-block size-3.5 mr-1" />
								<span class="align-middle">Monday 11:26 AM</span>
							</p>
						</div>
						<div
							class="flex items-center self-start gap-2 text-xs text-slate-500 shrink-0 dark:text-zink-300"
						>
							<div class=" size-1.5 bg-custom-500 rounded-full"></div>
							Yesterday
						</div>
					</a>
				{/if}
			</div>
		</div>
		<div class="flex items-center gap-2 p-4 border-t border-slate-200 dark:border-zink-500">
			<div class="grow">
				<a href="#!">Manage Notification</a>
			</div>
			<div class="shrink-0">
				<button
					type="button"
					class="px-2 py-1.5 text-xs text-white transition-all duration-200 ease-linear btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100"
					>View All Notification <LucideIcon
						name="MoveRight"
						class="inline-block size-3.5 ml-1"
					/></button
				>
			</div>
		</div>
	</DropdownMenu>
</Dropdown>
