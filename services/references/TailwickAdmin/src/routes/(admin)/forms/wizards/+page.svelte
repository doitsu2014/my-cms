<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import LucideIcon from '../../../../common/components/LucideIcon.svelte';
	let step = 1;

	function prev()
	{
		if(step >1){
			step -=1
		}
	}

	function next()
	{
		if(step <4){
			step +=1
		}
	}
</script>

<HeadTitle title="Wizard" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Wizard" pagetitle="Forms" />

	<div class="grid grid-cols-1 2xl:grid-cols-12">
		<div class="col-span-12 2xl:col-start-3 2xl:col-span-8">
			<div class="card">{step}
				<div class="card-body">
					<ul class="grid grid-cols-1 gap-3 lg:grid-cols-4 nav-tabs form-wizard">
						<li class="group/item {step == "1" ? "active":""}">
							<a href="#!" data-target="accountInfo" data-tab-id="1" class="block px-4 py-5 text-center rounded-md text-slate-500 bg-slate-50 dark:text-zink-200 dark:bg-zink-600 group-[.active]/item:bg-custom-500 group-[.active]/item:text-custom-50" on:click={() => step=1}>
								<LucideIcon name="Rocket" class="block h-6 mx-auto fill-slate-200 dark:fill-zink-400 group-[.active]/item:fill-slate-200 dark:group-[.active]/item:fill-custom-500"/>
								<span class="block mt-3 font-medium text-15">1. Account Info</span>
							</a>
						</li>
						<li class="group/item {step == "2" ? "active":""}">
							<a href="#!" data-target="personalInfo" data-tab-id="2" class="block px-4 py-5 text-center rounded-md text-slate-500 bg-slate-50 dark:text-zink-200 dark:bg-zink-600 group-[.active]/item:bg-custom-500 group-[.active]/item:text-custom-50" on:click={() => step=2}>
								<LucideIcon name="User2" class="block h-6 mx-auto fill-slate-200 dark:fill-zink-400 group-[.active]/item:fill-slate-200 dark:group-[.active]/item:fill-custom-500"/>
								<span class="block mt-3 font-medium text-15">2. Personal Info</span>
							</a>
						</li>
						<li class="group/item {step == "3" ? "active":""}">
							<a href="#!" data-target="addressInfo" data-tab-id="3" class="block px-4 py-5 text-center rounded-md text-slate-500 bg-slate-50 dark:text-zink-200 dark:bg-zink-600 group-[.active]/item:bg-custom-500 group-[.active]/item:text-custom-50" on:click={() => step=3}>
								<LucideIcon name="MapPin" class="block h-6 mx-auto fill-slate-200 dark:fill-zink-400 group-[.active]/item:fill-slate-200 dark:group-[.active]/item:fill-custom-500"/>
								<span class="block mt-3 font-medium text-15">3. Address Info</span>
							</a>
						</li>
						<li class="group/item {step == "4" ? "active":""}">
							<a href="#!" data-target="completed" data-tab-id="4" class="block px-4 py-5 text-center rounded-md text-slate-500 bg-slate-50 dark:text-zink-200 dark:bg-zink-600 group-[.active]/item:bg-custom-500 group-[.active]/item:text-custom-50" on:click={() => step=4}>
								<LucideIcon name="PartyPopper" class="block h-6 mx-auto fill-slate-200 dark:fill-zink-400 group-[.active]/item:fill-slate-200 dark:group-[.active]/item:fill-custom-500"/>
								<span class="block mt-3 font-medium text-15">4. Completed</span>
							</a>
						</li>
					</ul>

					<div class="mt-5 tab-content">
						<div class="tab-pane {step == "1" ? "block":"hidden"}">
							<h5 class="mb-3">Account Info</h5>
							<form action="#!">
								<label for="" class="inline-block mb-3 text-base font-medium">Select Business Type <span class="text-red-500">*</span></label>
								<div class="grid grid-cols-1 gap-3 mb-4 md:grid-cols-3 lg:grid-cols-4">
									<div>
										<input id="brandingRadio" name="businessType" class="hidden size-4 border rounded-full appearance-none peer/business bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-custom-500 dark:checked:bg-custom-500 checked:border-custom-500 dark:checked:border-custom-500" type="radio" value="" checked>
										<label for="brandingRadio" class="block px-3 text-center border rounded-md cursor-pointer border-slate-200 py-7 text-slate-500 dark:border-zink-500 dark:text-zink-200 peer-checked/business:border-custom-500 dark:peer-checked/business:border-custom-500 peer-checked/business:text-custom-500 dark:peer-checked/business:text-custom-500">
											<LucideIcon name="Gem" class="block size-8 mx-auto mb-3 stroke-1 fill-slate-100 dark:fill-zink-600"/>
											<span class="block font-medium text-15">Branding</span>
										</label>
									</div>

									<div>
										<input id="mobileDesignRadio" name="businessType" class="hidden size-4 border rounded-full appearance-none peer/business bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-custom-500 dark:checked:bg-custom-500 checked:border-custom-500 dark:checked:border-custom-500" type="radio" value="">
										<label for="mobileDesignRadio" class="block px-3 text-center border rounded-md cursor-pointer border-slate-200 py-7 text-slate-500 dark:border-zink-500 dark:text-zink-200 peer-checked/business:border-custom-500 dark:peer-checked/business:border-custom-500 peer-checked/business:text-custom-500 dark:peer-checked/business:text-custom-500">
											<LucideIcon name="Smartphone" class="block size-8 mx-auto mb-3 stroke-1 fill-slate-100 dark:fill-zink-600"/>
											<span class="block font-medium text-15">Mobile Design</span>
										</label>
									</div>

									<div>
										<input id="webDesignRadio" name="businessType" class="hidden size-4 border rounded-full appearance-none peer/business bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-custom-500 dark:checked:bg-custom-500 checked:border-custom-500 dark:checked:border-custom-500" type="radio" value="">
										<label for="webDesignRadio" class="block px-3 text-center border rounded-md cursor-pointer border-slate-200 py-7 text-slate-500 dark:border-zink-500 dark:text-zink-200 peer-checked/business:border-custom-500 dark:peer-checked/business:border-custom-500 peer-checked/business:text-custom-500 dark:peer-checked/business:text-custom-500">
											<LucideIcon name="Globe" class="block size-8 mx-auto mb-3 stroke-1 fill-slate-100 dark:fill-zink-600"/>
											<span class="block font-medium text-15">Web Design</span>
										</label>
									</div>
								</div><!--end grid-->
								<div class="grid grid-cols-1 gap-4 md:grid-cols-2">
									<div>
										<label for="nameInput" class="inline-block mb-2 text-base font-medium">Name <span class="text-red-500">*</span></label>
										<input type="text" id="nameInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Enter your name" required>
										<div id="nameError" class="text-red-500"></div>

									</div>
									<div>
										<label for="usernameInput" class="inline-block mb-2 text-base font-medium">Username <span class="text-red-500">*</span></label>
										<input type="text" id="usernameInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Enter username" required>
										<div id="usernameError" class="text-red-500"></div>
									</div>
									<div>
										<label for="phoneNumberInput" class="inline-block mb-2 text-base font-medium">Phone Number <span class="text-red-500">*</span></label>
										<input type="number" id="phoneNumberInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Phone number" required>
										<div id="phoneNumberError" class="text-red-500"></div>
									</div>
									<div>
										<label for="emailAddressInput" class="inline-block mb-2 text-base font-medium">Email Address</label>
										<input type="email" id="emailAddressInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Email address" required>
										<div id="emailError" class="text-red-500"></div>
									</div>
									<div>
										<label for="passwordInput" class="inline-block mb-2 text-base font-medium">Password <span class="text-red-500">*</span></label>
										<input type="password" id="passwordInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Password" required>
										<div id="passwordError" class="text-red-500"></div>
									</div>
									<div>
										<label for="passwordConfirmInput" class="inline-block mb-2 text-base font-medium">Password Confirm <span class="text-red-500">*</span></label>
										<input type="password" id="passwordConfirmInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Password confirm" required>
										<div id="cpasswordError" class="text-red-500"></div>
									</div>
								</div>
							</form>
						</div>
						<div class="tab-pane {step == "2" ? "block":"hidden"}">
							<h5 class="mb-3">Personal Info</h5>
							<form action="#!">
								<div class="grid grid-cols-1 gap-4 mb-4 md:grid-cols-2">
									<div>
										<label for="FirstNameInput" class="inline-block mb-2 text-base font-medium">First Name <span class="text-red-500">*</span></label>
										<input type="text" id="FirstNameInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="First name" required>
										<div id="FirstNameInputError" class="text-red-500"></div>
									</div>
									<div>
										<label for="lastNameInput" class="inline-block mb-2 text-base font-medium">Last Name <span class="text-red-500">*</span></label>
										<input type="text" id="lastNameInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Last name" required>
										<div id="lastNameInputError" class="text-red-500"></div>
									</div>
								</div><!--end grid-->
								<div class="grid grid-cols-1 gap-4 mb-4 md:grid-cols-2">
									<div>
										<label for="positionInput" class="inline-block mb-2 text-base font-medium">Position <span class="text-red-500">*</span></label>
										<input type="text" id="positionInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Position" required>
										<div id="positionInputError" class="text-red-500"></div>
									</div>
									<div>
										<label for="joiningDateInput" class="inline-block mb-2 text-base font-medium">Joining Date <span class="text-red-500">*</span></label>
										<input type="date" id="joiningDateInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Joining Date" required>
										<div id="joiningDateInputError" class="text-red-500"></div>
									</div>
								</div><!--end grid-->
								<div class="grid grid-cols-1 gap-4 mb-4 md:grid-cols-3">
									<div>
										<label for="birthDateInput" class="inline-block mb-2 text-base font-medium">Birth of Date <span class="text-red-500">*</span></label>
										<input type="date" id="birthDateInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Select date" required>
										<div id="birthDateInputError" class="text-red-500"></div>
									</div>
									<div>
										<label for="phoneNumberPersonalInput" class="inline-block mb-2 text-base font-medium">Phone Number <span class="text-red-500">*</span></label>
										<input type="number" id="phoneNumberPersonalInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Phone number" required>
										<div id="phoneNumberPersonalInputError" class="text-red-500"></div>
									</div>
									<div>
										<label for="emailPersonalInput" class="inline-block mb-2 text-base font-medium">Email Address <span class="text-red-500">*</span></label>
										<input type="email" id="emailPersonalInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Email address" required>
										<div id="emailPersonalInputError" class="text-red-500"></div>
									</div>
								</div>
								<div>
									<label for="description" class="inline-block mb-2 text-base font-medium">Description</label>
									<textarea class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" id="description" rows="5"></textarea>
								</div>
							</form>
						</div>
						<div class="tab-pane {step == "3" ? "block":"hidden"}">
							<h5 class="mb-3">Address</h5>
							<form action="#!">
								<div class="grid grid-cols-1 gap-4 mb-4 md:grid-cols-3">
									<div>
										<label for="FirstNameAddressInput" class="inline-block mb-2 text-base font-medium">First Name <span class="text-red-500">*</span></label>
										<input type="text" id="FirstNameAddressInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="First name" required>
										<div id="FirstNameAddressInputError" class="text-red-500"></div>
									</div>
									<div>
										<label for="middleNameAddressInput" class="inline-block mb-2 text-base font-medium">Middle Name <span class="text-red-500">*</span></label>
										<input type="text" id="middleNameAddressInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Middle name" required>
										<div id="middleNameAddressInputError" class="text-red-500"></div>
									</div>
									<div>
										<label for="lastNameAddressInput" class="inline-block mb-2 text-base font-medium">Last Name <span class="text-red-500">*</span></label>
										<input type="text" id="lastNameAddressInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Last name" required>
										<div id="lastNameAddressInputError" class="text-red-500"></div>
									</div>
								</div>
								<div class="grid grid-cols-1 gap-4 mb-4 md:grid-cols-2">
									<div>
										<label for="phoneNumberAddressInput" class="inline-block mb-2 text-base font-medium">Phone Number</label>
										<input type="number" id="phoneNumberAddressInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Phone number" required>
										<div id="phoneNumberAddressInputError" class="text-red-500"></div>
									</div>
									<div>
										<label for="emailAddress2Input" class="inline-block mb-2 text-base font-medium">Email Address</label>
										<input type="email" id="emailAddress2Input" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Email address" required>
										<div id="emailAddress2InputError" class="text-red-500"></div>
									</div>
									<div>
										<label for="addressLineInput" class="inline-block mb-2 text-base font-medium">Address Line 1</label>
										<input type="text" id="addressLineInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Address line 1" required>
										<div id="addressLineInputError" class="text-red-500"></div>
									</div>
									<div>
										<label for="addressLine2" class="inline-block mb-2 text-base font-medium">Address Line 2</label>
										<input type="text" id="addressLine2" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Address line 1" required>
										<div id="addressLine2Error" class="text-red-500"></div>
									</div>
								</div><!--end grid-->
								<div class="grid grid-cols-1 gap-4 mb-4 md:grid-cols-2 lg:grid-cols-3">
									<div>
										<label for="cityAddressInput" class="inline-block mb-2 text-base font-medium">City</label>
										<input type="text" id="cityAddressInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="City" required>
										<div id="cityAddressInputError" class="text-red-500"></div>
									</div>
									<div>
										<label for="stateAddressInput" class="inline-block mb-2 text-base font-medium">State</label>
										<input type="text" id="stateAddressInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="State" required>
										<div id="stateAddressInputError" class="text-red-500"></div>
									</div>
									<div>
										<label for="countryAddressInput" class="inline-block mb-2 text-base font-medium">Country</label>
										<input type="text" id="countryAddressInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Country" required>
										<div id="countryAddressInputError" class="text-red-500"></div>
									</div>
									<div>
										<label for="zipcodeAddressInput" class="inline-block mb-2 text-base font-medium">Zip code</label>
										<input type="number" id="zipcodeAddressInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="000000" required>
										<div id="zipcodeAddressInputError" class="text-red-500"></div>
									</div>
									<div>
										<label for="vtaAddressInput" class="inline-block mb-2 text-base font-medium">VTA Number</label>
										<input type="text" id="vtaAddressInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="VTA number" required>
										<div id="vtaAddressInputError" class="text-red-500"></div>
									</div>
									<div>
										<label for="prefixAddressInput" class="inline-block mb-2 text-base font-medium">Prefix</label>
										<input type="text" id="prefixAddressInput" class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Prefix" required>
										<div id="prefixAddressInputError" class="text-red-500"></div>
									</div>
								</div>
							</form>
						</div>
						<div class="tab-pane {step == "4" ? "block":"hidden"}">
							<div class="px-4 py-6 mx-auto text-center lg:w-2/3">
								<LucideIcon name="CheckCircle" class="block size-10 mx-auto mb-4 text-green-500 fill-green-100 dark:fill-green-500/20 animate-icons"/>
								<h5 class="mb-2">Registration Successfully 🎉</h5>
								<p class="mb-5 text-slate-500 text-15">We are thrilled to inform you that your registration has been successfully processed! Welcome to our community. Thank you for choosing us, and we look forward to serving you. Stay connected and enjoy your journey with us!</p>
								<button type="button" class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"><span class="align-middle">Try Login</span> <LucideIcon name="log-in" class="inline-block h-4 ltr:ml-1 rtl:mr-1"/></button>
							</div>
						</div>
					</div>
					<div class="flex justify-between gap-2 mt-5">
						
						<button type="button" data-action="prev" class="text-slate-500 btn bg-slate-200 border-slate-200 hover:text-slate-600 hover:bg-slate-300 hover:border-slate-300 focus:text-slate-600 focus:bg-slate-300 focus:border-slate-300 focus:ring focus:ring-slate-100 active:text-slate-600 active:bg-slate-300 active:border-slate-300 active:ring active:ring-slate-100 dark:bg-zink-600 dark:hover:bg-zink-500 dark:border-zink-600 dark:hover:border-zink-500 dark:text-zink-200 dark:ring-zink-400/50" on:click={prev}><LucideIcon name="MoveLeft" class="inline-block h-4 mr-1"/> <span class="align-middle">Previous</span></button>
						{#if step != 4}
						<button type="button" data-action="next" class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20" on:click={next}><span class="align-middle">Next</span> <LucideIcon name="MoveRight" class="inline-block h-4 ml-1"/></button>
						{/if}
					</div>
				</div>
			</div><!--end card-->
		</div>
	</div>
</div>
