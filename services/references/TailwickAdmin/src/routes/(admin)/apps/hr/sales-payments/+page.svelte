<script>
	import HeadTitle from '../../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../../common/components/Breadcrumb.svelte';
	import LucideIcon from '../../../../../common/components/LucideIcon.svelte';
	import CounterPage from '../../../../../common/components/CounterPage.svelte';
	import Modal from '../../../../../common/components/Modal.svelte';
	import Dropdown from '../../../../../common/components/Dropdown.svelte';
	import DropdownMenu from '../../../../../common/components/DropdownMenu.svelte';
	import DropdownToggle from '../../../../../common/components/DropdownToggle.svelte';
	import { PaymentsData } from '../../../../../common/data/hrmanagement';
	import Flatpickr from 'svelte-flatpickr';
	import 'flatpickr/dist/flatpickr.css';

	let isDeleteModal = false;
	const toggleDelete = () => (isDeleteModal = !isDeleteModal);

	let isAddModal = false;
	const toggleAdd = () => (isAddModal = !isAddModal);
</script>

<HeadTitle title="Payments" />

<CounterPage />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto relative">
	<Breadcrumb title="Payments" pagetitle="Sales" />

	<div class="card" id="ordersTable">
		<div class="card-body">
			<div class="grid grid-cols-1 gap-4 mb-5 lg:grid-cols-2 xl:grid-cols-12">
				<div class="xl:col-span-3">
					<div class="relative">
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
				</div>
				<!--end col-->
				<div class="xl:col-span-2">
					<Flatpickr
						type="text"
						options={{
							dateFormat: 'd M, Y'
						}}
						id="fromInput"
						class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
						placeholder="Select date"
					/>
				</div>
				<!--end col-->
				<div class="xl:col-span-2 xl:col-start-11">
					<select
						class="form-select border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
						data-choices
						data-choices-search-false
						name="statusFilterSelect"
						id="statusFilterSelect"
					>
						<option value="Paid">Paid</option>
						<option value="Pending">Pending</option>
						<option value="Failed">Failed</option>
					</select>
				</div>
			</div>
			<!--col grid-->
			<div class="-mx-5 overflow-x-auto">
				<table class="w-full whitespace-nowrap">
					<thead
						class="ltr:text-left rtl:text-right bg-slate-100 text-slate-500 dark:text-zink-200 dark:bg-zink-600"
					>
						<tr>
							<th
								class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold border-b border-slate-200 dark:border-zink-500"
								>Payment ID</th
							>
							<th
								class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold border-b border-slate-200 dark:border-zink-500"
								>Membership Plan</th
							>
							<th
								class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold border-b border-slate-200 dark:border-zink-500"
								>Date</th
							>
							<th
								class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold border-b border-slate-200 dark:border-zink-500"
								>Payment Type</th
							>
							<th
								class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold border-b border-slate-200 dark:border-zink-500"
								>Username</th
							>
							<th
								class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold border-b border-slate-200 dark:border-zink-500"
								>Amount</th
							>
							<th
								class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold border-b border-slate-200 dark:border-zink-500"
								>Status</th
							>
						</tr>
					</thead>
					<tbody>
						{#each PaymentsData as row}
							<tr>
								<td
									class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
									><a
										href="#!"
										class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"
										>{row.paymentId}</a
									></td
								>
								<td
									class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
									>{row.membershipPlan}</td
								>
								<td
									class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
									>{row.date}</td
								>
								<td
									class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
									>{row.paymentType}</td
								>
								<td
									class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
									>{row.username}</td
								>
								<td
									class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
									>{row.amount}</td
								>
								<td
									class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
								>
									{#if row.status == 'Paid'}
										<span
											class="px-2.5 py-0.5 inline-block text-xs font-medium rounded border bg-green-100 border-transparent text-green-500 dark:bg-green-500/20 dark:border-transparent"
										>
											<span
												class=" size-1.5 ltr:mr-1 rtl:ml-1 rounded-full bg-green-500 inline-block"
											></span> Paid
										</span>
									{:else if row.status == 'Pending'}
										<span
											class="px-2.5 py-0.5 inline-block text-xs font-medium rounded border bg-yellow-100 border-transparent text-yellow-500 dark:bg-yellow-500/20 dark:border-transparent"
										>
											<span
												class=" size-1.5 ltr:mr-1 rtl:ml-1 rounded-full bg-yellow-500 inline-block"
											></span> Pending
										</span>
									{:else if row.status == 'Failed'}
										<span
											class="px-2.5 py-0.5 inline-block text-xs font-medium rounded border bg-red-100 border-transparent text-red-500 dark:bg-red-500/20 dark:border-transparent"
										>
											<span class=" size-1.5 ltr:mr-1 rtl:ml-1 rounded-full bg-red-500 inline-block"
											></span> Failed
										</span>
									{/if}
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
			<div class="flex flex-col items-center mt-5 md:flex-row">
				<div class="mb-4 grow md:mb-0">
					<p class="text-slate-500 dark:text-zink-200">Showing <b>6</b> of <b>8</b> Results</p>
				</div>
				<ul class="flex flex-wrap items-center gap-2 shrink-0">
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 h-8 px-3 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-500 dark:[&.active]:text-custom-500 [&.active]:bg-custom-50 dark:[&.active]:bg-custom-500/10 [&.active]:border-custom-50 dark:[&.active]:border-custom-500/10 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
							><LucideIcon class="size-4 mr-1 rtl:rotate-180" name="ChevronLeft" /> Prev</a
						>
					</li>
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-500 dark:[&.active]:text-custom-500 [&.active]:bg-custom-50 dark:[&.active]:bg-custom-500/10 [&.active]:border-custom-50 dark:[&.active]:border-custom-500/10 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
							>1</a
						>
					</li>
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-500 dark:[&.active]:text-custom-500 [&.active]:bg-custom-50 dark:[&.active]:bg-custom-500/10 [&.active]:border-custom-50 dark:[&.active]:border-custom-500/10 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto active"
							>2</a
						>
					</li>
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-500 dark:[&.active]:text-custom-500 [&.active]:bg-custom-50 dark:[&.active]:bg-custom-500/10 [&.active]:border-custom-50 dark:[&.active]:border-custom-500/10 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
							>3</a
						>
					</li>
					<li>
						<a
							href="#!"
							class="inline-flex items-center justify-center bg-white dark:bg-zink-700 h-8 px-3 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-50 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-custom-500 dark:[&.active]:text-custom-500 [&.active]:bg-custom-50 dark:[&.active]:bg-custom-500/10 [&.active]:border-custom-50 dark:[&.active]:border-custom-500/10 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
							>Next <LucideIcon class="size-4 ml-1 rtl:rotate-180" name="ChevronRight" /></a
						>
					</li>
				</ul>
			</div>
		</div>
	</div>
</div>

<Modal modal-center className="-translate-y-2/4" isOpen={isDeleteModal} toggle={toggleDelete}>
	<div class="w-screen md:w-[25rem] bg-white shadow rounded-md dark:bg-zink-600">
		<div class="max-h-[calc(theme('height.screen')_-_180px)] overflow-y-auto px-6 py-8">
			<div class="float-right">
				<button
					data-modal-close="deleteModal"
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
						data-modal-close="deleteModal"
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

<Modal modal-center className="-translate-y-2/4" isOpen={isAddModal} toggle={toggleAdd}>
	<div class="w-screen md:w-[30rem] bg-white shadow rounded-md dark:bg-zink-600">
		<div class="flex items-center justify-between p-4 border-b dark:border-zink-500">
			<h5 class="text-16">Add Estimate</h5>
			<button
				data-modal-close="addEstimateModal"
				class="transition-all duration-200 ease-linear text-slate-400 hover:text-red-500"
				><LucideIcon name="x" class="size-5" /></button
			>
		</div>
		<div class="max-h-[calc(theme('height.screen')_-_180px)] p-4 overflow-y-auto">
			<form action="#!">
				<div class="grid grid-cols-1 gap-4 xl:grid-cols-12">
					<div class="xl:col-span-12">
						<label for="estimateInput" class="inline-block mb-2 text-base font-medium"
							>Estimate Number</label
						>
						<input
							type="text"
							id="estimateInput"
							class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
							placeholder="Estimate Number"
							value="#TWE20015420"
							disabled
						/>
					</div>
					<div class="xl:col-span-12">
						<label for="clientNameInput" class="inline-block mb-2 text-base font-medium"
							>Client Name</label
						>
						<input
							type="text"
							id="clientNameInput"
							class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
							placeholder="Client name"
						/>
					</div>
					<div class="xl:col-span-12">
						<label for="estimateBySelect" class="inline-block mb-2 text-base font-medium"
							>Estimate By</label
						>
						<select
							class="form-input border-slate-200 focus:outline-none focus:border-custom-500"
							data-choices
							data-choices-search-false
							name="estimateBySelect"
							id="estimateBySelect"
						>
							<option value="HR">HR</option>
							<option value="Admin">Admin</option>
						</select>
					</div>
					<div class="xl:col-span-6">
						<label for="estimateDateInput" class="inline-block mb-2 text-base font-medium"
							>Estimate Date</label
						>
						<Flatpickr
							type="text"
							options={{
								dateFormat: 'd M, Y'
							}}
							id="fromInput"
							class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
							placeholder="Select Date"
						/>
					</div>
					<div class="xl:col-span-6">
						<label for="expiryDateInput" class="inline-block mb-2 text-base font-medium"
							>Expiry Date</label
						>
						<Flatpickr
							type="text"
							options={{
								dateFormat: 'd M, Y'
							}}
							id="fromInput"
							class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
							placeholder="Select Date"
						/>
					</div>
					<div class="xl:col-span-12">
						<label for="amountInput" class="inline-block mb-2 text-base font-medium">Amount</label>
						<input
							type="number"
							id="amountInput"
							class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
							placeholder="$00.00"
						/>
					</div>
					<div class="xl:col-span-12">
						<label for="statusSelect" class="inline-block mb-2 text-base font-medium">Status</label>
						<select
							class="form-input border-slate-200 focus:outline-none focus:border-custom-500"
							name="statusSelect"
							id="statusSelect"
						>
							<option value="Accepted">Accepted</option>
							<option value="Declined">Declined</option>
							<option value="Expired">Expired</option>
						</select>
					</div>
				</div>
				<div class="flex justify-end gap-2 mt-4">
					<button
						type="reset"
						class="text-red-500 bg-white btn hover:text-red-500 hover:bg-red-100 focus:text-red-500 focus:bg-red-100 active:text-red-500 active:bg-red-100 dark:bg-zink-600 dark:hover:bg-red-500/10 dark:focus:bg-red-500/10 dark:active:bg-red-500/10"
						on:click={toggleAdd}>Cancel</button
					>
					<button
						type="submit"
						class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
						>Add Estimate</button
					>
				</div>
			</form>
		</div>
	</div>
</Modal>
