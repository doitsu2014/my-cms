<script>
    import {browser} from "$app/environment"
    import LucideIcon from "../common/components/LucideIcon.svelte"
    import Drawer from "../common/components/Drawer.svelte"
	import { createEventDispatcher, onDestroy, onMount } from "svelte";
	import { OverlayScrollbars } from 'overlayscrollbars';

    export let layout;
    export let isRightsidebarOpen;
    export let toggle;
    let skinLayout = "default";
    let layoutMode = "light";
    let topbarColor = "light";
    let sidebarColor = "light";
    let contentWidth = "fluid";
    let sidebarSize = "lg";
    let navigationType = "sticky";
    let dir = "ltr";

    const dispatch = createEventDispatcher();


    function changeSemiDark(checked){
        if(checked){
            changeTopbarColor("dark");
            if(layout == "vertical"){
                changeSidebarColor("dark");
            }
        }else{
            changeTopbarColor("light");
            if(layout == "vertical"){
                changeSidebarColor("light");
            }
        }
    }

    function changeSkinLayout(value){
        if(document.documentElement){
            document.documentElement.setAttribute("data-skin",value);
        }
    }

    function changeLayoutMode(value){
        if(document.documentElement){
            document.documentElement.setAttribute("data-mode",value);
        }
        layoutMode = value;
        changeTopbarColor(value);
        changeSidebarColor(value);
    }

    function changeTopbarColor(value){
        if(document.documentElement){
            document.documentElement.setAttribute("data-topbar",value);
        }
        topbarColor = value;
    }

    function changeSidebarColor(value){
        if(layout == "vertical"){
            if(document.documentElement){
                document.documentElement.setAttribute("data-sidebar",value);
                document.getElementById("sidebar-color").style.display = "block";
            }
            sidebarColor = value;
        }else{
            if(document.documentElement){
                document.documentElement.removeAttribute("data-sidebar");
                document.getElementById("sidebar-color").style.display = "none";
            }
        }
    }

    function changeContentWidth(value){
        if(document.documentElement){
            document.documentElement.setAttribute("data-content",value);
        }
        contentWidth = value;
    }

    function changeSidebarSize(value){
        if(layout == "vertical"){
            if(document.documentElement){
                document.documentElement.setAttribute("data-sidebar-size",value);
                document.getElementById("sidebar-size").style.display = "block";
            }
            sidebarSize = value;

            if(sidebarSize == "sm"){
                    const Instance = OverlayScrollbars(document.querySelector('#scrollbar'));
                    if (Instance) {
                        Instance.destroy();
                    }
            }else{
                const options = {
                scrollbars: {
                    visibility: 'auto',
                    autoHide: 'move',
                    autoHideDelay: 100,
                    dragScroll: true,
                    clickScroll: false,
                    theme: 'os-theme-dark',
                    pointers: ['mouse', 'touch', 'pen']
                }
            };
            const menuElement = document.querySelector('#scrollbar');
                if (menuElement) {
                    OverlayScrollbars(menuElement, options);
                }
            }

        }else{
            if(document.documentElement){
                document.documentElement.removeAttribute("data-sidebar-size");
                document.getElementById("sidebar-size").style.display = "none";
            }
        }
    }

    function changeNavigationType(value){
        if(document.documentElement){
            document.documentElement.setAttribute("data-navbar",value);
        }
        navigationType = value;
    }

    function changeDir(value){
        if(document.documentElement){
            document.documentElement.setAttribute("dir",value);
        }
        dir = value;
    }

        function windowResize() {
			let windowSize = document.documentElement.clientWidth;
			if (windowSize < 1025 && windowSize >= 768) {
                document.querySelector(".app-menu").classList.add("hidden");
				document.body.classList.remove("overflow-hidden");
                if(layout == "vertical") {
				    document.documentElement.setAttribute("data-sidebar-size", "sm");
                }
			}
			else if (windowSize < 768) {
				document.documentElement.setAttribute("data-sidebar-size", "lg");
			}
			else if (windowSize >= 1025) {
				document.body.classList.remove("overflow-hidden");
				document.querySelector(".app-menu").classList.add("hidden");
                if(layout == "vertical") {
				    document.documentElement.setAttribute("data-sidebar-size", "lg");
                }
			}
		}

    onMount(() =>{
        document.documentElement.setAttribute("data-layout",layout);
        changeSidebarColor(sidebarColor);
        changeSidebarSize(sidebarSize);

        changeNavigationType(navigationType);
        changeContentWidth(contentWidth);
        changeLayoutMode(layoutMode)
        changeSkinLayout(skinLayout);
        changeTopbarColor(topbarColor);
        changeDir(dir);

		setTimeout(() => {
			windowResize();
		}, 500);
		window.addEventListener("resize", windowResize);
    })

    onDestroy(() => {
        if(browser){
            window.removeEventListener("resize", windowResize);
        }
	})
