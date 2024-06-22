<script>
	import Dropdown from '../../../../common/components/Dropdown.svelte';
	import DropdownMenu from '../../../../common/components/DropdownMenu.svelte';
	import DropdownToggle from '../../../../common/components/DropdownToggle.svelte';
	import LucideIcon from '../../../../common/components/LucideIcon.svelte';
	import EmailMarketingChart from './EmailMarketingChart.svelte';
</script>

<div class="col-span-12 card lg:col-span-6 2xl:col-span-3">
	<div class="card-body">
		<div class="flex items-center">
			<h6 class="text-15 grow">Email Marketing</h6>

			<Dropdown class="relative shrink-0" direction="bottom-start">
				<DropdownToggle
					className="flex items-center justify-center  size-[30px] p-0 bg-white text-slate-500 btn hover:text-slate-500 hover:bg-slate-100 focus:text-slate-500 focus:bg-slate-100 active:text-slate-500 active:bg-slate-100 dark:bg-zink-700 dark:hover:bg-slate-500/10 dark:focus:bg-slate-500/10 dark:active:bg-slate-500/10"
				>
					<LucideIcon name="MoreHorizontal" class="size-3" />
				</DropdownToggle>
				<DropdownMenu
					tag="ul"
					class="absolute z-50 py-2 mt-1 ltr:text-left rtl:text-right list-none bg-white rounded-md shadow-md dropdown-menu min-w-[10rem] dark:bg-zink-600"
				>
					<li>
						<a
							class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
							href="#!">This Year</a
						>
					</li>
					<li>
						<a
							class="block px-4 py-1.5 text-base transition-all duration-200 ease-linear text-slate-600 dropdown-item hover:bg-slate-100 hover:text-slate-500 focus:bg-slate-100 focus:text-slate-500 dark:text-zink-100 dark:hover:bg-zink-500 dark:hover:text-zink-200 dark:focus:bg-zink-500 dark:focus:text-zink-200"
							href="#!">Last Year</a
						>
					</li>
				</DropdownMenu>
			</Dropdown>
		</div>

		<EmailMarketingChart chartColors={['bg-sky-500', 'bg-green-500', 'bg-red-500']} />
	</div>
</div>
<div class="col-span-12 card lg:col-span-6 2xl:col-span-3">
	<div class="flex flex-col h-full card-body">
		<div class="mb-auto">
			<h4 class="mb-2">Experience Our Fresh Email Composition Interface</h4>
			<p class="mb-5 text-slate-500 dark:text-zink-200">
				A local-part, the symbol @, and a domain, which may be a domain name or an IP address
				enclosed in brackets.
			</p>
			<a
				href="/apps/mailbox"
				class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
				>Compose Email</a
			>
		</div>
		<div>
			<img
				src="/assets/images/email-dashboard.png"
				alt=""
				class="h-48 ltr:ml-auto rtl:mr-auto -scale-x-100"
			/>
		</div>
	</div>
</div>
