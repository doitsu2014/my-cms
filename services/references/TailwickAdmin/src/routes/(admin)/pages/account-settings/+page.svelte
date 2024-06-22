<script>
	import Dropdown from '../../../../common/components/Dropdown.svelte';
	import DropdownToggle from '../../../../common/components/DropdownToggle.svelte';
	import DropdownMenu from '../../../../common/components/DropdownMenu.svelte';
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import LucideIcon from '../../../../common/components/LucideIcon.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import Flatpickr from 'svelte-flatpickr';
	import 'flatpickr/dist/flatpickr.css';

	let tabId = 'personal';
</script>

<HeadTitle title="Account Settings" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Account Settings" pagetitle="Pages" />

	<div class="card">
		<div class="card-body">
			<div class="grid grid-cols-1 gap-5 lg:grid-cols-12 2xl:grid-cols-12">
				<div class="lg:col-span-2 2xl:col-span-1">
					<div
						class="relative inline-block size-20 rounded-full shadow-md bg-slate-100 profile-user xl:size-28"
					>
						<img
							src="/assets/images/users/avatar-1.png"
							alt=""
							class="object-cover border-0 rounded-full img-thumbnail user-profile-image"
						/>
						<div
							class="absolute bottom-0 flex items-center justify-center size-8 rounded-full ltr:right-0 rtl:left-0 profile-photo-edit"
						>
							<input
								id="profile-img-file-input"
								type="file"
								class="hidden profile-img-file-input"
							/>
							<label
								for="profile-img-file-input"
								class="flex items-center justify-center size-8 bg-white rounded-full shadow-lg cursor-pointer dark:bg-zink-600 profile-photo-edit"
							>
								<LucideIcon
									name="ImagePlus"
									class="size-4 text-slate-500 dark:text-zink-200 fill-slate-100 dark:fill-zink-500"
								/>
							</label>
						</div>
					</div>
				</div>
				<!--end col-->
				<div class="lg:col-span-10 2xl:col-span-9">
					<h5 class="mb-1">
						Paula Keenan
						<LucideIcon
							name="BadgeCheck"
							class="inline-block size-4 text-sky-500 fill-sky-100 dark:fill-custom-500/20"
						/>
					</h5>
					<div class="flex gap-3 mb-4">
						<p class="text-slate-500 dark:text-zink-200">
							<LucideIcon
								name="UserCircle"
								class="inline-block size-4 ltr:mr-1 rtl:ml-1 text-slate-500 dark:text-zink-200 fill-slate-100 dark:fill-zink-500"
							/>
							CEO & Founder
						</p>
						<p class="text-slate-500 dark:text-zink-200">
							<LucideIcon
								name="MapPin"
								class="inline-block size-4 ltr:mr-1 rtl:ml-1 text-slate-500 dark:text-zink-200 fill-slate-100 dark:fill-zink-500"
							/>
							Los Angeles, California
						</p>
					</div>
					<ul
						class="flex flex-wrap gap-3 mt-4 text-center divide-x divide-slate-200 dark:divide-zink-500 rtl:divide-x-reverse"
					>
						<li class="px-5">
							<h5>1542</h5>
							<p class="text-slate-500 dark:text-zink-200">Following</p>
						</li>
						<li class="px-5">
							<h5>10.65k</h5>
							<p class="text-slate-500 dark:text-zink-200">Followers</p>
						</li>
						<li class="px-5">
							<h5>115+</h5>
							<p class="text-slate-500 dark:text-zink-200">Products</p>
						</li>
					</ul>
					<p class="mt-4 text-slate-500 dark:text-zink-200">
						Strong leader and negotiator adept at driving collaboration and innovation. Highly
						accomplished CEO & Founder with 10+ years of experience creating, launching and leading
						successful business ventures. Proven ability to build relationships, drive customer
						loyalty and increase profitability.
					</p>
					<div class="flex gap-2 mt-4">
						<a
							href="#!"
							class="flex items-center justify-center transition-all duration-200 ease-linear rounded size-9 text-sky-500 bg-sky-100 hover:bg-sky-200 dark:bg-sky-500/20 dark:hover:bg-sky-500/30"
						>
							<LucideIcon name="Facebook" class="size-4" />
						</a>
						<a
							href="#!"
							class="flex items-center justify-center text-pink-500 transition-all duration-200 ease-linear bg-pink-100 rounded size-9 hover:bg-pink-200 dark:bg-pink-500/20 dark:hover:bg-pink-500/30"
						>
							<LucideIcon name="Instagram" class="size-4" />
						</a>
						<a
							href="#!"
							class="flex items-center justify-center text-red-500 transition-all duration-200 ease-linear bg-red-100 rounded size-9 hover:bg-red-200 dark:bg-red-500/20 dark:hover:bg-red-500/30"
						>
							<LucideIcon name="Globe" class="size-4" />
						</a>
						<a
							href="#!"
							class="flex items-center justify-center transition-all duration-200 ease-linear rounded text-custom-500 bg-custom-100 size-9 hover:bg-custom-200 dark:bg-custom-500/20 dark:hover:bg-custom-500/30"
						>
							<LucideIcon name="Linkedin" class="size-4" />
						</a>
						<a
							href="#!"
							class="flex items-center justify-center text-pink-500 transition-all duration-200 ease-linear bg-pink-100 rounded size-9 hover:bg-pink-200 dark:bg-pink-500/20 dark:hover:bg-pink-500/30"
						>
							<LucideIcon name="Dribbble" class="size-4" />
						</a>
						<a
							href="#!"
							class="flex items-center justify-center transition-all duration-200 ease-linear rounded size-9 text-slate-500 bg-slate-100 hover:bg-slate-200 dark:bg-zink-600 dark:hover:bg-zink-500"
						>
							<LucideIcon name="Github" class="size-4" />
						</a>
					</div>
				</div>
				<div class="lg:col-span-12 2xl:col-span-2">
					<div class="flex gap-2 2xl:justify-end">
						<a
							href="mailto:themesdesign@gmail.com"
							class="flex items-center justify-center size-[37.5px] p-0 text-slate-500 btn bg-slate-100 hover:text-white hover:bg-slate-600 focus:text-white focus:bg-slate-600 focus:ring focus:ring-slate-100 active:text-white active:bg-slate-600 active:ring active:ring-slate-100 dark:bg-slate-500/20 dark:text-slate-400 dark:hover:bg-slate-500 dark:hover:text-white dark:focus:bg-slate-500 dark:focus:text-white dark:active:bg-slate-500 dark:active:text-white dark:ring-slate-400/20"
						>
							<LucideIcon name="Mail" class="size-4" />
						</a>
						<button
							type="button"
							class="text-white transition-all duration-200 ease-linear btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
							>Hire Us</button
						>
						<Dropdown className="relative" direction="bottom-start">
							<DropdownToggle
								className="flex items-center justify-center size-[37.5px] dropdown-toggle p-0 text-slate-500 btn bg-slate-100 hover:text-white hover:bg-slate-600 focus:text-white focus:bg-slate-600 focus:ring focus:ring-slate-100 active:text-white active:bg-slate-600 active:ring active:ring-slate-100 dark:bg-slate-500/20 dark:text-slate-400 dark:hover:bg-slate-500 dark:hover:text-white dark:focus:bg-slate-500 dark:focus:text-white dark:active:bg-slate-500 dark:active:text-white dark:ring-slate-400/20"
							>
								<LucideIcon name="MoreHorizontal" class="size-4" />
							</DropdownToggle>
							<DropdownMenu
								tag="ul"
								class="absolute z-50 py-2 mt-1 ltr:text-left rtl:text-right list-none bg-white rounded-md shadow-md dropdown-menu min-w-[10rem] dark:bg-zink-600"
							>
								<li class="px-3 mb-2 text-sm text-slate-500">Payments</li>
								<li>
									<a
										class="block px-4 py-1.5 text-base font-medium transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!">Create Invoice</a
									>
								</li>
								<li>
									<a
										class="block px-4 py-1.5 text-base font-medium transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!">Pending Billing</a
									>
								</li>
								<li>
									<a
										class="block px-4 py-1.5 text-base font-medium transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!">Genarate Bill</a
									>
								</li>
								<li>
									<a
										class="block px-4 py-1.5 text-base font-medium transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!">Subscription</a
									>
								</li>
							</DropdownMenu>
						</Dropdown>
					</div>
				</div>
			</div>
			<!--end grid-->
		</div>
		<div class="card-body !py-0">
			<ul class="flex flex-wrap w-full text-sm font-medium text-center nav-tabs">
				<li class="group {tabId == 'personal' ? 'active' : ''}">
					<a
						href={'javascript:void(0);'}
						on:click={() => (tabId = 'personal')}
						class="inline-block px-4 py-2 text-base transition-all duration-300 ease-linear rounded-t-md
						text-slate-500 dark:text-zink-200 border-b border-transparent group-[.active]:text-custom-500
						dark:group-[.active]:text-custom-500 group-[.active]:border-b-custom-500 hover:text-custom-500
						dark:hover:text-custom-500 active:text-custom-500 dark:active:text-custom-500
						-mb-[1px]">Personal Info</a
					>
				</li>
				<li class="group {tabId == 'integration' ? 'active' : ''}">
					<a
						href={'javascript:void(0);'}
						on:click={() => (tabId = 'integration')}
						class="inline-block px-4 py-2 text-base transition-all duration-300 ease-linear rounded-t-md
						text-slate-500 dark:text-zink-200 border-b border-transparent group-[.active]:text-custom-500
						dark:group-[.active]:text-custom-500 group-[.active]:border-b-custom-500 hover:text-custom-500
						dark:hover:text-custom-500 active:text-custom-500 dark:active:text-custom-500
						-mb-[1px]">Integration</a
					>
				</li>
				<li class="group {tabId == 'changePassword' ? 'active' : ''}">
					<a
						href={'javascript:void(0);'}
						on:click={() => (tabId = 'changePassword')}
						class="inline-block px-4 py-2 text-base transition-all duration-300 ease-linear rounded-t-md
						text-slate-500 dark:text-zink-200 border-b border-transparent group-[.active]:text-custom-500
						dark:group-[.active]:text-custom-500 group-[.active]:border-b-custom-500 hover:text-custom-500
						dark:hover:text-custom-500 active:text-custom-500 dark:active:text-custom-500 -mb-[1px]"
						>Change Password</a
					>
				</li>
				<li class="group {tabId == 'privacyPolicy' ? 'active' : ''}">
					<a
						href={'javascript:void(0);'}
						on:click={() => (tabId = 'privacyPolicy')}
						class="inline-block px-4 py-2 text-base transition-all duration-300 ease-linear rounded-t-md
						text-slate-500 dark:text-zink-200 border-b border-transparent group-[.active]:text-custom-500
						dark:group-[.active]:text-custom-500 group-[.active]:border-b-custom-500 hover:text-custom-500
						dark:hover:text-custom-500 active:text-custom-500 dark:active:text-custom-500 -mb-[1px]"
						>Privacy Policy</a
					>
				</li>
			</ul>
		</div>
	</div>

	<div class="tab-content">
		<div class="tab-pane {tabId == 'personal' ? 'block' : 'hidden'}" id="overviewTabs">
			<div class="card">
				<div class="card-body">
					<h6 class="mb-1 text-15">Personal Information</h6>
					<p class="mb-4 text-slate-500 dark:text-zink-200">
						Update your photo and personal details here easily.
					</p>
					<form action="#!">
						<div class="grid grid-cols-1 gap-5 xl:grid-cols-12">
							<div class="xl:col-span-6">
								<label for="inputValue" class="inline-block mb-2 text-base font-medium"
									>First Name</label
								>
								<input
									type="text"
									id="inputValue"
									class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
									placeholder="Enter your value"
									value="Paula"
								/>
							</div>
							<!--end col-->
							<div class="xl:col-span-6">
								<label for="inputValue" class="inline-block mb-2 text-base font-medium"
									>Last Name</label
								>
								<input
									type="text"
									id="inputValue"
									class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
									placeholder="Enter your value"
									value="Keenan"
								/>
							</div>
							<!--end col-->
							<div class="xl:col-span-6">
								<label for="inputValue" class="inline-block mb-2 text-base font-medium"
									>Phone Number</label
								>
								<input
									type="text"
									id="inputValue"
									class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
									placeholder="+214 8456 8459 23"
									value="+214 8456 8459 23"
								/>
							</div>
							<!--end col-->
							<div class="xl:col-span-6">
								<label for="inputValue" class="inline-block mb-2 text-base font-medium"
									>Email Address</label
								>
								<input
									type="email"
									id="inputValue"
									class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
									placeholder="Enter your email address"
									value="richardmarshall@tailwick.com"
								/>
							</div>
							<!--end col-->
							<div class="xl:col-span-6">
								<label for="joiningDateInput" class="inline-block mb-2 text-base font-medium"
									>Birth of Date</label
								>
								<Flatpickr
									options={{ dateFormat: 'd M, Y' }}
									class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
									name="date"
									placeholder="Select date"
								/>
							</div>
							<!--end col-->
							<div class="xl:col-span-6">
								<label for="joiningDateInput" class="inline-block mb-2 text-base font-medium"
									>Joining Date</label
								>
								<Flatpickr
									options={{ dateFormat: 'd M, Y' }}
									class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
									name="date"
									placeholder="Select date"
								/>
							</div>
							<!--end col-->
							<div class="xl:col-span-12">
								<label for="inputValue" class="inline-block mb-2 text-base font-medium">Skill</label
								>
								<select
									class="form-select border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
									id="choices-multiple-default"
								>
									<option value="Choice 1" selected>Choice 1</option>
									<option value="Choice 2">Choice 2</option>
									<option value="Choice 3">Choice 3</option>
									<option value="Choice 2">Choice 4</option>
									<option value="Choice 3">Choice 5 </option>
									<option value="Choice 4" disabled>Choice 4</option>
								</select>
							</div>
							<!--end col-->
							<div class="xl:col-span-6">
								<label for="inputValue" class="inline-block mb-2 text-base font-medium"
									>Designation</label
								>
								<input
									type="text"
									id="inputValue"
									class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
									placeholder="Enter your value"
									value="CEO & Founder"
								/>
							</div>
							<!--end col-->
							<div class="xl:col-span-6">
								<label for="inputValue" class="inline-block mb-2 text-base font-medium"
									>Website</label
								>
								<input
									type="text"
									id="inputValue"
									class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
									placeholder="Enter your value"
									value="www.themesdesign.in"
								/>
							</div>
							<!--end col-->
							<div class="xl:col-span-4">
								<label for="inputValue" class="inline-block mb-2 text-base font-medium">City</label>
								<select
									class="form-select border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
									id="choices-single-no-sorting"
									name="choices-single-no-sorting"
									data-choices
									data-choices-sorting-false
								>
									<option value="Madrid">Madrid</option>
									<option value="Toronto">Toronto</option>
									<option value="Vancouver">Vancouver</option>
									<option value="London">London</option>
									<option value="Manchester">Manchester</option>
									<option value="Liverpool">Liverpool</option>
									<option value="Paris">Paris</option>
									<option value="Malaga">Malaga</option>
									<option value="Washington" disabled>Washington</option>
									<option value="Lyon">Lyon</option>
									<option value="Marseille">Marseille</option>
									<option value="Hamburg">Hamburg</option>
									<option value="Munich">Munich</option>
									<option value="Barcelona">Barcelona</option>
									<option value="Berlin">Berlin</option>
									<option value="Montreal">Montreal</option>
									<option value="New York">New York</option>
									<option value="Michigan">Michigan</option>
								</select>
							</div>
							<!--end col-->
							<div class="xl:col-span-4">
								<label for="inputValue" class="inline-block mb-2 text-base font-medium"
									>Country</label
								>
								<select
									id="choices-single-no-sorting"
									name="choices-single-no-sorting"
									class="form-select border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
								>
									<option value="Madrid">USA</option>
									<option value="Toronto">Toronto</option>
									<option value="Vancouver">Vancouver</option>
									<option value="London">London</option>
									<option value="Manchester">Manchester</option>
									<option value="Liverpool">Liverpool</option>
									<option value="Paris">Paris</option>
									<option value="Malaga">Malaga</option>
									<option value="Washington" disabled>Washington</option>
									<option value="Lyon">Lyon</option>
									<option value="Marseille">Marseille</option>
									<option value="Hamburg">Hamburg</option>
									<option value="Munich">Munich</option>
									<option value="Barcelona">Barcelona</option>
									<option value="Berlin">Berlin</option>
									<option value="Montreal">Montreal</option>
									<option value="New York">New York</option>
									<option value="Michigan">Michigan</option>
								</select>
							</div>
							<!--end col-->
							<div class="xl:col-span-4">
								<label for="inputValue" class="inline-block mb-2 text-base font-medium"
									>Zip Code</label
								>
								<select
									id="choices-single-no-sorting"
									name="choices-single-no-sorting"
									class="form-select border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
								>
									<option value="Madrid">00012</option>
									<option value="Toronto">00014</option>
									<option value="Vancouver">00016</option>
									<option value="London">88800</option>
									<option value="Manchester">00100</option>
									<option value="Liverpool">00001</option>
								</select>
							</div>
							<!--end col-->
							<div class="xl:col-span-12">
								<label for="inputValue" class="block mb-2 text-base font-medium">Description</label>
								<textarea
									class="w-full form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
									id="exampleFormControlTextarea"
									placeholder="Enter your description"
									rows="5"
									>A Web Developer creates and designs different websites for clients. They are
									responsible for their aesthetic as well as their function. Professionals in this
									field may also need to be able to ensure sites are compatible with multiple types
									of media. Web Developers need to have a firm understanding of programming and
									graphical design. Having a strong resume that emphasizes these attributes makes it
									significantly easier to get hired as a Web Developer. As a web designer, my
									objective is to make a positive impact on clients, co-workers, and the Internet
									using my skills and experience to design compelling and attractive websites.
									Solving code problems. Editing &amp; Design with designing team in the company to
									build perfect web designs.</textarea
								>
							</div>
							<!--end col-->
						</div>
						<!--end grid-->
						<div class="flex justify-end mt-6 gap-x-4">
							<button
								type="button"
								class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
								>Updates</button
							>
							<button
								type="button"
								class="text-red-500 bg-red-100 btn hover:text-white hover:bg-red-600 focus:text-white focus:bg-red-600 focus:ring focus:ring-red-100 active:text-white active:bg-red-600 active:ring active:ring-red-100 dark:bg-red-500/20 dark:text-red-500 dark:hover:bg-red-500 dark:hover:text-white dark:focus:bg-red-500 dark:focus:text-white dark:active:bg-red-500 dark:active:text-white dark:ring-red-400/20"
								>Cancel</button
							>
						</div>
					</form>
					<!--end form-->
				</div>
			</div>

			<div class="grid grid-cols-1 gap-x-5 lg:grid-cols-2 2xl:grid-cols-3">
				<div class="card">
					<div class="card-body">
						<h6 class="mb-3 text-15">Payment History</h6>
						<div class="overflow-x-auto">
							<table class="w-full whitespace-nowrap">
								<thead class="ltr:text-left rtl:text-right">
									<tr>
										<th
											class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500"
										>
											Date</th
										>
										<th
											class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500"
										>
											Amount</th
										>
										<th
											class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500"
										>
											Status</th
										>
										<th
											class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500"
										>
											Description</th
										>
										<th
											class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500"
										>
											PDF</th
										>
									</tr>
								</thead>
								<tbody>
									<tr>
										<td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"
											>2 April, 2023</td
										>
										<td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"
											>$321.00
										</td>
										<td
											class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500 text-green-500"
										>
											Paid</td
										>
										<td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">
											Subscription</td
										>
										<td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"
											><a
												href="#!"
												class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"
												><i class="ri-download-2-line" /></a
											></td
										>
									</tr>
									<tr>
										<td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"
											>8 June, 2023</td
										>
										<td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"
											>$184.00
										</td>
										<td
											class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500 text-yellow-500"
										>
											Pending</td
										>
										<td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"
											>April Autofile</td
										>
										<td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"
											><a
												href="#!"
												class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"
												><i class="ri-download-2-line" /></a
											></td
										>
									</tr>
									<tr>
										<td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"
											>15 July, 2023</td
										>
										<td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"
											>$325.00
										</td>
										<td
											class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500 text-yellow-500"
										>
											Pending</td
										>
										<td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">
											Subscription</td
										>
										<td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"
											><a
												href="#!"
												class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"
												><i class="ri-download-2-line" /></a
											></td
										>
									</tr>
									<tr>
										<td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"
											>29 July, 2023</td
										>
										<td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"
											>$321.00
										</td>
										<td
											class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500 text-green-500"
										>
											Paid</td
										>
										<td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"
											>Prepaid Bill</td
										>
										<td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"
											><a
												href="#!"
												class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"
												><i class="ri-download-2-line" /></a
											></td
										>
									</tr>
									<tr>
										<td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"
											>12 Aug, 2023</td
										>
										<td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"
											>$189.99
										</td>
										<td
											class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500 text-green-500"
										>
											Paid</td
										>
										<td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500">
											Subscription</td
										>
										<td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"
											><a
												href="#!"
												class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"
												><i class="ri-download-2-line" /></a
											></td
										>
									</tr>
								</tbody>
							</table>
						</div>
					</div>
				</div>
				<!--end card-->
				<div class="card">
					<div class="card-body">
						<h6 class="mb-3 text-15">Card Overview</h6>

						<div class="flex flex-col gap-3">
							<a
								href="#!"
								class="flex items-center gap-3 p-2 border rounded-md border-slate-200 dark:border-zink-500"
							>
								<div class="flex items-center justify-center size-12 rounded-md shrink-0">
									<img src="/assets/images/payment/img-01.png" alt="" class="h-10" />
								</div>
								<div class="grow">
									<h5 class="mb-1 text-15">Mastercard</h5>
									<p class="text-sm text-slate-500 dark:text-zink-200">XXXX XXXX XXXX 1501</p>
								</div>
								<div>
									<span
										class="px-2.5 py-0.5 inline-block text-xs font-medium rounded border bg-sky-100 border-transparent text-sky-500 dark:bg-sky-500/20 dark:border-transparent"
										>Primary</span
									>
								</div>
							</a>
							<a
								href="#!"
								class="flex items-center gap-3 p-2 border rounded-md border-slate-200 dark:border-zink-500"
							>
								<div class="flex items-center justify-center size-12 rounded-md shrink-0">
									<img src="/assets/images/payment/img-02.png" alt="" class="h-10" />
								</div>
								<div class="grow">
									<h5 class="mb-1 text-15">Visa Card</h5>
									<p class="text-sm text-slate-500 dark:text-zink-200">XXXX XXXX XXXX 9871</p>
								</div>
							</a>
							<a
								href="#!"
								class="flex items-center gap-3 p-2 border rounded-md border-slate-200 dark:border-zink-500"
							>
								<div class="flex items-center justify-center size-12 rounded-md shrink-0">
									<img src="/assets/images/payment/img-03.png" alt="" class="h-10" />
								</div>
								<div class="grow">
									<h5 class="mb-1 text-15">Discover Network</h5>
									<p class="text-sm text-slate-500 dark:text-zink-200">XXXX XXXX XXXX 7435</p>
								</div>
							</a>
						</div>
						<div class="mt-3 text-center">
							<a href="#!" class="inline-block underline text-custom-500"
								>Add New Card
								<LucideIcon name="MoveRight" class="inline-block size-4 ml-2 rtl:-rotate-180" />
							</a>
						</div>
					</div>
				</div>
				<!--end card-->
				<div class="card">
					<div class="card-body">
						<div class="flex items-center mb-3">
							<h6 class="grow text-15">Plan Details</h6>
							<a href="#!" class="underline text-custom-500">Change Plan</a>
						</div>
						<div class="grid grid-cols-1 gap-5 mb-3 md:grid-cols-3">
							<div>
								<p class="mb-1 text-slate-500 dark:text-zink-200">Plan Type</p>
								<h6>Regular Plan</h6>
							</div>
							<!--end col-->
							<div>
								<p class="mb-1 text-slate-500 dark:text-zink-200">Monthly Limit</p>
								<h6>2 Download</h6>
							</div>
							<!--end col-->
							<div>
								<p class="mb-1 text-slate-500 dark:text-zink-200">Cost</p>
								<h6>$26/Monthly</h6>
							</div>
							<!--end col-->
						</div>
						<!--end grid-->
						<div
							class="px-4 py-3 mb-2 text-sm rounded-md text-slate-500 dark:text-zink-200 bg-slate-50 dark:bg-zink-600"
						>
							<h6 class="mb-1">Our discounted pricing is available until August 16, 2023.</h6>
							<p>
								After this date, our pricing will transition to $22. For more details, click <a
									href="#!"
									class="font-semibold text-slate-700 dark:text-zink-50">Learn More</a
								>
							</p>
						</div>
						<p class="mb-2 text-slate-500 dark:text-zink-200">Plan Benefits</p>
						<ul class="flex flex-col gap-1 list-disc list-inside">
							<li>Build with TailwindCSS & Vite</li>
							<li>Multiple Layouts</li>
							<li>Easy to Customize</li>
						</ul>
					</div>
				</div>
				<!--end card-->
			</div>
			<!--end grid-->
		</div>
		<!--end tab pane-->
		<div class="tab-pane {tabId == 'integration' ? 'block' : 'hidden'}" id="documentsTabs">
			<h6 class="mb-4 text-15">Connected Apps</h6>
			<div class="grid grid-cols-1 gap-x-5 sm:grid-cols-2 xl:grid-cols-4">
				<div class="card">
					<div class="card-body">
						<div class="flex gap-3 mb-5">
							<div class="grow">
								<img src="/assets/images/brand/meta.png" alt="" class="h-8" />
							</div>
							<div class="shrink-0">
								<button
									type="button"
									class="px-2 py-1.5 text-xs bg-white border-dashed text-custom-500 btn border-custom-500 hover:text-custom-500 hover:bg-custom-50 hover:border-custom-600 focus:text-custom-600 focus:bg-custom-50 focus:border-custom-600 active:text-custom-600 active:bg-custom-50 active:border-custom-600 dark:bg-zink-700 dark:ring-custom-400/20 dark:hover:bg-custom-800/20 dark:focus:bg-custom-800/20 dark:active:bg-custom-800/20"
								>
									<LucideIcon name="Cable" class="inline-block size-3 ltr:mr-1 rtl:ml-1" /> Connect
								</button>
							</div>
						</div>

						<h6 class="mb-2 text-16">Meta</h6>
						<p class="text-slate-500 dark:text-zink-200">
							Meta Platforms Inc, which used to be called Facebook Inc.
						</p>
					</div>
				</div>
				<!--end col-->
				<div class="card">
					<div class="card-body">
						<div class="flex gap-3 mb-5">
							<div class="grow">
								<img src="/assets/images/brand/figma.png" alt="" class="h-8" />
							</div>
							<div class="shrink-0">
								<button
									type="button"
									class="px-2 py-1.5 text-xs bg-white border-dashed text-custom-500 btn border-custom-500 hover:text-custom-500 hover:bg-custom-50 hover:border-custom-600 focus:text-custom-600 focus:bg-custom-50 focus:border-custom-600 active:text-custom-600 active:bg-custom-50 active:border-custom-600 dark:bg-zink-700 dark:ring-custom-400/20 dark:hover:bg-custom-800/20 dark:focus:bg-custom-800/20 dark:active:bg-custom-800/20"
								>
									<LucideIcon name="Cable" class="inline-block size-3 ltr:mr-1 rtl:ml-1" /> Connect
								</button>
							</div>
						</div>

						<h6 class="mb-2 text-16">Figma Tools</h6>
						<p class="text-slate-500 dark:text-zink-200">
							Figma is a cloud-based design tool that is similar to Sketch in functionality and
							features.
						</p>
					</div>
				</div>
				<!--end col-->
				<div class="card">
					<div class="card-body">
						<div class="flex gap-3 mb-5">
							<div class="grow">
								<img src="/assets/images/brand/telegram.png" alt="" class="h-8" />
							</div>
							<div class="shrink-0">
								<button
									type="button"
									class="px-2 py-1.5 text-xs bg-white border-dashed text-custom-500 btn border-custom-500 hover:text-custom-500 hover:bg-custom-50 hover:border-custom-600 focus:text-custom-600 focus:bg-custom-50 focus:border-custom-600 active:text-custom-600 active:bg-custom-50 active:border-custom-600 dark:bg-zink-700 dark:ring-custom-400/20 dark:hover:bg-custom-800/20 dark:focus:bg-custom-800/20 dark:active:bg-custom-800/20"
								>
									<LucideIcon name="Cable" class="inline-block size-3 ltr:mr-1 rtl:ml-1" /> Connect
								</button>
							</div>
						</div>

						<h6 class="mb-2 text-16">Telegram Channel</h6>
						<p class="text-slate-500 dark:text-zink-200">
							Telegram Tools offers multiple useful tools for Telegram users.
						</p>
					</div>
				</div>
				<!--end col-->
			</div>
		</div>
		<!--end tab pane-->
		<div class="tab-pane {tabId == 'changePassword' ? 'block' : 'hidden'}" id="projectsTabs">
			<div class="card">
				<div class="card-body">
					<h6 class="mb-4 text-15">Changes Password</h6>
					<form action="#!">
						<div class="grid grid-cols-1 gap-5 xl:grid-cols-12">
							<div class="xl:col-span-4">
								<label for="inputValue" class="inline-block mb-2 text-base font-medium"
									>Old Password*</label
								>
								<div class="relative">
									<input
										type="password"
										class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
										id="oldpasswordInput"
										placeholder="Enter current password"
									/>
									<button class="absolute top-2 ltr:right-4 rtl:left-4" type="button"
										><i class="align-middle ri-eye-fill text-slate-500 dark:text-zink-200"
										></i></button
									>
								</div>
							</div>
							<!--end col-->
							<div class="xl:col-span-4">
								<label for="inputValue" class="inline-block mb-2 text-base font-medium"
									>New Password*</label
								>
								<div class="relative">
									<input
										type="password"
										class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
										id="oldpasswordInput"
										placeholder="Enter new password"
									/>
									<button class="absolute top-2 ltr:right-4 rtl:left-4" type="button"
										><i class="align-middle ri-eye-fill text-slate-500 dark:text-zink-200"
										></i></button
									>
								</div>
							</div>
							<!--end col-->
							<div class="xl:col-span-4">
								<label for="inputValue" class="inline-block mb-2 text-base font-medium"
									>Confirm Password*</label
								>
								<div class="relative">
									<input
										type="password"
										class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
										id="oldpasswordInput"
										placeholder="Confirm password"
									/>
									<button class="absolute top-2 ltr:right-4 rtl:left-4" type="button"
										><i class="align-middle ri-eye-fill text-slate-500 dark:text-zink-200"
										></i></button
									>
								</div>
							</div>
							<!--end col-->
							<div class="flex items-center xl:col-span-6">
								<a href={'javascript:void(0);'} class="underline text-custom-500 text-13"
									>Forgot Password ?</a
								>
							</div>
							<div class="flex justify-end xl:col-span-6">
								<button
									type="button"
									class="text-white bg-green-500 border-green-500 btn hover:text-white hover:bg-green-600 hover:border-green-600 focus:text-white focus:bg-green-600 focus:border-green-600 focus:ring focus:ring-green-100 active:text-white active:bg-green-600 active:border-green-600 active:ring active:ring-green-100 dark:ring-green-400/10"
									>Change Password</button
								>
							</div>
						</div>
						<!--end grid-->
					</form>
				</div>
			</div>
		</div>
		<!--end tab pane-->
		<div class="tab-pane {tabId == 'privacyPolicy' ? 'block' : 'hidden'}" id="followersTabs">
			<div class="card">
				<div class="card-body">
					<h6 class="mb-4 text-15">Security & Privacy</h6>
					<div class="space-y-6">
						<div class="flex flex-col justify-between gap-2 md:flex-row">
							<div>
								<h4 class="text-15">Two-factor Authentication</h4>
								<p class="mt-1 text-slate-500 dark:text-zink-200">
									Two-factor authentication is an enhanced security. Once enabled, you'll be
									required to give two types of identification when you log into Google
									Authentication and SMS are Supported.
								</p>
							</div>
							<div class="shrink-0">
								<button
									type="button"
									class="py-1 text-xs px-1.5 text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
									>Enable Two-factor Authentication</button
								>
							</div>
						</div>
						<div class="flex flex-col justify-between gap-2 md:flex-row">
							<div>
								<h4 class="text-15">Secondary Verification</h4>
								<p class="mt-1 text-slate-500 dark:text-zink-200">
									The first factor is a password and the second commonly includes a text with a code
									sent to your smartphone, or biometrics using your fingerprint, face, or retina.
								</p>
							</div>
							<div class="shrink-0">
								<button
									type="button"
									class="py-1 text-xs px-1.5 text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
									>Set up secondary method</button
								>
							</div>
						</div>
						<div class="flex flex-col justify-between gap-2 md:flex-row">
							<div>
								<h4 class="text-15">Backup Codes</h4>
								<p class="mt-1 text-slate-500 dark:text-zink-200">
									A backup code is automatically generated for you when you turn on two-factor
									authentication through your iOS or Android Twitter app. You can also generate a
									backup code on twitter.com.
								</p>
							</div>
							<div class="shrink-0">
								<button
									type="button"
									class="py-1 text-xs px-1.5 text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
									>Generate backup codes</button
								>
							</div>
						</div>
					</div>
					<h6 class="inline-block mt-6 mb-4 underline text-15">Application Notifications:</h6>
					<div class="space-y-6">
						<div class="flex justify-between gap-2">
							<div>
								<h4 class="text-15">Direct messages</h4>
								<p class="mt-1 text-slate-500 dark:text-zink-200">
									Messages from people you follow
								</p>
							</div>
							<div class="shrink-0">
								<div
									class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2"
								>
									<input
										type="checkbox"
										name="directMessage"
										id="directMessage"
										class="absolute block size-5 transition duration-300 ease-linear border-2 rounded-full appearance-none cursor-pointer border-slate-200 dark:border-zink-600 bg-white/80 dark:bg-zink-400 peer/published checked:bg-custom-500 dark:checked:bg-custom-500 ltr:checked:right-0 rtl:checked:left-0 checked:border-custom-100 dark:checked:border-custom-900 arrow-none checked:bg-none"
										checked
									/>
									<label
										for="directMessage"
										class="block h-5 overflow-hidden duration-300 ease-linear border rounded-full cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-custom-100 dark:peer-checked/published:bg-custom-900 peer-checked/published:border-custom-100 dark:peer-checked/published:border-custom-900"
									></label>
								</div>
							</div>
						</div>
						<div class="flex justify-between gap-2">
							<div>
								<h4 class="text-15">Show email notifications</h4>
								<p class="mt-1 text-slate-500 dark:text-zink-200">
									Under Settings, choose Notifications. Under Select an account, choose the account
									to enable notifications for.
								</p>
							</div>
							<div class="shrink-0">
								<div
									class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2"
								>
									<input
										type="checkbox"
										name="emailNotification"
										id="emailNotification"
										class="absolute block size-5 transition duration-300 ease-linear border-2 rounded-full appearance-none cursor-pointer border-slate-200 dark:border-zink-600 bg-white/80 dark:bg-zink-400 peer/published checked:bg-custom-500 dark:checked:bg-custom-500 ltr:checked:right-0 rtl:checked:left-0 checked:border-custom-100 dark:checked:border-custom-900 arrow-none checked:bg-none"
										checked
									/>
									<label
										for="emailNotification"
										class="block h-5 overflow-hidden duration-300 ease-linear border rounded-full cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-custom-100 dark:peer-checked/published:bg-custom-900 peer-checked/published:border-custom-100 dark:peer-checked/published:border-custom-900"
									></label>
								</div>
							</div>
						</div>
						<div class="flex justify-between gap-2">
							<div>
								<h4 class="text-15">Show chat notifications</h4>
								<p class="mt-1 text-slate-500 dark:text-zink-200">
									Messages from people you follow
								</p>
							</div>
							<div class="shrink-0">
								<div
									class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2"
								>
									<input
										type="checkbox"
										name="chatNotification"
										id="chatNotification"
										class="absolute block size-5 transition duration-300 ease-linear border-2 rounded-full appearance-none cursor-pointer border-slate-200 dark:border-zink-600 bg-white/80 dark:bg-zink-400 peer/published checked:bg-custom-500 dark:checked:bg-custom-500 ltr:checked:right-0 rtl:checked:left-0 checked:border-custom-100 dark:checked:border-custom-900 arrow-none checked:bg-none"
										checked
									/>
									<label
										for="chatNotification"
										class="block h-5 overflow-hidden duration-300 ease-linear border rounded-full cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-custom-100 dark:peer-checked/published:bg-custom-900 peer-checked/published:border-custom-100 dark:peer-checked/published:border-custom-900"
									></label>
								</div>
							</div>
						</div>
						<div class="flex justify-between gap-2">
							<div>
								<h4 class="text-15">Show purchase notifications</h4>
								<p class="mt-1 text-slate-500 dark:text-zink-200">
									Get real-time purchase alerts to protect yourself from fraudulent charges.
								</p>
							</div>
							<div class="shrink-0">
								<div
									class="relative inline-block w-10 align-middle transition duration-200 ease-in ltr:mr-2 rtl:ml-2"
								>
									<input
										type="checkbox"
										name="showPurchase"
										id="showPurchase"
										class="absolute block size-5 transition duration-300 ease-linear border-2 rounded-full appearance-none cursor-pointer border-slate-200 dark:border-zink-600 bg-white/80 dark:bg-zink-400 peer/published checked:bg-custom-500 dark:checked:bg-custom-500 ltr:checked:right-0 rtl:checked:left-0 checked:border-custom-100 dark:checked:border-custom-900 arrow-none checked:bg-none"
										checked
									/>
									<label
										for="customSoftSwitch"
										class="block h-5 overflow-hidden duration-300 ease-linear border rounded-full cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-custom-100 dark:peer-checked/published:bg-custom-900 peer-checked/published:border-custom-100 dark:peer-checked/published:border-custom-900"
									></label>
								</div>
							</div>
						</div>
					</div>
					<h6 class="inline-block mt-6 mb-4 underline text-15">Delete This Account:</h6>
					<p class="mt-1 text-slate-500 dark:text-zink-200">
						Go to the Data & Privacy section of your profile Account. Scroll to "Your data & privacy
						options." Delete your Profile Account. Follow the instructions to delete your account :
					</p>
					<form action="">
						<div class="max-w-xs mt-4">
							<div>
								<input
									type="password"
									class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
									id="oldpasswordInput"
									placeholder="Enter password"
								/>
							</div>
							<div class="flex mt-4 gap-x-2">
								<button
									type="button"
									class="text-red-500 bg-red-100 btn hover:text-white hover:bg-red-600 focus:text-white focus:bg-red-600 focus:ring focus:ring-red-100 active:text-white active:bg-red-600 active:ring active:ring-red-100 dark:bg-red-500/20 dark:text-red-500 dark:hover:bg-red-500 dark:hover:text-white dark:focus:bg-red-500 dark:focus:text-white dark:active:bg-red-500 dark:active:text-white dark:ring-red-400/20"
									>Close & delete this Account</button
								>
								<button
									type="button"
									class="text-slate-500 btn bg-slate-200 border-slate-200 hover:text-slate-600 hover:bg-slate-300 hover:border-slate-300 focus:text-slate-600 focus:bg-slate-300 focus:border-slate-300 focus:ring focus:ring-slate-100 active:text-slate-600 active:bg-slate-300 active:border-slate-300 active:ring active:ring-slate-100 dark:bg-zink-600 dark:hover:bg-zink-500 dark:border-zink-600 dark:hover:border-zink-500 dark:text-zink-200 dark:ring-zink-400/50"
									>Cancel</button
								>
							</div>
						</div>
						<!--end col-->
					</form>
				</div>
			</div>
		</div>
		<!--end tab pane-->
	</div>
</div>
