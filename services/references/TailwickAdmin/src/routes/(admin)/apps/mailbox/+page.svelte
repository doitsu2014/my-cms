<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import LucideIcon from '../../../../common/components/LucideIcon.svelte';
	import { OverlayScrollbars } from 'overlayscrollbars';
	import { onMount } from 'svelte';
	import Modal from '../../../../common/components/Modal.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import Dropdown from '../../../../common/components/Dropdown.svelte';
	import DropdownToggle from '../../../../common/components/DropdownToggle.svelte';
	import DropdownMenu from '../../../../common/components/DropdownMenu.svelte';

	let allcheck = false;
	let isOpenEmailOverview = false;
	let mailList = [
		{
			id: 1,
			name: 'Tonya Johnson',
			subject: 'Maintain the overall pristine status...',
			time: '10:59 AM',
			unread: true,
			active: true
		},
		{
			id: 2,
			name: 'Infra Teach',
			subject: 'How Custom Software Can Solve Your Business Challenges.',
			time: '04:15 PM',
			unread: false,
			active: true
		},

		{
			id: 3,
			name: 'Facebook',
			subject:
				'Post Basic High School, Ratanpur(GA) and 7 others are new Group suggestions for you',
			time: '12:29 PM',
			unread: false,
			active: true
		},
		{
			id: 4,
			name: 'Linkedin',
			subject: 'Front End Developer-HTML/CSS role at Gainserv: 1 connection',
			time: '08:44 PM',
			unread: false,
			active: true
		},
		{
			id: 5,
			name: 'Jobcy - Job Board & Landing Page',
			subject: 'You’re invited: Job Search Tactics for Entry-Level Candidates',
			time: '08:10 PM',
			unread: false,
			active: true
		},
		{
			id: 6,
			name: 'Holly Kavanaugh',
			subject: 'Information Technology Rules Annual Reminder',
			time: '04:33 PM',
			unread: true,
			active: true
		},
		{
			id: 7,
			name: 'Jose White',
			subject: 'I just requested to connect - Paula Keenan, respond to Meet’s now.',
			time: '01:19 PM',
			unread: false,
			active: true
		},
		{
			id: 8,
			name: 'Patricia Garcia',
			subject:
				'List all mail-enabled users who have specific permissions on the selected mailboxes, in other words their delegates.',
			time: '01:20 AM',
			unread: true,
			active: true
		},
		{
			id: 9,
			name: 'Tonya Johnson',
			subject:
				'Maintain the overall pristine status and health of your Exchange Online environment with M365 Manager Plus’ reports on the number of inactive mailboxes.',
			time: '10:57 AM',
			unread: false,
			active: true
		},
		{
			id: 10,
			name: 'Themesdesign',
			subject: 'Your License expired',
			time: '11:54 PM',
			unread: true,
			active: true
		},
		{
			id: 1,
			name: 'Tonya Johnson',
			subject: 'Maintain the overall pristine status...',
			time: '10:59 AM',
			unread: false,
			active: true
		},
		{
			id: 2,
			name: 'Infra Teach',
			subject: 'How Custom Software Can Solve Your Business Challenges.',
			time: '04:15 PM',
			unread: false,
			active: true
		},

		{
			id: 3,
			name: 'Facebook',
			subject:
				'Post Basic High School, Ratanpur(GA) and 7 others are new Group suggestions for you',
			time: '12:29 PM',
			unread: false,
			active: true
		},
		{
			id: 4,
			name: 'Linkedin',
			subject: 'Front End Developer-HTML/CSS role at Gainserv: 1 connection',
			time: '08:44 PM',
			unread: true,
			active: true
		},
		{
			id: 5,
			name: 'Jobcy - Job Board & Landing Page',
			subject: 'You’re invited: Job Search Tactics for Entry-Level Candidates',
			time: '08:10 PM',
			unread: false,
			active: true
		},
		{
			id: 6,
			name: 'Holly Kavanaugh',
			subject: 'Information Technology Rules Annual Reminder',
			time: '04:33 PM',
			unread: true,
			active: true
		},
		{
			id: 7,
			name: 'Jose White',
			subject: 'I just requested to connect - Paula Keenan, respond to Meet’s now.',
			time: '01:19 PM',
			unread: true,
			active: true
		},
		{
			id: 8,
			name: 'Patricia Garcia',
			subject:
				'List all mail-enabled users who have specific permissions on the selected mailboxes, in other words their delegates.',
			time: '01:20 AM',
			unread: true,
			active: true
		},
		{
			id: 9,
			name: 'Tonya Johnson',
			subject:
				'Maintain the overall pristine status and health of your Exchange Online environment with M365 Manager Plus’ reports on the number of inactive mailboxes.',
			time: '10:57 AM',
			unread: false,
			active: true
		},
		{
			id: 10,
			name: 'Themesdesign',
			subject: 'Your License expired',
			time: '11:54 PM',
			unread: false,
			active: true
		}
	];

	let isOpen = false;
	const toggleModal = () => {
		isOpen = !isOpen;
	};

	let isDeleteOpen = false;
	const toggleDeleteModal = () => {
		isDeleteOpen = !isDeleteOpen;
	};

	onMount(() => {
		const options = {
			scrollbars: {
				visibility: 'auto',
				autoHide: 'move',
				autoHideDelay: 100,
				dragScroll: true,
				clickScroll: false,
				theme: 'os-theme-dark',
				pointers: ['mouse', 'touch', 'pen']
			}
		};
		const menuElement = document.querySelectorAll('.scrollbar');
		if (menuElement) {
			menuElement.forEach(function (e) {
				OverlayScrollbars(e, options);
			});
		}
	});
