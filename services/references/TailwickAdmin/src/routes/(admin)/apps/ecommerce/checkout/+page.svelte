<script>
	import HeadTitle from '../../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../../common/components/Breadcrumb.svelte';
	import LucideIcon from '../../../../../common/components/LucideIcon.svelte';
</script>

<HeadTitle title="Checkout" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto relative">
	<Breadcrumb title="Checkout" pagetitle="Ecommerce" />

	<div class="grid grid-cols-1 xl:grid-cols-12 gap-x-5">
		<div class="xl:col-span-12">
			<div
				class="flex gap-1 px-4 py-3 mb-5 text-sm text-green-500 border border-green-200 rounded-md md:items-center bg-green-50 dark:bg-green-400/20 dark:border-green-500/50"
			>
				<LucideIcon name="ShoppingBag" class="h-4 shrink-0" />
				<p>
					The minimum order requirement is <b>$1,800</b>. To meet this threshold, please add
					additional products with a combined value of <b>$300</b>.
				</p>
			</div>
		</div>
		<!--end col-->
		<div class="xl:col-span-8">
			<div class="flex items-center gap-3 mb-5">
				<div class="grow">
					<a
						href="#!"
						class="transition-all duration-300 ease-linear text-custom-500 hover:text-custom-600"
						><LucideIcon
							name="ChevronLeft"
							class="inline-block size-4 align-middle ltr:mr-1 rtl:ml-1 rtl:rotate-180"
						/> <span class="align-middle">Back to Cart</span></a
					>
				</div>
				<div class="shrink-0">
					<button
						type="button"
						class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
						><span class="align-middle">Place Order</span>
						<LucideIcon
							name="MoveRight"
							class="inline-block size-4 align-middle ltr:ml-1 rtl:mr-1 rtl:rotate-180"
						/></button
					>
				</div>
			</div>
			<div class="card">
				<div class="card-body">
					<h6 class="mb-4 text-15">Shipping Information</h6>
					<form action="#!">
						<div class="grid grid-cols-1 gap-5 md:grid-cols-2 xl:grid-cols-12">
							<div class="xl:col-span-4">
								<label for="firstNameInput" class="inline-block mb-2 text-base font-medium"
									>First Name</label
								>
								<input
									type="text"
									id="firstNameInput"
									class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
									placeholder="Enter First Name"
								/>
							</div>
							<!--end col-->
							<div class="xl:col-span-4">
								<label for="middleNameInput" class="inline-block mb-2 text-base font-medium"
									>Middle Name</label
								>
								<input
									type="text"
									id="middleNameInput"
									class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
									placeholder="Enter Middle Name"
								/>
							</div>
							<!--end col-->
							<div class="xl:col-span-4">
								<label for="lastNameInput" class="inline-block mb-2 text-base font-medium"
									>Last Name</label
								>
								<input
									type="text"
									id="lastNameInput"
									class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
									placeholder="Enter Last Name"
								/>
							</div>
							<!--end col-->
							<div class="xl:col-span-4">
								<label for="phoneNumberInput" class="inline-block mb-2 text-base font-medium"
									>Phone Number</label
								>
								<input
									type="text"
									id="phoneNumberInput"
									class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
									placeholder="(012) 345 678 9010"
								/>
							</div>
							<!--end col-->
							<div class="xl:col-span-4">
								<label for="alternativeNumberInput" class="inline-block mb-2 text-base font-medium"
									>Alternative Number</label
								>
								<input
									type="text"
									id="alternativeNumberInput"
									class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
									placeholder="(012) 345 678 9010"
								/>
							</div>
							<!--end col-->
							<div class="xl:col-span-4">
								<label for="emailAddressInput" class="inline-block mb-2 text-base font-medium"
									>Email Address</label
								>
								<input
									type="email"
									id="emailAddressInput"
									class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
									placeholder="Enter email"
								/>
							</div>
							<!--end col-->
							<div class="xl:col-span-12">
								<label for="streetAddressInput" class="inline-block mb-2 text-base font-medium"
									>Street Address</label
								>
								<input
									type="text"
									id="streetAddressInput"
									class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
									placeholder="Street address"
								/>
							</div>
							<!--end col-->
							<div class="xl:col-span-12">
								<label for="townCityInput" class="inline-block mb-2 text-base font-medium"
									>Town/City</label
								>
								<input
									type="text"
									id="townCityInput"
									class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
									placeholder="Town/City"
								/>
							</div>
							<!--end col-->
							<div class="xl:col-span-4">
								<label for="stateInput" class="inline-block mb-2 text-base font-medium">State</label
								>
								<input
									type="text"
									id="stateInput"
									class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
									placeholder="State"
								/>
							</div>
							<!--end col-->
							<div class="xl:col-span-4">
								<label for="countryInput" class="inline-block mb-2 text-base font-medium"
									>Country</label
								>
								<select
									id="countryInput"
									name="countryInput"
									data-choices
									class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
								>
									<option value="Afghanistan">Afghanistan</option>
									<option value="Åland Islands">Åland Islands</option>
									<option value="Albania">Albania</option>
									<option value="Algeria">Algeria</option>
									<option value="American Samoa">American Samoa</option>
									<option value="Andorra">Andorra</option>
									<option value="Angola">Angola</option>
									<option value="Anguilla">Anguilla</option>
									<option value="Antarctica">Antarctica</option>
									<option value="Antigua and Barbuda">Antigua and Barbuda</option>
									<option value="Argentina">Argentina</option>
									<option value="Armenia">Armenia</option>
									<option value="Aruba">Aruba</option>
									<option value="Australia">Australia</option>
									<option value="Austria">Austria</option>
									<option value="Azerbaijan">Azerbaijan</option>
									<option value="Bahamas">Bahamas</option>
									<option value="Bahrain">Bahrain</option>
									<option value="Bangladesh">Bangladesh</option>
									<option value="Barbados">Barbados</option>
									<option value="Belarus">Belarus</option>
									<option value="Belgium">Belgium</option>
									<option value="Belize">Belize</option>
									<option value="Benin">Benin</option>
									<option value="Bermuda">Bermuda</option>
									<option value="Bhutan">Bhutan</option>
									<option value="Bolivia">Bolivia</option>
									<option value="Bosnia and Herzegovina">Bosnia and Herzegovina</option>
									<option value="Botswana">Botswana</option>
									<option value="Bouvet Island">Bouvet Island</option>
									<option value="Brazil">Brazil</option>
									<option value="British Indian Ocean Territory"
										>British Indian Ocean Territory</option
									>
									<option value="Brunei Darussalam">Brunei Darussalam</option>
									<option value="Bulgaria">Bulgaria</option>
									<option value="Burkina Faso">Burkina Faso</option>
									<option value="Burundi">Burundi</option>
									<option value="Cambodia">Cambodia</option>
									<option value="Cameroon">Cameroon</option>
									<option value="Canada">Canada</option>
									<option value="Cape Verde">Cape Verde</option>
									<option value="Cayman Islands">Cayman Islands</option>
									<option value="Central African Republic">Central African Republic</option>
									<option value="Chad">Chad</option>
									<option value="Chile">Chile</option>
									<option value="China">China</option>
									<option value="Christmas Island">Christmas Island</option>
									<option value="Cocos (Keeling) Islands">Cocos (Keeling) Islands</option>
									<option value="Colombia">Colombia</option>
									<option value="Comoros">Comoros</option>
									<option value="Congo">Congo</option>
									<option value="Congo, The Democratic Republic of The"
										>Congo, The Democratic Republic of The</option
									>
									<option value="Cook Islands">Cook Islands</option>
									<option value="Costa Rica">Costa Rica</option>
									<option value="Cote D'ivoire">Cote D'ivoire</option>
									<option value="Croatia">Croatia</option>
									<option value="Cuba">Cuba</option>
									<option value="Cyprus">Cyprus</option>
									<option value="Czech Republic">Czech Republic</option>
									<option value="Denmark">Denmark</option>
									<option value="Djibouti">Djibouti</option>
									<option value="Dominica">Dominica</option>
									<option value="Dominican Republic">Dominican Republic</option>
									<option value="Ecuador">Ecuador</option>
									<option value="Egypt">Egypt</option>
									<option value="El Salvador">El Salvador</option>
									<option value="Equatorial Guinea">Equatorial Guinea</option>
									<option value="Eritrea">Eritrea</option>
									<option value="Estonia">Estonia</option>
									<option value="Ethiopia">Ethiopia</option>
									<option value="Falkland Islands (Malvinas)">Falkland Islands (Malvinas)</option>
									<option value="Faroe Islands">Faroe Islands</option>
									<option value="Fiji">Fiji</option>
									<option value="Finland">Finland</option>
									<option value="France">France</option>
									<option value="French Guiana">French Guiana</option>
									<option value="French Polynesia">French Polynesia</option>
									<option value="French Southern Territories">French Southern Territories</option>
									<option value="Gabon">Gabon</option>
									<option value="Gambia">Gambia</option>
									<option value="Georgia">Georgia</option>
									<option value="Germany">Germany</option>
									<option value="Ghana">Ghana</option>
									<option value="Gibraltar">Gibraltar</option>
									<option value="Greece">Greece</option>
									<option value="Greenland">Greenland</option>
									<option value="Grenada">Grenada</option>
									<option value="Guadeloupe">Guadeloupe</option>
									<option value="Guam">Guam</option>
									<option value="Guatemala">Guatemala</option>
									<option value="Guernsey">Guernsey</option>
									<option value="Guinea">Guinea</option>
									<option value="Guinea-bissau">Guinea-bissau</option>
									<option value="Guyana">Guyana</option>
									<option value="Haiti">Haiti</option>
									<option value="Heard Island and Mcdonald Islands"
										>Heard Island and Mcdonald Islands</option
									>
									<option value="Holy See (Vatican City State)"
										>Holy See (Vatican City State)</option
									>
									<option value="Honduras">Honduras</option>
									<option value="Hong Kong">Hong Kong</option>
									<option value="Hungary">Hungary</option>
									<option value="Iceland">Iceland</option>
									<option value="India">India</option>
									<option value="Indonesia">Indonesia</option>
									<option value="Iran, Islamic Republic of">Iran, Islamic Republic of</option>
									<option value="Iraq">Iraq</option>
									<option value="Ireland">Ireland</option>
									<option value="Isle of Man">Isle of Man</option>
									<option value="Israel">Israel</option>
									<option value="Italy">Italy</option>
									<option value="Jamaica">Jamaica</option>
									<option value="Japan">Japan</option>
									<option value="Jersey">Jersey</option>
									<option value="Jordan">Jordan</option>
									<option value="Kazakhstan">Kazakhstan</option>
									<option value="Kenya">Kenya</option>
									<option value="Kiribati">Kiribati</option>
									<option value="Korea, Democratic People's Republic of"
										>Korea, Democratic People's Republic of</option
									>
									<option value="Korea, Republic of">Korea, Republic of</option>
									<option value="Kuwait">Kuwait</option>
									<option value="Kyrgyzstan">Kyrgyzstan</option>
									<option value="Lao People's Democratic Republic"
										>Lao People's Democratic Republic</option
									>
									<option value="Latvia">Latvia</option>
									<option value="Lebanon">Lebanon</option>
									<option value="Lesotho">Lesotho</option>
									<option value="Liberia">Liberia</option>
									<option value="Libyan Arab Jamahiriya">Libyan Arab Jamahiriya</option>
									<option value="Liechtenstein">Liechtenstein</option>
									<option value="Lithuania">Lithuania</option>
									<option value="Luxembourg">Luxembourg</option>
									<option value="Macao">Macao</option>
									<option value="Macedonia, The Former Yugoslav Republic of"
										>Macedonia, The Former Yugoslav Republic of</option
									>
									<option value="Madagascar">Madagascar</option>
									<option value="Malawi">Malawi</option>
									<option value="Malaysia">Malaysia</option>
									<option value="Maldives">Maldives</option>
									<option value="Mali">Mali</option>
									<option value="Malta">Malta</option>
									<option value="Marshall Islands">Marshall Islands</option>
									<option value="Martinique">Martinique</option>
									<option value="Mauritania">Mauritania</option>
									<option value="Mauritius">Mauritius</option>
									<option value="Mayotte">Mayotte</option>
									<option value="Mexico">Mexico</option>
									<option value="Micronesia, Federated States of"
										>Micronesia, Federated States of</option
									>
									<option value="Moldova, Republic of">Moldova, Republic of</option>
									<option value="Monaco">Monaco</option>
									<option value="Mongolia">Mongolia</option>
									<option value="Montenegro">Montenegro</option>
									<option value="Montserrat">Montserrat</option>
									<option value="Morocco">Morocco</option>
									<option value="Mozambique">Mozambique</option>
									<option value="Myanmar">Myanmar</option>
									<option value="Namibia">Namibia</option>
									<option value="Nauru">Nauru</option>
									<option value="Nepal">Nepal</option>
									<option value="Netherlands">Netherlands</option>
									<option value="Netherlands Antilles">Netherlands Antilles</option>
									<option value="New Caledonia">New Caledonia</option>
									<option value="New Zealand">New Zealand</option>
									<option value="Nicaragua">Nicaragua</option>
									<option value="Niger">Niger</option>
									<option value="Nigeria">Nigeria</option>
									<option value="Niue">Niue</option>
									<option value="Norfolk Island">Norfolk Island</option>
									<option value="Northern Mariana Islands">Northern Mariana Islands</option>
									<option value="Norway">Norway</option>
									<option value="Oman">Oman</option>
									<option value="Pakistan">Pakistan</option>
									<option value="Palau">Palau</option>
									<option value="Palestinian Territory, Occupied"
										>Palestinian Territory, Occupied</option
									>
									<option value="Panama">Panama</option>
									<option value="Papua New Guinea">Papua New Guinea</option>
									<option value="Paraguay">Paraguay</option>
									<option value="Peru">Peru</option>
									<option value="Philippines">Philippines</option>
									<option value="Pitcairn">Pitcairn</option>
									<option value="Poland">Poland</option>
									<option value="Portugal">Portugal</option>
									<option value="Puerto Rico">Puerto Rico</option>
									<option value="Qatar">Qatar</option>
									<option value="Reunion">Reunion</option>
									<option value="Romania">Romania</option>
									<option value="Russian Federation">Russian Federation</option>
									<option value="Rwanda">Rwanda</option>
									<option value="Saint Helena">Saint Helena</option>
									<option value="Saint Kitts and Nevis">Saint Kitts and Nevis</option>
									<option value="Saint Lucia">Saint Lucia</option>
									<option value="Saint Pierre and Miquelon">Saint Pierre and Miquelon</option>
									<option value="Saint Vincent and The Grenadines"
										>Saint Vincent and The Grenadines</option
									>
									<option value="Samoa">Samoa</option>
									<option value="San Marino">San Marino</option>
									<option value="Sao Tome and Principe">Sao Tome and Principe</option>
									<option value="Saudi Arabia">Saudi Arabia</option>
									<option value="Senegal">Senegal</option>
									<option value="Serbia">Serbia</option>
									<option value="Seychelles">Seychelles</option>
									<option value="Sierra Leone">Sierra Leone</option>
									<option value="Singapore">Singapore</option>
									<option value="Slovakia">Slovakia</option>
									<option value="Slovenia">Slovenia</option>
									<option value="Solomon Islands">Solomon Islands</option>
									<option value="Somalia">Somalia</option>
									<option value="South Africa">South Africa</option>
									<option value="South Georgia and The South Sandwich Islands"
										>South Georgia and The South Sandwich Islands</option
									>
									<option value="Spain">Spain</option>
									<option value="Sri Lanka">Sri Lanka</option>
									<option value="Sudan">Sudan</option>
									<option value="Suriname">Suriname</option>
									<option value="Svalbard and Jan Mayen">Svalbard and Jan Mayen</option>
									<option value="Swaziland">Swaziland</option>
									<option value="Sweden">Sweden</option>
									<option value="Switzerland">Switzerland</option>
									<option value="Syrian Arab Republic">Syrian Arab Republic</option>
									<option value="Taiwan">Taiwan</option>
									<option value="Tajikistan">Tajikistan</option>
									<option value="Tanzania, United Republic of">Tanzania, United Republic of</option>
									<option value="Thailand">Thailand</option>
									<option value="Timor-leste">Timor-leste</option>
									<option value="Togo">Togo</option>
									<option value="Tokelau">Tokelau</option>
									<option value="Tonga">Tonga</option>
									<option value="Trinidad and Tobago">Trinidad and Tobago</option>
									<option value="Tunisia">Tunisia</option>
									<option value="Turkey">Turkey</option>
									<option value="Turkmenistan">Turkmenistan</option>
									<option value="Turks and Caicos Islands">Turks and Caicos Islands</option>
									<option value="Tuvalu">Tuvalu</option>
									<option value="Uganda">Uganda</option>
									<option value="Ukraine">Ukraine</option>
									<option value="United Arab Emirates">United Arab Emirates</option>
									<option value="United Kingdom">United Kingdom</option>
									<option value="United States">United States</option>
									<option value="United States Minor Outlying Islands"
										>United States Minor Outlying Islands</option
									>
									<option value="Uruguay">Uruguay</option>
									<option value="Uzbekistan">Uzbekistan</option>
									<option value="Vanuatu">Vanuatu</option>
									<option value="Venezuela">Venezuela</option>
									<option value="Viet Nam">Viet Nam</option>
									<option value="Virgin Islands, British">Virgin Islands, British</option>
									<option value="Virgin Islands, U.S.">Virgin Islands, U.S.</option>
									<option value="Wallis and Futuna">Wallis and Futuna</option>
									<option value="Western Sahara">Western Sahara</option>
									<option value="Yemen">Yemen</option>
									<option value="Zambia">Zambia</option>
									<option value="Zimbabwe">Zimbabwe</option>
								</select>
							</div>
							<!--end col-->
							<div class="xl:col-span-4">
								<label for="zipcodeInput" class="inline-block mb-2 text-base font-medium"
									>ZipCode</label
								>
								<input
									type="text"
									id="zipcodeInput"
									class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
									placeholder="ZipCode"
								/>
							</div>
							<!--end col-->
						</div>
						<!--end grid-->
					</form>
				</div>
			</div>
			<!--end card-->

			<div class="card">
				<div class="card-body">
					<h6 class="mb-4 text-15">Delivery</h6>

					<div class="grid grid-cols-1 gap-5 xl:grid-cols-2">
						<div class="flex items-center gap-3">
							<input
								id="deliveryOption1"
								class="size-4 border rounded-full appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-purple-500 checked:border-purple-500 dark:checked:bg-purple-500 dark:checked:border-purple-500 checked:disabled:bg-purple-400 checked:disabled:border-purple-400 peer"
								type="radio"
								name="deliveryChoose"
								value="express-delivery"
								checked
							/>
							<label
								for="deliveryOption1"
								class="flex flex-col gap-4 p-5 border rounded-md cursor-pointer md:flex-row border-slate-200 dark:border-zink-500 peer-checked:border-purple-500 dark:peer-checked:border-purple-700 grow"
							>
								<span class="shrink-0">
									<img src="/assets/images/brand/delivery-1.png" alt="" class="h-12" />
								</span>
								<span class="grow">
									<span class="block mb-1 font-semibold text-15">Express Delivery</span>
									<span class="text-slate-500 dark:text-zink-200"
										>Expected delivery: 01 Nov, Wednesday</span
									>
								</span>
								<span class="shrink-0">
									<span class="block text-lg font-semibold">$11.99</span>
								</span>
							</label>
						</div>
						<div class="flex items-center gap-3">
							<input
								id="deliveryOption2"
								class="size-4 border rounded-full appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-purple-500 checked:border-purple-500 dark:checked:bg-purple-500 dark:checked:border-purple-500 checked:disabled:bg-purple-400 checked:disabled:border-purple-400 peer"
								type="radio"
								name="deliveryChoose"
								value="express-delivery"
							/>
							<label
								for="deliveryOption2"
								class="flex flex-col gap-4 p-5 border rounded-md cursor-pointer md:flex-row border-slate-200 dark:border-zink-500 peer-checked:border-purple-500 dark:peer-checked:border-purple-700 grow"
							>
								<span class="shrink-0">
									<img src="/assets/images/brand/delivery-2.png" alt="" class="h-12" />
								</span>
								<span class="grow">
									<span class="block mb-1 font-semibold text-15">Air Logistic Delivery</span>
									<span class="text-slate-500 dark:text-zink-200"
										>Expected delivery: 06 Nov, Monday</span
									>
								</span>
								<span class="shrink-0">
									<span class="block text-lg font-semibold">$5</span>
								</span>
							</label>
						</div>
						<div class="flex items-center gap-3">
							<input
								id="deliveryOption3"
								class="size-4 border rounded-full appearance-none cursor-pointer bg-slate-100 border-slate-200 dark:bg-zink-600 dark:border-zink-500 checked:bg-purple-500 checked:border-purple-500 dark:checked:bg-purple-500 dark:checked:border-purple-500 checked:disabled:bg-purple-400 checked:disabled:border-purple-400 peer"
								type="radio"
								name="deliveryChoose"
								value="express-delivery"
							/>
							<label
								for="deliveryOption3"
								class="flex flex-col gap-4 p-5 border rounded-md cursor-pointer md:flex-row border-slate-200 dark:border-zink-500 peer-checked:border-purple-500 dark:peer-checked:border-purple-700 grow"
							>
								<span class="shrink-0">
									<img src="/assets/images/brand/delivery-3.png" alt="" class="h-12" />
								</span>
								<span class="grow">
									<span class="block mb-1 font-semibold text-15">Free Delivery</span>
									<span class="text-slate-500 dark:text-zink-200"
										>Expected delivery: 11 Nov, Saturday</span
									>
								</span>
								<span class="shrink-0">
									<span class="block text-lg font-semibold">$0</span>
								</span>
							</label>
						</div>
					</div>
				</div>
			</div>
			<!--end card-->

			<div class="card">
				<div class="card-body">
					<h6 class="mb-4 text-15">Payment Information</h6>
					<form action="#!">
						<div class="grid grid-cols-1 gap-5 xl:grid-cols-12">
							<div class="xl:col-span-12">
								<label for="cardNumberInput" class="inline-block mb-2 text-base font-medium"
									>Card Number</label
								>
								<input
									type="text"
									pattern="\d*"
									maxlength="16"
									id="cardNumberInput"
									class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
									placeholder="XXXX XXXX XXXX XXXX"
								/>
							</div>
							<!--end col-->
							<div class="xl:col-span-6">
								<label for="expiringInput" class="inline-block mb-2 text-base font-medium"
									>Expiring (MM/YY)</label
								>
								<input
									type="text"
									pattern="\d*"
									maxlength="4"
									id="expiringInput"
									class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
									placeholder="MM/YY"
								/>
							</div>
							<!--end col-->
							<div class="xl:col-span-6">
								<label for="cvvInput" class="inline-block mb-2 text-base font-medium"
									>CVV Code</label
								>
								<input
									type="text"
									pattern="\d*"
									maxlength="3"
									id="cvvInput"
									class="form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200"
									placeholder="000"
								/>
							</div>
							<!--end col-->
						</div>
						<!--end grid-->
					</form>

					<div class="mt-3">
						<h6 class="mb-1">We accept the following cards</h6>
						<div class="flex items-center gap-2">
							<img src="/assets/images/payment/img-01.png" alt="" class="h-8" />
							<img src="/assets/images/payment/img-02.png" alt="" class="h-8" />
							<img src="/assets/images/payment/img-03.png" alt="" class="h-8" />
							<img src="/assets/images/payment/img-04.png" alt="" class="h-8" />
						</div>
					</div>
				</div>
			</div>
			<!--end card-->
		</div>
		<!--end col-->
		<div class="xl:col-span-4">
			<div class="card">
				<div class="card-body">
					<h6 class="mb-4 text-15">Orders Summary</h6>
					<div
						class="px-4 py-3 mb-4 text-sm text-red-500 border border-transparent rounded-md bg-red-50 dark:bg-red-400/20"
					>
						These products are limited, checkout within <span class="font-bold">03m 21s</span>
					</div>
					<div class="overflow-x-auto">
						<table class="w-full">
							<tbody>
								<tr>
									<td
										class="px-3.5 py-4 border-b border-dashed first:pl-0 last:pr-0 border-slate-200 dark:border-zink-500"
									>
										<div class="flex items-center gap-3">
											<div
												class="flex items-center justify-center size-12 rounded-md bg-slate-100 shrink-0"
											>
												<img src="/assets/images/product/img-08.png" alt="" class="h-8" />
											</div>
											<div class="grow">
												<h6 class="mb-1 text-15">
													<a
														href="/apps/ecommerce/products-overview"
														class="transition-all duration-300 ease-linear hover:text-custom-500"
														>Roar Twill Blue Baseball Cap</a
													>
												</h6>
												<p class="text-slate-500 dark:text-zink-200">$149.99 x 02</p>
											</div>
										</div>
									</td>
									<td
										class="px-3.5 py-4 border-b border-dashed first:pl-0 last:pr-0 border-slate-200 dark:border-zink-500 ltr:text-right rtl:text-left"
										>$299.98</td
									>
								</tr>
								<tr>
									<td
										class="px-3.5 py-4 border-b border-dashed first:pl-0 last:pr-0 border-slate-200 dark:border-zink-500"
									>
										<div class="flex items-center gap-3">
											<div
												class="flex items-center justify-center size-12 rounded-md bg-slate-100 shrink-0"
											>
												<img src="/assets/images/product/img-04.png" alt="" class="h-8" />
											</div>
											<div class="grow">
												<h6 class="mb-1 text-15">
													<a
														href="/apps/ecommerce/products-overview"
														class="transition-all duration-300 ease-linear hover:text-custom-500"
														>Mesh Ergonomic Green Chair</a
													>
												</h6>
												<p class="text-slate-500 dark:text-zink-200">$362.21 x 1</p>
											</div>
										</div>
									</td>
									<td
										class="px-3.5 py-4 border-b border-dashed first:pl-0 last:pr-0 border-slate-200 dark:border-zink-500 ltr:text-right rtl:text-left"
										>$362.21</td
									>
								</tr>
								<tr>
									<td
										class="px-3.5 py-4 border-b border-dashed first:pl-0 last:pr-0 border-slate-200 dark:border-zink-500"
									>
										<div class="flex items-center gap-3">
											<div
												class="flex items-center justify-center size-12 rounded-md bg-slate-100 shrink-0"
											>
												<img src="/assets/images/product/img-01.png" alt="" class="h-8" />
											</div>
											<div class="grow">
												<h6 class="mb-1 text-15">
													<a
														href="/apps/ecommerce/products-overview"
														class="transition-all duration-300 ease-linear hover:text-custom-500"
														>Smartest Printed T-shirt</a
													>
												</h6>
												<p class="text-slate-500 dark:text-zink-200">$89.99 x 03</p>
											</div>
										</div>
									</td>
									<td
										class="px-3.5 py-4 border-b border-dashed first:pl-0 last:pr-0 border-slate-200 dark:border-zink-500 ltr:text-right rtl:text-left"
										>$269.97</td
									>
								</tr>
								<tr>
									<td
										class="px-3.5 pt-4 pb-3 first:pl-0 last:pr-0 text-slate-500 dark:text-zink-200"
									>
										Sub Total
									</td>
									<td class="px-3.5 pt-4 pb-3 first:pl-0 last:pr-0 ltr:text-right rtl:text-left"
										>$932.16</td
									>
								</tr>
								<tr>
									<td class="px-3.5 py-3 first:pl-0 last:pr-0 text-slate-500 dark:text-zink-200">
										Estimated Tax (18%)
									</td>
									<td class="px-3.5 py-3 first:pl-0 last:pr-0 ltr:text-right rtl:text-left"
										>$167.79</td
									>
								</tr>
								<tr>
									<td class="px-3.5 py-3 first:pl-0 last:pr-0 text-slate-500 dark:text-zink-200">
										Item Discounts (12%)
									</td>
									<td class="px-3.5 py-3 first:pl-0 last:pr-0 ltr:text-right rtl:text-left"
										>-$111.86</td
									>
								</tr>
								<tr>
									<td class="px-3.5 py-3 first:pl-0 last:pr-0 text-slate-500 dark:text-zink-200">
										Shipping Charge
									</td>
									<td class="px-3.5 py-3 first:pl-0 last:pr-0 ltr:text-right rtl:text-left">$0</td>
								</tr>
								<tr class="font-semibold">
									<td class="px-3.5 pt-3 first:pl-0 last:pr-0 text-slate-500 dark:text-zink-200">
										Total Amount (USD)
									</td>
									<td class="px-3.5 pt-3 first:pl-0 last:pr-0 ltr:text-right rtl:text-left"
										>$988.09</td
									>
								</tr>
							</tbody>
						</table>
					</div>
					<div class="mt-4">
						<button
							type="button"
							class="w-full text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
							><span class="align-middle">Place Order</span>
							<LucideIcon
								name="MoveRight"
								class="inline-block size-4 align-middle ltr:ml-1 rtl:mr-1 rtl:rotate-180"
							/></button
						>
					</div>
				</div>
			</div>
			<h6 class="mb-4 underline text-16">Additional Service</h6>
			<div class="card">
				<div class="flex flex-col gap-3 md:items-center card-body md:flex-row">
					<div class="grow">
						<h6 class="mb-1 text-15">Care + Package</h6>
						<p class="text-slate-500 dark:text-zink-200">2 year of additional care</p>
					</div>
					<div class="shrink-0">
						<b>$24.99</b>
					</div>
					<div class="shrink-0">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in">
							<input
								type="checkbox"
								name="carePackage"
								id="carePackage"
								class="absolute block size-5 transition duration-300 ease-linear border-2 rounded-full appearance-none cursor-pointer border-slate-200 dark:border-zink-600 bg-white/80 dark:bg-zink-400 peer/published checked:bg-custom-500 dark:checked:bg-custom-500 ltr:checked:right-0 rtl:checked:left-0 checked:border-custom-100 dark:checked:border-custom-900 arrow-none checked:bg-none"
							/>
							<label
								for="carePackage"
								class="block h-5 overflow-hidden duration-300 ease-linear border rounded-full cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-custom-100 dark:peer-checked/published:bg-custom-900 peer-checked/published:border-custom-100 dark:peer-checked/published:border-custom-900"
							></label>
						</div>
					</div>
				</div>
			</div>
			<!--end card-->
			<div class="card">
				<div class="flex flex-col gap-3 md:items-center card-body md:flex-row">
					<div class="grow">
						<h6 class="mb-1 text-15">Environment Friendly</h6>
						<p class="text-slate-500 dark:text-zink-200">
							The primary goal of eco-warriors is creating
						</p>
					</div>
					<div class="shrink-0">
						<b>$19.99</b>
					</div>
					<div class="shrink-0">
						<div class="relative inline-block w-10 align-middle transition duration-200 ease-in">
							<input
								type="checkbox"
								name="friendlyCheckbox"
								id="friendlyCheckbox"
								class="absolute block size-5 transition duration-300 ease-linear border-2 rounded-full appearance-none cursor-pointer border-slate-200 dark:border-zink-600 bg-white/80 dark:bg-zink-400 peer/published checked:bg-custom-500 dark:checked:bg-custom-500 ltr:checked:right-0 rtl:checked:left-0 checked:border-custom-100 dark:checked:border-custom-900 arrow-none checked:bg-none"
							/>
							<label
								for="friendlyCheckbox"
								class="block h-5 overflow-hidden duration-300 ease-linear border rounded-full cursor-pointer cursor-pointertransition border-slate-200 dark:border-zink-500 bg-slate-200 dark:bg-zink-600 peer-checked/published:bg-custom-100 dark:peer-checked/published:bg-custom-900 peer-checked/published:border-custom-100 dark:peer-checked/published:border-custom-900"
							></label>
						</div>
					</div>
				</div>
			</div>
			<!--end card-->
		</div>
		<!--end col-->
	</div>
</div>
