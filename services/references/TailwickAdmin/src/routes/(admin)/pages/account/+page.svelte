<script>
	import Dropdown from '../../../../common/components/Dropdown.svelte';
	import DropdownToggle from '../../../../common/components/DropdownToggle.svelte';
	import DropdownMenu from '../../../../common/components/DropdownMenu.svelte';
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import LucideIcon from '../../../../common/components/LucideIcon.svelte';
	import RecentStatisticsChart from './RecentStatisticsChart.svelte';
	let tabId = 'overview';
</script>

<HeadTitle title="Account" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<div class="mt-1 -ml-3 -mr-3 rounded-none card">
		<div class="card-body !px-2.5">
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
						Paula Keenan <LucideIcon
							name="BadgeCheck"
							class="inline-block size-4 text-sky-500 fill-sky-100 dark:fill-custom-500/20"
						/>
					</h5>
					<div class="flex gap-3 mb-4">
						<p class="text-slate-500 dark:text-zink-200">
							<LucideIcon
								name="UserCircle"
								class="inline-block size-4 ltr:mr-1 rtl:ml-1 text-slate-500 dark:text-zink-200 fill-slate-100 dark:fill-zink-500"
							/> CEO & Founder
						</p>
						<p class="text-slate-500 dark:text-zink-200">
							<LucideIcon
								name="MapPin"
								class="inline-block size-4 ltr:mr-1 rtl:ml-1 text-slate-500 dark:text-zink-200 fill-slate-100 dark:fill-zink-500"
							/> Los Angeles, California
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
							><LucideIcon name="Mail" class="size-4" /></a
						>
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
		<div class="card-body !px-2.5 !py-0">
			<ul class="flex flex-wrap w-full text-sm font-medium text-center nav-tabs">
				<li class="group {tabId == 'overview' ? 'active' : ''}">
					<a
						href={'javascript:void(0);'}
						class="inline-block px-4 py-2 text-base transition-all duration-300 ease-linear rounded-t-md text-slate-500 dark:text-zink-200 border-b border-transparent group-[.active]:text-custom-500 dark:group-[.active]:text-custom-500 group-[.active]:border-b-custom-500 dark:group-[.active]:border-b-custom-500 hover:text-custom-500 dark:hover:text-custom-500 active:text-custom-500 dark:active:text-custom-500 -mb-[1px]"
						on:click={() => (tabId = 'overview')}>Overview</a
					>
				</li>
				<li class="group {tabId == 'documents' ? 'active' : ''}">
					<a
						href={'javascript:void(0);'}
						class="inline-block px-4 py-2 text-base transition-all duration-300 ease-linear rounded-t-md text-slate-500 dark:text-zink-200 border-b border-transparent group-[.active]:text-custom-500 dark:group-[.active]:text-custom-500 group-[.active]:border-b-custom-500 dark:group-[.active]:border-b-custom-500 hover:text-custom-500 dark:hover:text-custom-500 active:text-custom-500 dark:active:text-custom-500 -mb-[1px]"
						on:click={() => (tabId = 'documents')}>Documents</a
					>
				</li>
				<li class="group {tabId == 'projects' ? 'active' : ''}">
					<a
						href={'javascript:void(0);'}
						class="inline-block px-4 py-2 text-base transition-all duration-300 ease-linear rounded-t-md text-slate-500 dark:text-zink-200 border-b border-transparent group-[.active]:text-custom-500 dark:group-[.active]:text-custom-500 group-[.active]:border-b-custom-500 dark:group-[.active]:border-b-custom-500 hover:text-custom-500 dark:hover:text-custom-500 active:text-custom-500 dark:active:text-custom-500 -mb-[1px]"
						on:click={() => (tabId = 'projects')}>Projects</a
					>
				</li>
				<li class="group {tabId == 'followers' ? 'active' : ''}">
					<a
						href={'javascript:void(0);'}
						class="inline-block px-4 py-2 text-base transition-all duration-300 ease-linear rounded-t-md text-slate-500 dark:text-zink-200 border-b border-transparent group-[.active]:text-custom-500 dark:group-[.active]:text-custom-500 group-[.active]:border-b-custom-500 dark:group-[.active]:border-b-custom-500 hover:text-custom-500 dark:hover:text-custom-500 active:text-custom-500 dark:active:text-custom-500 -mb-[1px]"
						on:click={() => (tabId = 'followers')}>Followers</a
					>
				</li>
			</ul>
		</div>
	</div>

	<div class="tab-content">
		<div class="tab-pane {tabId == 'overview' ? 'block' : 'hidden'}" id="overviewTabs">
			<div class="grid grid-cols-1 gap-x-5 2xl:grid-cols-12">
				<div class="2xl:col-span-9">
					<div class="grid grid-cols-1 gap-x-5 xl:grid-cols-12">
						<div class="xl:col-span-9">
							<div class="card">
								<div class="card-body">
									<h6 class="mb-3 text-15">Recent Statistics</h6>
									<RecentStatisticsChart chartColors={['bg-custom-500', 'bg-purple-500']} />
								</div>
							</div>
						</div>
						<!--end col-->
						<div class="text-center card bg-custom-500 xl:col-span-3">
							<div class="flex flex-col h-full card-body">
								<img src="/assets/images/medal.png" alt="" class="w-2/6 mx-auto" />
								<div class="mt-5 mb-auto">
									<h5 class="mb-1 text-white">Congratulation Paula</h5>
									<p class="text-custom-200">
										on your outstanding achievement! Your hard work and dedication have truly paid
										off.
									</p>
								</div>
								<div class="p-3 mt-5 rounded-md bg-custom-600">
									<h2 class="mb-1 text-white">1054</h2>
									<p class="text-custom-200">It's very easy to convert your points to cash now.</p>
								</div>
							</div>
						</div>
						<!--end col-->
					</div>
					<!--end grid-->
					<div class="card">
						<div class="card-body">
							<h6 class="mb-3 text-15">Overview</h6>
							<p class="mb-2 text-slate-500 dark:text-zink-200">
								A Web Developer creates and designs different websites for clients. They are
								responsible for their aesthetic as well as their function. Professionals in this
								field may also need to be able to ensure sites are compatible with multiple types of
								media. Web Developers need to have a firm understanding of programming and graphical
								design. Having a strong resume that emphasizes these attributes makes it
								significantly easier to get hired as a Web Developer.
							</p>
							<p class="text-slate-500 dark:text-zink-200">
								As a web designer, my objective is to make a positive impact on clients, co-workers,
								and the Internet using my skills and experience to design compelling and attractive
								websites. Solving code problems. Editing & Design with designing team in the company
								to build perfect web designs.
							</p>
						</div>
					</div>
				</div>
				<!--end col-->
				<div class="2xl:col-span-3">
					<div class="card">
						<div class="card-body">
							<h6 class="mb-4 text-15">Personal Information</h6>
							<div class="overflow-x-auto">
								<table class="w-full ltr:text-left rtl:ext-right">
									<tbody>
										<tr>
											<th class="py-2 font-semibold ps-0" scope="row">Designation</th>
											<td class="py-2 text-right text-slate-500 dark:text-zink-200"
												>CEO & Founder</td
											>
										</tr>
										<tr>
											<th class="py-2 font-semibold ps-0" scope="row">Phone No</th>
											<td class="py-2 text-right text-slate-500 dark:text-zink-200">617 219 6245</td
											>
										</tr>
										<tr>
											<th class="py-2 font-semibold ps-0" scope="row">Birth of Date</th>
											<td class="py-2 text-right text-slate-500 dark:text-zink-200">15 Dec, 1998</td
											>
										</tr>
										<tr>
											<th class="py-2 font-semibold ps-0" scope="row">Website</th>
											<td class="py-2 text-right text-slate-500 dark:text-zink-200"
												><a href="http://themesdesign.in/" target="_blank" class="text-custom-500"
													>www.themesdesign.in</a
												></td
											>
										</tr>
										<tr>
											<th class="py-2 font-semibold ps-0" scope="row">Email</th>
											<td class="py-2 text-right text-slate-500 dark:text-zink-200"
												>paula@themesdesign.com</td
											>
										</tr>
										<tr>
											<th class="py-2 font-semibold ps-0" scope="row">Location</th>
											<td class="py-2 text-right text-slate-500 dark:text-zink-200"
												>Los Angeles, California</td
											>
										</tr>
										<tr>
											<th class="pt-2 font-semibold ps-0" scope="row">Joining Date</th>
											<td class="pt-2 text-right text-slate-500 dark:text-zink-200">01 July 2023</td
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
							<h6 class="mb-4 text-15">Earning Reports</h6>

							<div class="divide-y divide-slate-200 dark:divide-zink-500">
								<div class="flex items-center gap-3 pb-3">
									<div
										class="flex items-center justify-center size-12 rounded-full bg-slate-100 dark:bg-zink-600"
									>
										<LucideIcon name="Wallet" class="size-5 text-slate-500 dark:text-zink-200" />
									</div>
									<div>
										<h6 class="text-lg">$245.98</h6>
										<p class="text-slate-500 dark:text-zink-200">Total Earning</p>
									</div>
								</div>
								<div class="flex items-center gap-3 py-3">
									<div
										class="flex items-center justify-center size-12 rounded-full bg-slate-100 dark:bg-zink-600"
									>
										<LucideIcon name="Goal" class="size-5 text-slate-500 dark:text-zink-200" />
									</div>
									<div>
										<h6 class="text-lg">$125.23</h6>
										<p class="text-slate-500 dark:text-zink-200">Items Earning</p>
									</div>
								</div>
								<div class="flex items-center gap-3 pt-3">
									<div
										class="flex items-center justify-center size-12 rounded-full bg-slate-100 dark:bg-zink-600"
									>
										<LucideIcon name="Package" class="size-5 text-slate-500 dark:text-zink-200" />
									</div>
									<div>
										<h6 class="text-lg">$21.49</h6>
										<p class="text-slate-500 dark:text-zink-200">Contributor Bonus</p>
									</div>
								</div>
							</div>
						</div>
					</div>
					<!--end card-->
				</div>
				<!--end col-->
			</div>
			<!--end grid-->

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
											>Date</th
										>
										<th
											class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500"
											>Amount</th
										>
										<th
											class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500"
											>Status</th
										>
										<th
											class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500"
											>Description</th
										>
										<th
											class="px-3.5 py-2.5 font-semibold border-b border-slate-200 dark:border-zink-500"
											>PDF</th
										>
									</tr>
								</thead>
								<tbody>
									<tr>
										<td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"
											>2 April, 2023</td
										>
										<td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"
											>$321.00</td
										>
										<td
											class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500 text-green-500"
											>Paid</td
										>
										<td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"
											>Subscription</td
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
											>$184.00</td
										>
										<td
											class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500 text-yellow-500"
											>Pending</td
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
											>$325.00</td
										>
										<td
											class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500 text-yellow-500"
											>Pending</td
										>
										<td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"
											>Subscription</td
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
											>$321.00</td
										>
										<td
											class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500 text-green-500"
											>Paid</td
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
											>$189.99</td
										>
										<td
											class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500 text-green-500"
											>Paid</td
										>
										<td class="px-3.5 py-2.5 border-y border-slate-200 dark:border-zink-500"
											>Subscription</td
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
								>Add New Card <LucideIcon
									name="MoveRight"
									class="inline-block size-4 ml-2 rtl:-rotate-180"
								/></a
							>
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
		<div class="tab-pane {tabId == 'documents' ? 'block' : 'hidden'}" id="documentsTabs">
			<div class="flex items-center gap-3 mb-4">
				<h5 class="underline grow">Documents</h5>
				<div class="shrink-0">
					<button
						type="button"
						class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
						>Add Document</button
					>
				</div>
			</div>
			<div class="overflow-x-auto">
				<table class="w-full align-middle border-separate whitespace-nowrap border-spacing-y-1">
					<thead class="text-left bg-white dark:bg-zink-700">
						<tr>
							<th class="px-3.5 py-2.5 font-semibold border-b border-transparent">
								<div class="flex items-center h-full">
									<input
										id="Checkbox1"
										class="size-4 bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800"
										type="checkbox"
										value=""
									/>
								</div>
							</th>
							<th class="px-3.5 py-2.5 font-semibold border-b border-transparent">Documents Type</th
							>
							<th class="px-3.5 py-2.5 font-semibold border-b border-transparent">Documents Name</th
							>
							<th class="px-3.5 py-2.5 font-semibold border-b border-transparent">File Size</th>
							<th class="px-3.5 py-2.5 font-semibold border-b border-transparent">Modify Date</th>
							<th class="px-3.5 py-2.5 font-semibold border-b border-transparent">Uploaded</th>
							<th class="px-3.5 py-2.5 font-semibold border-b border-transparent">Status</th>
							<th class="px-3.5 py-2.5 font-semibold border-b border-transparent text-right"
								>Action</th
							>
						</tr>
					</thead>
					<tbody>
						<tr class="bg-white dark:bg-zink-700">
							<td class="px-3.5 py-2.5 border-y border-transparent">
								<div class="flex items-center h-full">
									<input
										id="Checkbox2"
										class="size-4 bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800"
										type="checkbox"
										value=""
									/>
								</div>
							</td>
							<td class="px-3.5 py-2.5 border-y border-transparent">
								<span
									class="px-2.5 py-0.5 inline-block text-xs font-medium rounded border bg-slate-100 border-transparent text-slate-500 dark:bg-slate-500/20 dark:text-zink-200 dark:border-transparent"
									>Docs</span
								>
							</td>
							<td class="px-3.5 py-2.5 border-y border-transparent">Tailwick Docs File</td>
							<td class="px-3.5 py-2.5 border-y border-transparent">2.5MB</td>
							<td class="px-3.5 py-2.5 border-y border-transparent">15 Feb, 2023</td>
							<td class="px-3.5 py-2.5 border-y border-transparent">Admin</td>
							<td class="px-3.5 py-2.5 border-y border-transparent"
								><span
									class="ppx-2.5 py-0.5 inline-block text-xs font-medium rounded border bg-green-100 border-transparent text-green-500 dark:bg-green-500/20 dark:border-transparent"
									>Successful</span
								></td
							>
							<td class="px-3.5 py-2.5 border-y border-transparent">
								<div class="flex items-center justify-end gap-2">
									<a
										href="#!"
										class="flex items-center justify-center size-8 transition-all duration-150 ease-linear rounded-md bg-slate-100 hover:bg-slate-200 dark:bg-zink-600 dark:hover:bg-zink-500"
										><LucideIcon name="Eye" class="size-3" /></a
									>
									<a
										href="#!"
										class="flex items-center justify-center size-8 transition-all duration-150 ease-linear rounded-md bg-slate-100 hover:bg-slate-200 dark:bg-zink-600 dark:hover:bg-zink-500"
										><LucideIcon name="FileEdit" class="size-3" /></a
									>
									<a
										href="#!"
										class="flex items-center justify-center size-8 transition-all duration-150 ease-linear rounded-md bg-slate-100 hover:bg-slate-200 dark:bg-zink-600 dark:hover:bg-zink-500"
										><LucideIcon name="ArrowDownToLine" class="size-3" /></a
									>
									<a
										href="#!"
										class="flex items-center justify-center size-8 transition-all duration-150 ease-linear rounded-md bg-slate-100 hover:bg-slate-200 dark:bg-zink-600 dark:hover:bg-zink-500"
										><LucideIcon name="Trash2" class="size-3" /></a
									>
								</div>
							</td>
						</tr>
						<tr class="bg-white dark:bg-zink-700">
							<td class="px-3.5 py-2.5 border-y border-transparent">
								<div class="flex items-center h-full">
									<input
										id="Checkbox2"
										class="size-4 bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800"
										type="checkbox"
										value=""
									/>
								</div>
							</td>
							<td class="px-3.5 py-2.5 border-y border-transparent">
								<span
									class="px-2.5 py-0.5 inline-block text-xs font-medium rounded border bg-slate-100 border-transparent text-slate-500 dark:bg-slate-500/20 dark:text-zink-200 dark:border-transparent"
									>PSD</span
								>
							</td>
							<td class="px-3.5 py-2.5 border-y border-transparent">Tailwick Design Kit.psd</td>
							<td class="px-3.5 py-2.5 border-y border-transparent">234.87 MB</td>
							<td class="px-3.5 py-2.5 border-y border-transparent">29 Jan, 2023</td>
							<td class="px-3.5 py-2.5 border-y border-transparent">Themesdesign</td>
							<td class="px-3.5 py-2.5 border-y border-transparent"
								><span
									class="ppx-2.5 py-0.5 inline-block text-xs font-medium rounded border bg-green-100 border-transparent text-green-500 dark:bg-green-500/20 dark:border-transparent"
									>Successful</span
								></td
							>
							<td class="px-3.5 py-2.5 border-y border-transparent">
								<div class="flex items-center justify-end gap-2">
									<a
										href="#!"
										class="flex items-center justify-center size-8 transition-all duration-150 ease-linear rounded-md bg-slate-100 hover:bg-slate-200 dark:bg-zink-600 dark:hover:bg-zink-500"
										><LucideIcon name="Eye" class="size-3" /></a
									>
									<a
										href="#!"
										class="flex items-center justify-center size-8 transition-all duration-150 ease-linear rounded-md bg-slate-100 hover:bg-slate-200 dark:bg-zink-600 dark:hover:bg-zink-500"
										><LucideIcon name="FileEdit" class="size-3" /></a
									>
									<a
										href="#!"
										class="flex items-center justify-center size-8 transition-all duration-150 ease-linear rounded-md bg-slate-100 hover:bg-slate-200 dark:bg-zink-600 dark:hover:bg-zink-500"
										><LucideIcon name="ArrowDownToLine" class="size-3" /></a
									>
									<a
										href="#!"
										class="flex items-center justify-center size-8 transition-all duration-150 ease-linear rounded-md bg-slate-100 hover:bg-slate-200 dark:bg-zink-600 dark:hover:bg-zink-500"
										><LucideIcon name="Trash2" class="size-3" /></a
									>
								</div>
							</td>
						</tr>
						<tr class="bg-white dark:bg-zink-700">
							<td class="px-3.5 py-2.5 border-y border-transparent">
								<div class="flex items-center h-full">
									<input
										id="Checkbox2"
										class="size-4 bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800"
										type="checkbox"
										value=""
									/>
								</div>
							</td>
							<td class="px-3.5 py-2.5 border-y border-transparent">
								<span
									class="px-2.5 py-0.5 inline-block text-xs font-medium rounded border bg-slate-100 border-transparent text-slate-500 dark:bg-slate-500/20 dark:text-zink-200 dark:border-transparent"
									>SVG</span
								>
							</td>
							<td class="px-3.5 py-2.5 border-y border-transparent">home Pattern Wave.svg</td>
							<td class="px-3.5 py-2.5 border-y border-transparent">3.87 MB</td>
							<td class="px-3.5 py-2.5 border-y border-transparent">24 Sept, 2023</td>
							<td class="px-3.5 py-2.5 border-y border-transparent">Admin</td>
							<td class="px-3.5 py-2.5 border-y border-transparent"
								><span
									class="px-2.5 py-0.5 inline-block text-xs font-medium rounded border bg-red-100 border-transparent text-red-500 dark:bg-red-500/20 dark:border-transparent"
									>Error</span
								></td
							>
							<td class="px-3.5 py-2.5 border-y border-transparent">
								<div class="flex items-center justify-end gap-2">
									<a
										href="#!"
										class="flex items-center justify-center size-8 transition-all duration-150 ease-linear rounded-md bg-slate-100 hover:bg-slate-200 dark:bg-zink-600 dark:hover:bg-zink-500"
										><LucideIcon name="Eye" class="size-3" /></a
									>
									<a
										href="#!"
										class="flex items-center justify-center size-8 transition-all duration-150 ease-linear rounded-md bg-slate-100 hover:bg-slate-200 dark:bg-zink-600 dark:hover:bg-zink-500"
										><LucideIcon name="FileEdit" class="size-3" /></a
									>
									<a
										href="#!"
										class="flex items-center justify-center size-8 transition-all duration-150 ease-linear rounded-md bg-slate-100 hover:bg-slate-200 dark:bg-zink-600 dark:hover:bg-zink-500"
										><LucideIcon name="ArrowDownToLine" class="size-3" /></a
									>
									<a
										href="#!"
										class="flex items-center justify-center size-8 transition-all duration-150 ease-linear rounded-md bg-slate-100 hover:bg-slate-200 dark:bg-zink-600 dark:hover:bg-zink-500"
										><LucideIcon name="Trash2" class="size-3" /></a
									>
								</div>
							</td>
						</tr>
						<tr class="bg-white dark:bg-zink-700">
							<td class="px-3.5 py-2.5 border-y border-transparent">
								<div class="flex items-center h-full">
									<input
										id="Checkbox2"
										class="size-4 bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800"
										type="checkbox"
										value=""
									/>
								</div>
							</td>
							<td class="px-3.5 py-2.5 border-y border-transparent">
								<span
									class="px-2.5 py-0.5 inline-block text-xs font-medium rounded border bg-slate-100 border-transparent text-slate-500 dark:bg-slate-500/20 dark:text-zink-200 dark:border-transparent"
									>SCSS</span
								>
							</td>
							<td class="px-3.5 py-2.5 border-y border-transparent">tailwind.scss</td>
							<td class="px-3.5 py-2.5 border-y border-transparent">0.100 KB</td>
							<td class="px-3.5 py-2.5 border-y border-transparent">03 April, 2023</td>
							<td class="px-3.5 py-2.5 border-y border-transparent">Paula</td>
							<td class="px-3.5 py-2.5 border-y border-transparent"
								><span
									class="ppx-2.5 py-0.5 inline-block text-xs font-medium rounded border bg-green-100 border-transparent text-green-500 dark:bg-green-500/20 dark:border-transparent"
									>Successful</span
								></td
							>
							<td class="px-3.5 py-2.5 border-y border-transparent">
								<div class="flex items-center justify-end gap-2">
									<a
										href="#!"
										class="flex items-center justify-center size-8 transition-all duration-150 ease-linear rounded-md bg-slate-100 hover:bg-slate-200 dark:bg-zink-600 dark:hover:bg-zink-500"
										><LucideIcon name="Eye" class="size-3" /></a
									>
									<a
										href="#!"
										class="flex items-center justify-center size-8 transition-all duration-150 ease-linear rounded-md bg-slate-100 hover:bg-slate-200 dark:bg-zink-600 dark:hover:bg-zink-500"
										><LucideIcon name="FileEdit" class="size-3" /></a
									>
									<a
										href="#!"
										class="flex items-center justify-center size-8 transition-all duration-150 ease-linear rounded-md bg-slate-100 hover:bg-slate-200 dark:bg-zink-600 dark:hover:bg-zink-500"
										><LucideIcon name="ArrowDownToLine" class="size-3" /></a
									>
									<a
										href="#!"
										class="flex items-center justify-center size-8 transition-all duration-150 ease-linear rounded-md bg-slate-100 hover:bg-slate-200 dark:bg-zink-600 dark:hover:bg-zink-500"
										><LucideIcon name="Trash2" class="size-3" /></a
									>
								</div>
							</td>
						</tr>
						<tr class="bg-white dark:bg-zink-700">
							<td class="px-3.5 py-2.5 border-y border-transparent">
								<div class="flex items-center h-full">
									<input
										id="Checkbox2"
										class="size-4 bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800"
										type="checkbox"
										value=""
									/>
								</div>
							</td>
							<td class="px-3.5 py-2.5 border-y border-transparent">
								<span
									class="px-2.5 py-0.5 inline-block text-xs font-medium rounded border bg-slate-100 border-transparent text-slate-500 dark:bg-slate-500/20 dark:text-zink-200 dark:border-transparent"
									>MP4</span
								>
							</td>
							<td class="px-3.5 py-2.5 border-y border-transparent">Tailwick Guide Video.mp4</td>
							<td class="px-3.5 py-2.5 border-y border-transparent">149.33 MB</td>
							<td class="px-3.5 py-2.5 border-y border-transparent">12 Nov, 2023</td>
							<td class="px-3.5 py-2.5 border-y border-transparent">Themesdesign</td>
							<td class="px-3.5 py-2.5 border-y border-transparent"
								><span
									class="px-2.5 py-0.5 inline-block text-xs font-medium rounded border bg-yellow-100 border-transparent text-yellow-500 dark:bg-yellow-500/20 dark:border-transparent"
									>Pending</span
								></td
							>
							<td class="px-3.5 py-2.5 border-y border-transparent">
								<div class="flex items-center justify-end gap-2">
									<a
										href="#!"
										class="flex items-center justify-center size-8 transition-all duration-150 ease-linear rounded-md bg-slate-100 hover:bg-slate-200 dark:bg-zink-600 dark:hover:bg-zink-500"
										><LucideIcon name="Eye" class="size-3" /></a
									>
									<a
										href="#!"
										class="flex items-center justify-center size-8 transition-all duration-150 ease-linear rounded-md bg-slate-100 hover:bg-slate-200 dark:bg-zink-600 dark:hover:bg-zink-500"
										><LucideIcon name="FileEdit" class="size-3" /></a
									>
									<a
										href="#!"
										class="flex items-center justify-center size-8 transition-all duration-150 ease-linear rounded-md bg-slate-100 hover:bg-slate-200 dark:bg-zink-600 dark:hover:bg-zink-500"
										><LucideIcon name="ArrowDownToLine" class="size-3" /></a
									>
									<a
										href="#!"
										class="flex items-center justify-center size-8 transition-all duration-150 ease-linear rounded-md bg-slate-100 hover:bg-slate-200 dark:bg-zink-600 dark:hover:bg-zink-500"
										><LucideIcon name="Trash2" class="size-3" /></a
									>
								</div>
							</td>
						</tr>
					</tbody>
				</table>
			</div>
			<div class="flex flex-col items-center gap-4 mt-4 mb-4 md:flex-row">
				<div class="grow">
					<p class="text-slate-500 dark:text-zink-200">Showing <b>6</b> of <b>18</b> Results</p>
				</div>
				<ul class="flex flex-wrap items-center gap-2 shrink-0">
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border border-slate-200 dark:border-zink-500 rounded text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-50 dark:[&.active]:text-custom-50 [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
							><LucideIcon class="size-4 rtl:rotate-180" name="ChevronLeft" /></a
						>
					</li>
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border border-slate-200 dark:border-zink-500 rounded text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-50 dark:[&.active]:text-custom-50 [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
							>1</a
						>
					</li>
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border border-slate-200 dark:border-zink-500 rounded text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-50 dark:[&.active]:text-custom-50 [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
							>2</a
						>
					</li>
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border border-slate-200 dark:border-zink-500 rounded text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-50 dark:[&.active]:text-custom-50 [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto active"
							>3</a
						>
					</li>
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border border-slate-200 dark:border-zink-500 rounded text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-50 dark:[&.active]:text-custom-50 [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
							>4</a
						>
					</li>
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border border-slate-200 dark:border-zink-500 rounded text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-50 dark:[&.active]:text-custom-50 [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
							>5</a
						>
					</li>
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border border-slate-200 dark:border-zink-500 rounded text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-50 dark:[&.active]:text-custom-50 [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
							>6</a
						>
					</li>
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border border-slate-200 dark:border-zink-500 rounded text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-50 dark:[&.active]:text-custom-50 [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
							><LucideIcon class="size-4 rtl:rotate-180" name="ChevronRight" /></a
						>
					</li>
				</ul>
			</div>
		</div>
		<!--end tab pane-->
		<div class="tab-pane {tabId == 'projects' ? 'block' : 'hidden'}" id="projectsTabs">
			<div class="flex items-center gap-3 mb-4">
				<h5 class="underline grow">Projects</h5>
				<div class="shrink-0">
					<button
						type="button"
						class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
						>Add Project</button
					>
				</div>
			</div>
			<div class="grid grid-cols-1 gap-x-5 md:grid-cols-2 2xl:grid-cols-4">
				<div class="card">
					<div class="card-body">
						<div class="flex">
							<div class="grow">
								<img src="/assets/images/brand/adwords.png" alt="" class="h-11" />
							</div>
							<div class="shrink-0">
								<Dropdown className="relative" direction="bottom-start">
									<DropdownToggle
										className="flex items-center justify-center size-[37.5px] dropdown-toggle p-0 text-slate-500 btn bg-slate-200 border-slate-200 hover:text-slate-600 hover:bg-slate-300 hover:border-slate-300 focus:text-slate-600 focus:bg-slate-300 focus:border-slate-300 focus:ring focus:ring-slate-100 active:text-slate-600 active:bg-slate-300 active:border-slate-300 active:ring active:ring-slate-100 dark:bg-zink-600 dark:hover:bg-zink-500 dark:border-zink-600 dark:hover:border-zink-500 dark:text-zink-200 dark:ring-zink-400/50"
									>
										<LucideIcon name="MoreHorizontal" class="size-4" />
									</DropdownToggle>
									<DropdownMenu
										tag="ul"
										class="absolute z-50 py-2 mt-1 ltr:text-left rtl:text-right list-none bg-white rounded-md shadow-md dropdown-menu min-w-[10rem] dark:bg-zink-600"
									>
										<li>
											<a
												class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear bg-white text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500"
												href="#!"
												><LucideIcon name="Eye" class="inline-block size-3 mr-1" /> Overview</a
											>
										</li>
										<li>
											<a
												class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear bg-white text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500"
												href="#!"
												><LucideIcon name="FileEdit" class="inline-block size-3 mr-1" /> Edit</a
											>
										</li>
										<li>
											<a
												class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear bg-white text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500"
												href="#!"
												><LucideIcon name="Trash2" class="inline-block size-3 mr-1" /> Delete</a
											>
										</li>
									</DropdownMenu>
								</Dropdown>
							</div>
						</div>

						<div class="mt-4">
							<h6 class="mb-1 text-16"><a href="#!">Chat App</a></h6>
							<p class="text-slate-500 dark:text-zink-200">
								Allows you to communicate with your customers in web chat rooms.
							</p>
						</div>
						<div
							class="flex w-full gap-3 mt-6 text-center divide-x divide-slate-200 dark:divide-zink-500 rtl:divide-x-reverse"
						>
							<div class="px-3 grow">
								<h6 class="mb-1">16 July, 2023</h6>
								<p class="text-slate-500 dark:text-zink-200">Due Date</p>
							</div>
							<div class="px-3 grow">
								<h6 class="mb-1">$8,740.00</h6>
								<p class="text-slate-500 dark:text-zink-200">Budget</p>
							</div>
						</div>
						<div class="w-full h-1.5 mt-6 rounded-full bg-slate-100 dark:bg-zink-600">
							<div class="h-1.5 rounded-full bg-custom-500" style="width: 25%"></div>
						</div>
					</div>
				</div>
				<!--end card & col-->
				<div class="card">
					<div class="card-body">
						<div class="flex">
							<div class="grow">
								<img src="/assets/images/brand/app-store.png" alt="" class="h-11" />
							</div>
							<div class="shrink-0">
								<Dropdown className="relative" direction="bottom-start">
									<DropdownToggle
										className="flex items-center justify-center size-[37.5px] dropdown-toggle p-0 text-slate-500 btn bg-slate-200 border-slate-200 hover:text-slate-600 hover:bg-slate-300 hover:border-slate-300 focus:text-slate-600 focus:bg-slate-300 focus:border-slate-300 focus:ring focus:ring-slate-100 active:text-slate-600 active:bg-slate-300 active:border-slate-300 active:ring active:ring-slate-100 dark:bg-zink-600 dark:hover:bg-zink-500 dark:border-zink-600 dark:hover:border-zink-500 dark:text-zink-200 dark:ring-zink-400/50"
									>
										<LucideIcon name="MoreHorizontal" class="size-4" />
									</DropdownToggle>
									<DropdownMenu
										tag="ul"
										class="absolute z-50 py-2 mt-1 ltr:text-left rtl:text-right list-none bg-white rounded-md shadow-md dropdown-menu min-w-[10rem] dark:bg-zink-600"
									>
										<li>
											<a
												class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear bg-white text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500"
												href="#!"
												><LucideIcon name="Eye" class="inline-block size-3 mr-1" /> Overview</a
											>
										</li>
										<li>
											<a
												class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear bg-white text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500"
												href="#!"
												><LucideIcon name="FileEdit" class="inline-block size-3 mr-1" /> Edit</a
											>
										</li>
										<li>
											<a
												class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear bg-white text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500"
												href="#!"
												><LucideIcon name="Trash2" class="inline-block size-3 mr-1" /> Delete</a
											>
										</li>
									</DropdownMenu>
								</Dropdown>
							</div>
						</div>

						<div class="mt-4">
							<h6 class="mb-1 text-16"><a href="#!">Business Template - UI/UX design</a></h6>
							<p class="text-slate-500 dark:text-zink-200">
								UX design process is iterative and non-linear, includes a lot of research.
							</p>
						</div>
						<div
							class="flex w-full gap-3 mt-6 text-center divide-x divide-slate-200 dark:divide-zink-500 rtl:divide-x-reverse"
						>
							<div class="px-3 grow">
								<h6 class="mb-1">28 Nov, 2023</h6>
								<p class="text-slate-500 dark:text-zink-200">Due Date</p>
							</div>
							<div class="px-3 grow">
								<h6 class="mb-1">$10,254.00</h6>
								<p class="text-slate-500 dark:text-zink-200">Budget</p>
							</div>
						</div>
						<div class="w-full h-1.5 mt-6 rounded-full bg-slate-100 dark:bg-zink-600">
							<div class="h-1.5 rounded-full bg-sky-500" style="width: 61%"></div>
						</div>
					</div>
				</div>
				<!--end card & col-->
				<div class="card">
					<div class="card-body">
						<div class="flex">
							<div class="grow">
								<img src="/assets/images/brand/android.png" alt="" class="h-11" />
							</div>
							<div class="shrink-0">
								<Dropdown className="relative" direction="bottom-start">
									<DropdownToggle
										className="flex items-center justify-center size-[37.5px] dropdown-toggle p-0 text-slate-500 btn bg-slate-200 border-slate-200 hover:text-slate-600 hover:bg-slate-300 hover:border-slate-300 focus:text-slate-600 focus:bg-slate-300 focus:border-slate-300 focus:ring focus:ring-slate-100 active:text-slate-600 active:bg-slate-300 active:border-slate-300 active:ring active:ring-slate-100 dark:bg-zink-600 dark:hover:bg-zink-500 dark:border-zink-600 dark:hover:border-zink-500 dark:text-zink-200 dark:ring-zink-400/50"
									>
										<LucideIcon name="MoreHorizontal" class="size-4" />
									</DropdownToggle>
									<DropdownMenu
										tag="ul"
										class="absolute z-50 py-2 mt-1 ltr:text-left rtl:text-right list-none bg-white rounded-md shadow-md dropdown-menu min-w-[10rem] dark:bg-zink-600"
									>
										<li>
											<a
												class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear bg-white text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500"
												href="#!"
												><LucideIcon name="Eye" class="inline-block size-3 mr-1" /> Overview</a
											>
										</li>
										<li>
											<a
												class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear bg-white text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500"
												href="#!"
												><LucideIcon name="FileEdit" class="inline-block size-3 mr-1" /> Edit</a
											>
										</li>
										<li>
											<a
												class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear bg-white text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500"
												href="#!"
												><LucideIcon name="Trash2" class="inline-block size-3 mr-1" /> Delete</a
											>
										</li>
									</DropdownMenu>
								</Dropdown>
							</div>
						</div>

						<div class="mt-4">
							<h6 class="mb-1 text-16"><a href="#!">ABC Project Customization</a></h6>
							<p class="text-slate-500 dark:text-zink-200">
								The process of tailoring the overall project delivery process to meet the
								requirements.
							</p>
						</div>
						<div
							class="flex w-full gap-3 mt-6 text-center divide-x divide-slate-200 dark:divide-zink-500 rtl:divide-x-reverse"
						>
							<div class="px-3 grow">
								<h6 class="mb-1">20 Oct, 2023</h6>
								<p class="text-slate-500 dark:text-zink-200">Due Date</p>
							</div>
							<div class="px-3 grow">
								<h6 class="mb-1">$9,832.00</h6>
								<p class="text-slate-500 dark:text-zink-200">Budget</p>
							</div>
						</div>
						<div class="w-full h-1.5 mt-6 rounded-full bg-slate-100 dark:bg-zink-600">
							<div class="h-1.5 rounded-full bg-green-500" style="width: 87%"></div>
						</div>
					</div>
				</div>
				<!--end card & col-->
				<div class="card">
					<div class="card-body">
						<div class="flex">
							<div class="grow">
								<img src="/assets/images/brand/figma.png" alt="" class="h-11" />
							</div>
							<div class="shrink-0">
								<Dropdown className="relative" direction="bottom-start">
									<DropdownToggle
										className="flex items-center justify-center size-[37.5px] dropdown-toggle p-0 text-slate-500 btn bg-slate-200 border-slate-200 hover:text-slate-600 hover:bg-slate-300 hover:border-slate-300 focus:text-slate-600 focus:bg-slate-300 focus:border-slate-300 focus:ring focus:ring-slate-100 active:text-slate-600 active:bg-slate-300 active:border-slate-300 active:ring active:ring-slate-100 dark:bg-zink-600 dark:hover:bg-zink-500 dark:border-zink-600 dark:hover:border-zink-500 dark:text-zink-200 dark:ring-zink-400/50"
									>
										<LucideIcon name="MoreHorizontal" class="size-4" />
									</DropdownToggle>
									<DropdownMenu
										tag="ul"
										class="absolute z-50 py-2 mt-1 ltr:text-left rtl:text-right list-none bg-white rounded-md shadow-md dropdown-menu min-w-[10rem] dark:bg-zink-600"
									>
										<li>
											<a
												class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear bg-white text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500"
												href="#!"
												><LucideIcon name="Eye" class="inline-block size-3 mr-1" /> Overview</a
											>
										</li>
										<li>
											<a
												class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear bg-white text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500"
												href="#!"
												><LucideIcon name="FileEdit" class="inline-block size-3 mr-1" /> Edit</a
											>
										</li>
										<li>
											<a
												class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear bg-white text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500"
												href="#!"
												><LucideIcon name="Trash2" class="inline-block size-3 mr-1" /> Delete</a
											>
										</li>
									</DropdownMenu>
								</Dropdown>
							</div>
						</div>

						<div class="mt-4">
							<h6 class="mb-1 text-16"><a href="#!">Tailwick Design</a></h6>
							<p class="text-slate-500 dark:text-zink-200">
								Drawing created with Microsoft Expression Design, a drawing and design program for
								Windows.
							</p>
						</div>
						<div
							class="flex w-full gap-3 mt-6 text-center divide-x divide-slate-200 dark:divide-zink-500 rtl:divide-x-reverse"
						>
							<div class="px-3 grow">
								<h6 class="mb-1">07 Dec, 2023</h6>
								<p class="text-slate-500 dark:text-zink-200">Due Date</p>
							</div>
							<div class="px-3 grow">
								<h6 class="mb-1">$11,971.00</h6>
								<p class="text-slate-500 dark:text-zink-200">Budget</p>
							</div>
						</div>
						<div class="w-full h-1.5 mt-6 rounded-full bg-slate-100 dark:bg-zink-600">
							<div class="h-1.5 bg-purple-500 rounded-full" style="width: 65%"></div>
						</div>
					</div>
				</div>
				<!--end card & col-->
				<div class="card">
					<div class="card-body">
						<div class="flex">
							<div class="grow">
								<img src="/assets/images/brand/gmail.png" alt="" class="h-11" />
							</div>
							<div class="shrink-0">
								<Dropdown className="relative" direction="bottom-start">
									<DropdownToggle
										className="flex items-center justify-center size-[37.5px] dropdown-toggle p-0 text-slate-500 btn bg-slate-200 border-slate-200 hover:text-slate-600 hover:bg-slate-300 hover:border-slate-300 focus:text-slate-600 focus:bg-slate-300 focus:border-slate-300 focus:ring focus:ring-slate-100 active:text-slate-600 active:bg-slate-300 active:border-slate-300 active:ring active:ring-slate-100 dark:bg-zink-600 dark:hover:bg-zink-500 dark:border-zink-600 dark:hover:border-zink-500 dark:text-zink-200 dark:ring-zink-400/50"
									>
										<LucideIcon name="MoreHorizontal" class="size-4" />
									</DropdownToggle>
									<DropdownMenu
										tag="ul"
										class="absolute z-50 py-2 mt-1 ltr:text-left rtl:text-right list-none bg-white rounded-md shadow-md dropdown-menu min-w-[10rem] dark:bg-zink-600"
									>
										<li>
											<a
												class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear bg-white text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500"
												href="#!"
												><LucideIcon name="Eye" class="inline-block size-3 mr-1" /> Overview</a
											>
										</li>
										<li>
											<a
												class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear bg-white text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500"
												href="#!"
												><LucideIcon name="FileEdit" class="inline-block size-3 mr-1" /> Edit</a
											>
										</li>
										<li>
											<a
												class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear bg-white text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500"
												href="#!"
												><LucideIcon name="Trash2" class="inline-block size-3 mr-1" /> Delete</a
											>
										</li>
									</DropdownMenu>
								</Dropdown>
							</div>
						</div>

						<div class="mt-4">
							<h6 class="mb-1 text-16"><a href="#!">HR Management</a></h6>
							<p class="text-slate-500 dark:text-zink-200">
								The strategic approach to nurturing and supporting employees and ensuring a
								positive.
							</p>
						</div>
						<div
							class="flex w-full gap-3 mt-6 text-center divide-x divide-slate-200 dark:divide-zink-500 rtl:divide-x-reverse"
						>
							<div class="px-3 grow">
								<h6 class="mb-1">02 Jan, 2024</h6>
								<p class="text-slate-500 dark:text-zink-200">Due Date</p>
							</div>
							<div class="px-3 grow">
								<h6 class="mb-1">$7,546.00</h6>
								<p class="text-slate-500 dark:text-zink-200">Budget</p>
							</div>
						</div>
						<div class="w-full h-1.5 mt-6 rounded-full bg-slate-100 dark:bg-zink-600">
							<div class="h-1.5 bg-purple-500 rounded-full" style="width: 65%"></div>
						</div>
					</div>
				</div>
				<!--end card & col-->
				<div class="card">
					<div class="card-body">
						<div class="flex">
							<div class="grow">
								<img src="/assets/images/brand/meta.png" alt="" class="h-11" />
							</div>
							<div class="shrink-0">
								<Dropdown className="relative" direction="bottom-start">
									<DropdownToggle
										className="flex items-center justify-center size-[37.5px] dropdown-toggle p-0 text-slate-500 btn bg-slate-200 border-slate-200 hover:text-slate-600 hover:bg-slate-300 hover:border-slate-300 focus:text-slate-600 focus:bg-slate-300 focus:border-slate-300 focus:ring focus:ring-slate-100 active:text-slate-600 active:bg-slate-300 active:border-slate-300 active:ring active:ring-slate-100 dark:bg-zink-600 dark:hover:bg-zink-500 dark:border-zink-600 dark:hover:border-zink-500 dark:text-zink-200 dark:ring-zink-400/50"
									>
										<LucideIcon name="MoreHorizontal" class="size-4" />
									</DropdownToggle>
									<DropdownMenu
										tag="ul"
										class="absolute z-50 py-2 mt-1 ltr:text-left rtl:text-right list-none bg-white rounded-md shadow-md dropdown-menu min-w-[10rem] dark:bg-zink-600"
									>
										<li>
											<a
												class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear bg-white text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500"
												href="#!"
												><LucideIcon name="Eye" class="inline-block size-3 mr-1" /> Overview</a
											>
										</li>
										<li>
											<a
												class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear bg-white text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500"
												href="#!"
												><LucideIcon name="FileEdit" class="inline-block size-3 mr-1" /> Edit</a
											>
										</li>
										<li>
											<a
												class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear bg-white text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500"
												href="#!"
												><LucideIcon name="Trash2" class="inline-block size-3 mr-1" /> Delete</a
											>
										</li>
									</DropdownMenu>
								</Dropdown>
							</div>
						</div>

						<div class="mt-4">
							<h6 class="mb-1 text-16"><a href="#!">Finance Apps</a></h6>
							<p class="text-slate-500 dark:text-zink-200">
								A personal budget app is a technology solution that is connected.
							</p>
						</div>
						<div
							class="flex w-full gap-3 mt-6 text-center divide-x divide-slate-200 dark:divide-zink-500 rtl:divide-x-reverse"
						>
							<div class="px-3 grow">
								<h6 class="mb-1">10 Feb, 2024</h6>
								<p class="text-slate-500 dark:text-zink-200">Due Date</p>
							</div>
							<div class="px-3 grow">
								<h6 class="mb-1">$13,745.00</h6>
								<p class="text-slate-500 dark:text-zink-200">Budget</p>
							</div>
						</div>
						<div class="w-full h-1.5 mt-6 rounded-full bg-slate-100 dark:bg-zink-600">
							<div class="h-1.5 bg-purple-500 rounded-full" style="width: 65%"></div>
						</div>
					</div>
				</div>
				<!--end card & col-->
				<div class="card">
					<div class="card-body">
						<div class="flex">
							<div class="grow">
								<img src="/assets/images/brand/search.png" alt="" class="h-11" />
							</div>
							<div class="shrink-0">
								<Dropdown className="relative" direction="bottom-start">
									<DropdownToggle
										className="flex items-center justify-center size-[37.5px] dropdown-toggle p-0 text-slate-500 btn bg-slate-200 border-slate-200 hover:text-slate-600 hover:bg-slate-300 hover:border-slate-300 focus:text-slate-600 focus:bg-slate-300 focus:border-slate-300 focus:ring focus:ring-slate-100 active:text-slate-600 active:bg-slate-300 active:border-slate-300 active:ring active:ring-slate-100 dark:bg-zink-600 dark:hover:bg-zink-500 dark:border-zink-600 dark:hover:border-zink-500 dark:text-zink-200 dark:ring-zink-400/50"
									>
										<LucideIcon name="MoreHorizontal" class="size-4" />
									</DropdownToggle>
									<DropdownMenu
										tag="ul"
										class="absolute z-50 py-2 mt-1 ltr:text-left rtl:text-right list-none bg-white rounded-md shadow-md dropdown-menu min-w-[10rem] dark:bg-zink-600"
									>
										<li>
											<a
												class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear bg-white text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500"
												href="#!"
												><LucideIcon name="Eye" class="inline-block size-3 mr-1" /> Overview</a
											>
										</li>
										<li>
											<a
												class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear bg-white text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500"
												href="#!"
												><LucideIcon name="FileEdit" class="inline-block size-3 mr-1" /> Edit</a
											>
										</li>
										<li>
											<a
												class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear bg-white text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500"
												href="#!"
												><LucideIcon name="Trash2" class="inline-block size-3 mr-1" /> Delete</a
											>
										</li>
									</DropdownMenu>
								</Dropdown>
							</div>
						</div>

						<div class="mt-4">
							<h6 class="mb-1 text-16"><a href="#!">Mailbox Design</a></h6>
							<p class="text-slate-500 dark:text-zink-200">
								An email template is an HTML preformatted email that you can use to create your own.
							</p>
						</div>
						<div
							class="flex w-full gap-3 mt-6 text-center divide-x divide-slate-200 dark:divide-zink-500 rtl:divide-x-reverse"
						>
							<div class="px-3 grow">
								<h6 class="mb-1">19 Feb, 2024</h6>
								<p class="text-slate-500 dark:text-zink-200">Due Date</p>
							</div>
							<div class="px-3 grow">
								<h6 class="mb-1">$9,120.00</h6>
								<p class="text-slate-500 dark:text-zink-200">Budget</p>
							</div>
						</div>
						<div class="w-full h-1.5 mt-6 rounded-full bg-slate-100 dark:bg-zink-600">
							<div class="h-1.5 bg-purple-500 rounded-full" style="width: 65%"></div>
						</div>
					</div>
				</div>
				<!--end card & col-->
				<div class="card">
					<div class="card-body">
						<div class="flex">
							<div class="grow">
								<img src="/assets/images/brand/twitter.png" alt="" class="h-11" />
							</div>
							<div class="shrink-0">
								<Dropdown className="relative" direction="bottom-start">
									<DropdownToggle
										className="flex items-center justify-center size-[37.5px] dropdown-toggle p-0 text-slate-500 btn bg-slate-200 border-slate-200 hover:text-slate-600 hover:bg-slate-300 hover:border-slate-300 focus:text-slate-600 focus:bg-slate-300 focus:border-slate-300 focus:ring focus:ring-slate-100 active:text-slate-600 active:bg-slate-300 active:border-slate-300 active:ring active:ring-slate-100 dark:bg-zink-600 dark:hover:bg-zink-500 dark:border-zink-600 dark:hover:border-zink-500 dark:text-zink-200 dark:ring-zink-400/50"
									>
										<LucideIcon name="MoreHorizontal" class="size-4" />
									</DropdownToggle>
									<DropdownMenu
										tag="ul"
										class="absolute z-50 py-2 mt-1 ltr:text-left rtl:text-right list-none bg-white rounded-md shadow-md dropdown-menu min-w-[10rem] dark:bg-zink-600"
									>
										<li>
											<a
												class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear bg-white text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500"
												href="#!"
												><LucideIcon name="Eye" class="inline-block size-3 mr-1" /> Overview</a
											>
										</li>
										<li>
											<a
												class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear bg-white text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500"
												href="#!"
												><LucideIcon name="FileEdit" class="inline-block size-3 mr-1" /> Edit</a
											>
										</li>
										<li>
											<a
												class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear bg-white text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500"
												href="#!"
												><LucideIcon name="Trash2" class="inline-block size-3 mr-1" /> Delete</a
											>
										</li>
									</DropdownMenu>
								</Dropdown>
							</div>
						</div>

						<div class="mt-4">
							<h6 class="mb-1 text-16"><a href="#!">Banking Management</a></h6>
							<p class="text-slate-500 dark:text-zink-200">
								Bank management refers to the process of managing the Bank's statutory activity.
							</p>
						</div>
						<div
							class="flex w-full gap-3 mt-6 text-center divide-x divide-slate-200 dark:divide-zink-500 rtl:divide-x-reverse"
						>
							<div class="px-3 grow">
								<h6 class="mb-1">01 March, 2024</h6>
								<p class="text-slate-500 dark:text-zink-200">Due Date</p>
							</div>
							<div class="px-3 grow">
								<h6 class="mb-1">$24,863.00</h6>
								<p class="text-slate-500 dark:text-zink-200">Budget</p>
							</div>
						</div>
						<div class="w-full h-1.5 mt-6 rounded-full bg-slate-100 dark:bg-zink-600">
							<div class="h-1.5 bg-purple-500 rounded-full" style="width: 65%"></div>
						</div>
					</div>
				</div>
				<!--end card & col-->
			</div>
			<!--end grid-->
			<div class="flex flex-col items-center gap-4 mt-2 mb-4 md:flex-row">
				<div class="grow">
					<p class="text-slate-500 dark:text-zink-200">Showing <b>8</b> of <b>30</b> Results</p>
				</div>
				<ul class="flex flex-wrap items-center gap-2 shrink-0">
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border border-slate-200 dark:border-zink-500 rounded text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-50 dark:[&.active]:text-custom-50 [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
							><LucideIcon class="size-4 rtl:rotate-180" name="ChevronsLeft" /></a
						>
					</li>
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border border-slate-200 dark:border-zink-500 rounded text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-50 dark:[&.active]:text-custom-50 [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
							><LucideIcon class="size-4 rtl:rotate-180" name="ChevronLeft" /></a
						>
					</li>
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border border-slate-200 dark:border-zink-500 rounded text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-50 dark:[&.active]:text-custom-50 [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
							>1</a
						>
					</li>
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border border-slate-200 dark:border-zink-500 rounded text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-50 dark:[&.active]:text-custom-50 [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
							>2</a
						>
					</li>
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border border-slate-200 dark:border-zink-500 rounded text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-50 dark:[&.active]:text-custom-50 [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto active"
							>3</a
						>
					</li>
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border border-slate-200 dark:border-zink-500 rounded text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-50 dark:[&.active]:text-custom-50 [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
							>4</a
						>
					</li>
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border border-slate-200 dark:border-zink-500 rounded text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-50 dark:[&.active]:text-custom-50 [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
							>5</a
						>
					</li>
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border border-slate-200 dark:border-zink-500 rounded text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-50 dark:[&.active]:text-custom-50 [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
							>6</a
						>
					</li>
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border border-slate-200 dark:border-zink-500 rounded text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-50 dark:[&.active]:text-custom-50 [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
							><LucideIcon class="size-4 rtl:rotate-180" name="ChevronRight" /></a
						>
					</li>
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border border-slate-200 dark:border-zink-500 rounded text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-50 dark:[&.active]:text-custom-50 [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
							><LucideIcon class="size-4 rtl:rotate-180" name="ChevronsRight" /></a
						>
					</li>
				</ul>
			</div>
		</div>
		<!--end tab pane-->
		<div class="tab-pane {tabId == 'followers' ? 'block' : 'hidden'}" id="followersTabs">
			<h5 class="mb-4 underline">Followers</h5>

			<div class="grid grid-cols-1 md:grid-cols-2 2xl:grid-cols-4 gap-x-5">
				<div class="relative card">
					<div class="card-body">
						<p
							class="absolute inline-block px-5 py-1 text-xs ltr:left-0 rtl:right-0 text-custom-600 bg-custom-100 dark:bg-custom-500/20 top-5 ltr:rounded-e rtl:rounded-l"
						>
							Executive Operations
						</p>
						<div class="flex items-center justify-end">
							<p class="text-slate-500 dark:text-zink-200">Doj : 15 Jan, 2023</p>
						</div>
						<div class="mt-4 text-center">
							<div class="flex justify-center">
								<div class="size-20 overflow-hidden rounded-full bg-slate-100">
									<img src="/assets/images/users/avatar-3.png" alt="" class="" />
								</div>
							</div>
							<a href="#!"><h4 class="mt-4 mb-2 font-semibold text-16">Ralaphe Flores</h4></a>
							<div class="text-slate-500 dark:text-zink-200">
								<p class="mb-1">floral12@tailwick.com</p>
								<p>+213 617 219 6245</p>
								<p
									class="inline-block px-3 py-1 my-4 font-semibold rounded-md text-slate-600 bg-slate-100 dark:bg-zink-600 dark:text-zink-200"
								>
									Exp. : 1.5 years
								</p>
								<h4 class="text-15 text-custom-500">
									Salary : $463.42 <span
										class="text-xs font-normal text-slate-500 dark:text-zink-200"
										>/ Month<span></span></span
									>
								</h4>
							</div>
						</div>
					</div>
				</div>
				<!--end card-->
				<div class="relative card">
					<div class="card-body">
						<p
							class="absolute inline-block px-5 py-1 text-xs text-green-600 bg-green-100 ltr:left-0 rtl:right-0 dark:bg-green-500/20 top-5 ltr:rounded-e rtl:rounded-l"
						>
							Project Manager
						</p>
						<div class="flex items-center justify-end">
							<p class="text-slate-500 dark:text-zink-200">Doj : 29 Feb, 2023</p>
						</div>
						<div class="mt-4 text-center">
							<div class="flex justify-center">
								<div class="size-20 overflow-hidden rounded-full bg-slate-100">
									<img src="/assets/images/users/avatar-2.png" alt="" class="" />
								</div>
							</div>
							<a href="#!">
								<h4 class="mt-4 mb-2 font-semibold text-16">James Lash</h4>
							</a>
							<div class="text-slate-500 dark:text-zink-200">
								<p class="mb-1">jameslash@tailwick.com</p>
								<p>+210 85 383 2388</p>
								<p
									class="inline-block px-3 py-1 my-4 font-semibold rounded-md text-slate-600 bg-slate-100 dark:bg-zink-600 dark:text-zink-200"
								>
									Exp. : 0.5 years
								</p>
								<h4 class="text-15 text-custom-500">
									Salary : $701.77 <span
										class="text-xs font-normal text-slate-500 dark:text-zink-200"
										>/ Month<span></span></span
									>
								</h4>
							</div>
						</div>
					</div>
				</div>
				<!--end card-->
				<div class="relative card">
					<div class="card-body">
						<p
							class="absolute inline-block px-5 py-1 text-xs ltr:left-0 rtl:right-0 text-sky-600 bg-sky-100 dark:bg-sky-500/20 top-5 ltr:rounded-e rtl:rounded-l"
						>
							React Developer
						</p>
						<div class="flex items-center justify-end">
							<p class="text-slate-500 dark:text-zink-200">Doj : 04 March, 2023</p>
						</div>
						<div class="mt-4 text-center">
							<div class="flex justify-center">
								<div class="size-20 overflow-hidden rounded-full bg-slate-100">
									<img src="/assets/images/users/avatar-4.png" alt="" class="" />
								</div>
							</div>
							<a href="#!">
								<h4 class="mt-4 mb-2 font-semibold text-16">Angus Garnsey</h4>
							</a>
							<div class="text-slate-500 dark:text-zink-200">
								<p class="mb-1">angusgarnsey@tailwick.com</p>
								<p>+210 41521 1325</p>
								<p
									class="inline-block px-3 py-1 my-4 font-semibold rounded-md text-slate-600 bg-slate-100 dark:bg-zink-600 dark:text-zink-200"
								>
									Exp. : 0.7 years
								</p>
								<h4 class="text-15 text-custom-500">
									Salary : $478.32 <span
										class="text-xs font-normal text-slate-500 dark:text-zink-200"
										>/ Month<span></span></span
									>
								</h4>
							</div>
						</div>
					</div>
				</div>
				<!--end card-->
				<div class="relative card">
					<div class="card-body">
						<p
							class="absolute inline-block px-5 py-1 text-xs text-yellow-600 bg-yellow-100 ltr:left-0 rtl:right-0 dark:bg-yellow-500/20 top-5 ltr:rounded-e rtl:rounded-l"
						>
							Shopify Developer
						</p>
						<div class="flex items-center justify-end">
							<p class="text-slate-500 dark:text-zink-200">Doj : 11 March, 2023</p>
						</div>
						<div class="mt-4 text-center">
							<div class="flex justify-center">
								<div class="size-20 overflow-hidden rounded-full bg-slate-100">
									<img src="/assets/images/users/avatar-5.png" alt="" class="" />
								</div>
							</div>
							<a href="#!">
								<h4 class="mt-4 mb-2 font-semibold text-16">Matilda Marston</h4>
							</a>
							<div class="text-slate-500 dark:text-zink-200">
								<p class="mb-1">matildamarston@tailwick.com</p>
								<p>+210 082 288 1065</p>
								<p
									class="inline-block px-3 py-1 my-4 font-semibold rounded-md text-slate-600 bg-slate-100 dark:bg-zink-600 dark:text-zink-200"
								>
									Exp. : 1 years
								</p>
								<h4 class="text-15 text-custom-500">
									Salary : $120.37 <span
										class="text-xs font-normal text-slate-500 dark:text-zink-200"
										>/ Month<span></span></span
									>
								</h4>
							</div>
						</div>
					</div>
				</div>
				<!--end card-->
				<div class="relative card">
					<div class="card-body">
						<p
							class="absolute inline-block px-5 py-1 text-xs text-red-600 bg-red-100 ltr:left-0 rtl:right-0 dark:bg-red-500/20 top-5 ltr:rounded-e rtl:rounded-l"
						>
							Angular Developer
						</p>
						<div class="flex items-center justify-end">
							<p class="text-slate-500 dark:text-zink-200">Doj : 22 March, 2023</p>
						</div>
						<div class="mt-4 text-center">
							<div class="flex justify-center">
								<div class="size-20 overflow-hidden rounded-full bg-slate-100">
									<img src="/assets/images/users/avatar-6.png" alt="" class="" />
								</div>
							</div>
							<a href="#!">
								<h4 class="mt-4 mb-2 font-semibold text-16">Zachary Benjamin</h4>
							</a>
							<div class="text-slate-500 dark:text-zink-200">
								<p class="mb-1">zacharybenjamin@tailwick.com</p>
								<p>+120 348 9730 237</p>
								<p
									class="inline-block px-3 py-1 my-4 font-semibold rounded-md text-slate-600 bg-slate-100 dark:bg-zink-600 dark:text-zink-200"
								>
									Exp. : 0 years
								</p>
								<h4 class="text-15 text-custom-500">
									Salary : $89.99 <span
										class="text-xs font-normal text-slate-500 dark:text-zink-200"
										>/ Month<span></span></span
									>
								</h4>
							</div>
						</div>
					</div>
				</div>
				<!--end card-->
				<div class="relative card">
					<div class="card-body">
						<p
							class="absolute inline-block px-5 py-1 text-xs text-purple-600 bg-purple-100 ltr:left-0 rtl:right-0 dark:bg-purple-500/20 top-5 ltr:rounded-e rtl:rounded-l"
						>
							Graphic Designer
						</p>
						<div class="flex items-center justify-end">
							<p class="text-slate-500 dark:text-zink-200">Doj : 09 June, 2023</p>
						</div>
						<div class="mt-4 text-center">
							<div class="flex justify-center">
								<div class="size-20 overflow-hidden rounded-full bg-slate-100">
									<img src="/assets/images/users/avatar-7.png" alt="" class="" />
								</div>
							</div>
							<a href="#!">
								<h4 class="mt-4 mb-2 font-semibold text-16">Ruby Chomley</h4>
							</a>
							<div class="text-slate-500 dark:text-zink-200">
								<p class="mb-1">rubychomley@tailwick.com</p>
								<p>+120 1234 56789</p>
								<p
									class="inline-block px-3 py-1 my-4 font-semibold rounded-md text-slate-600 bg-slate-100 dark:bg-zink-600 dark:text-zink-200"
								>
									Exp. : 0.2 years
								</p>
								<h4 class="text-15 text-custom-500">
									Salary : $214.82 <span
										class="text-xs font-normal text-slate-500 dark:text-zink-200"
										>/ Month<span></span></span
									>
								</h4>
							</div>
						</div>
					</div>
				</div>
				<!--end card-->
				<div class="relative card">
					<div class="card-body">
						<p
							class="absolute inline-block px-5 py-1 text-xs text-yellow-600 bg-yellow-100 ltr:left-0 rtl:right-0 dark:bg-yellow-500/20 top-5 ltr:rounded-e rtl:rounded-l"
						>
							Shopify Developer
						</p>
						<div class="flex items-center justify-end">
							<p class="text-slate-500 dark:text-zink-200">Doj : 27 June, 2023</p>
						</div>
						<div class="mt-4 text-center">
							<div class="flex justify-center">
								<div class="size-20 overflow-hidden rounded-full bg-slate-100">
									<img src="/assets/images/users/avatar-8.png" alt="" class="" />
								</div>
							</div>
							<a href="#!">
								<h4 class="mt-4 mb-2 font-semibold text-16">Jesse Edouardy</h4>
							</a>
							<div class="text-slate-500 dark:text-zink-200">
								<p class="mb-1">jessedouard@tailwick.com</p>
								<p>+87 044 017 3869</p>
								<p
									class="inline-block px-3 py-1 my-4 font-semibold rounded-md text-slate-600 bg-slate-100 dark:bg-zink-600 dark:text-zink-200"
								>
									Exp. : 1.7 years
								</p>
								<h4 class="text-15 text-custom-500">
									Salary : $278.96 <span
										class="text-xs font-normal text-slate-500 dark:text-zink-200"
										>/ Month<span></span></span
									>
								</h4>
							</div>
						</div>
					</div>
				</div>
				<!--end card-->
				<div class="relative card">
					<div class="card-body">
						<p
							class="absolute inline-block px-5 py-1 text-xs text-orange-600 bg-orange-100 ltr:left-0 rtl:right-0 dark:bg-orange-500/20 top-5 ltr:rounded-e rtl:rounded-l"
						>
							Team Leader
						</p>
						<div class="flex items-center justify-end">
							<p class="text-slate-500 dark:text-zink-200">Doj : 15 July, 2023</p>
						</div>
						<div class="mt-4 text-center">
							<div class="flex justify-center">
								<div class="size-20 overflow-hidden rounded-full bg-slate-100">
									<img src="/assets/images/users/avatar-9.png" alt="" class="" />
								</div>
							</div>
							<a href="#!">
								<h4 class="mt-4 mb-2 font-semibold text-16">Xavier Bower</h4>
							</a>
							<div class="text-slate-500 dark:text-zink-200">
								<p class="mb-1">xavierbower@tailwick.com</p>
								<p>+159 98765 32451</p>
								<p
									class="inline-block px-3 py-1 my-4 font-semibold rounded-md text-slate-600 bg-slate-100 dark:bg-zink-600 dark:text-zink-200"
								>
									Exp. : 6.7 years
								</p>
								<h4 class="text-15 text-custom-500">
									Salary : $901.94 <span
										class="text-xs font-normal text-slate-500 dark:text-zink-200"
										>/ Month<span></span></span
									>
								</h4>
							</div>
						</div>
					</div>
				</div>
				<!--end card-->
			</div>
			<!--end grid-->
			<div class="flex flex-col items-center gap-4 mb-4 md:flex-row">
				<div class="grow">
					<p class="text-slate-500 dark:text-zink-200">Showing <b>8</b> of <b>18</b> Results</p>
				</div>
				<ul class="flex flex-wrap items-center gap-2">
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border border-slate-200 dark:border-zink-500 rounded text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-50 dark:[&.active]:text-custom-50 [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
							><LucideIcon class="size-4 rtl:rotate-180" name="ChevronLeft" /></a
						>
					</li>
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border border-slate-200 dark:border-zink-500 rounded text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-50 dark:[&.active]:text-custom-50 [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
							>1</a
						>
					</li>
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border border-slate-200 dark:border-zink-500 rounded text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-50 dark:[&.active]:text-custom-50 [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
							>2</a
						>
					</li>
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border border-slate-200 dark:border-zink-500 rounded text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-50 dark:[&.active]:text-custom-50 [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto active"
							>3</a
						>
					</li>
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border border-slate-200 dark:border-zink-500 rounded text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-50 dark:[&.active]:text-custom-50 [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
							>4</a
						>
					</li>
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border border-slate-200 dark:border-zink-500 rounded text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-50 dark:[&.active]:text-custom-50 [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
							>5</a
						>
					</li>
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border border-slate-200 dark:border-zink-500 rounded text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-50 dark:[&.active]:text-custom-50 [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
							>6</a
						>
					</li>
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border border-slate-200 dark:border-zink-500 rounded text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-50 dark:[&.active]:text-custom-50 [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
							><LucideIcon class="size-4 rtl:rotate-180" name="ChevronRight" /></a
						>
					</li>
				</ul>
			</div>
		</div>
		<!--end tab pane-->
	</div>
</div>
