<script>
	import HeadTitle from '../../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../../common/components/Breadcrumb.svelte';
	import LucideIcon from '../../../../../common/components/LucideIcon.svelte';
	import CounterPage from '../../../../../common/components/CounterPage.svelte';
	import Modal from '../../../../../common/components/Modal.svelte';
	import Dropdown from '../../../../../common/components/Dropdown.svelte';
	import DropdownToggle from '../../../../../common/components/DropdownToggle.svelte';
	import DropdownMenu from '../../../../../common/components/DropdownMenu.svelte';
	import { EmployeeSalaryData } from '../../../../../common/data/hrmanagement';
	import Flatpickr from 'svelte-flatpickr';
	import 'flatpickr/dist/flatpickr.css';
</script>

<HeadTitle title="Employee Salary" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto relative">
	<Breadcrumb title="Employee Salary" pagetitle="Payroll" />

	<div class="card">
		<div class="card-body">
			<div class="grid grid-cols-1 gap-5 2xl:grid-cols-12">
				<div class="2xl:col-span-5">
					<h5 class="mb-1">
						$78,564.00 <small class="font-normal text-slate-500 dark:text-zink-200">/ year</small>
					</h5>
					<p class="text-slate-500 dark:text-zink-200">Average Base Salary (USD)</p>
					<div class="mt-6 mb-4">
						<div class="flex w-full h-10 rounded-md bg-slate-200 dark:bg-zink-600">
							<div
								class="flex items-center justify-center h-10 text-white bg-custom-500 ltr:last:rounded-r-md rtl:last:rounded-l-md ltr:first:rounded-l-md rtl:first:rounded-r-md"
								style="width: 20%"
							>
								20%
							</div>
							<div
								class="flex items-center justify-center h-10 text-white bg-green-500 ltr:last:rounded-r-md rtl:last:rounded-l-md ltr:first:rounded-l-md rtl:first:rounded-r-md"
								style="width: 15%"
							>
								15%
							</div>
							<div
								class="flex items-center justify-center h-10 text-white bg-purple-500 ltr:last:rounded-r-md rtl:last:rounded-l-md ltr:first:rounded-l-md rtl:first:rounded-r-md"
								style="width: 35%"
							>
								35%
							</div>
							<div
								class="flex items-center justify-center h-10 text-white bg-orange-500 ltr:last:rounded-r-md rtl:last:rounded-l-md ltr:first:rounded-l-md rtl:first:rounded-r-md"
								style="width: 30%"
							>
								30%
							</div>
						</div>
					</div>
					<p>The average salary for all employees is $78,564.</p>
				</div>
				<!--end col-->
				<div class="2xl:col-span-5 2xl:col-start-8">
					<div class="overflow-x-auto">
						<table class="w-full">
							<tbody>
								<tr>
									<td class="px-3.5 first:pl-0 last:pr-0 py-2 border-y border-transparent"
										>Base Salary</td
									>
									<td
										class="px-3.5 first:pl-0 last:pr-0 py-2 border-y border-transparent font-semibold"
										>$45,932</td
									>
								</tr>
								<tr>
									<td class="px-3.5 first:pl-0 last:pr-0 py-2 border-y border-transparent">Bonus</td
									>
									<td
										class="px-3.5 first:pl-0 last:pr-0 py-2 border-y border-transparent font-semibold"
										>$16,000</td
									>
								</tr>
								<tr>
									<td class="px-3.5 first:pl-0 last:pr-0 py-2 border-y border-transparent"
										>Profit Sharing</td
									>
									<td
										class="px-3.5 first:pl-0 last:pr-0 py-2 border-y border-transparent font-semibold"
										>$2.5k to 4.5k</td
									>
								</tr>
								<tr>
									<td class="px-3.5 first:pl-0 last:pr-0 py-2 border-y border-transparent"
										>Total Pay</td
									>
									<td
										class="px-3.5 first:pl-0 last:pr-0 py-2 border-y border-transparent font-semibold"
										>$78,564.00</td
									>
								</tr>
							</tbody>
						</table>
					</div>
				</div>
				<!--end col-->
			</div>
			<!--end grid-->
		</div>
	</div>

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
					<div class="lg:ltr:text-right lg:rtl:text-left">
						<a
							href="/apps/hr/add-payslip"
							type="buton"
							class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
							><LucideIcon name="Plus" class="inline-block size-4" />
							<span class="align-middle">Create Salary</span></a
						>
					</div>
				</div>
			</div>
			<!--col grid-->
			<div class="-mx-5 overflow-x-auto">
				<table class="w-full whitespace-nowrap">
					<thead class="ltr:text-left rtl:text-right">
						<tr class="bg-slate-100 dark:bg-zink-600">
							<th
								class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold border-b border-slate-200 dark:border-zink-500"
								>Employee ID</th
							>
							<th
								class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold border-b border-slate-200 dark:border-zink-500"
								>Joining Date</th
							>
							<th
								class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold border-b border-slate-200 dark:border-zink-500"
								>Name</th
							>
							<th
								class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold border-b border-slate-200 dark:border-zink-500"
								>Designation</th
							>
							<th
								class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold border-b border-slate-200 dark:border-zink-500"
								>Email Id</th
							>
							<th
								class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold border-b border-slate-200 dark:border-zink-500"
								>Phone Number</th
							>
							<th
								class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold border-b border-slate-200 dark:border-zink-500"
								>Salary</th
							>
							<th
								class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold border-b border-slate-200 dark:border-zink-500"
								>Bonus</th
							>
							<th
								class="px-3.5 py-2.5 first:pl-5 last:pr-5 font-semibold border-b border-slate-200 dark:border-zink-500"
								>Action</th
							>
						</tr>
					</thead>
					<tbody>
						{#each EmployeeSalaryData as row}
							<tr>
								<td
									class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
									><a
										href="#!"
										class="transition-all duration-150 ease-linear text-custom-500 hover:text-custom-600"
										>{row.employeeId}</a
									></td
								>
								<td
									class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
									>{row.joiningDate}</td
								>
								<td
									class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
								>
									<a href="#!" class="flex items-center gap-3">
										<div class="size-6 rounded-full shrink-0 bg-slate-100">
											<img src={row.img} alt="" class="h-6 rounded-full" />
										</div>
										<h6 class="grow">{row.name}</h6>
									</a>
								</td>
								<td
									class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
									>{row.designation}</td
								>
								<td
									class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
									>{row.emailId}</td
								>
								<td
									class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
									>{row.phoneNumber}</td
								>
								<td
									class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
									>{row.salary}</td
								>
								<td
									class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500"
									>{row.bonus}</td
								>
								<td
									class="px-3.5 py-2.5 first:pl-5 last:pr-5 border-y border-slate-200 dark:border-zink-500 flex items-center gap-2"
								>
									<a
										href="/apps/hr/payslip"
										class="px-2 py-1.5 text-xs text-white btn bg-sky-500 border-sky-500 hover:text-white hover:bg-sky-600 hover:border-sky-600 focus:text-white focus:bg-sky-600 focus:border-sky-600 focus:ring focus:ring-sky-100 active:text-white active:bg-sky-600 active:border-sky-600 active:ring active:ring-sky-100 dark:ring-sky-400/20"
										>Generate Slip</a
									>
									<Dropdown className="relative">
										<DropdownToggle
											className="flex items-center justify-center  size-[30px] dropdown-toggle p-0 text-slate-500 btn bg-slate-100 hover:text-white hover:bg-slate-600 focus:text-white focus:bg-slate-600 focus:ring focus:ring-slate-100 active:text-white active:bg-slate-600 active:ring active:ring-slate-100 dark:bg-slate-500/20 dark:text-slate-400 dark:hover:bg-slate-500 dark:hover:text-white dark:focus:bg-slate-500 dark:focus:text-white dark:active:bg-slate-500 dark:active:text-white dark:ring-slate-400/20"
										>
											<LucideIcon name="MoreHorizontal" class="size-3" />
										</DropdownToggle>
										<DropdownMenu
											class="absolute z-50 py-2 mt-1 ltr:text-left rtl:text-right list-none bg-white rounded-md shadow-md min-w-[10rem] dark:bg-zink-600"
											tag="ul"
										>
											<li>
												<a
													class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
													href="/pages/account"
													><LucideIcon name="Eye" class="inline-block size-3 ltr:mr-1 rtl:ml-1" />
													<span class="align-middle">Overview</span></a
												>
											</li>
											<li>
												<a
													class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
													href="/apps/hr/add-payslip"
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
