<script>
	import Dropdown from './Dropdown.svelte';
	import DropdownToggle from './DropdownToggle.svelte';
	import DropdownMenu from './DropdownMenu.svelte';
	import languages from './../data/languages';
	import { setupI18n } from '../../services/i18n';

	let lngSrc = '/assets/images/flags/20/us.svg';
	function handleLocaleChange(lang, flag) {
		setupI18n({ withLocale: lang });
		lngSrc = flag;
	}
</script>

<Dropdown className="relative flex items-center h-header" direction="bottom-end">
	<DropdownToggle
		className="inline-flex justify-center items-center p-0 text-topbar-item transition-all size-[37.5px] duration-200 ease-linear bg-topbar rounded-md dropdown-toggle btn hover:bg-topbar-item-bg-hover hover:text-topbar-item-hover group-data-[topbar=dark]:bg-topbar-dark group-data-[topbar=dark]:hover:bg-topbar-item-bg-hover-dark group-data-[topbar=dark]:hover:text-topbar-item-hover-dark group-data-[topbar=brand]:bg-topbar-brand group-data-[topbar=brand]:hover:bg-topbar-item-bg-hover-brand group-data-[topbar=brand]:hover:text-topbar-item-hover-brand group-data-[topbar=dark]:dark:bg-zink-700 group-data-[topbar=dark]:dark:hover:bg-zink-600 group-data-[topbar=dark]:dark:text-zink-500 group-data-[topbar=dark]:dark:hover:text-zink-50"
	>
		<img src={lngSrc} alt="" id="header-lang-img" class="h-5 rounded-sm" />
	</DropdownToggle>
	<DropdownMenu
		class="absolute z-50 p-4 ltr:text-left rtl:text-right bg-white rounded-md shadow-md !top-4 dropdown-menu min-w-[10rem] flex flex-col gap-4 dark:bg-zink-600"
		aria-labelledby="flagsDropdown"
	>
		{#each languages as language}
			<a
				href="#!"
				class="flex items-center gap-3 group/items language"
				title={language.label}
				on:click={() => handleLocaleChange(language.value, language.flag)}
			>
				<img src={language.flag} alt="" class="object-cover h-4 rounded-full" />
				<h6
					class="transition-all duration-200 ease-linear font-15medium text- text-slate-600 dark:text-zink-200 group-hover/items:text-custom-500"
				>
					{language.label}
				</h6>
			</a>
		{/each}
	</DropdownMenu>
</Dropdown>
