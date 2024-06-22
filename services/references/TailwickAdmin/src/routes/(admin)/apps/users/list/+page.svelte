<script>
	import HeadTitle from '../../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../../common/components/Breadcrumb.svelte';
	import LucideIcon from '../../../../../common/components/LucideIcon.svelte';
	import { UserListViewData } from '../../../../../common/data/users';
	import Dropdown from '../../../../../common/components/Dropdown.svelte';
	import DropdownMenu from '../../../../../common/components/DropdownMenu.svelte';
	import DropdownToggle from '../../../../../common/components/DropdownToggle.svelte';
</script>

<HeadTitle title="List View" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto relative">
	<Breadcrumb title="List View" pagetitle="Users" />
	<div class="grid grid-cols-1 gap-x-5 xl:grid-cols-12">
		<div class="xl:col-span-12">
			<div class="card" id="usersTable">
				<div class="card-body">
					<div class="flex items-center">
						<h6 class="text-15 grow">Users List</h6>
						<div class="shrink-0">
							<button
								type="button"
								class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
								><LucideIcon name="Plus" class="inline-block size-4" />
								<span class="align-middle">Add User</span></button
							>
						</div>
					</div>
				</div>
				<div class="!py-3.5 card-body border-y border-dashed border-slate-200 dark:border-zink-500">
					<form action="#!">
						<div class="grid grid-cols-1 gap-5 xl:grid-cols-12">
							<div class="relative xl:col-span-2">
								<input
									type="text"
									class="ltr:pl-8 rtl:pr-8 search form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
									placeholder="Search for name, email, phone number etc..."
									autocomplete="off"
								/>
								<LucideIcon
									name="Search"
									class="inline-block size-4 absolute ltr:left-2.5 rtl:right-2.5 top-2.5 text-slate-500 dark:text-zink-200 fill-slate-100 dark:fill-zink-600"
								/>
							</div>
							<!--end col-->
							<div class="xl:col-span-2">
								<select
									class="form-select border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
									id="choices-single-default"
								>
									<option value="">Select Status</option>
									<option value="Verified">Verified</option>
									<option value="Waiting">Waiting</option>
									<option value="Rejected">Rejected</option>
									<option value="Hidden">Hidden</option>
								</select>
							</div>
							<!--end col-->
							<div class="xl:col-span-3 xl:col-start-10">
								<div class="flex gap-2 xl:justify-end">
									<div>
										<button
											type="button"
											class="bg-white border-dashed text-custom-500 btn border-custom-500 hover:text-custom-500 hover:bg-custom-50 hover:border-custom-600 focus:text-custom-600 focus:bg-custom-50 focus:border-custom-600 active:text-custom-600 active:bg-custom-50 active:border-custom-600 dark:bg-zink-700 dark:ring-custom-400/20 dark:hover:bg-custom-800/20 dark:focus:bg-custom-800/20 dark:active:bg-custom-800/20"
											><LucideIcon name="Download" class="inline-block size-4" />
											<span class="align-middle">Import</span></button
										>
									</div>
									<button
										class="flex items-center justify-center size-[37.5px] p-0 text-slate-500 btn bg-slate-100 hover:text-white hover:bg-slate-600 focus:text-white focus:bg-slate-600 focus:ring focus:ring-slate-100 active:text-white active:bg-slate-600 active:ring active:ring-slate-100 dark:bg-slate-500/20 dark:text-slate-400 dark:hover:bg-slate-500 dark:hover:text-white dark:focus:bg-slate-500 dark:focus:text-white dark:active:bg-slate-500 dark:active:text-white dark:ring-slate-400/20"
										><LucideIcon name="SlidersHorizontal" class="size-4" /></button
									>
								</div>
							</div>
							<!--end col-->
						</div>
						<!--end grid-->
					</form>
				</div>
				<div class="card-body">
					<div class="-mx-5 -mb-5 overflow-x-auto">
						<table class="w-full border-separate table-custom border-spacing-y-1 whitespace-nowrap">
							<thead class="text-left">
								<tr
									class="relative rounded-md bg-slate-100 dark:bg-zink-600 after:absolute ltr:after:border-l-2 rtl:after:border-r-2 ltr:after:left-0 rtl:after:right-0 after:top-0 after:bottom-0 after:border-transparent [&.active]:after:border-custom-500 [&.active]:bg-slate-100 dark:[&.active]:bg-zink-600"
								>
									<th class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold">
										<div class="flex items-center h-full">
											<input
												id="CheckboxAll"
												class="size-4 bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800 cursor-pointer"
												type="checkbox"
											/>
										</div>
									</th>
									<th class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold sort">User ID</th>
									<th class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold sort">Name</th>
									<th class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold sort">Location</th>
									<th class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold sort">Email</th>
									<th class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold sort">Phone Number</th
									>
									<th class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold sort">Joining Date</th
									>
									<th class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold sort">Status</th>
									<th class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold">Action</th>
								</tr>
							</thead>
							<tbody class="list">
								{#each UserListViewData as row}
									<tr
										class="relative rounded-md after:absolute ltr:after:border-l-2 rtl:after:border-r-2 ltr:after:left-0 rtl:after:right-0 after:top-0 after:bottom-0 after:border-transparent [&.active]:after:border-custom-500 [&.active]:bg-slate-100 dark:[&.active]:bg-zink-600"
									>
										<td class="px-3.5 py-2.5 first:pl-5 last:pr-5">
											<div class="flex items-center h-full">
												<input
													id="Checkbox1"
													class="size-4 bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800 cursor-pointer"
													type="checkbox"
												/>
											</div>
										</td>
										<td class="px-3.5 py-2.5 first:pl-5 last:pr-5">
											<a
												href="#!"
												class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600 user-id"
												>{row.userId}</a
											></td
										>
										<td class="px-3.5 py-2.5 first:pl-5 last:pr-5">
											<div class="flex items-center gap-2">
												<div
													class="flex items-center justify-center size-10 font-medium rounded-full shrink-0 bg-slate-200 text-slate-800 dark:text-zink-50 dark:bg-zink-600"
												>
													{#if row.img}
														<img src={row.img} alt="" class="h-10 rounded-full" />
													{:else}
														{row.name.charAt(0)}
													{/if}
												</div>
												<div class="grow">
													<h6 class="mb-1"><a href="#!" class="name">{row.name}</a></h6>
													<p class="text-slate-500 dark:text-zink-200">{row.designation}</p>
												</div>
											</div>
										</td>
										<td class="px-3.5 py-2.5 first:pl-5 last:pr-5 location">{row.location}</td>
										<td class="px-3.5 py-2.5 first:pl-5 last:pr-5 email">{row.email}</td>
										<td class="px-3.5 py-2.5 first:pl-5 last:pr-5 phone-number"
											>{row.phoneNumber}</td
										>
										<td class="px-3.5 py-2.5 first:pl-5 last:pr-5 joining-date"
											>{row.joiningDate}</td
										>
										<td class="px-3.5 py-2.5 first:pl-5 last:pr-5">
											{#if row.status == 'Verified'}
												<span
													class="px-2.5 py-0.5 text-xs font-medium rounded border bg-green-100 border-transparent text-green-500 dark:bg-green-500/20 dark:border-transparent inline-flex items-center status"
													><LucideIcon name="CheckCircle" class="size-3 mr-1.5" /> Verified</span
												>
											{:else if row.status == 'Waiting'}
												<span
													class="px-2.5 py-0.5 inline-flex items-center text-xs font-medium rounded border bg-slate-100 border-transparent text-slate-500 dark:bg-slate-500/20 dark:text-zink-200 dark:border-transparent status"
												>
													<LucideIcon name="Loader" class="size-3 mr-1.5" /> Waiting</span
												>
											{:else if row.status == 'Rejected'}
												<span
													class="px-2.5 py-0.5 inline-flex items-center text-xs font-medium rounded border bg-red-100 border-transparent text-red-500 dark:bg-red-500/20 dark:border-transparent status"
												>
													<LucideIcon name="X" class="size-3 mr-1.5" />
													Rejected</span
												>
											{/if}
										</td>
										<td class="px-3.5 py-2.5 first:pl-5 last:pr-5">
											<Dropdown class="relative" direction="bottom-start">
												<DropdownToggle
													className="flex items-center justify-center  size-[30px] dropdown-toggle p-0 text-slate-500 btn bg-slate-100 hover:text-white hover:bg-slate-600 focus:text-white focus:bg-slate-600 focus:ring focus:ring-slate-100 active:text-white active:bg-slate-600 active:ring active:ring-slate-100 dark:bg-slate-500/20 dark:text-slate-400 dark:hover:bg-slate-500 dark:hover:text-white dark:focus:bg-slate-500 dark:focus:text-white dark:active:bg-slate-500 dark:active:text-white dark:ring-slate-400/20"
												>
													<LucideIcon name="MoreHorizontal" class="size-3" />
												</DropdownToggle>
												<DropdownMenu
													tag="ul"
													class="absolute z-50 py-2 mt-1 ltr:text-left rtl:text-right list-none bg-white rounded-md shadow-md min-w-[10rem] dark:bg-zink-600"
												>
													<li>
														<a
															class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
															href="/pages/account"
															><LucideIcon
																name="eye"
																class="inline-block size-3 ltr:mr-1 rtl:ml-1"
															/> <span class="align-middle">Overview</span></a
														>
													</li>
													<li>
														<a
															class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
															href="#!"
															><LucideIcon
																name="FileEdit"
																class="inline-block size-3 ltr:mr-1 rtl:ml-1"
															/> <span class="align-middle">Edit</span></a
														>
													</li>
													<li>
														<a
															class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
															href="#!"
															><LucideIcon
																name="Trash2"
																class="inline-block size-3 ltr:mr-1 rtl:ml-1"
															/> <span class="align-middle">Delete</span></a
														>
													</li>
												</DropdownMenu>
											</Dropdown>
										</td>
									</tr>
								{/each}
							</tbody>
						</table>
						<div class="noresult" style="display: none">
							<div class="py-6 text-center">
								<LucideIcon
									name="Search"
									class="size-6 mx-auto text-sky-500 fill-sky-100 dark:fill-sky-500/20"
								/>
								<h5 class="mt-2">Sorry! No Result Found</h5>
								<p class="mb-0 text-slate-500 dark:text-zink-200">
									We've searched more than 199+ users We did not find any users for you search.
								</p>
							</div>
						</div>
					</div>
					<div class="flex flex-col items-center mt-8 md:flex-row">
						<div class="mb-4 grow md:mb-0">
							<p class="text-slate-500 dark:text-zink-200">
								Showing <b>10</b> of <b>57</b> Results
							</p>
						</div>
						<ul class="flex flex-wrap items-center gap-2">
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
			</div>
			<!--end card-->
		</div>
		<!--end col-->
	</div>
</div>
