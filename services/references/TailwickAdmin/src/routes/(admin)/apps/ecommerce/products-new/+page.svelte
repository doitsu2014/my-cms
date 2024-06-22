<script>
    import HeadTitle from "../../../../../common/components/HeadTitle.svelte";
	import Breadcrumb from "../../../../../common/components/Breadcrumb.svelte";
	import LucideIcon from "../../../../../common/components/LucideIcon.svelte";
	import Dropzone from "svelte-file-dropzone/Dropzone.svelte";

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
</script>
<HeadTitle title="Overview" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto relative">
    <Breadcrumb title="Add New" pagetitle="Products"/>

    <div class="grid grid-cols-1 xl:grid-cols-12 gap-x-5">
        <div class="xl:col-span-9">
            <div class="card">
                <div class="card-body">
                    <h6 class="mb-4 text-15">Create Product</h6>

                    <form action="#!">
                        <div class="grid grid-cols-1 gap-5 lg:grid-cols-2 xl:grid-cols-12">
                            <div class="xl:col-span-6">
                                <label for="productNameInput" class="inline-block mb-2 text-base font-medium">Product Title</label>
                                <input type="text" id="productNameInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Product title" required>
                                <p class="mt-1 text-sm text-slate-400 dark:text-zink-200">Do not exceed 20 characters when entering the product name.</p>
                            </div><!--end col-->
                            <div class="xl:col-span-6">
                                <label for="productCodeInput" class="inline-block mb-2 text-base font-medium">Product Code</label>
                                <input type="text" id="productCodeInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Product Code" value="TWT145015" disabled required>
                                <p class="mt-1 text-sm text-slate-400 dark:text-zink-200">Code will be generated automatically</p>
                            </div><!--end col-->
                            <div class="xl:col-span-4">
                                <label for="qualityInput" class="inline-block mb-2 text-base font-medium">Quantity</label>
                                <input type="number" id="qualityInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Quantity" required>
                            </div><!--end col-->
                            <div class="xl:col-span-4">
                                <label for="skuInput" class="inline-block mb-2 text-base font-medium">SKU</label>
                                <input type="text" id="skuInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="TWT-LP-ALU-08" required>
                            </div><!--end col-->
                            <div class="xl:col-span-4">
                                <label for="brandInput" class="inline-block mb-2 text-base font-medium">Brand</label>
                                <input type="text" id="brandInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Brand" required>
                            </div><!--end col-->
                            <div class="xl:col-span-4">
                                <label for="categorySelect" class="inline-block mb-2 text-base font-medium">Category</label>
                                <select class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" data-choices data-choices-search-false name="categorySelect" id="categorySelect">
                                    <option value="">Select Category</option>
                                    <option value="Mobiles, Computers">Mobiles, Computers</option>
                                    <option value="TV, Appliances, Electronics">TV, Appliances, Electronics</option>
                                    <option value="Men's Fashion">Men's Fashion</option>
                                    <option value="Women's Fashion">Women's Fashion</option>
                                    <option value="Home, Kitchen, Pets">Home, Kitchen, Pets</option>
                                    <option value="Beauty, Health, Grocery">Beauty, Health, Grocery</option>
                                    <option value="Books">Books</option>
                                </select>
                            </div><!--end col-->
                            <div class="xl:col-span-4">
                                <label for="productTypeSelect" class="inline-block mb-2 text-base font-medium">Product Type</label>
                                <select class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" data-choices data-choices-search-false name="productTypeSelect" id="productTypeSelect">
                                    <option value="">Select Type</option>
                                    <option value="Single">Single</option>
                                    <option value="Unit">Unit</option>
                                    <option value="Boxed">Boxed</option>
                                </select>
                            </div><!--end col-->
                            <div class="xl:col-span-4">
                                <label for="genderSelect" class="inline-block mb-2 text-base font-medium">Gender</label>
                                <select class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" data-choices data-choices-search-false name="genderSelect" id="genderSelect">
                                    <option value="">Select Gender</option>
                                    <option value="Male">Male</option>
                                    <option value="Female">Female</option>
                                    <option value="Unisex">Unisex</option>
                                </select>
                            </div><!--end col-->
                            <div class="xl:col-span-6">
                                <label for="qualityInput" class="inline-block mb-2 text-base font-medium">Colors Variant</label>
                                <div class="flex flex-wrap items-center gap-2">
                                    <div>
                                        <input id="selectColor1" class="inline-block size-5 align-middle border rounded-sm appearance-none cursor-pointer bg-sky-500 border-sky-500 checked:bg-sky-500 checked:border-sky-500 disabled:opacity-75 disabled:cursor-default" type="checkbox" value="color1" name="selectColor">
                                    </div>
                                    <div>
                                        <input id="selectColor2" class="inline-block size-5 align-middle bg-orange-500 border border-orange-500 rounded-sm appearance-none cursor-pointer checked:bg-orange-500 checked:border-orange-500 disabled:opacity-75 disabled:cursor-default" type="checkbox" value="color2" name="selectColor" checked>
                                    </div>
                                    <div>
                                        <input id="selectColor3" class="inline-block size-5 align-middle bg-green-500 border border-green-500 rounded-sm appearance-none cursor-pointer checked:bg-green-500 checked:border-green-500 disabled:opacity-75 disabled:cursor-default" type="checkbox" value="color3" name="selectColor">
                                    </div>
                                    <div>
                                        <input id="selectColor4" class="inline-block size-5 align-middle bg-purple-500 border border-purple-500 rounded-sm appearance-none cursor-pointer checked:bg-purple-500 checked:border-purple-500 disabled:opacity-75 disabled:cursor-default" type="checkbox" value="color4" name="selectColor">
                                    </div>
                                    <div>
                                        <input id="selectColor5" class="inline-block size-5 align-middle bg-yellow-500 border border-yellow-500 rounded-sm appearance-none cursor-pointer checked:bg-yellow-500 checked:border-yellow-500 disabled:opacity-75 disabled:cursor-default" type="checkbox" value="color5" name="selectColor">
                                    </div>
                                    <div>
                                        <input id="selectColor6" class="inline-block size-5 align-middle bg-red-500 border border-red-500 rounded-sm appearance-none cursor-pointer checked:bg-red-500 checked:border-red-500 disabled:opacity-75 disabled:cursor-default" type="checkbox" value="color6" name="selectColor">
                                    </div>
                                    <div>
                                        <input id="selectColor7" class="inline-block size-5 align-middle border rounded-sm appearance-none cursor-pointer bg-slate-500 border-slate-500 checked:bg-slate-500 checked:border-slate-500 disabled:opacity-75 disabled:cursor-default" type="checkbox" value="color7" name="selectColor">
                                    </div>
                                    <div>
                                        <input id="selectColor8" class="inline-block size-5 align-middle border rounded-sm appearance-none cursor-pointer bg-slate-900 border-slate-900 checked:bg-slate-900 checked:border-slate-900 disabled:opacity-75 disabled:cursor-default" type="checkbox" value="color7" name="selectColor">
                                    </div>
                                    <div>
                                        <input id="selectColor9" class="inline-block size-5 align-middle border rounded-sm appearance-none cursor-pointer bg-slate-200 border-slate-200 checked:bg-slate-200 checked:border-slate-200 disabled:opacity-75 disabled:cursor-default" type="checkbox" value="color7" name="selectColor">
                                    </div>
                                    <div>
                                        <a href="#!" class="flex items-center justify-center size-5 border rounded-sm border-slate-200 dark:border-zink-500"><LucideIcon name="Pencil" class="size-3"/></a>
                                    </div>
                                </div>
                            </div><!--end col-->
                            <div class="xl:col-span-6">
                                <div class="inline-block mb-2 text-base font-medium">Size</div>
                                <div class="flex flex-wrap items-center gap-2">
                                    <div>
                                        <input id="selectSizeXS" class="hidden peer" type="checkbox" value="XS" name="selectSize">
                                        <label for="selectSizeXS" class="flex items-center justify-center size-10 text-xs border rounded-md cursor-pointer border-slate-200 dark:border-zink-500 peer-checked:bg-custom-50 dark:peer-checked:bg-custom-500/20 peer-checked:border-custom-300 dark:peer-checked:border-custom-700 peer-disabled:bg-slate-50 dark:peer-disabled:bg-slate-500/15 peer-disabled:border-slate-100 dark:peer-disabled:border-slate-800 peer-disabled:cursor-default peer-disabled:text-slate-500 dark:peer-disabled:text-zink-200">XS</label>
                                    </div>
                                    <div>
                                        <input id="selectSizeS" class="hidden peer" type="checkbox" value="S" name="selectSize" checked>
                                        <label for="selectSizeS" class="flex items-center justify-center size-10 text-xs border rounded-md cursor-pointer border-slate-200 dark:border-zink-500 peer-checked:bg-custom-50 dark:peer-checked:bg-custom-500/20 peer-checked:border-custom-300 dark:peer-checked:border-custom-700 peer-disabled:bg-slate-50 dark:peer-disabled:bg-slate-500/15 peer-disabled:border-slate-100 dark:peer-disabled:border-slate-800 peer-disabled:cursor-default peer-disabled:text-slate-500 dark:peer-disabled:text-zink-200">S</label>
                                    </div>
                                    <div>
                                        <input id="selectSizeM" class="hidden peer" type="checkbox" value="M" name="selectSize">
                                        <label for="selectSizeM" class="flex items-center justify-center size-10 text-xs border rounded-md cursor-pointer border-slate-200 dark:border-zink-500 peer-checked:bg-custom-50 dark:peer-checked:bg-custom-500/20 peer-checked:border-custom-300 dark:peer-checked:border-custom-700 peer-disabled:bg-slate-50 dark:peer-disabled:bg-slate-500/15 peer-disabled:border-slate-100 dark:peer-disabled:border-slate-800 peer-disabled:cursor-default peer-disabled:text-slate-500 dark:peer-disabled:text-zink-200">M</label>
                                    </div>
                                    <div>
                                        <input id="selectSizeL" class="hidden peer" type="checkbox" value="L" name="selectSize">
                                        <label for="selectSizeL" class="flex items-center justify-center size-10 text-xs border rounded-md cursor-pointer border-slate-200 dark:border-zink-500 peer-checked:bg-custom-50 dark:peer-checked:bg-custom-500/20 peer-checked:border-custom-300 dark:peer-checked:border-custom-700 peer-disabled:bg-slate-50 dark:peer-disabled:bg-slate-500/15 peer-disabled:border-slate-100 dark:peer-disabled:border-slate-800 peer-disabled:cursor-default peer-disabled:text-slate-500 dark:peer-disabled:text-zink-200">L</label>
                                    </div>
                                    <div>
                                        <input id="selectSizeXL" class="hidden peer" type="checkbox" value="XL" name="selectSize">
                                        <label for="selectSizeXL" class="flex items-center justify-center size-10 text-xs border rounded-md cursor-pointer border-slate-200 dark:border-zink-500 peer-checked:bg-custom-50 dark:peer-checked:bg-custom-500/20 peer-checked:border-custom-300 dark:peer-checked:border-custom-700 peer-disabled:bg-slate-50 dark:peer-disabled:bg-slate-500/15 peer-disabled:border-slate-100 dark:peer-disabled:border-slate-800 peer-disabled:cursor-default peer-disabled:text-slate-500 dark:peer-disabled:text-zink-200">XL</label>
                                    </div>
                                    <div>
                                        <input id="selectSize2XL" class="hidden peer" type="checkbox" value="2XL" name="selectSize">
                                        <label for="selectSize2XL" class="flex items-center justify-center size-10 text-xs border rounded-md cursor-pointer border-slate-200 dark:border-zink-500 peer-checked:bg-custom-50 dark:peer-checked:bg-custom-500/20 peer-checked:border-custom-300 dark:peer-checked:border-custom-700 peer-disabled:bg-slate-50 dark:peer-disabled:bg-slate-500/15 peer-disabled:border-slate-100 dark:peer-disabled:border-slate-800 peer-disabled:cursor-default peer-disabled:text-slate-500 dark:peer-disabled:text-zink-200">2XL</label>
                                    </div>
                                    <div>
                                        <input id="selectSize3XL" class="hidden peer" type="checkbox" value="3XL" name="selectSize">
                                        <label for="selectSize3XL" class="flex items-center justify-center size-10 text-xs border rounded-md cursor-pointer border-slate-200 dark:border-zink-500 peer-checked:bg-custom-50 dark:peer-checked:bg-custom-500/20 peer-checked:border-custom-300 dark:peer-checked:border-custom-700 peer-disabled:bg-slate-50 dark:peer-disabled:bg-slate-500/15 peer-disabled:border-slate-100 dark:peer-disabled:border-slate-800 peer-disabled:cursor-default peer-disabled:text-slate-500 dark:peer-disabled:text-zink-200">3XL</label>
                                    </div>
                                </div>
                            </div><!--end col-->
                            <div class="lg:col-span-2 xl:col-span-12">
                                <label for="genderSelect" class="inline-block mb-2 text-base font-medium">Product Images</label>
                                <Dropzone on:drop={handleFilesSelect} containerClasses="flex items-center justify-center border rounded-md cursor-pointer !bg-slate-100 dropzone !border-slate-200 dark:!bg-zink-600 dark:!border-zink-500 dz-clickable">
                                    <div class="w-full py-5 text-lg text-center dz-message needsclick">
                                        <div class="mb-3">
                                            <LucideIcon name="UploadCloud" class="block size-12 mx-auto text-slate-500 fill-slate-200 dark:text-zink-200 dark:fill-zink-500"/>
                                        </div>

                                        <h5 class="mb-0 font-normal text-slate-500 dark:text-zink-200 text-15">Drag and drop your product images or <a href="#!">browse</a> your product images</h5>
                                    </div>
                                </Dropzone>
                                <ul class="flex flex-wrap mb-0 gap-x-5" id="dropzone-preview2">
                                {#each files.accepted as item,index}
                                    <li class="mt-5" id="dropzone-preview-list2">
                                       
                                        <div class="border rounded border-slate-200 dark:border-zink-500">
                                            <div class="p-2 text-center">
                                                <div>
                                                    <div class="p-2 mx-auto rounded-md size-14 bg-slate-100 dark:bg-zink-600">
                                                        <!-- svelte-ignore a11y-img-redundant-alt -->
                                                        <img class="block w-full h-full rounded-md" src={files.preview[index]} alt="Dropzone-Image">
                                                    </div>
                                                </div>
                                                <div class="pt-3">
                                                    <h5 class="mb-1 text-15">{item.name}</h5>
                                                    <p class="mb-0 text-slate-500 dark:text-zink-200">{item.size}</p>
                                                    <strong class="error text-danger"></strong>
                                                </div>
                                                <div class="mt-2">
                                                    <button class="px-2 py-1.5 text-xs text-white bg-red-500 border-red-500 btn hover:text-white hover:bg-red-600 hover:border-red-600 focus:text-white focus:bg-red-600 focus:border-red-600 focus:ring focus:ring-red-100 active:text-white active:bg-red-600 active:border-red-600 active:ring active:ring-red-100 dark:ring-custom-400/20" on:click={(e)=>
                                                        handleRemoveFile(e, index)}>Delete</button>
                                                </div>
                                            </div>
                                        </div>
                                    </li>
                                    {/each}
                                </ul>
                            </div>
                            <div class="lg:col-span-2 xl:col-span-12">
                                <div>
                                    <label for="productDescription" class="inline-block mb-2 text-base font-medium">Description</label>
                                    <textarea class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" id="productDescription" placeholder="Enter Description" rows="5"></textarea>
                                </div>
                            </div>
                            <div class="xl:col-span-4">
                                <label for="productPrice" class="inline-block mb-2 text-base font-medium">Price</label>
                                <input type="number" id="productPrice" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="$0.00" required>
                            </div><!--end col-->
                            <div class="xl:col-span-4">
                                <label for="productDiscounts" class="inline-block mb-2 text-base font-medium">Discounts</label>
                                <input type="number" id="productDiscounts" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="0%" required>
                            </div><!--end col-->
                            <div class="xl:col-span-4">
                                <label for="taxApplicable" class="inline-block mb-2 text-base font-medium">TAX Applicable</label>
                                <select class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" data-choices data-choices-search-false name="taxApplicable" id="taxApplicable">
                                    <option value="">Select TAX Applicable</option>
                                    <option value="none">none</option>
                                    <option value="Exclusive">Exclusive</option>
                                    <option value="Professional">Professional</option>
                                    <option value="Entertainment">Entertainment</option>
                                </select>
                            </div><!--end col-->
                            <div class="xl:col-span-4">
                                <label for="publishDateTime" class="inline-block mb-2 text-base font-medium">Publish Date & Time</label>
                                <input type="number" id="publishDateTime" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Select date & time" data-provider="flatpickr" data-date-format="d M, Y" data-enable-time required>
                            </div><!--end col-->
                            <div class="xl:col-span-4">
                                <label for="productStatus" class="inline-block mb-2 text-base font-medium">Status</label>
                                <select class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" data-choices data-choices-search-false name="productStatus" id="productStatus">
                                    <option value="Draft">Draft</option>
                                    <option value="Published">Published</option>
                                    <option value="Scheduled">Scheduled</option>
                                    <option value="Entertainment">Entertainment</option>
                                </select>
                            </div><!--end col-->
                            <div class="xl:col-span-4">
                                <label for="productVisibility" class="inline-block mb-2 text-base font-medium">Visibility</label>
                                <select class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" data-choices data-choices-search-false name="productVisibility" id="productVisibility">
                                    <option value="Public">Public</option>
                                    <option value="Hidden">Hidden</option>
                                </select>
                            </div><!--end col-->
                            <div class="lg:col-span-2 xl:col-span-12">
                                <label for="productTag" class="inline-block mb-2 text-base font-medium">Product Tag</label>
                                <input class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" id="productTag" data-choices data-choices-text-unique-true type="text" value="Fashion, Clothes, Headphones">
                            </div><!--end col-->
                        </div><!--end grid-->
                        <div class="flex justify-end gap-2 mt-4">
                            <button type="reset" class="text-red-500 bg-white btn hover:text-red-500 hover:bg-red-100 focus:text-red-500 focus:bg-red-100 active:text-red-500 active:bg-red-100 dark:bg-zink-700 dark:hover:bg-red-500/10 dark:focus:bg-red-500/10 dark:active:bg-red-500/10">Reset</button>
                            <button type="submit" class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20">Create Product</button>
                            <button type="button" class="text-white bg-green-500 border-green-500 btn hover:text-white hover:bg-green-600 hover:border-green-600 focus:text-white focus:bg-green-600 focus:border-green-600 focus:ring focus:ring-green-100 active:text-white active:bg-green-600 active:border-green-600 active:ring active:ring-green-100 dark:ring-green-400/10">Draft & Preview</button>
                        </div>
                    </form>
                </div>
            </div><!--end card-->
        </div><!--end col-->
        <div class="xl:col-span-3">
            <div class="card sticky top-[calc(theme('spacing.header')_*_1.3)]">
                <div class="card-body">
                    <h6 class="mb-4 text-15">Product Card Preview</h6>

                    <div class="px-5 py-8 rounded-md bg-sky-50 dark:bg-zink-600">
                        <img src="/assets/images/product/img-03.png" alt="" class="block mx-auto h-44">
                    </div>

                    <div class="mt-3">
                        <h5 class="mb-2">$145.99 <small class="font-normal line-through">299.99</small></h5>
                        <h6 class="mb-1 text-15">Fastcolors Typography Men</h6>
                        <p class="text-slate-500 dark:text-zink-200">Men's Fashion</p>
                    </div>
                    <h6 class="mt-3 mb-2 text-15">Colors</h6>
                    <div class="flex flex-wrap items-center gap-2">
                        <div>
                            <input id="selectColorPre1" class="inline-block size-5 align-middle border rounded-full appearance-none cursor-pointer bg-sky-500 border-sky-500 checked:bg-sky-500 checked:border-sky-500 disabled:opacity-75 disabled:cursor-default" type="checkbox" value="color1" name="selectColorPre">
                        </div>
                        <div>
                            <input id="selectColorPre2" class="inline-block size-5 align-middle bg-orange-500 border border-orange-500 rounded-full appearance-none cursor-pointer checked:bg-orange-500 checked:border-orange-500 disabled:opacity-75 disabled:cursor-default" type="checkbox" value="color2" name="selectColorPre" checked="">
                        </div>
                        <div>
                            <input id="selectColorPre3" class="inline-block size-5 align-middle bg-green-500 border border-green-500 rounded-full appearance-none cursor-pointer checked:bg-green-500 checked:border-green-500 disabled:opacity-75 disabled:cursor-default" type="checkbox" value="color3" name="selectColorPre">
                        </div>
                        <div>
                            <input id="selectColorPre4" class="inline-block size-5 align-middle bg-purple-500 border border-purple-500 rounded-full appearance-none cursor-pointer checked:bg-purple-500 checked:border-purple-500 disabled:opacity-75 disabled:cursor-default" type="checkbox" value="color4" name="selectColorPre">
                        </div>
                    </div>

                    <h6 class="mt-3 mb-2 text-15">Colors</h6>
                    <div class="flex flex-wrap items-center gap-2">
                        <div>
                            <input id="selectSizePreXS" class="hidden peer" type="checkbox" value="XS" name="selectSizePre">
                            <label for="selectSizePreXS" class="flex items-center justify-center size-8 text-xs border rounded-md cursor-pointer border-slate-200 dark:border-zink-500 peer-checked:bg-custom-50 dark:peer-checked:bg-custom-500/20 peer-checked:border-custom-300 dark:peer-checked:border-custom-700 peer-disabled:bg-slate-50 dark:peer-disabled:bg-slate-500/15 peer-disabled:border-slate-100 dark:peer-disabled:border-slate-800 peer-disabled:cursor-default peer-disabled:text-slate-500 dark:peer-disabled:text-zink-200">XS</label>
                        </div>
                        <div>
                            <input id="selectSizePreS" class="hidden peer" type="checkbox" value="S" name="selectSizePre">
                            <label for="selectSizePreS" class="flex items-center justify-center size-8 text-xs border rounded-md cursor-pointer border-slate-200 dark:border-zink-500 peer-checked:bg-custom-50 dark:peer-checked:bg-custom-500/20 peer-checked:border-custom-300 dark:peer-checked:border-custom-700 peer-disabled:bg-slate-50 dark:peer-disabled:bg-slate-500/15 peer-disabled:border-slate-100 dark:peer-disabled:border-slate-800 peer-disabled:cursor-default peer-disabled:text-slate-500 dark:peer-disabled:text-zink-200">S</label>
                        </div>
                        <div>
                            <input id="selectSizePreM" class="hidden peer" type="checkbox" value="M" name="selectSizePre">
                            <label for="selectSizePreM" class="flex items-center justify-center size-8 text-xs border rounded-md cursor-pointer border-slate-200 dark:border-zink-500 peer-checked:bg-custom-50 dark:peer-checked:bg-custom-500/20 peer-checked:border-custom-300 dark:peer-checked:border-custom-700 peer-disabled:bg-slate-50 dark:peer-disabled:bg-slate-500/15 peer-disabled:border-slate-100 dark:peer-disabled:border-slate-800 peer-disabled:cursor-default peer-disabled:text-slate-500 dark:peer-disabled:text-zink-200">M</label>
                        </div>
                        <div>
                            <input id="selectSizePreL" class="hidden peer" type="checkbox" value="L" name="selectSizePre">
                            <label for="selectSizePreL" class="flex items-center justify-center size-8 text-xs border rounded-md cursor-pointer border-slate-200 dark:border-zink-500 peer-checked:bg-custom-50 dark:peer-checked:bg-custom-500/20 peer-checked:border-custom-300 dark:peer-checked:border-custom-700 peer-disabled:bg-slate-50 dark:peer-disabled:bg-slate-500/15 peer-disabled:border-slate-100 dark:peer-disabled:border-slate-800 peer-disabled:cursor-default peer-disabled:text-slate-500 dark:peer-disabled:text-zink-200">L</label>
                        </div>
                        <div>
                            <input id="selectSizePreXL" class="hidden peer" type="checkbox" value="XL" name="selectSizePre">
                            <label for="selectSizePreXL" class="flex items-center justify-center size-8 text-xs border rounded-md cursor-pointer border-slate-200 dark:border-zink-500 peer-checked:bg-custom-50 dark:peer-checked:bg-custom-500/20 peer-checked:border-custom-300 dark:peer-checked:border-custom-700 peer-disabled:bg-slate-50 dark:peer-disabled:bg-slate-500/15 peer-disabled:border-slate-100 dark:peer-disabled:border-slate-800 peer-disabled:cursor-default peer-disabled:text-slate-500 dark:peer-disabled:text-zink-200">XL</label>
                        </div>
                    </div>
                    <div class="flex gap-2 mt-4">
                        <button type="button" class="w-full bg-white border-dashed text-custom-500 btn border-custom-500 hover:text-custom-500 hover:bg-custom-50 hover:border-custom-600 focus:text-custom-600 focus:bg-custom-50 focus:border-custom-600 active:text-custom-600 active:bg-custom-50 active:border-custom-600 dark:bg-zink-700 dark:ring-custom-400/20 dark:hover:bg-custom-800/20 dark:focus:bg-custom-800/20 dark:active:bg-custom-800/20">Create Products</button>
                        <button type="button" class="w-full text-white bg-purple-500 border-purple-500 btn hover:text-white hover:bg-purple-600 hover:border-purple-600 focus:text-white focus:bg-purple-600 focus:border-purple-600 focus:ring focus:ring-purple-100 active:text-white active:bg-purple-600 active:border-purple-600 active:ring active:ring-purple-100 dark:ring-purple-400/10">Draft</button>
                    </div>
                </div>
            </div><!--end card-->
        </div><!--end col-->
    </div>
</div>
