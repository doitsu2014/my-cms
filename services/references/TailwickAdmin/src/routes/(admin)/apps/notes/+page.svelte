<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import LucideIcon from '../../../../common/components/LucideIcon.svelte';
	import Modal from '../../../../common/components/Modal.svelte';
	import { NotesData } from '../../../../common/data/notes';
	import Flatpickr from 'svelte-flatpickr';
	import 'flatpickr/dist/flatpickr.css';
	import Dropdown from '../../../../common/components/Dropdown.svelte';
	import DropdownToggle from '../../../../common/components/DropdownToggle.svelte';
	import DropdownMenu from '../../../../common/components/DropdownMenu.svelte';

	let category = 'all';
	let notesList = NotesData;

	const Filter = (value) => {
		category = value;
		if (category === 'all') {
			notesList = NotesData;
		} else {
			notesList = NotesData.filter((e) => e.category == value);
		}
	};

	function isCategory(val) {
		switch (val) {
			case 'personal':
				return 'size-4 mt-1 border border-dashed rounded-full dropdown-toggle shrink-0 bg-sky-100 border-sky-500 dark:bg-sky-500/20';
			case 'business':
				return 'size-4 mt-1 border border-dashed rounded-full dropdown-toggle shrink-0 bg-orange-100 border-orange-500 dark:bg-orange-500/20';
			case 'social':
				return 'size-4 mt-1 border border-dashed rounded-full dropdown-toggle shrink-0 bg-purple-100 border-purple-500 dark:bg-purple-500/20';
			case 'home':
				return 'size-4 mt-1 border border-dashed rounded-full dropdown-toggle shrink-0 bg-green-100 border-green-500 dark:bg-green-500/20';
		}
	}

	let isAddModal = false;
	const toggleAdd = () => (isAddModal = !isAddModal);

	let isDeleteModal = false;
	const toggleDelete = () => (isDeleteModal = !isDeleteModal);
</script>

