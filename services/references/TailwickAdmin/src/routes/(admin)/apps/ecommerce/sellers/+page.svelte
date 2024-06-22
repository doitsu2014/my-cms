<script>
	import HeadTitle from '../../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../../common/components/Breadcrumb.svelte';
	import LucideIcon from '../../../../../common/components/LucideIcon.svelte';
	import Dropdown from '../../../../../common/components/Dropdown.svelte';
	import DropdownToggle from '../../../../../common/components/DropdownToggle.svelte';
	import DropdownMenu from '../../../../../common/components/DropdownMenu.svelte';
	import Modal from '../../../../../common/components/Modal.svelte';
	import data from '../../../../../common/data/sellers';
	import Dropzone from 'svelte-file-dropzone/Dropzone.svelte';

	let files = {
		accepted: [],
		rejected: [],
		preview: []
	};

	function handleFilesSelect(e) {
		const { acceptedFiles, fileRejections } = e.detail;
		files.accepted = [...files.accepted, ...acceptedFiles];
		files.rejected = [...files.rejected, ...fileRejections];

		const uploadedfiles = event.target.files;

		for (var i = 0; i < uploadedfiles.length; i++) {
			const file = uploadedfiles[i];
			if (file) {
				const reader = new FileReader();

				reader.onload = (e) => {
					files.preview = [...files.preview, e.target.result];
				};

				reader.readAsDataURL(file);
			}
		}
	}

	function handleRemoveFile(e, index) {
		files.accepted.splice(index, 1);
		files.preview.splice(index, 1);
		files.accepted = [...files.accepted];
	}

	let isDeleteModal = false;
	const toggleDelete = () => (isDeleteModal = !isDeleteModal);

	let isAddModal = false;
	const toggleAddModal = () => (isAddModal = !isAddModal);
</script>

