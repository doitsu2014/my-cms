<script>
	import HeadTitle from '../../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../../common/components/Breadcrumb.svelte';
	import LucideIcon from '../../../../../common/components/LucideIcon.svelte';
	import Modal from '../../../../../common/components/Modal.svelte';
	import { EventData } from '../../../../../common/data/socialMedia';
	import Flatpickr from 'svelte-flatpickr';
	import 'flatpickr/dist/flatpickr.css';
	import Dropdown from '../../../../../common/components/Dropdown.svelte';
	import DropdownToggle from '../../../../../common/components/DropdownToggle.svelte';
	import DropdownMenu from '../../../../../common/components/DropdownMenu.svelte';
	import Sidebar from '../Sidebar.svelte';

	let isAddModal = false;
	const toggleAdd = () => (isAddModal = !isAddModal);

	let isDeleteModal = false;
	const toggleDelete = () => (isDeleteModal = !isDeleteModal);

	function toggleFollow(e) {
		if (e) {
			e.target.parentElement.classList.toggle('active');
		}
	}
</script>

<HeadTitle title="Event" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto relative">
	<Breadcrumb title="Event" pagetitle="Social Media" />
	<div class="grid grid-cols-1 xl:grid-cols-12 gap-x-5">
		<Sidebar link="events" />

		<div class="xl:col-span-9">
			<div class="grid items-center grid-cols-1 gap-4 mb-4 xl:grid-cols-12">
				<div class="xl:col-span-3">
					<h6 class="mb-0 text-15">Upcoming Events</h6>
				</div>
				<div class="flex gap-2 xl:col-span-4 xl:col-start-9">
					<div class="relative grow">
						<input
							type="text"
							class="ltr:pl-8 rtl:pr-8 search form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
							placeholder="Search for ..."
							autocomplete="off"
						/>
						<LucideIcon
							name="Search"
							class="inline-block size-4 absolute ltr:left-2.5 rtl:right-2.5 top-2.5 text-slate-500 dark:text-zink-200 fill-slate-100 dark:fill-zink-600"
						/>
					</div>
					<button
						on:click={toggleAdd}
						type="button"
						class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
						><LucideIcon name="Plus" class="inline-block size-4" />
						<span class="align-middle">Add Event</span></button
					>
				</div>
			</div>
			<div
				class="px-4 py-3 mb-4 text-sm text-green-500 border border-green-200 rounded-md bg-green-50 dark:bg-green-400/20 dark:border-green-500/50"
			>
				<span class="font-bold">Join us for the upcoming event:</span> Community Clean-Up Day on
				October 15, 2023
				<a
					href="#!"
					class="px-2.5 py-0.5 text-xs font-medium inline-block rounded border transition-all duration-200 ease-linear bg-green-100 border-transparent text-green-500 hover:bg-green-200 dark:bg-green-400/20 dark:hover:bg-green-400/30 dark:border-transparent ltr:ml-1 rtl:mr-1"
					>Register Now</a
				>
			</div>
			<div class="overflow-x-auto">
				<table class="w-full border-separate table-custom border-spacing-y-2 whitespace-nowrap">
					<thead class="ltr:text-left rtl:text-right">
						<tr
							class="relative bg-white rounded-md after:absolute ltr:after:border-l-2 rtl:after:border-r-2 ltr:after:left-0 rtl:after:right-0 after:top-0 after:bottom-0 after:border-transparent dark:bg-zink-700"
						>
							<th class="px-3.5 py-2.5 font-semibold sort">Event Name</th>
							<th class="px-3.5 py-2.5 font-semibold sort">Start Date</th>
							<th class="px-3.5 py-2.5 font-semibold sort">End Date</th>
							<th class="px-3.5 py-2.5 font-semibold sort">Number Registered</th>
							<th class="px-3.5 py-2.5 font-semibold sort">Total</th>
							<th class="px-3.5 py-2.5 font-semibold sort">Status</th>
							<th class="px-3.5 py-2.5 font-semibold">Action</th>
						</tr>
					</thead>
					<tbody class="list">
						{#each EventData as item}
							<tr
								class="relative bg-white rounded-md after:absolute ltr:after:border-l-2 rtl:after:border-r-2 ltr:after:left-0 rtl:after:right-0 after:top-0 after:bottom-0 after:border-transparent dark:bg-zink-700"
							>
								<td class="px-3.5 py-2.5 event_name">{item.eventName}</td>
								<td class="px-3.5 py-2.5 start_date">{item.startDate}</td>
								<td class="px-3.5 py-2.5 end_date">{item.endDate}</td>
								<td class="px-3.5 py-2.5 number">{item.numberRegistered}</td>
								<td class="px-3.5 py-2.5 total">{item.total}</td>
								<td class="px-3.5 py-2.5">
									{#if item.status == 'Ongoing'}
										<span
											class="px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-green-100 border-green-200 text-green-500 dark:bg-green-500/20 dark:border-green-500/20 status"
											>Ongoing</span
										>
									{:else if item.status == 'Draft'}
										<span
											class="px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-custom-100 border-custom-200 text-custom-500 dark:bg-custom-500/20 dark:border-custom-500/20 status"
											>Draft</span
										>
									{:else if item.status == 'Closed'}
										<span
											class="px-2.5 py-0.5 text-xs inline-block font-medium rounded border bg-red-100 border-red-200 text-red-500 dark:bg-red-500/20 dark:border-red-500/20 status"
											>Closed</span
										>
									{/if}
								</td>
								<td class="px-3.5 py-2.5">
									<Dropdown class="relative" direction="bottom-start">
										<DropdownToggle
											className="flex items-center justify-center  size-[30px] dropdown-toggle p-0 text-slate-500 btn bg-slate-100 hover:text-white hover:bg-slate-600 focus:text-white focus:bg-slate-600 focus:ring focus:ring-slate-100 active:text-white active:bg-slate-600 active:ring active:ring-slate-100 dark:bg-slate-500/20 dark:text-slate-400 dark:hover:bg-slate-500 dark:hover:text-white dark:focus:bg-slate-500 dark:focus:text-white dark:active:bg-slate-500 dark:active:text-white dark:ring-slate-400/20"
										>
											<LucideIcon name="MoreHorizontal" class="size-3" />
										</DropdownToggle>
										<DropdownMenu
											tag="ul"
											class="absolute z-50 py-2 mt-1 ltr:text-left rtl:text-right list-none bg-white rounded-md shadow-md dropdown-menu min-w-[10rem] dark:bg-zink-600"
										>
											<li>
												<a
													class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
													href="#!"
													><LucideIcon name="Eye" class="inline-block size-3 ltr:mr-1 rtl:ml-1" />
													<span class="align-middle">Overview</span></a
												>
											</li>
											<li>
												<a
													class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
													href="#!"
													on:click={toggleAdd}
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
													on:click={toggleDelete}
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
			</div>
			<div class="flex justify-center mt-3 mb-5">
				<button
					type="button"
					class="flex items-center text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
				>
					<svg
						class="size-4 ltr:mr-2 rtl:ml-2 animate-spin"
						xmlns="http://www.w3.org/2000/svg"
						fill="none"
						viewBox="0 0 24 24"
					>
						<circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"
						></circle>
						<path
							class="opacity-75"
							fill="currentColor"
							d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
						></path>
					</svg>
					Load More
				</button>
			</div>
		</div>
	</div>
</div>

<Modal modal-center className="-translate-y-2/4" isOpen={isAddModal} toggle={toggleAdd}>
	<div
		class="w-screen md:w-[30rem] bg-white shadow rounded-md dark:bg-zink-600 flex flex-col h-full"
	>
		<div class="flex items-center justify-between p-4 border-b dark:border-zink-500">
			<h5 class="text-16">Create a Event</h5>
			<button
				class="transition-all duration-200 ease-linear text-slate-400 hover:text-red-500"
				on:click={toggleAdd}><LucideIcon name="X" class="size-5" /></button
			>
		</div>
		<div class="max-h-[calc(theme('height.screen')_-_180px)] p-4 overflow-y-auto">
			<form action="#!">
				<div class="mb-4">
					<label for="eventTitle" class="inline-block mb-2 text-base font-medium">Title</label>
					<input
						type="text"
						id="eventTitle"
						class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
						placeholder="Event title"
					/>
				</div>
				<div class="mb-4">
					<label for="eventDateInput" class="inline-block mb-2 text-base font-medium"
						>Event Date</label
					>
					<!-- <input type="text" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200 flatpickr-input" id="eventDateInput" data-provider="flatpickr" data-date-format="d M, Y" readonly="readonly" placeholder="Select Date"> -->

					<Flatpickr
						dateFormat="d M, Y"
						class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200 flatpickr-input"
						placeholder="Select Date"
						readonly
					/>
				</div>
				<div class="mb-4">
					<label for="eventTimeInput" class="inline-block mb-2 text-base font-medium"
						>Event Time</label
					>
					<!-- <input type="text" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200 flatpickr-input" id="eventTimeInput" data-provider="timepickr" data-time-basic="true" placeholder="Select Time"> -->
					<Flatpickr
						dateFormat="d M, Y"
						options={{ enableTime: true, noCalendar: true }}
						class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200 flatpickr-input"
						placeholder="Select Date"
						readonly
					/>
				</div>
				<div class="mb-4">
					<label for="totalSeat" class="inline-block mb-2 text-base font-medium">Total Seat</label>
					<input
						class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
						id="totalSeat"
						type="number"
						placeholder="0"
					/>
				</div>
				<div class="mb-4">
					<label for="statusSelect" class="inline-block mb-2 text-base font-medium">Status</label>
					<select
						class="form-select border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
						id="statusSelect"
						name="statusSelect"
						data-choices
						data-choices-search-false
					>
						<option value="Ongoing">Ongoing</option>
						<option value="Draft">Draft</option>
						<option value="Closed">Closed</option>
					</select>
				</div>
				<div class="text-right">
					<button
						type="submit"
						class="text-white transition-all duration-200 ease-linear btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
						>Create Event</button
					>
				</div>
			</form>
		</div>
	</div>
</Modal>

<Modal modal-center className="-translate-y-2/4" isOpen={isDeleteModal} toggle={toggleDelete}>
	<div class="w-screen md:w-[25rem] bg-white shadow rounded-md dark:bg-zink-600">
		<div class="max-h-[calc(theme('height.screen')_-_180px)] overflow-y-auto px-6 py-8">
			<div class="float-right">
				<button
					class="transition-all duration-200 ease-linear text-slate-500 hover:text-red-500"
					on:click={toggleDelete}><LucideIcon name="X" class="size-5" /></button
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
						class="bg-white text-slate-500 btn hover:text-slate-500 hover:bg-slate-100 focus:text-slate-500 focus:bg-slate-100 active:text-slate-500 active:bg-slate-100 dark:bg-zink-600 dark:hover:bg-slate-500/10 dark:focus:bg-slate-500/10 dark:active:bg-slate-500/10"
						on:click={toggleDelete}>Cancel</button
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