</script>

<HeadTitle title="Mailbox" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto relative">
	<Breadcrumb title="Mailbox" pagetitle="Apps" />
	<div class="grid grid-cols-12 xl:flex-row gap-x-5">
		<div class="col-span-12 lg:col-span-3 2xl:col-span-2 shrink-0 card">
			<div class="card-body">
				<button
					type="button"
					class="w-full text-white bg-red-500 border-red-500 btn hover:text-white hover:bg-red-600 hover:border-red-600 focus:text-white focus:bg-red-600 focus:border-red-600 focus:ring focus:ring-red-100 active:text-white active:bg-red-600 active:border-red-600 active:ring active:ring-red-100 dark:ring-custom-400/20"
					on:click={toggleModal}
				>
					<LucideIcon name="Plus" class="inline-block size-4 mr-1" />
					<span class="align-middle">Compose Email</span>
				</button>
				<ul class="mt-4">
					<li>
						<a
							href="#!"
							class="transition-all duration-300 ease-linear text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 inline-block py-2 [&.active]:text-custom-500 dark:[&.active]:text-custom-500"
						>
							<LucideIcon name="Inbox" class="inline-block size-4 align-middle rtl:ml-2 ltr:mr-2" />
							<span class="align-middle">Inbox</span>
						</a>
					</li>
					<li>
						<a
							href="#!"
							class="transition-all duration-300 ease-linear text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 inline-block py-2 [&.active]:text-custom-500 dark:[&.active]:text-custom-500"
						>
							<LucideIcon name="Star" class="inline-block size-4 align-middle rtl:ml-2 ltr:mr-2" />
							<span class="align-middle">Starred</span>
						</a>
					</li>
					<li>
						<a
							href="#!"
							class="transition-all duration-300 ease-linear text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 inline-block py-2 [&.active]:text-custom-500 dark:[&.active]:text-custom-500"
						>
							<LucideIcon
								name="PenSquare"
								class="inline-block size-4 align-middle rtl:ml-2 ltr:mr-2"
							/>
							<span class="align-middle">Draft</span>
						</a>
					</li>
					<li>
						<a
							href="#!"
							class="transition-all duration-300 ease-linear text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 inline-block py-2 [&.active]:text-custom-500 dark:[&.active]:text-custom-500"
						>
							<LucideIcon
								name="StepForward"
								class="inline-block size-4 align-middle rtl:ml-2 ltr:mr-2"
							/> <span class="align-middle">Important</span>
						</a>
					</li>
					<li>
						<a
							href="#!"
							class="transition-all duration-300 ease-linear text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 inline-block py-2 [&.active]:text-custom-500 dark:[&.active]:text-custom-500"
						>
							<LucideIcon
								name="AlertOctagon"
								class="inline-block size-4 align-middle rtl:ml-2 ltr:mr-2"
							/> <span class="align-middle">Spam</span>
						</a>
					</li>
					<li>
						<a
							href="#!"
							class="transition-all duration-300 ease-linear text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 inline-block py-2 [&.active]:text-custom-500 dark:[&.active]:text-custom-500"
						>
							<LucideIcon
								name="UserCircle"
								class="inline-block size-4 align-middle rtl:ml-2 ltr:mr-2"
							/>
							<span class="align-middle">Contacts</span>
						</a>
					</li>
					<li>
						<a
							href="#!"
							class="transition-all duration-300 ease-linear text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 inline-block py-2 [&.active]:text-custom-500 dark:[&.active]:text-custom-500"
						>
							<LucideIcon
								name="Trash2"
								class="inline-block size-4 align-middle rtl:ml-2 ltr:mr-2"
							/>
							<span class="align-middle">Trash</span>
						</a>
					</li>
					<li class="py-3">
						<p class="text-sm uppercase">Category</p>
					</li>
					<li>
						<a
							href="#!"
							class="transition-all duration-300 ease-linear text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 inline-block py-2 [&.active]:text-custom-500 dark:[&.active]:text-custom-500"
						>
							<LucideIcon
								name="RadioTower"
								class="inline-block size-4 align-middle rtl:ml-2 ltr:mr-2"
							/>
							<span class="align-middle">Social</span>
						</a>
					</li>
					<li>
						<a
							href="#!"
							class="transition-all duration-300 ease-linear text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 inline-block py-2 [&.active]:text-custom-500 dark:[&.active]:text-custom-500"
						>
							<LucideIcon
								name="Squirrel"
								class="inline-block size-4 align-middle rtl:ml-2 ltr:mr-2"
							/>
							<span class="align-middle">Updates</span>
						</a>
					</li>
					<li>
						<a
							href="#!"
							class="transition-all duration-300 ease-linear text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 inline-block py-2 [&.active]:text-custom-500 dark:[&.active]:text-custom-500"
						>
							<LucideIcon
								name="FileText"
								class="inline-block size-4 align-middle rtl:ml-2 ltr:mr-2"
							/>
							<span class="align-middle">Forums</span>
						</a>
					</li>
					<li>
						<a
							href="#!"
							class="transition-all duration-300 ease-linear text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 inline-block py-2 [&.active]:text-custom-500 dark:[&.active]:text-custom-500"
						>
							<LucideIcon name="Tag" class="inline-block size-4 align-middle rtl:ml-2 ltr:mr-2" />
							<span class="align-middle">Promotions</span>
						</a>
					</li>
					<li class="py-3">
						<p class="text-sm uppercase">Label</p>
					</li>
					<li>
						<a
							href="#!"
							class="transition-all duration-300 ease-linear text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 inline-block py-2 [&.active]:text-custom-500 dark:[&.active]:text-custom-500"
						>
							<LucideIcon
								name="CircleDot"
								class="inline-block size-4 text-red-500 align-middle rtl:ml-2 ltr:mr-2"
							/> <span class="align-middle">Support</span>
						</a>
					</li>
					<li>
						<a
							href="#!"
							class="transition-all duration-300 ease-linear text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 inline-block py-2 [&.active]:text-custom-500 dark:[&.active]:text-custom-500"
						>
							<LucideIcon
								name="CircleDot"
								class="inline-block size-4 text-purple-500 align-middle rtl:ml-2 ltr:mr-2"
							/> <span class="align-middle">Freelance</span>
						</a>
					</li>
					<li>
						<a
							href="#!"
							class="transition-all duration-300 ease-linear text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 inline-block py-2 [&.active]:text-custom-500 dark:[&.active]:text-custom-500"
						>
							<LucideIcon
								name="CircleDot"
								class="inline-block size-4 align-middle rtl:ml-2 ltr:mr-2 text-sky-500"
							/> <span class="align-middle">Personal</span>
						</a>
					</li>
				</ul>
			</div>
		</div>
		<!--end-->
		<div class="col-span-12 lg:col-span-9 2xl:col-span-10 grow card">
			<div id="emailList" class={isOpenEmailOverview ? 'hidden' : 'block'}>
				<div class="card-body">
					<div class="grid items-center grid-cols-1 gap-4 2xl:grid-cols-12">
						<div class="2xl:col-span-5">
							<div
								class="flex flex-wrap items-center gap-3 divide-x rtl:divide-x-reverse divide-slate-200 dark:divide-zink-500"
							>
								<div class="flex items-center gap-3">
									<div class="flex items-center gap-2" id="email-topbar-actions">
										<input
											id="checkboxAll"
											class="form-check-input size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800"
											type="checkbox"
											on:change={(e) => (allcheck = e.target.checked)}
										/>
									</div>
									<a
										href="#!"
										class="flex items-center justify-center size-6 transition-all duration-200 ease-linear text-slate-500 dark:text-zink-200 hover:text-slate-600 dark:hover:text-zink-50"
									>
										<LucideIcon name="RefreshCw" class="size-4" />
									</a>
								</div>
								<div class="flex items-center gap-3 ltr:pl-2 rtl:pr-2">
									<a
										href="#!"
										class="flex items-center justify-center size-6 transition-all duration-200 ease-linear text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500"
									>
										<LucideIcon name="ArchiveRestore" class="size-4" />
									</a>
									<a
										href="#!"
										class="flex items-center justify-center size-6 transition-all duration-200 ease-linear text-slate-500 dark:text-zink-200 hover:text-orange-500 dark:hover:text-orange-500"
									>
										<LucideIcon name="AlertOctagon" class="size-4" />
									</a>
									<a
										href="#!"
										class="flex items-center justify-center size-6 transition-all duration-200 ease-linear text-slate-500 dark:text-zink-200 hover:text-red-500 dark:hover:text-red-500"
										on:click={toggleDeleteModal}
									>
										<LucideIcon name="Trash2" class="size-4" />
									</a>
								</div>
								<div class="flex items-center gap-3 ltr:pl-2 rtl:pr-2">
									<a
										href="#!"
										class="flex items-center justify-center size-6 transition-all duration-200 ease-linear text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500"
										id="mark-all-read"
									>
										<LucideIcon name="Mail" class="size-4" />
									</a>
									<a
										href="#!"
										class="flex items-center justify-center size-6 transition-all duration-200 ease-linear text-slate-500 dark:text-zink-200 hover:text-green-500 dark:hover:text-green-500"
									>
										<LucideIcon name="Tag" class="size-4" />
									</a>
									<a
										href="#!"
										class="flex items-center justify-center size-6 transition-all duration-200 ease-linear text-slate-500 dark:text-zink-200 hover:text-sky-500 dark:hover:text-sky-500"
									>
										<LucideIcon name="Clock" class="size-4" />
									</a>
								</div>
								<div
									class="hidden px-4 py-3 text-sm unreadConversations-alert text-yellow-500 border border-transparent rounded-md bg-yellow-50 dark:bg-yellow-400/20"
									id="unreadConversations"
								>
									<span class="font-bold">No Unread Conversations</span>
								</div>
							</div>
						</div>
						<!--end col-->
						<div class="flex items-center gap-2 2xl:col-span-4 2xl:col-start-9">
							<div class="relative grow">
								<input
									type="text"
									id="searchResultList"
									value=""
									class="ltr:pl-8 rtl:pr-8 search form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
									placeholder="Search for ..."
									autocomplete="off"
								/>
								<LucideIcon
									name="Search"
									class="inline-block size-4 absolute ltr:left-2.5 rtl:right-2.5 top-2.5 text-slate-500 dark:text-zink-200 fill-slate-100 dark:fill-zink-600"
								/>
							</div>
							<Dropdown className="relative shrink-0" direction="bottom-start">
								<DropdownToggle
									className="flex items-center justify-center w-[39px] h-[39px] dropdown-toggle p-0 text-slate-500 btn bg-slate-100 hover:text-white hover:bg-slate-600 focus:text-white focus:bg-slate-600 focus:ring focus:ring-slate-100 active:text-white active:bg-slate-600 active:ring active:ring-slate-100 dark:bg-slate-500/20 dark:text-slate-400 dark:hover:bg-slate-500 dark:hover:text-white dark:focus:bg-slate-500 dark:focus:text-white dark:active:bg-slate-500 dark:active:text-white dark:ring-slate-400/20"
								>
									<LucideIcon name="MoreHorizontal" class="size-5" />
								</DropdownToggle>
								<DropdownMenu
									tag="ul"
									class="absolute z-50 py-2 mt-1 text-left list-none bg-white rounded-md shadow-md dropdown-menu min-w-max dark:bg-zink-600"
								>
									<li>
										<a
											class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
											href="#!">Show more message</a
										>
									</li>
									<li>
										<a
											class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
											href="#!">Important & Unread</a
										>
									</li>
									<li>
										<a
											class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
											href="#!">Hide section when empty</a
										>
									</li>
									<li>
										<a
											class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
											href="#!">All Delete</a
										>
									</li>
								</DropdownMenu>
							</Dropdown>
						</div>
						<!--end col-->
					</div>
					<!--end grid-->
				</div>
				<div class="xl:max-h-[calc(100vh_-_300px)] scrollbar">
					<div class="!pt-0 card-body">
						<div class="overflow-x-auto">
							<table class="w-full whitespace-nowrap">
								<tbody class="elmLoader" id="mail-list">
									{#each mailList as mail}
										<tr
											class="relative before:absolute ltr:before:left-0 rtl:before:right-0 before:border [&.checked]:before:border-custom-500 before:inset-y-0 before:border-transparent {mail.unread ==
											true
												? ''
												: 'unread'} group/mail"
										>
											<td
												class="px-3.5 py-2.5 border-y border-transparent first:pl-0 last:pr-0 w-20"
											>
												<div
													class="flex items-center gap-3 ltr:pl-2 rtl:pr-2 checkbox-wrapper-mail"
												>
													<input
														id="checkbox"
														class="itemCheckbox size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800"
														type="checkbox"
														checked={allcheck}
													/>
													<a
														href="#!"
														class="transition-all duration-200 ease-linear text-slate-500 hover:text-yellow-500 dark:text-zink-200 dark:hover:text-yellow-500"
													>
														<LucideIcon name="Star" class="size-4" />
													</a>
													<a
														href="#!"
														class="transition-all duration-200 ease-linear text-slate-500 hover:text-yellow-500 dark:text-zink-200 dark:hover:text-yellow-500"
													>
														<LucideIcon name="StepForward" class="size-4" />
													</a>
												</div>
											</td>
											<!-- svelte-ignore a11y-no-static-element-interactions -->
											<td
												class="px-3.5 py-2.5 border-y border-transparent mailBox first:pl-0 last:pr-0 text-slate-500 group-[.unread]/mail:text-slate-800 dark:text-zink-200 dark:group-[.unread]/mail:text-zink-50"
												on:click={() => (isOpenEmailOverview = !isOpenEmailOverview)}
											>
												<!-- svelte-ignore a11y-click-events-have-key-events -->
												<div class="grid items-center grid-cols-12 gap-3">
													<div class="col-span-4 lg:col-span-2">
														<a href="#!" class="block truncate before:inset-0">{mail.name}</a>
													</div>
													<div class="col-span-8 lg:col-span-10">
														<p class="truncate">{mail.subject}</p>
													</div>
												</div>
											</td>
											<td
												class="px-3.5 py-2.5 border-y border-transparent first:pl-0 last:pr-0 text-end w-20 text-slate-500 group-[.unread]/mail:text-slate-800 dark:text-zink-200 dark:group-[.unread]/mail:text-zink-50"
											>
												{mail.time}</td
											>
										</tr>
									{/each}
								</tbody>
							</table>
						</div>
					</div>
				</div>
			</div>
			<div id="emailOverview" class={isOpenEmailOverview ? '' : ' hidden'}>
				<div class="card-body">
					<div class="flex gap-2">
						<div class="grow">
							<button
								id="closeChatRightSidebar"
								class="inline-flex items-center justify-center h-8 transition-all duration-200 ease-linear rounded-md shrink-0 text-slate-500 hover:text-custom-500"
								on:click={() => (isOpenEmailOverview = !isOpenEmailOverview)}
							>
								<LucideIcon name="ChevronsLeft" class="size-4 mx-auto" />
							</button>
							<h6 class="mb-1 text-16">How Custom Software Can Solve Your Business Challenges</h6>
							<p class="text-slate-500 dark:text-zink-200">
								<a href="#!">infrateach@tailwick.com</a> (Aug 6, 2023, 9:04 PM)
							</p>
						</div>
						<div class="flex gap-3 shrink-0">
							<a
								href="#!"
								class="flex items-center justify-center size-6 transition-all duration-200 ease-linear text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500"
							>
								<LucideIcon name="ArchiveRestore" class="size-4" />
							</a>
							<a
								href="#!"
								class="flex items-center justify-center size-6 transition-all duration-200 ease-linear text-slate-500 dark:text-zink-200 hover:text-sky-500 dark:hover:text-sky-500"
							>
								<LucideIcon name="Printer" class="size-4" />
							</a>
							<a
								href="#!"
								class="flex items-center justify-center size-6 transition-all duration-200 ease-linear text-slate-500 dark:text-zink-200 hover:text-yellow-500 dark:hover:text-yellow-500"
							>
								<LucideIcon name="Star" class="size-4" />
							</a>
							<a
								href="#!"
								class="flex items-center justify-center size-6 transition-all duration-200 ease-linear text-slate-500 dark:text-zink-200 hover:text-red-500 dark:hover:text-red-500"
							>
								<LucideIcon name="Trash2" class="size-4" />
							</a>
						</div>
					</div>
				</div>
				<div class="xl:max-h-[calc(100vh_-_385px)] scrollbar">
					<div class="card-body !pt-0">
						<div class="flex gap-3 mt-3 first:mt-0">
							<div
								class="relative flex items-center justify-center font-semibold rounded-full text-slate-500 size-9 bg-slate-100 shrink-0 dark:text-zink-200 dark:bg-zink-600"
							>
								<img src="/assets/images/users/avatar-5.png" alt="" class="rounded-full h-9" />
							</div>
							<div class="grow">
								<div class="flex items-center">
									<div class="grow">
										<h6>Infra Teach</h6>
										<p class="text-slate-500 dark:text-zink-200">
											<a href="#!">infrateach@tailwick.com</a>
										</p>
									</div>
									<div class="shrink-0">Aug 6, 2023, 9:04 PM</div>
								</div>
								<div class="p-4 mt-3 rounded-md bg-slate-100 dark:bg-zink-600">
									<p class="mb-2 last:mb-0">Hi,</p>
									<p class="mb-2 last:mb-0">
										Custom software solutions are tailor-made software applications designed to meet
										the unique needs of a specific business or organization. Unlike off-the-shelf
										software, which offers a standardized solution for a broad range of users,
										custom software is precisely crafted to align with the workflows, processes, and
										objectives of a particular business.
									</p>
									<p class="mb-2 last:mb-0">
										The key advantage of custom software lies in its ability to be scalable and
										flexible. It can evolve alongside the business, accommodating changing
										requirements and supporting expansion. By adapting to the specific needs of the
										organization, custom software empowers businesses to gain a competitive edge,
										differentiate themselves in the market, and deliver enhanced experiences to
										their customers.
									</p>
									<p class="mb-2 last:mb-0">Thank You</p>
								</div>
							</div>
						</div>
						<div class="flex gap-3 mt-3 first:mt-0">
							<div
								class="relative flex items-center justify-center font-semibold bg-pink-100 rounded-full text-slate-500 size-9 shrink-0 dark:bg-pink-500/20 dark:text-zink-200"
							>
								<img src="/assets/images/users/avatar-1.png" alt="" class="rounded-full h-9" />
							</div>
							<div class="grow">
								<div class="flex items-center">
									<div class="grow">
										<h6>Me</h6>
										<p class="text-slate-500 dark:text-zink-200">
											<a href="#!">paulakeenan@tailwick.com</a>
										</p>
									</div>
									<div class="shrink-0">07 Nov, 2023, 10:14 PM</div>
								</div>
								<div class="p-4 mt-3 rounded-md bg-slate-100 dark:bg-zink-600">
									<p class="mb-2 last:mb-0">Hi,</p>
									<p class="mb-2 last:mb-0">
										I hope this email finds you well. Let me start by saying that I am a big fan of
										your work and it has inspired me to push myself beyond what I thought were my
										limits!
									</p>
									<p class="mb-2 last:mb-0">
										After taking a good look at [target company] I realize that you could improve in
										[improvement area]. I have helped many others improve in the same area and I‘d
										be more than happy to talk with you about it!
									</p>
									<p class="mb-2 last:mb-0">
										Would you be available for a quick call to discuss how our [product/service]
										could help you?
									</p>
									<p class="mb-2 last:mb-0">Regards,</p>
									<p class="mb-2 last:mb-0">Themesdesign</p>
								</div>
							</div>
						</div>
						<div class="flex gap-3 mt-3 first:mt-0">
							<div
								class="relative flex items-center justify-center font-semibold rounded-full text-slate-500 size-9 bg-slate-100 shrink-0 dark:text-zink-200 dark:bg-zink-600"
							>
								<img src="/assets/images/users/avatar-5.png" alt="" class="rounded-full h-9" />
							</div>
							<div class="grow">
								<div class="flex items-center">
									<div class="grow">
										<h6>Infra Teach</h6>
										<p class="text-slate-500 dark:text-zink-200">
											<a href="#!">infrateach@tailwick.com</a>
										</p>
									</div>
									<div class="shrink-0">07 Nov, 2023, 10:42 PM</div>
								</div>
								<div class="p-4 mt-3 rounded-md bg-slate-100 dark:bg-zink-600">
									<p class="mb-2 last:mb-0">Hello, Themesdesign</p>
									<p class="mb-2 last:mb-0">
										You are probably very busy, I totally understand that!
									</p>
									<p class="mb-2 last:mb-0">
										It would be great to hear back from you. So, please let me know when you find
										some time.
									</p>
								</div>
							</div>
						</div>
					</div>
				</div>
				<div class="card-body">
					<div class="flex items-center gap-2">
						<div class="grow">
							<input
								type="text"
								id="inputText"
								class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
								placeholder="Enter Message"
								required
								autocomplete="off"
							/>
						</div>
						<div class="flex gap-2 shrink-0">
							<button
								type="button"
								class="flex items-center justify-center size-[37.5px] transition-all duration-200 ease-linear p-0 text-slate-500 btn bg-transparent border-transparent hover:text-slate-700 focus:text-slate-700 active:text-slate-700 dark:text-zink-200 dark:hover:text-zink-50 dark:focus:text-zink-50 dark:active:text-zink-50"
							>
								<LucideIcon name="Mic" class="size-4" />
							</button>
							<button
								type="button"
								class="flex items-center justify-center size-[37.5px] transition-all duration-200 ease-linear p-0 text-slate-500 btn bg-transparent border-transparent hover:text-slate-700 focus:text-slate-700 active:text-slate-700 dark:text-zink-200 dark:hover:text-zink-50 dark:focus:text-zink-50 dark:active:text-zink-50"
							>
								<LucideIcon name="Image" class="size-4" />
							</button>
							<button
								type="button"
								class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
							>
								<LucideIcon name="Send" class="inline-block size-4 mr-1 align-middle" />
								<span class="align-middle">Send</span>
							</button>
						</div>
					</div>
				</div>
			</div>
		</div>
		<!--end-->
	</div>
