<script>
	import { browser } from '$app/environment';
	import LucideIcon from './LucideIcon.svelte';

	function setLayoutMode(mode, modeType, modeTypeId, html) {
		var isModeTypeId = document.getElementById(modeTypeId);
		html.setAttribute(mode, modeType);
		if (isModeTypeId) {
			document.getElementById(modeTypeId).click();
		}
	}

	function onChangeLayoutMode() {
		if (browser) {
			var html = document.getElementsByTagName('HTML')[0];
			html.hasAttribute('data-mode') && html.getAttribute('data-mode') == 'dark'
				? setLayoutMode('data-mode', 'light', 'dataModeLight', html)
				: setLayoutMode('data-mode', 'dark', 'dataModeDark', html);
		}
	}
</script>

<div class="relative flex items-center h-header">
	<button
		type="button"
		on:click={onChangeLayoutMode}
		class="inline-flex relative justify-center items-center p-0 text-topbar-item transition-all size-[37.5px] duration-200 ease-linear bg-topbar rounded-md btn hover:bg-topbar-item-bg-hover hover:text-topbar-item-hover group-data-[topbar=dark]:bg-topbar-dark group-data-[topbar=dark]:hover:bg-topbar-item-bg-hover-dark group-data-[topbar=dark]:hover:text-topbar-item-hover-dark group-data-[topbar=brand]:bg-topbar-brand group-data-[topbar=brand]:hover:bg-topbar-item-bg-hover-brand group-data-[topbar=brand]:hover:text-topbar-item-hover-brand group-data-[topbar=dark]:dark:bg-zink-700 group-data-[topbar=dark]:dark:hover:bg-zink-600 group-data-[topbar=brand]:text-topbar-item-brand group-data-[topbar=dark]:dark:hover:text-zink-50 group-data-[topbar=dark]:dark:text-zink-200 group-data-[topbar=dark]:text-topbar-item-dark"
		id="light-dark-mode"
	>
		<LucideIcon
			name="Sun"
			class="inline-block size-5 stroke-1 fill-slate-100 group-data-[topbar=dark]:fill-topbar-item-bg-hover-dark group-data-[topbar=brand]:fill-topbar-item-bg-hover-brand"
		/>
	</button>
</div>
