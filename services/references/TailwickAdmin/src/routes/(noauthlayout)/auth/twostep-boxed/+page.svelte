<script>
    import HeadTitle from "../../../../common/components/HeadTitle.svelte"
    import Dropdown from "../../../../common/components/Dropdown.svelte"
    import DropdownToggle from "../../../../common/components/DropdownToggle.svelte"

    import { classList } from 'svelte-body';
	import { browser } from '$app/environment';

	if (browser) {
		document.body.setAttribute("class","");
	}
</script>
<svelte:body use:classList={'flex items-center justify-center min-h-screen px-4 py-16 bg-cover bg-auth-pattern dark:bg-auth-pattern-dark dark:text-zink-100 font-public'} />

<HeadTitle title="Sign In" />

<div class="mb-0 border-none shadow-none xl:w-2/3 card bg-white/70 dark:bg-zink-500/70">
    <div class="grid grid-cols-1 gap-0 lg:grid-cols-12">
        <div class="lg:col-span-5">
            <div class="!px-12 !py-12 card-body h-full flex flex-col">

                <div class="my-auto">
                    <div class="text-center">
                        <h4 class="mb-2 text-custom-500 dark:text-custom-500">Verify Email</h4>
                        <p class="mb-8 text-slate-500 dark:text-zink-200">Please enter the <b class="font-medium">4</b> digit code sent to <b class="font-medium">tailwick@themesdesign.com</b></p>
                    </div>
                    
                    <form autocomplete="off" action="/">
                        <div class="grid grid-cols-1 gap-2 sm:grid-cols-2 lg:grid-cols-4">
                            <div>
                                <label for="digit1-input" class="hidden">Digit 1</label>
                                <input type="text" class="text-lg text-center form-input dark:bg-zink-600/50 border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" required placeholder="0" onkeyup="moveToNext(1, event)" maxlength="1" id="digit1-input">
                            </div>
                            <div>
                                <label for="digit2-input" class="hidden">Digit 2</label>
                                <input type="text" class="text-lg text-center form-input dark:bg-zink-600/50 border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" required placeholder="0" onkeyup="moveToNext(2, event)" maxlength="1" id="digit2-input">
                            </div>
                    
                            <div>
                                <label for="digit3-input" class="hidden">Digit 3</label>
                                <input type="text" class="text-lg text-center form-input dark:bg-zink-600/50 border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" required placeholder="0" onkeyup="moveToNext(3, event)" maxlength="1" id="digit3-input">
                            </div>
                    
                            <div>
                                <label for="digit4-input" class="hidden">Digit 4</label>
                                <input type="text" class="text-lg text-center form-input dark:bg-zink-600/50 border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" required placeholder="0" onkeyup="moveToNext(4, event)" maxlength="1" id="digit4-input">
                            </div>
                        </div>
                        <div class="mt-10">
                            <button type="submit" class="w-full text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20">Confirm</button>
                        </div>
                    </form>
                </div>
            </div>
        </div>
        <div class="mx-2 mt-2 mb-2 border-none shadow-none lg:col-span-7 card bg-white/60 dark:bg-zink-500/60">
            <div class="!px-10 !pt-10 h-full !pb-0 card-body flex flex-col">
                <div class="flex items-center justify-between gap-3">
                    <div class="grow">
                        <a href="/">
                            <img src="/assets/images/logo-light.png" alt="" class="hidden h-6 dark:block">
                            <img src="/assets/images/logo-dark.png" alt="" class="block h-6 dark:hidden">
                        </a>
                    </div>
                    <div class="shrink-0">
                        <!-- <div class="relative dropdown text-end">
                            <button type="button" class="inline-flex items-center gap-3 transition-all duration-200 ease-linear dropdown-toggle btn border-slate-200 dark:border-zink-400/60 group/items focus:border-custom-500 dark:focus:border-custom-500" id="dropdownMenuButton" data-bs-toggle="dropdown">
                                <img src="/assets/images/flags/us.svg" alt="" class="object-cover h-5 rounded-full">
                                <h6 class="text-base font-medium transition-all duration-200 ease-linear text-slate-600 group-hover/items:text-custom-500 dark:text-zink-200 dark:group-hover/items:text-custom-500">English</h6>
                            </button>
        
                            <div class="absolute z-50 hidden p-3 mt-1 text-left list-none bg-white rounded-md shadow-md dropdown-menu min-w-[9rem] flex flex-col gap-3 dark:bg-zink-600" aria-labelledby="dropdownMenuButton">
                                <a href="#!" class="flex items-center gap-3 group/items">
                                    <img src="/assets/images/flags/us.svg" alt="" class="object-cover h-4 rounded-full">
                                    <h6 class="text-sm font-medium transition-all duration-200 ease-linear text-slate-600 group-hover/items:text-custom-500 dark:text-zink-200 dark:group-hover/items:text-custom-500">English</h6>
                                </a>
                                <a href="#!" class="flex items-center gap-3 group/items">
                                    <img src="/assets/images/flags/es.svg" alt="" class="object-cover h-4 rounded-full">
                                    <h6 class="text-sm font-medium transition-all duration-200 ease-linear text-slate-600 group-hover/items:text-custom-500 dark:text-zink-200 dark:group-hover/items:text-custom-500">Spanish</h6>
                                </a>
                                <a href="#!" class="flex items-center gap-3 group/items">
                                    <img src="/assets/images/flags/de.svg" alt="" class="object-cover h-4 rounded-full">
                                    <h6 class="text-sm font-medium transition-all duration-200 ease-linear text-slate-600 group-hover/items:text-custom-500 dark:text-zink-200 dark:group-hover/items:text-custom-500">German</h6>
                                </a>
                                <a href="#!" class="flex items-center gap-3 group/items">
                                    <img src="/assets/images/flags/fr.svg" alt="" class="object-cover h-4 rounded-full">
                                    <h6 class="text-sm font-medium transition-all duration-200 ease-linear text-slate-600 group-hover/items:text-custom-500 dark:text-zink-200 dark:group-hover/items:text-custom-500">French</h6>
                                </a>
                                <a href="#!" class="flex items-center gap-3 group/items">
                                    <img src="/assets/images/flags/jp.svg" alt="" class="object-cover h-4 rounded-full">
                                    <h6 class="text-sm font-medium transition-all duration-200 ease-linear text-slate-600 group-hover/items:text-custom-500 dark:text-zink-200 dark:group-hover/items:text-custom-500">Japanese</h6>
                                </a>
                                <a href="#!" class="flex items-center gap-3 group/items">
                                    <img src="/assets/images/flags/it.svg" alt="" class="object-cover h-4 rounded-full">
                                    <h6 class="text-sm font-medium transition-all duration-200 ease-linear text-slate-600 group-hover/items:text-custom-500 dark:text-zink-200 dark:group-hover/items:text-custom-500">Italian</h6>
                                </a>
                                <a href="#!" class="flex items-center gap-3 group/items">
                                    <img src="/assets/images/flags/ru.svg" alt="" class="object-cover h-4 rounded-full">
                                    <h6 class="text-sm font-medium transition-all duration-200 ease-linear text-slate-600 group-hover/items:text-custom-500 dark:text-zink-200 dark:group-hover/items:text-custom-500">Russian</h6>
                                </a>
                                <a href="#!" class="flex items-center gap-3 group/items">
                                    <img src="/assets/images/flags/ae.svg" alt="" class="object-cover h-4 rounded-full">
                                    <h6 class="text-sm font-medium transition-all duration-200 ease-linear text-slate-600 group-hover/items:text-custom-500 dark:text-zink-200 dark:group-hover/items:text-custom-500">Arabic</h6>
                                </a>
                            </div>
                        </div> -->
                        <Dropdown className="relative text-end">
                            <DropdownToggle
                                className="inline-flex items-center gap-3 transition-all duration-200 ease-linear dropdown-toggle btn border-slate-200 dark:border-zink-400/60 group/items focus:border-custom-500 dark:focus:border-custom-500">
                                <img src="/assets/images/flags/us.svg" alt="" class="object-cover h-5 rounded-full">
                                <h6 class="text-base font-medium transition-all duration-200 ease-linear text-slate-600 group-hover/items:text-custom-500 dark:text-zink-200 dark:group-hover/items:text-custom-500">English</h6>
                            </DropdownToggle>
                        </Dropdown>
                    </div>
                </div>
                <div class="mt-auto">
                    <img src="/assets/images/auth/img-01.png" alt="" class="md:max-w-[32rem] mx-auto">
                </div>
            </div>
        </div>
    </div>
</div>