</div>

<Modal modal-center className="-translate-y-2/4" {isOpen} toggle={toggleModal}>
	<div class="w-screen lg:w-[55rem] bg-white shadow rounded-md dark:bg-zink-600">
		<div class="flex items-center justify-between p-5 border-b dark:border-zink-500">
			<h5 class="text-16">New Message</h5>
			<button
				data-modal-close="addEmailModal"
				class="transition-all duration-200 ease-linear text-slate-500 hover:text-red-500"
				on:click={toggleModal}><LucideIcon name="X" class="size-5" /></button
			>
		</div>
		<div class="max-h-[calc(theme('height.screen')_-_180px)] p-5 overflow-y-auto">
			<form action="#!">
				<div class="grid grid-cols-1 gap-5 xl:grid-cols-12">
					<div class="xl:col-span-12">
						<div class="flex items-center gap-3 mb-2">
							<label for="emailTo" class="inline-block text-base font-medium grow">To</label>
							<div>
								<a href="#!" class="mr-1 inlin-block">Cc</a>
								<a href="#!">Bcc</a>
							</div>
						</div>
						<input
							class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
							id="emailTo"
							data-choices
							data-choices-limit="3"
							data-choices-removeItem
							type="text"
							value="tailwick@themesdesign.com"
						/>
					</div>
					<div class="xl:col-span-12">
						<div>
							<label for="subjectInput" class="inline-block mb-2 text-base font-medium"
								>Subject</label
							>
							<input
								class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
								id="subjectInput"
								value="Subject"
							/>
						</div>
					</div>
					<div class="xl:col-span-12">
						<div>
							<label for="textArea" class="inline-block mb-2 text-base font-medium"
								>Example Textarea</label
							>
							<textarea
								class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
								id="textArea"
								rows="3"
							></textarea>
						</div>
					</div>
				</div>

				<div class="flex justify-end gap-2 mt-4">
					<button
						type="reset"
						class="text-red-500 bg-white btn hover:text-red-500 hover:bg-red-100 focus:text-red-500 focus:bg-red-100 active:text-red-500 active:bg-red-100 dark:bg-zink-600 dark:hover:bg-red-500/10 dark:focus:bg-red-500/10 dark:active:bg-red-500/10"
						on:click={toggleModal}>Cancel</button
					>
					<button
						type="submit"
						class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
						>Send</button
					>
				</div>
			</form>
		</div>
	</div>
