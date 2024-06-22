<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import LucideIcon from '../../../../common/components/LucideIcon.svelte';
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

	let files2 = {
		accepted: [],
		rejected: [],
		preview: []
	};

	function handleFilesSelect2(e) {
		const { acceptedFiles, fileRejections } = e.detail;
		files2.accepted = [...files2.accepted, ...acceptedFiles];
		files2.rejected = [...files2.rejected, ...fileRejections];

		const uploadedfiles = event.target.files;

		for (var i = 0; i < uploadedfiles.length; i++) {
			const file = uploadedfiles[i];
			if (file) {
				const reader = new FileReader();

				reader.onload = (e) => {
					files2.preview = [...files2.preview, e.target.result];
				};

				reader.readAsDataURL(file);
			}
		}
	}

	function handleRemoveFile2(e, index) {
		files2.accepted.splice(index, 1);
		files2.preview.splice(index, 1);
		files2.accepted = [...files2.accepted];
	}
</script>

<HeadTitle title="File Upload" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="File Upload" pagetitle="Forms" />

	<div class="card">
		<div class="card-body">
			<h6 class="mb-4 text-15">Dropzone</h6>
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

					<h5 class="mb-0 font-normal text-slate-500 text-15">
						Drag and drop your files or <a href="#!">browse</a> your files
					</h5>
				</div>
			</Dropzone>

			<ul class="mb-0" id="dropzone-preview">
				{#each files.accepted as item, index}
					<li class="mt-2" id="dropzone-preview-list">
						<!-- This is used as the file preview template -->
						<div class="border rounded border-slate-200 dark:border-zink-500">
							<div class="flex p-2">
								<div class="shrink-0 me-3">
									<div class="p-2 rounded-md size-14 bg-slate-100 dark:bg-zink-600">
										<!-- svelte-ignore a11y-img-redundant-alt -->
										<img
											class="block w-full h-full rounded-md"
											src={files.preview[index]}
											alt="Dropzone-Image"
										/>
									</div>
								</div>
								<div class="grow">
									<div class="pt-1">
										<h5 class="mb-1 text-15">{item.name}</h5>
										<p class="mb-0 text-slate-500 dark:text-zink-200">
											{(item.size / 1024).toFixed(2)} KB
										</p>
										<strong class="error text-danger"></strong>
									</div>
								</div>
								<div class="shrink-0 ms-3">
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
	</div>

	<div class="card">
		<div class="card-body">
			<h6 class="mb-4 text-15">Bordered Dashed Dropzone</h6>
			<Dropzone
				on:drop={handleFilesSelect2}
				containerClasses="flex items-center justify-center bg-white border border-dashed rounded-md cursor-pointer dropzone !border-slate-300 dropzone2 dark:!bg-zink-700 dark:!border-zink-500"
			>
				<div class="w-full py-5 text-lg text-center dz-message needsclick">
					<div class="mb-3">
						<LucideIcon
							name="UploadCloud"
							class="block size-12 mx-auto text-slate-500 fill-slate-200 dark:text-zink-200 dark:fill-zink-500"
						/>
					</div>

					<h5 class="mb-0 font-normal text-slate-500 text-15">
						Drag and drop your files or <a href="#!">browse</a> your files
					</h5>
				</div>
			</Dropzone>

			<ul class="flex flex-wrap mb-0 gap-x-5" id="dropzone-preview">
				{#each files2.accepted as item, index}
					<li class="mt-5" id="dropzone-preview-list2">
						<div class="border rounded border-slate-200 dark:border-zink-500">
							<div class="p-2 text-center">
								<div>
									<div class="p-2 mx-auto rounded-md size-14 bg-slate-100 dark:bg-zink-600">
										<!-- svelte-ignore a11y-img-redundant-alt -->
										<img
											class="block w-full h-full rounded-md"
											src={files2.preview[index]}
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
										on:click={(e) => handleRemoveFile2(e, index)}>Delete</button
									>
								</div>
							</div>
						</div>
					</li>
				{/each}
			</ul>
		</div>
	</div>
</div>
