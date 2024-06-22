<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import LucideIcon from '../../../../common/components/LucideIcon.svelte';

	let firstName = '';
	let firstNameErr = '';
	let lastName = '';
	let lastNameErr = '';
	let emailName = '';
	let emailNameErr = '';
	let phoneNo = '';
	let phoneNoErr = '';

	function frmSubmit() {

		if (firstName == '') {
			firstNameErr = "First Name is required."
			return false;
		}

		if (lastName == '') {
			lastNameErr = "last Name is required."
			return false;
		}

		if (emailName == '') {
			emailNameErr = "Email is required."
			return false;
		} else if (!isValidEmail(emailName)) {
			emailNameErr = 'Please enter a valid email.';
			return false;
		}

		if (phoneNo === '') {
			phoneNoErr = 'Phone Number is required.';
			return false;
		} else if (!isValidPhoneNumber(phoneNo)) {
			phoneNoErr = 'Please enter a valid 10-digit phone number.';
			return false;
    	}

		firstNameErr = '';
		lastNameErr = '';
		emailNameErr = '';
		phoneNoErr = '';

		alert('Form submitted successfully!');

	}

	function isValidEmail(email) {
		const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
		return emailRegex.test(email);
	}


	function isValidPhoneNumber(phoneNumber) {
		const phoneRegex = /^\d{10}$/; // Validates exactly 10 digits
		return phoneRegex.test(phoneNumber);
	}
</script>

<HeadTitle title="Forms Validation" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Forms Validation" pagetitle="Forms" />

	<div class="card">
		<div class="card-body">
			<h6 class="mb-4 text-15">Browser Default</h6>
			<form action="#!">
				<div class="grid grid-cols-1 gap-x-5 md:grid-cols-2 xl:grid-cols-3">
					<div class="mb-4">
						<label for="firstNameInput2" class="inline-block mb-2 text-base font-medium">First Name <span
								class="text-red-500">*</span></label>
						<input type="text" id="firstNameInput2"
							class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
							placeholder="Enter First Name" value="Sophia" required>
					</div>
					<div class="mb-4">
						<label for="lastNameInput2" class="inline-block mb-2 text-base font-medium">Last Name <span
								class="text-red-500">*</span></label>
						<input type="text" id="lastNameInput2"
							class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
							placeholder="Enter Last Name" value="Bethany" required>
					</div>
					<div class="mb-4">
						<label for="UsernameInput" class="inline-block mb-2 text-base font-medium">Username <span
								class="text-red-500">*</span></label>
						<input type="text" id="UsernameInput"
							class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
							placeholder="Username" required>
					</div>
					<div class="mb-4">
						<label for="cityInput" class="inline-block mb-2 text-base font-medium">City <span
								class="text-red-500">*</span></label>
						<input type="text" id="cityInput"
							class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
							placeholder="Enter city" required>
					</div>
					<div class="mb-4">
						<label for="stateInput" class="inline-block mb-2 text-base font-medium">State <span
								class="text-red-500">*</span></label>
						<select
							class="form-select border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
							id="stateInput" required>
							<option selected="" disabled="" value="">Choose...</option>
							<option>Name</option>
						</select>
					</div>
					<div class="mb-4">
						<label for="zipInput" class="inline-block mb-2 text-base font-medium">Zip <span
								class="text-red-500">*</span></label>
						<input type="number" id="zipInput"
							class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
							placeholder="Enter zip code" required>
					</div>
				</div>
				<div class="flex justify-end gap-2">
					<button type="button"
						class="text-red-500 bg-white btn hover:text-red-500 hover:bg-red-100 focus:text-red-500 focus:bg-red-100 active:text-red-500 active:bg-red-100 dark:bg-zink-700 dark:hover:bg-red-500/10 dark:focus:bg-red-500/10 dark:active:bg-red-500/10"><i
							data-lucide="x" class="inline-block size-4"></i> <span
							class="align-middle">Cancel</span></button>
					<button type="submit"
						class="text-white transition-all duration-200 ease-linear btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100">Submit</button>
				</div>
			</form>
		</div>
	</div>

	<div class="card">
		<div class="card-body">
			<h6 class="mb-4 text-15">Sign Up Form</h6>

			<div class="mx-auto md:max-w-lg">
				<form action="#!" id="signUp" on:submit|preventDefault={frmSubmit}>
					<div class="grid grid-cols-1 gap-x-5 xl:grid-cols-2">
						<div class="mb-4">
							<label for="firstNameInput" class="inline-block mb-2 text-base font-medium">First Name <span
									class="text-red-500">*</span></label>
							<input type="text" id="firstNameInput" bind:value={firstName}
								class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
								placeholder="Enter firstname">
							<p id="firstNameError" class="mt-1 text-sm text-red-500">{firstNameErr}</p>
						</div>
						<div class="mb-4">
							<label for="lastNameInput" class="inline-block mb-2 text-base font-medium">Last Name <span
									class="text-red-500">*</span></label>
							<input type="text" id="lastNameInput" bind:value={lastName}
								class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
								placeholder="Enter lastname">
							<p id="lastNameError" class="mt-1 text-sm text-red-500">{lastNameErr}</p>
						</div>
					</div><!--end grid-->
					<div class="mb-4">
						<label for="emailInput" class="inline-block mb-2 text-base font-medium">Email Address <span
								class="text-red-500">*</span></label>
						<input type="email" id="emailInput" bind:value={emailName}
							class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
							placeholder="example@gmail.com">
						<p id="emailError" class="mt-1 text-sm text-red-500">{emailNameErr}</p>
					</div>
					<div class="mb-4">
						<label for="phoneNumberInput" class="inline-block mb-2 text-base font-medium">Phone Number <span
								class="text-red-500">*</span></label>
						<input type="number" id="phoneNumberInput" bind:value={phoneNo}
							class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
							placeholder="+(254) 0123 456 789">
						<p id="phoneNumberError" class="mt-1 text-sm text-red-500">{phoneNoErr}</p>
					</div>
					<div class="mb-4">
						<input id="termsCondition"
							class="size-4 border rounded-sm appearance-none bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-custom-500 checked:border-custom-500 dark:checked:bg-custom-500 dark:checked:border-custom-500 checked:disabled:bg-custom-400 checked:disabled:border-custom-400"
							type="checkbox">
						<label for="termsCondition" class="text-sm align-top text-slate-400">By registering you agree to
							the Tailwick <a href="#!" class="underline text-custom-500">Terms of Use</a></label>
						<p id="termsError" class="mt-1 text-sm text-red-500"></p>
					</div>
					<div class="flex justify-end gap-2 mt-5">
						<button type="button"
							class="text-red-500 bg-white btn hover:text-red-500 hover:bg-red-100 focus:text-red-500 focus:bg-red-100 active:text-red-500 active:bg-red-100 dark:bg-zink-700 dark:hover:bg-red-500/10 dark:focus:bg-red-500/10 dark:active:bg-red-500/10"><LucideIcon
								name="X" class="inline-block size-4"/> <span
								class="align-middle">Cancel</span></button>
						<button type="submit"
							class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20">Submit</button>
					</div>
				</form>
			</div>

		</div>
	</div>
</div>