<HeadTitle title="Sellers" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto relative">
	<Breadcrumb title="Sellers" pagetitle="Ecommerce" />

	<form action="#!" class="mb-5">
		<div class="grid grid-cols-1 gap-5 lg:grid-cols-12">
			<div class="relative lg:col-span-3">
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
			<!--end col-->
			<div class="ltr:lg:text-right rtl:lg:text-left lg:col-span-3 lg:col-start-10">
				<button
					type="button"
					class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
					on:click={toggleAddModal}
					><LucideIcon name="Plus" class="inline-block size-4" />
					<span class="align-middle">Add Seller</span></button
				>
			</div>
			<!--end col-->
		</div>
		<!--end grid-->
	</form>

	<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 2xl:grid-cols-12 gap-x-5">
		{#each data.sellerlist as seller}
			<div class="2xl:col-span-3">
				<div class="card">
					<div class="card-body">
						<div class="flex items-center gap-2 mb-4">
							<div class="grow">
								<a href="#!" class="group/item toggle-button {seller.isfavorite ? 'active' : ''}">
									<LucideIcon
										name="Heart"
										class="size-5 text-slate-500 dark:text-zink-200 fill-slate-200 dark:fill-zink-500 transition-all duration-150 ease-linear group-[.active]/item:text-yellow-500 dark:group-[.active]/item:text-yellow-500 group-[.active]/item:fill-yellow-200 dark:group-[.active]/item:fill-yellow-500/20 group-hover/item:text-yellow-500 dark:group-hover/item:text-yellow-500 group-hover/item:fill-yellow-200 dark:group-hover/item:fill-yellow-500/20"
									/></a
								>
							</div>
							<Dropdown class="relative shrink-0" direction="bottom-start">
								<DropdownToggle
									className="flex items-center justify-center  size-[30px] dropdown-toggle p-0 text-slate-500 btn bg-slate-100 hover:text-white hover:bg-slate-600 focus:text-white focus:bg-slate-600 focus:ring focus:ring-slate-100 active:text-white active:bg-slate-600 active:ring active:ring-slate-100 dark:bg-slate-500/20 dark:text-slate-400 dark:hover:bg-slate-500 dark:hover:text-white dark:focus:bg-slate-500 dark:focus:text-white dark:active:bg-slate-500 dark:active:text-white dark:ring-slate-400/20"
								>
									<LucideIcon name="MoreHorizontal" class=" size-3" />
								</DropdownToggle>
								<DropdownMenu
									tag="ul"
									class="absolute z-50 py-2 mt-1 ltr:text-left rtl:text-right list-none bg-white rounded-md shadow-md dropdown-menu min-w-[10rem] dark:bg-zink-600"
								>
									<li>
										<a
											class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
											href="#!"
											><LucideIcon name="Eye" class="inline-block size-3 mr-1" />
											<span class="align-middle">Overview</span></a
										>
									</li>
									<li>
										<a
											class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
											href="#!"
											><LucideIcon name="FileEdit" class="inline-block size-3 mr-1" />
											<span class="align-middle">Edit</span></a
										>
									</li>
									<li>
										<a
											class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
											href="#!"
											on:click={toggleDelete}
											><LucideIcon name="Trash2" class="inline-block size-3 mr-1" />
											<span class="align-middle">Delete</span></a
										>
									</li>
								</DropdownMenu>
							</Dropdown>
						</div>
						<div
							class="flex items-center justify-center size-16 mx-auto rounded-full bg-slate-100 outline outline-slate-100 outline-offset-1 dark:bg-zink-600 dark:outline-zink-600"
						>
							<img src={seller.img} alt="" class="h-10 rounded-full" />
						</div>

						<div class="mt-4 mb-6 text-center">
							<h6 class="text-16"><a href="#!">{seller.companyname}</a></h6>
							<p class="text-slate-500 dark:text-zink-200">{seller.sallername}</p>
						</div>
						<div
							class="grid grid-cols-1 gap-5 text-center divide-y md:divide-y-0 md:divide-x sm:grid-cols-3 2xl:grid-cols-12 divide-slate-200 divide-dashed dark:divide-zink-500 rtl:divide-x-reverse"
						>
							<div class="p-2 2xl:col-span-4">
								<h6 class="mb-1">{seller.sales}</h6>
								<p class="text-slate-500 dark:text-zink-200">Sales</p>
							</div>
							<!--end col-->
							<div class="p-2 2xl:col-span-4">
								<h6 class="mb-1">{seller.products}</h6>
								<p class="text-slate-500 dark:text-zink-200">Product</p>
							</div>
							<!--end col-->
							<div class="p-2 2xl:col-span-4">
								<h6 class="mb-1">{seller.revenue}</h6>
								<p class="text-slate-500 dark:text-zink-200">Revenue</p>
							</div>
							<!--end col-->
						</div>
						<!--end grid-->
					</div>
				</div>
				<!--end card-->
			</div>
		{/each}
	</div>

	<div class="flex flex-col items-center mb-5 md:flex-row">
		<div class="mb-4 grow md:mb-0">
			<p class="text-slate-500 dark:text-zink-200">Showing <b>12</b> of <b>44</b> Results</p>
		</div>
		<ul class="flex flex-wrap items-center gap-2 shrink-0">
			<li>
				<a
					href="#!"
					class="inline-flex items-center justify-center bg-white dark:bg-zink-700 h-8 px-3 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-100 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-white dark:[&.active]:text-white [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
					><LucideIcon class="size-4 mr-1 rtl:rotate-180" name="ChevronLeft" /> Prev</a
				>
			</li>
			<li>
				<a
					href="#!"
					class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-100 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-white dark:[&.active]:text-white [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
					>1</a
				>
			</li>
			<li>
				<a
					href="#!"
					class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-100 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-white dark:[&.active]:text-white [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto active"
					>2</a
				>
			</li>
			<li>
				<a
					href="#!"
					class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-100 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-white dark:[&.active]:text-white [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
					>3</a
				>
			</li>
			<li>
				<a
					href="#!"
					class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-100 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-white dark:[&.active]:text-white [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
					>4</a
				>
			</li>
			<li>
				<a
					href="#!"
					class="inline-flex items-center justify-center bg-white dark:bg-zink-700 size-8 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-100 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-white dark:[&.active]:text-white [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
					>5</a
				>
			</li>
			<li>
				<a
					href="#!"
					class="inline-flex items-center justify-center bg-white dark:bg-zink-700 h-8 px-3 transition-all duration-150 ease-linear border rounded border-slate-200 dark:border-zink-500 text-slate-500 dark:text-zink-200 hover:text-custom-500 dark:hover:text-custom-500 hover:bg-custom-100 dark:hover:bg-custom-500/10 focus:bg-custom-50 dark:focus:bg-custom-500/10 focus:text-custom-500 dark:focus:text-custom-500 [&.active]:text-white dark:[&.active]:text-white [&.active]:bg-custom-500 dark:[&.active]:bg-custom-500 [&.active]:border-custom-500 dark:[&.active]:border-custom-500 [&.active]:hover:text-custom-700 dark:[&.active]:hover:text-custom-700 [&.disabled]:text-slate-400 dark:[&.disabled]:text-zink-300 [&.disabled]:cursor-auto"
					>Next <LucideIcon class="size-4 ml-1 rtl:rotate-180" name="ChevronRight" /></a
				>
			</li>
		</ul>
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

<Modal modal-center className="-translate-y-2/4" isOpen={isAddModal} toggle={toggleAddModal}>
	<div class="w-screen md:w-[30rem] bg-white shadow rounded-md dark:bg-zink-600">
		<div class="flex items-center justify-between p-4 border-b dark:border-zink-500">
			<h5 class="text-16">Add Seller</h5>
			<button
				data-modal-close="addSellerModal"
				class="transition-all duration-200 ease-linear text-slate-400 hover:text-red-500"
				><LucideIcon name="x" class="size-5" /></button
			>
		</div>
		<div class="max-h-[calc(theme('height.screen')_-_180px)] p-4 overflow-y-auto">
			<form action="#!">
				<div class="mb-3">
					<label for="companyLogo" class="inline-block mb-2 text-base font-medium"
						>Company Logo</label
					>
					<Dropzone
						on:drop={handleFilesSelect}
						containerClasses="flex items-center justify-center border rounded-md cursor-pointer !bg-slate-100 dropzone !border-slate-200 dark:!bg-zink-600 dark:!border-zink-500 dz-clickable"
					>
						<div class="w-full py-5 text-lg text-center dz-message needsclick">
							<div class="mb-3">
								<LucideIcon
									name="UploadCloud"
									class="block size-12 mx-auto text-slate-500 fill-slate-200 dark:text-zink-200 dark:fill-zink-500"
								/>
							</div>

							<h5 class="mb-0 font-normal text-slate-500 dark:text-zink-200 text-15">
								Drag and drop your logo or <a href="#!">browse</a> your logo
							</h5>
						</div>
					</Dropzone>
					<ul class="flex flex-wrap mb-0 gap-x-5" id="dropzone-preview2">
						{#each files.accepted as item, index}
							<li class="mt-5" id="dropzone-preview-list2">
								<div class="border rounded border-slate-200 dark:border-zink-500">
									<div class="p-2 text-center">
										<div>
											<div class="p-2 mx-auto rounded-md size-14 bg-slate-100 dark:bg-zink-600">
												<!-- svelte-ignore a11y-img-redundant-alt -->
												<img
													class="block w-full h-full rounded-md"
													src={files.preview[index]}
													alt="Dropzone-Image"
												/>
											</div>
										</div>
										<div class="pt-3">
											<h5 class="mb-1 text-15">{item.name}</h5>
											<p class="mb-0 text-slate-500 dark:text-zink-200">
												{(item.size / 1024).toFixed(2)} KB
											</p>
											<strong class="error text-danger"></strong>
										</div>
										<div class="mt-2">
											<button
												class="px-2 py-1.5 text-xs text-white bg-red-500 border-red-500 btn hover:text-white hover:bg-red-600 hover:border-red-600 focus:text-white focus:bg-red-600 focus:border-red-600 focus:ring focus:ring-red-100 active:text-white active:bg-red-600 active:border-red-600 active:ring active:ring-red-100 dark:ring-custom-400/20"
												on:click={(e) => handleRemoveFile(e, index)}>Delete</button
											>
										</div>
									</div>
								</div>
							</li>
						{/each}
					</ul>
				</div>
				<div class="mb-3">
					<label for="companyNameInput" class="inline-block mb-2 text-base font-medium"
						>Company Name</label
					>
					<input
						type="text"
						id="companyNameInput"
						class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
						placeholder="Seller name"
						required
					/>
				</div>
				<div class="mb-3">
					<label for="ownerName" class="inline-block mb-2 text-base font-medium">Owner Name</label>
					<input
						type="text"
						id="ownerName"
						class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
						placeholder="Owner name"
						required
					/>
				</div>
				<div class="flex justify-end gap-2 mt-4">
					<button
						type="reset"
						data-modal-close="addSellerModal"
						class="text-red-500 bg-white btn hover:text-red-500 hover:bg-red-100 focus:text-red-500 focus:bg-red-100 active:text-red-500 active:bg-red-100 dark:bg-zink-600 dark:hover:bg-red-500/10 dark:focus:bg-red-500/10 dark:active:bg-red-500/10"
						>Cancel</button
					>
					<button
						type="submit"
						class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
						>Add Seller</button
					>
				</div>
			</form>
		</div>
	</div>
</Modal>