<HeadTitle title="Notes" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto relative">
	<Breadcrumb title="Notes" pagetitle="Apps" />

	<div class="card">
		<div class="card-body">
			<div class="grid grid-cols-1 gap-5 xl:grid-cols-12">
				<div class="xl:col-span-4">
					<ul class="flex flex-wrap w-full gap-2 text-sm font-medium text-center filter-btns grow">
						<li>
							<a
								href={'javascript:void(0);'}
								class="inline-block px-4 py-2 text-base transition-all duration-300 ease-linear rounded-md text-slate-500 dark:text-zink-200 border border-transparent [&.active]:bg-custom-500 dar:[&.active]:bg-custom-500 [&.active]:text-white dark:[&.active]:text-white hover:text-custom-500 dark:hover:text-custom-500 active:text-custom-500 dark:active:text-custom-500 -mb-[1px] {category ==
								'all'
									? 'active'
									: ''}"
								on:click={() => Filter('all')}>All</a
							>
						</li>
						<li>
							<a
								href={'javascript:void(0);'}
								class="inline-block px-4 py-2 text-base transition-all duration-300 ease-linear rounded-md text-slate-500 dark:text-zink-200 border border-transparent [&.active]:bg-custom-500 dar:[&.active]:bg-custom-500 [&.active]:text-white dark:[&.active]:text-white hover:text-custom-500 dark:hover:text-custom-500 active:text-custom-500 dark:active:text-custom-500 -mb-[1px] {category ==
								'business'
									? 'active'
									: ''}"
								on:click={() => Filter('business')}>Business</a
							>
						</li>
						<li>
							<a
								href={'javascript:void(0);'}
								class="inline-block px-4 py-2 text-base transition-all duration-300 ease-linear rounded-md text-slate-500 dark:text-zink-200 border border-transparent [&.active]:bg-custom-500 dar:[&.active]:bg-custom-500 [&.active]:text-white dark:[&.active]:text-white hover:text-custom-500 dark:hover:text-custom-500 active:text-custom-500 dark:active:text-custom-500 -mb-[1px] {category ==
								'social'
									? 'active'
									: ''}"
								on:click={() => Filter('social')}>Social</a
							>
						</li>
						<li>
							<a
								href={'javascript:void(0);'}
								class="inline-block px-4 py-2 text-base transition-all duration-300 ease-linear rounded-md text-slate-500 dark:text-zink-200 border border-transparent [&.active]:bg-custom-500 dar:[&.active]:bg-custom-500 [&.active]:text-white dark:[&.active]:text-white hover:text-custom-500 dark:hover:text-custom-500 active:text-custom-500 dark:active:text-custom-500 -mb-[1px] {category ==
								'home'
									? 'active'
									: ''}"
								on:click={() => Filter('home')}>Home</a
							>
						</li>
						<li>
							<a
								href={'javascript:void(0);'}
								class="inline-block px-4 py-2 text-base transition-all duration-300 ease-linear rounded-md text-slate-500 dark:text-zink-200 border border-transparent [&.active]:bg-custom-500 dar:[&.active]:bg-custom-500 [&.active]:text-white dark:[&.active]:text-white hover:text-custom-500 dark:hover:text-custom-500 active:text-custom-500 dark:active:text-custom-500 -mb-[1px] {category ==
								'personal'
									? 'active'
									: ''}"
								on:click={() => Filter('personal')}>Personal</a
							>
						</li>
					</ul>
				</div>

				<div class="xl:col-start-10 xl:col-span-3">
					<div class="flex gap-3">
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
						<div class="shrink-0">
							<button
								type="button"
								class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
								on:click={toggleAdd}
								><LucideIcon name="Plus" class="inline-block size-4" />
								<span class="align-middle">Add Note</span></button
							>
						</div>
					</div>
				</div>
				<!--end col-->
			</div>
		</div>
	</div>

	<div class="grid grid-cols-1 gap-x-5 md:grid-cols-2 xl:grid-cols-4" id="notes-list">
		{#each notesList as note}
			<div class="card product-item">
				<div class="flex flex-col h-full card-body">
					<div>
						<Dropdown className="relative ltr:float-right rtl:float-left">
							<DropdownToggle
								className="flex items-center justify-center  size-[30px] dropdown-toggle p-0 text-slate-500 btn bg-slate-100 hover:text-white hover:bg-slate-600 focus:text-white focus:bg-slate-600 focus:ring focus:ring-slate-100 active:text-white active:bg-slate-600 active:ring active:ring-slate-100 dark:bg-slate-500/20 dark:text-slate-400 dark:hover:bg-slate-500 dark:hover:text-white dark:focus:bg-slate-500 dark:focus:text-white dark:active:bg-slate-500 dark:active:text-white dark:ring-slate-400/20"
							>
								<LucideIcon name="MoreHorizontal" class="size-3" />
							</DropdownToggle>
							<DropdownMenu
								tag="ul"
								class="absolute z-50 py-2 mt-1 text-left list-none bg-white rounded-md shadow-md dropdown-menu min-w-[10rem] dark:bg-zink-600"
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
										data-edit-id="`+ datas[i].id + `"
										class="edit-item-btn block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!"
										><LucideIcon name="FileEdit" class="inline-block size-3 ltr:mr-1 rtl:ml-1" />
										<span class="align-middle">Edit</span></a
									>
								</li>
								<li>
									<a
										class="remove-item-btn block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
										href="#!"
										on:click={toggleDelete}
										><LucideIcon name="Trash2" class="inline-block size-3 ltr:mr-1 rtl:ml-1" />
										<span class="align-middle">Delete</span></a
									>
								</li>
							</DropdownMenu>
						</Dropdown>
						<div class="flex items-center gap-2 mb-5">
							<Dropdown className="relative">
								<DropdownToggle className="category-dropdown {isCategory(note.category)}"
								></DropdownToggle>
								<DropdownMenu
									tag="ul"
									class="absolute z-50 py-2 mt-1 text-left list-none bg-white rounded-md shadow-md dropdown-menu min-w-[10rem] dark:bg-zink-600"
								>
									<li>
										<a
											class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
											href="#!">Personal</a
										>
									</li>
									<li>
										<a
											class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
											href="#!">Business</a
										>
									</li>
									<li>
										<a
											class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
											href="#!">Social</a
										>
									</li>
									<li>
										<a
											class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
											href="#!">Home</a
										>
									</li>
								</DropdownMenu>
							</Dropdown>
							<h5 class="text-16">{note.title}</h5>
						</div>
					</div>

					<div class="js-read-smore" data-read-smore-words="40" data-read-smore-inline="true">
						<p class="text-slate-500 dark:text-zink-200">
							{@html note.description}
						</p>
					</div>

					<div class="flex items-center justify-between gap-3 pt-4 mt-auto">
						<div class="shrink-0">
							<a href="#!" class="group/item toggle-button {note.isActive ? 'active' : ''}"
								><LucideIcon
									name="Star"
									class="size-5 text-slate-500 dark:text-zink-200 dark:fill-zink-600 fill-slate-200 transition-all duration-150 ease-linear group-[.active]/item:text-yellow-500 dark:group-[.active]/item:text-yellow-500 group-[.active]/item:fill-yellow-200 dark:group-[.active]/item:fill-yellow-500/50 group-hover/item:text-yellow-500 dark:group-hover/item:text-yellow-500 group-hover/item:fill-yellow-200 dark:group-hover/item:fill-yellow-500/50"
								/></a
							>
						</div>
						<p class="text-slate-500 dark:text-zink-200 shrink-0">{note.date}</p>
					</div>
				</div>
			</div>
		{/each}
	</div>

	<div class="flex flex-col items-center gap-5 mb-5 md:flex-row" id="paginationItems">
		<div class="grow">
			<p class="text-slate-500 dark:text-zink-200">
				Showing <b>{notesList.length}</b> of <b>{NotesData.length}</b> Results
			</p>
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
					><LucideIcon class="size-4 rtl:rotate-180" name="ChevronRight" /></a
				>
			</li>
		</ul>
	</div>
</div>

<Modal modal-center className="-translate-y-2/4" isOpen={isAddModal} toggle={toggleAdd}>
	<div class="w-screen xl:w-[55rem] bg-white shadow rounded-md dark:bg-zink-600">
		<div class="flex items-center justify-between p-5 border-b dark:border-zink-500">
			<h5 class="text-16" id="addNewNoteLabel">Add Note</h5>
			<button
				data-modal-close="addNotesModal"
				id="notesModal-close"
				class="transition-all duration-200 ease-linear text-slate-400 hover:text-red-500"
				on:click={toggleAdd}><LucideIcon name="X" class="size-5" /></button
			>
		</div>
		<div class="max-h-[calc(theme('height.screen')_-_180px)] overflow-y-auto p-5">
			<form novalidate class="create-form">
				<input type="hidden" value="" name="id" id="id" />
				<input type="hidden" value="add" name="action" id="action" />
				<input type="hidden" id="id-field" />
				<div
					id="alert-error-msg"
					class="hidden px-4 py-3 text-sm text-red-500 border border-transparent rounded-md bg-red-50 dark:bg-red-400/20"
				></div>
				<div class="grid grid-cols-1 gap-5 xl:grid-cols-12">
					<div class="xl:col-span-4">
						<label for="createDateInput" class="inline-block mb-2 text-base font-medium"
							>Create Date</label
						>
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
					<div class="xl:col-span-4">
						<label for="notesTitleInput" class="inline-block mb-2 text-base font-medium"
							>Note Title</label
						>
						<input
							type="text"
							id="notesTitleInput"
							class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
							placeholder="Title"
							required
						/>
					</div>
					<div class="xl:col-span-4">
						<div>
							<label for="categorySelect" class="inline-block mb-2 text-base font-medium"
								>Category</label
							>
							<select
								class="form-select border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
								data-choices
								data-choices-search-false
								name="categorySelect"
								id="statusSelect"
							>
								<option value="">Select Category</option>
								<option value="business">Business</option>
								<option value="personal">Personal</option>
								<option value="home">Home</option>
								<option value="social">Social</option>
							</select>
						</div>
					</div>
					<div class="xl:col-span-12">
						<div>
							<label for="textArea" class="inline-block mb-2 text-base font-medium"
								>Description</label
							>
							<textarea
								class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
								id="textArea"
								rows="6"
							></textarea>
						</div>
					</div>
				</div>

				<div class="flex justify-end gap-2 mt-4">
					<button
						type="reset"
						data-modal-close="addNotesModal"
						class="text-red-500 bg-white btn hover:text-red-500 hover:bg-red-100 focus:text-red-500 focus:bg-red-100 active:text-red-500 active:bg-red-100 dark:bg-zink-600 dark:hover:bg-red-500/10 dark:focus:bg-red-500/10 dark:active:bg-red-500/10"
						on:click={toggleAdd}>Cancel</button
					>
					<button
						type="submit"
						id="addNew"
						class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
						>Add Note</button
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