</Modal>

<Modal modal-center className="-translate-y-2/4" isOpen={isDeleteOpen} toggle={toggleDeleteModal}>
	<div class="w-screen md:w-[25rem] bg-white shadow rounded-md dark:bg-zink-600">
		<div class="max-h-[calc(theme('height.screen')_-_180px)] overflow-y-auto px-6 py-8">
			<div class="float-right">
				<button
					data-modal-close="deleteModal"
					class="transition-all duration-200 ease-linear text-slate-500 hover:text-red-500"
					on:click={toggleDeleteModal}><LucideIcon name="X" class="size-5" /></button
				>
			</div>
			<img src="/assets/images/delete.png" alt="" class="block h-12 mx-auto" />
			<div class="mt-5 text-center">
				<h5 class="mb-1">Are you sure?</h5>
				<p class="text-slate-500 dark:text-zink-200">
					Are you certain you want to delete this record?
				</p>
				<div class="flex justify-center gap-2 mt-6">
					<button
						type="reset"
						data-modal-close="deleteModal"
						class="bg-white text-slate-500 btn hover:text-slate-500 hover:bg-slate-100 focus:text-slate-500 focus:bg-slate-100 active:text-slate-500 active:bg-slate-100 dark:bg-zink-600 dark:hover:bg-slate-500/10 dark:focus:bg-slate-500/10 dark:active:bg-slate-500/10"
						on:click={toggleDeleteModal}>Cancel</button
					>
					<button
						type="submit"
						class="text-white bg-red-500 border-red-500 btn hover:text-white hover:bg-red-600 hover:border-red-600 focus:text-white focus:bg-red-600 focus:border-red-600 focus:ring focus:ring-red-100 active:text-white active:bg-red-600 active:border-red-600 active:ring active:ring-red-100 dark:ring-custom-400/20"
						>Yes, Delete It!</button
					>
				</div>
			</div>
		</div>
	</div>
</Modal>
