<script>
	import HeadTitle from '../../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../../common/components/Breadcrumb.svelte';
	import LucideIcon from '../../../../../common/components/LucideIcon.svelte';
	import Modal from '../../../../../common/components/Modal.svelte';
	import { DepartmentsListData } from '../../../../../common/data/hrmanagement';

	let isAddModal = false;
	const toggleAdd = () => (isAddModal = !isAddModal);

	let isDeleteModal = false;
	const toggleDelete = () => (isDeleteModal = !isDeleteModal);
</script>

<HeadTitle title="Departments" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto relative">
	<Breadcrumb title="Departments" pagetitle="HR Management" />

	<div class="card" id="ordersTable">
		<div class="card-body">
			<div class="flex items-center gap-3 mb-4">
				<h6 class="text-15 grow">Departments</h6>
				<div class="shrink-0">
					<a
						href="#!"
						type="button"
						class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
						on:click={toggleAdd}
						><LucideIcon name="Plus" class="inline-block size-4" />
						<span class="align-middle">Add Department</span></a
					>
				</div>
			</div>
			<div class="overflow-x-auto">
				<table class="w-full whitespace-nowrap">
					<thead class="ltr:text-left rtl:text-right">
						<tr>
							<th class="px-3.5 py-2.5 font-semibold border border-slate-200 dark:border-zink-500"
								>#</th
							>
							<th class="px-3.5 py-2.5 font-semibold border border-slate-200 dark:border-zink-500"
								>Department Name</th
							>
							<th class="px-3.5 py-2.5 font-semibold border border-slate-200 dark:border-zink-500"
								>Head of Dep.</th
							>
							<th class="px-3.5 py-2.5 font-semibold border border-slate-200 dark:border-zink-500"
								>Phone Number</th
							>
							<th class="px-3.5 py-2.5 font-semibold border border-slate-200 dark:border-zink-500"
								>Email</th
							>
							<th class="px-3.5 py-2.5 font-semibold border border-slate-200 dark:border-zink-500"
								>Employee</th
							>
							<th class="px-3.5 py-2.5 font-semibold border border-slate-200 dark:border-zink-500"
								>Action</th
							>
						</tr>
					</thead>
					<tbody class="">
						{#each DepartmentsListData as deapartment}
							<tr>
								<td class="px-3.5 py-2.5 border border-slate-200 dark:border-zink-500"
									>{deapartment.id}</td
								>
								<td class="px-3.5 py-2.5 border border-slate-200 dark:border-zink-500"
									>{deapartment.departmentName}</td
								>
								<td class="px-3.5 py-2.5 border border-slate-200 dark:border-zink-500"
									>{deapartment.HOD}</td
								>
								<td class="px-3.5 py-2.5 border border-slate-200 dark:border-zink-500"
									>{deapartment.phone}</td
								>
								<td class="px-3.5 py-2.5 border border-slate-200 dark:border-zink-500"
									>{deapartment.email}</td
								>
								<td class="px-3.5 py-2.5 border border-slate-200 dark:border-zink-500"
									>{deapartment.employee}</td
								>
								<td class="px-3.5 py-2.5 border border-slate-200 dark:border-zink-500">
									<div class="flex gap-2">
										<a
											href="#!"
											class="flex items-center justify-center size-8 transition-all duration-200 ease-linear rounded-md bg-slate-100 dark:bg-zink-600 dark:text-zink-200 text-slate-500 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-100 dark:hover:bg-custom-500/20"
											on:click={toggleAdd}><LucideIcon name="Pencil" class="size-4" /></a
										>
										<a
											href="#!"
											class="flex items-center justify-center size-8 transition-all duration-200 ease-linear rounded-md bg-slate-100 dark:bg-zink-600 dark:text-zink-200 text-slate-500 hover:text-red-500 dark:hover:text-red-500 hover:bg-red-100 dark:hover:bg-red-500/20"
											on:click={toggleDelete}><LucideIcon name="Trash2" class="size-4" /></a
										>
									</div>
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
			<div class="flex flex-col items-center mt-5 md:flex-row">
				<div class="mb-4 grow md:mb-0">
					<p class="text-slate-500 dark:text-zink-200">Showing <b>5</b> of <b>8</b> Results</p>
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

<Modal modal-center className="-translate-y-2/4" isOpen={isAddModal} toggle={toggleAdd}>
	<div class="w-screen md:w-[30rem] bg-white shadow rounded-md dark:bg-zink-600">
		<div class="flex items-center justify-between p-4 border-b dark:border-zink-500">
			<h5 class="text-16">Add Department</h5>
			<button
				class="transition-all duration-200 ease-linear text-slate-400 hover:text-red-500"
				on:click={toggleAdd}><LucideIcon name="X" class="size-5" /></button
			>
		</div>
		<div class="max-h-[calc(theme('height.screen')_-_180px)] p-4 overflow-y-auto">
			<form action="#!">
				<div class="grid grid-cols-1 gap-4 xl:grid-cols-12">
					<div class="xl:col-span-12">
						<label for="departmentInput" class="inline-block mb-2 text-base font-medium"
							>Department Name</label
						>
						<input
							type="text"
							id="departmentInput"
							class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
							placeholder="Department name"
						/>
					</div>
					<div class="xl:col-span-12">
						<label for="headOfInput" class="inline-block mb-2 text-base font-medium"
							>Head of Dep. Name</label
						>
						<input
							type="text"
							id="headOfInput"
							class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
							placeholder="Head name"
						/>
					</div>
					<div class="xl:col-span-12">
						<label for="phoneNumberInput" class="inline-block mb-2 text-base font-medium"
							>Phone Number</label
						>
						<input
							type="number"
							id="phoneNumberInput"
							class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
							placeholder="(012) 1234 562 3145"
						/>
					</div>
					<div class="xl:col-span-12">
						<label for="emailInput" class="inline-block mb-2 text-base font-medium">Email</label>
						<input
							type="text"
							id="emailInput"
							class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
							placeholder="Enter email"
						/>
					</div>
					<div class="xl:col-span-12">
						<label for="employeeNumberInput" class="inline-block mb-2 text-base font-medium"
							>Total Employee</label
						>
						<input
							type="number"
							id="employeeNumberInput"
							class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
							placeholder="0"
						/>
					</div>
				</div>
				<div class="flex justify-end gap-2 mt-4">
					<button
						type="reset"
						data-modal-close="addDepartmentModal"
						class="text-red-500 bg-white btn hover:text-red-500 hover:bg-red-100 focus:text-red-500 focus:bg-red-100 active:text-red-500 active:bg-red-100 dark:bg-zink-600 dark:hover:bg-red-500/10 dark:focus:bg-red-500/10 dark:active:bg-red-500/10"
						on:click={toggleAdd}>Cancel</button
					>
					<button
						type="submit"
						class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
						>Add Department</button
					>
				</div>
			</form>
		</div>
	</div>
</Modal>