</script>

<div class="fixed items-center hidden bottom-6 right-12 h-header group-data-[navbar=hidden]:flex">
    <button on:click={toggle} type="button" class="inline-flex items-center justify-center size-12 p-0 transition-all duration-200 ease-linear rounded-md shadow-lg text-sky-50 bg-sky-500">
        <LucideIcon name="Settings" class="inline-block size-5"/>
    </button>
</div>

<Drawer isOpen={isRightsidebarOpen} {toggle} id="customizerButton" className="fixed inset-y-0 flex flex-col w-full transition-transform duration-300 ease-in-out transform bg-white shadow ltr:right-0 rtl:left-0 md:w-96 z-drawer dark:bg-zink-600">
    <div class="flex justify-between p-4 border-b border-slate-200 dark:border-zink-500">
        <div class="grow">
            <h5 class="mb-1 text-16">Tailwick Theme Customizer</h5>
            <p class="font-normal text-slate-500 dark:text-zink-200">Choose your themes & layouts etc.</p>
        </div>
        <div class="shrink-0">
            <button on:click={toggle} class="transition-all duration-150 ease-linear text-slate-500 hover:text-slate-800 dark:text-zink-200 dark:hover:text-zink-50">
                <LucideIcon name="X" class="size-4"/>
            </button>
        </div>
    </div>
    <div class="h-full p-6 overflow-y-auto">
        <div>
            <h5 class="mb-3 underline capitalize text-15">Choose Layouts</h5>

            <div id="semi-dark">
                <div class="flex items-center">
                    <div class="relative inline-block w-10 mr-2 align-middle transition duration-200 ease-in">
                        <input type="checkbox" name="customDefaultSwitch" on:change={e => {changeSemiDark(e.target.checked)}} value="dark" id="customDefaultSwitch" class="absolute block size-5 transition duration-300 ease-linear border-2 rounded-full appearance-none cursor-pointer border-slate-200 bg-white/80 peer/published checked:bg-white checked:right-0 checked:border-custom-500 arrow-none dark:border-zink-500 dark:bg-zink-500 dark:checked:bg-zink-400 checked:bg-none">
                        <label for="customDefaultSwitch" class="block h-5 overflow-hidden transition duration-300 ease-linear border rounded-full cursor-pointer border-slate-200 bg-slate-200 peer-checked/published:bg-custom-500 peer-checked/published:border-custom-500 dark:border-zink-500 dark:bg-zink-600"></label>
                    </div>
                    <label for="customDefaultSwitch" class="inline-block text-base font-medium">Semi Dark (Sidebar & Header)</label>
                </div>
            </div>
        </div>

        <div class="mt-6">
            <!-- data-skin="" -->
            <h5 class="mb-3 underline capitalize text-15">Skin Layouts</h5>
            <div class="grid grid-cols-1 mb-5 gap-7 sm:grid-cols-2">
                <div class="relative">
                    <input on:change={() => changeSkinLayout("default")} id="layoutSkitOne" name="dataLayoutSkin" class="absolute size-4 border rounded-full appearance-none cursor-pointer ltr:right-2 rtl:left-2 top-2 vertical-menu-btn bg-slate-100 border-slate-300 checked:bg-custom-500 checked:border-custom-500 dark:bg-zink-400 dark:border-zink-500" type="radio" value="default" checked={skinLayout == "default"}>
                    <label class="block w-full h-24 p-0 overflow-hidden border rounded-lg cursor-pointer border-slate-200 dark:border-zink-500 bg-slate-50 dark:bg-zink-600" for="layoutSkitOne">
                        <span class="flex h-full gap-0">
                            <span class="shrink-0">
                                <span class="flex flex-col h-full gap-1 p-1 ltr:border-r rtl:border-l border-slate-200 dark:border-zink-500">
                                    <span class="block p-1 px-2 mb-2 rounded bg-slate-100 dark:bg-zink-400"></span>
                                    <span class="block p-1 px-2 pb-0 bg-slate-100 dark:bg-zink-500"></span>
                                    <span class="block p-1 px-2 pb-0 bg-slate-100 dark:bg-zink-500"></span>
                                    <span class="block p-1 px-2 pb-0 bg-slate-100 dark:bg-zink-500"></span>
                                </span>
                            </span>
                            <span class="grow">
                                <span class="flex flex-col h-full">
                                    <span class="block h-3 bg-slate-100 dark:bg-zink-500"></span>
                                    <span class="block h-3 mt-auto bg-slate-100 dark:bg-zink-500"></span>
                                </span>
                            </span>
                        </span>
                    </label>
                    <h5 class="mt-2 text-center text-15">Default</h5>
                </div>
        
                <div class="relative">
                    <input id="layoutSkitTwo" on:change={() => changeSkinLayout("bordered")} name="dataLayoutSkin" class="absolute size-4 border rounded-full appearance-none cursor-pointer ltr:right-2 rtl:left-2 top-2 vertical-menu-btn bg-slate-100 border-slate-300 checked:bg-custom-500 checked:border-custom-500 dark:bg-zink-400 dark:border-zink-500" type="radio" value="bordered"
                    checked={skinLayout == "bordered"}>
                    <label class="block w-full h-24 p-0 overflow-hidden border rounded-lg cursor-pointer border-slate-200 dark:border-zink-500" for="layoutSkitTwo">
                        <span class="flex h-full gap-0">
                            <span class="shrink-0">
                                <span class="flex flex-col h-full gap-1 p-1 ltr:border-r rtl:border-l border-slate-200 dark:border-zink-500">
                                    <span class="block p-1 px-2 mb-2 rounded bg-slate-100 dark:bg-zink-400"></span>
                                    <span class="block p-1 px-2 pb-0 bg-slate-100 dark:bg-zink-500"></span>
                                    <span class="block p-1 px-2 pb-0 bg-slate-100 dark:bg-zink-500"></span>
                                    <span class="block p-1 px-2 pb-0 bg-slate-100 dark:bg-zink-500"></span>
                                </span>
                            </span>
                            <span class="grow">
                                <span class="flex flex-col h-full">
                                    <span class="block h-3 border-b border-slate-200 dark:border-zink-500"></span>
                                    <span class="block h-3 mt-auto border-t border-slate-200 dark:border-zink-500"></span>
                                </span>
                            </span>
                        </span>
                    </label>
                    <h5 class="mt-2 text-center text-15">Bordered</h5>
                </div>
            </div>
        </div>
        
        <div class="mt-6">
            <!-- data-mode="" -->
            <h5 class="mb-3 underline capitalize text-15">Light & Dark</h5>
            <div class="flex gap-3">
                <button type="button" id="dataModeLight" name="dataMode" value="light" class="transition-all duration-200 ease-linear bg-white border-dashed text-slate-500 btn border-slate-200 hover:text-slate-500 hover:bg-slate-50 hover:border-slate-200 [&.active]:text-custom-500 [&.active]:bg-custom-50 [&.active]:border-custom-200 dark:bg-zink-600 dark:text-zink-200 dark:border-zink-400 dark:hover:bg-zink-600 dark:hover:text-zink-100 dark:hover:border-zink-400 dark:[&.active]:bg-custom-500/10 dark:[&.active]:border-custom-500/30 dark:[&.active]:text-custom-500 {layoutMode == "light"? "active":""}" on:click={() => changeLayoutMode("light")}>Light Mode</button>
                <button type="button" id="dataModeDark" name="dataMode" value="drk" class="transition-all duration-200 ease-linear bg-white border-dashed text-slate-500 btn border-slate-200 hover:text-slate-500 hover:bg-slate-50 hover:border-slate-200 [&.active]:text-custom-500 [&.active]:bg-custom-50 [&.active]:border-custom-200 dark:bg-zink-600 dark:text-zink-200 dark:border-zink-400 dark:hover:bg-zink-600 dark:hover:text-zink-100 dark:hover:border-zink-400 dark:[&.active]:bg-custom-500/10 dark:[&.active]:border-custom-500/30 dark:[&.active]:text-custom-500 {layoutMode == "dark"? "active":""}" on:click={() => changeLayoutMode("dark")}>Dark Mode</button>
            </div>
        </div>

        <div class="mt-6">
            <!-- dir="ltr" -->
            <h5 class="mb-3 underline capitalize text-15">LTR & RTL</h5>
            <div class="flex flex-wrap gap-3">
                <button type="button"  id="diractionOne" name="dir" value="ltr" class="transition-all duration-200 ease-linear bg-white border-dashed text-slate-500 btn border-slate-200 hover:text-slate-500 hover:bg-slate-50 hover:border-slate-200 [&.active]:text-custom-500 [&.active]:bg-custom-50 [&.active]:border-custom-200 dark:bg-zink-600 dark:text-zink-200 dark:border-zink-400 dark:hover:bg-zink-600 dark:hover:text-zink-100 dark:hover:border-zink-400 dark:[&.active]:bg-custom-500/10 dark:[&.active]:border-custom-500/30 dark:[&.active]:text-custom-500 {dir == "ltr" ? "active":""}" on:click={() => changeDir("ltr")}>LTR Mode</button>
                <button type="button"  id="diractionTwo" name="dir" value="rtl" class="transition-all duration-200 ease-linear bg-white border-dashed text-slate-500 btn border-slate-200 hover:text-slate-500 hover:bg-slate-50 hover:border-slate-200 [&.active]:text-custom-500 [&.active]:bg-custom-50 [&.active]:border-custom-200 dark:bg-zink-600 dark:text-zink-200 dark:border-zink-400 dark:hover:bg-zink-600 dark:hover:text-zink-100 dark:hover:border-zink-400 dark:[&.active]:bg-custom-500/10 dark:[&.active]:border-custom-500/30 dark:[&.active]:text-custom-500 {dir == "rtl" ? "active":""}" on:click={() => changeDir("rtl")}>RTL Mode</button>
            </div>
        </div>

        <div class="mt-6">
            <!-- data-content -->
            <h5 class="mb-3 underline capitalize text-15">Content Width</h5>
            <div class="flex gap-3">
                <button type="button" id="datawidthOne" name="datawidth" value="fluid" class="transition-all duration-200 ease-linear bg-white border-dashed text-slate-500 btn border-slate-200 hover:text-slate-500 hover:bg-slate-50 hover:border-slate-200 [&.active]:text-custom-500 [&.active]:bg-custom-50 [&.active]:border-custom-200 dark:bg-zink-600 dark:text-zink-200 dark:border-zink-400 dark:hover:bg-zink-600 dark:hover:text-zink-100 dark:hover:border-zink-400 dark:[&.active]:bg-custom-500/10 dark:[&.active]:border-custom-500/30 dark:[&.active]:text-custom-500 {contentWidth == "fluid"? "active":""}" on:click={() => changeContentWidth("fluid")}>Fluid</button>
                <button type="button" id="datawidthTwo" name="datawidth" value="boxed" class="transition-all duration-200 ease-linear bg-white border-dashed text-slate-500 btn border-slate-200 hover:text-slate-500 hover:bg-slate-50 hover:border-slate-200 [&.active]:text-custom-500 [&.active]:bg-custom-50 [&.active]:border-custom-200 dark:bg-zink-600 dark:text-zink-200 dark:border-zink-400 dark:hover:bg-zink-600 dark:hover:text-zink-100 dark:hover:border-zink-400 dark:[&.active]:bg-custom-500/10 dark:[&.active]:border-custom-500/30 dark:[&.active]:text-custom-500 {contentWidth == "boxed"? "active":""}" on:click={() => changeContentWidth("boxed")}>Boxed</button>
            </div>
        </div>

        <div class="mt-6" id="sidebar-size">
            <!-- data-sidebar-size="" -->
            <h5 class="mb-3 underline capitalize text-15">Sidebar Size</h5>
            <div class="flex flex-wrap gap-3">
                <button type="button" id="sidebarSizelg" name="sidebarSize" value="lg" class="transition-all duration-200 ease-linear bg-white border-dashed text-slate-500 btn border-slate-200 hover:text-slate-500 hover:bg-slate-50 hover:border-slate-200 [&.active]:text-custom-500 [&.active]:bg-custom-50 [&.active]:border-custom-200 dark:bg-zink-600 dark:text-zink-200 dark:border-zink-400 dark:hover:bg-zink-600 dark:hover:text-zink-100 dark:hover:border-zink-400 dark:[&.active]:bg-custom-500/10 dark:[&.active]:border-custom-500/30 dark:[&.active]:text-custom-500 {sidebarSize == "lg"? "active":""}" on:click={() => changeSidebarSize("lg")}>Default</button>
                <button type="button" id="sidebarSizemd" name="sidebarSize" value="md" class="transition-all duration-200 ease-linear bg-white border-dashed text-slate-500 btn border-slate-200 hover:text-slate-500 hover:bg-slate-50 hover:border-slate-200 [&.active]:text-custom-500 [&.active]:bg-custom-50 [&.active]:border-custom-200 dark:bg-zink-600 dark:text-zink-200 dark:border-zink-400 dark:hover:bg-zink-600 dark:hover:text-zink-100 dark:hover:border-zink-400 dark:[&.active]:bg-custom-500/10 dark:[&.active]:border-custom-500/30 dark:[&.active]:text-custom-500 {sidebarSize == "md"? "active":""}" on:click={() => changeSidebarSize("md")}>Compact</button>
                <button type="button" id="sidebarSizesm" name="sidebarSize" value="sm" class="transition-all duration-200 ease-linear bg-white border-dashed text-slate-500 btn border-slate-200 hover:text-slate-500 hover:bg-slate-50 hover:border-slate-200 [&.active]:text-custom-500 [&.active]:bg-custom-50 [&.active]:border-custom-200 dark:bg-zink-600 dark:text-zink-200 dark:border-zink-400 dark:hover:bg-zink-600 dark:hover:text-zink-100 dark:hover:border-zink-400 dark:[&.active]:bg-custom-500/10 dark:[&.active]:border-custom-500/30 dark:[&.active]:text-custom-500 {sidebarSize == "sm"? "active":""}" on:click={() => changeSidebarSize("sm")}>Small (Icon)</button>
            </div>
        </div>

        <div class="mt-6">
            <!-- data-navbar="" -->
            <h5 class="mb-3 underline capitalize text-15">Navigation Type</h5>
            <div class="flex flex-wrap gap-3">
                <button type="button" id="navbarTwo" name="navbar" value="sticky" class="transition-all duration-200 ease-linear bg-white border-dashed text-slate-500 btn border-slate-200 hover:text-slate-500 hover:bg-slate-50 hover:border-slate-200 [&.active]:text-custom-500 [&.active]:bg-custom-50 [&.active]:border-custom-200 dark:bg-zink-600 dark:text-zink-200 dark:border-zink-400 dark:hover:bg-zink-600 dark:hover:text-zink-100 dark:hover:border-zink-400 dark:[&.active]:bg-custom-500/10 dark:[&.active]:border-custom-500/30 dark:[&.active]:text-custom-500 {navigationType == "sticky"? "active":""}" on:click={() => changeNavigationType("sticky")}>Sticky</button>
                <button type="button" id="navbarOne" name="navbar" value="scroll" class="transition-all duration-200 ease-linear bg-white border-dashed text-slate-500 btn border-slate-200 hover:text-slate-500 hover:bg-slate-50 hover:border-slate-200 [&.active]:text-custom-500 [&.active]:bg-custom-50 [&.active]:border-custom-200 dark:bg-zink-600 dark:text-zink-200 dark:border-zink-400 dark:hover:bg-zink-600 dark:hover:text-zink-100 dark:hover:border-zink-400 dark:[&.active]:bg-custom-500/10 dark:[&.active]:border-custom-500/30 dark:[&.active]:text-custom-500 {navigationType == "scroll"? "active":""}" on:click={() => changeNavigationType("scroll")}>Scroll</button>
                <button type="button" id="navbarThree" name="navbar" value="bordered" class="transition-all duration-200 ease-linear bg-white border-dashed text-slate-500 btn border-slate-200 hover:text-slate-500 hover:bg-slate-50 hover:border-slate-200 [&.active]:text-custom-500 [&.active]:bg-custom-50 [&.active]:border-custom-200 dark:bg-zink-600 dark:text-zink-200 dark:border-zink-400 dark:hover:bg-zink-600 dark:hover:text-zink-100 dark:hover:border-zink-400 dark:[&.active]:bg-custom-500/10 dark:[&.active]:border-custom-500/30 dark:[&.active]:text-custom-500 {navigationType == "bordered"? "active":""}" on:click={() => changeNavigationType("bordered")}>Bordered</button>
                <button type="button" id="navbarFour" name="navbar" value="hidden" class="transition-all duration-200 ease-linear bg-white border-dashed text-slate-500 btn border-slate-200 hover:text-slate-500 hover:bg-slate-50 hover:border-slate-200 [&.active]:text-custom-500 [&.active]:bg-custom-50 [&.active]:border-custom-200 dark:bg-zink-600 dark:text-zink-200 dark:border-zink-400 dark:hover:bg-zink-600 dark:hover:text-zink-100 dark:hover:border-zink-400 dark:[&.active]:bg-custom-500/10 dark:[&.active]:border-custom-500/30 dark:[&.active]:text-custom-500 {navigationType == "hidden"? "active":""}" on:click={() => changeNavigationType("hidden")}>Hidden</button>
            </div>
        </div>

        <div class="mt-6" id="sidebar-color">
            <!-- data-sidebar="" light, dark, brand, modern-->
            <h5 class="mb-3 underline capitalize text-15">Sizebar Colors</h5>
            <div class="flex flex-wrap gap-3">
                <button type="button" id="sidebarColorOne" name="sidebarColor" value="light" class="flex items-center justify-center size-10 bg-white border rounded-md border-slate-200 group {sidebarColor == "light"? "active":""}" on:click={() => changeSidebarColor("light")}>
                    <LucideIcon name="CheckIcon" class="size-5 hidden group-[.active]:inline-block text-slate-600"/></button>
                <button type="button" id="sidebarColorTwo" name="sidebarColor" value="dark" class="flex items-center justify-center size-10 border rounded-md border-zink-900 bg-zink-900 group {sidebarColor == "dark"? "active":""}" on:click={() => changeSidebarColor("dark")}
                ><LucideIcon name="CheckIcon" class="size-5 hidden group-[.active]:inline-block text-white"/></button>
                <button type="button" id="sidebarColorThree" name="sidebarColor" value="brand" class="flex items-center justify-center size-10 border rounded-md border-custom-800 bg-custom-800 group {sidebarColor == "brand"? "active":""}" on:click={() => changeSidebarColor("brand")}>
                    <LucideIcon name="CheckIcon" class="size-5 hidden group-[.active]:inline-block text-white"/></button>
                <button type="button" id="sidebarColorFour" name="sidebarColor" value="modern" class="flex items-center justify-center size-10 border rounded-md border-purple-950 bg-gradient-to-t from-red-400 to-purple-500 group {sidebarColor == "modern"? "active":""}" on:click={() => changeSidebarColor("modern")}>
                    <LucideIcon name="CheckIcon" class="size-5 hidden group-[.active]:inline-block text-white"/></button>
            </div>
        </div>
        
        <div class="mt-6">
            <!-- data-topbar="" light, dark, brand, modern-->
            <h5 class="mb-3 underline capitalize text-15">Topbar Colors</h5>
            <div class="flex flex-wrap gap-3">
                <button type="button" id="topbarColorOne" name="topbarColor" value="light" class="flex items-center justify-center size-10 bg-white border rounded-md border-slate-200 group {topbarColor == "light"? "active":""}" on:click={() => changeTopbarColor("light")}><LucideIcon name="CheckIcon" class="size-5 hidden group-[.active]:inline-block text-slate-600"/></button>
                <button type="button" id="topbarColorTwo" name="topbarColor" value="dark" class="flex items-center justify-center size-10 border rounded-md border-zink-900 bg-zink-900 group {topbarColor == "dark"? "active":""}" on:click={() => changeTopbarColor("dark")}><LucideIcon name="CheckIcon" class="size-5 hidden group-[.active]:inline-block text-white"/></button>
                <button type="button" id="topbarColorThree" name="topbarColor" value="brand" class="flex items-center justify-center size-10 border rounded-md border-custom-800 bg-custom-800 group {topbarColor == "brand"? "active":""}" on:click={() => changeTopbarColor("brand")}><LucideIcon name="CheckIcon" class="size-5 hidden group-[.active]:inline-block text-white"/></button>
            </div>
        </div>
        
    </div>
    <div class="flex items-center justify-between gap-3 p-4 border-t border-slate-200 dark:border-zink-500">
        <a href="#!" class="w-full text-white transition-all duration-200 ease-linear bg-red-500 border-red-500 btn hover:text-white hover:bg-red-600 hover:border-red-600 focus:text-white focus:bg-red-600 focus:border-red-600 focus:ring focus:ring-red-100 active:text-white active:bg-red-600 active:border-red-600 active:ring active:ring-red-100">Buy Now</a>
    </div>
</Drawer>