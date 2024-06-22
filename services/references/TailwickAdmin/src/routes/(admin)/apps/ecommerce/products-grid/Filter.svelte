<script>
	import LucideIcon from "../../../../../common/components/LucideIcon.svelte";

    const toggleCollpase = (e) => {
        if(e.target.parentElement.parentElement.querySelector(".collapsible-content")){
            e.target.parentElement.parentElement.querySelector(".collapsible-content").classList.toggle("hidden")
            e.target.parentElement.classList.toggle("show");
        }
    }   
</script>
<div class="hidden 2xl:col-span-3 2xl:block">
    <div class="card">
        <div class="card-body">
            <div class="flex items-center gap-3">
                <h6 class="text-15 grow">Filter</h6>
                <div class="shrink-0">
                    <a href="#!" class="underline transition-all duration-200 ease-linear hover:text-custom-500">Clear All</a>
                </div>
            </div>

            <div class="relative mt-4">
                <input type="text" class="ltr:pl-8 rtl:pr-8 search form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Search for ..." autocomplete="off">
                <LucideIcon name="Search" class="inline-block size-4 absolute ltr:left-2.5 rtl:right-2.5 top-2.5 text-slate-500 dark:text-zink-200 fill-slate-100 dark:fill-zink-600"/>
            </div>

            <div class="mt-4 collapsible">
                <button class="flex items-center w-full text-left collapsible-header group">
                    <!-- svelte-ignore a11y-click-events-have-key-events -->
                    <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
                    <h6 class="underline grow" on:click={e => toggleCollpase(e)}>Color</h6>
                    <div class="shrink-0 text-slate-500 dark:text-zink-200">
                        <LucideIcon name="ChevronDown" class="hidden size-4 group-[.show]:inline-block"/>
                        <LucideIcon name="ChevronUp" class="inline-block size-4 group-[.show]:hidden"/>
                    </div>
                </button>
                <div class="mt-4 collapsible-content">
                    <div class="flex flex-wrap items-center gap-2">
                        <input id="color1" class="size-5 border rounded-sm appearance-none cursor-pointer border-custom-500 bg-custom-500 checked:bg-custom-500 checked:border-custom-500" type="checkbox" value="">
                        <input id="color2" class="size-5 bg-red-300 border border-red-300 rounded-sm appearance-none cursor-pointer checked:bg-red-300 checked:border-red-300" type="checkbox" value="">
                        <input id="color3" class="size-5 bg-green-300 border border-green-300 rounded-sm appearance-none cursor-pointer checked:bg-green-300 checked:border-green-300" type="checkbox" value="">
                        <input id="color4" class="size-5 border rounded-sm appearance-none cursor-pointer border-slate-500 bg-slate-500 checked:bg-slate-500 checked:border-slate-500" type="checkbox" value="">
                        <input id="color5" class="size-5 bg-purple-500 border border-purple-500 rounded-sm appearance-none cursor-pointer checked:bg-purple-500 checked:border-purple-500" type="checkbox" value="">
                        <input id="color6" class="size-5 border rounded-sm appearance-none cursor-pointer bg-sky-500 border-sky-500 checked:bg-sky-500 checked:border-sky-500" type="checkbox" value="">
                        <input id="color7" class="size-5 bg-yellow-500 border border-yellow-500 rounded-sm appearance-none cursor-pointer checked:bg-yellow-500 checked:border-yellow-500" type="checkbox" value="">
                        <input id="color7" class="size-5 bg-green-500 border border-green-500 rounded-sm appearance-none cursor-pointer checked:bg-green-500 checked:border-green-500" type="checkbox" value="">
                        <input id="color8" class="size-5 border rounded-sm appearance-none cursor-pointer bg-slate-800 border-slate-800 checked:bg-slate-800 checked:border-slate-800" type="checkbox" value="">
                        <input id="color9" class="size-5 border rounded-sm appearance-none cursor-pointer bg-slate-200 border-slate-200 checked:bg-slate-200 checked:border-slate-200" type="checkbox" value="">
                        <input id="color10" class="size-5 border rounded-sm appearance-none cursor-pointer bg-emerald-300 border-embg-emerald-300 checked:bg-emerald-300 checked:border-embg-emerald-300" type="checkbox" value="">
                    </div>
                </div>
            </div>
            <div class="mt-4 collapsible">
                <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
                <button class="flex items-center w-full text-left collapsible-header group">
                    <!-- svelte-ignore a11y-click-events-have-key-events -->
                    <h6 class="underline grow"  on:click={e => toggleCollpase(e)}>Price</h6>
                    <div class="shrink-0 text-slate-500 dark:text-zink-200">
                        <LucideIcon name="ChevronDown" class="hidden size-4 group-[.show]:inline-block"/>
                        <LucideIcon name="ChevronUp" class="inline-block size-4 group-[.show]:hidden"/>
                    </div>
                </button>
                <div class="mt-4 collapsible-content">
                    <div class="flex flex-col gap-2">
                        <div class="flex items-center gap-2">
                            <input id="priceAll" class="size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800" type="checkbox" value="">
                            <label for="priceAll" class="align-middle cursor-pointer">
                                All
                            </label>
                        </div>
                        <div class="flex items-center gap-2">
                            <input id="price1" class="size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800" type="checkbox" value="">
                            <label for="price1" class="align-middle cursor-pointer">
                                $0.00 – $110.00
                            </label>
                        </div>
                        <div class="flex items-center gap-2">
                            <input id="price2" class="size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800" type="checkbox" value="">
                            <label for="price2" class="align-middle cursor-pointer">
                                $110.00 – $220.00
                            </label>
                        </div>
                        <div class="flex items-center gap-2">
                            <input id="price3" class="size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800" type="checkbox" value="">
                            <label for="price3" class="align-middle cursor-pointer">
                                $220 - $330
                            </label>
                        </div>
                        <div class="flex items-center gap-2">
                            <input id="price4" class="size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800" type="checkbox" value="">
                            <label for="price4" class="align-middle cursor-pointer">
                                $330 - $550
                            </label>
                        </div>
                        <div class="flex items-center gap-2">
                            <input id="price5" class="size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800" type="checkbox" value="">
                            <label for="price5" class="align-middle cursor-pointer">
                                $550+
                            </label>
                        </div>
                        <div class="flex items-center gap-2">
                            <input id="price6" class="size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800" type="checkbox" value="">
                            <label for="price6" class="align-middle cursor-pointer">
                                Low to High
                            </label>
                        </div>
                        <div class="flex items-center gap-2">
                            <input id="price6" class="size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800" type="checkbox" value="">
                            <label for="price6" class="align-middle cursor-pointer">
                                High to Low
                            </label>
                        </div>
                    </div>
                </div>
            </div>
            <div class="mt-4 collapsible">
                <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
                <button class="flex items-center w-full text-left collapsible-header group">
                    <!-- svelte-ignore a11y-click-events-have-key-events -->
                    <h6 class="underline grow"  on:click={e => toggleCollpase(e)}>Product Category</h6>
                    <div class="shrink-0 text-slate-500 dark:text-zink-200">
                        <LucideIcon name="ChevronDown" class="hidden size-4 group-[.show]:inline-block"/>
                        <LucideIcon name="ChevronUp" class="inline-block size-4 group-[.show]:hidden"/>
                    </div>
                </button>
                <div class="mt-4 collapsible-content">
                    <div class="flex flex-col gap-2">
                        <div class="flex items-center gap-2">
                            <input id="categoryAll" class="size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800" type="checkbox" value="">
                            <label for="categoryAll" class="align-middle cursor-pointer">
                                All
                            </label>
                        </div>
                        <div class="flex items-center gap-2">
                            <input id="category1" class="size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800" type="checkbox" value="">
                            <label for="category1" class="align-middle cursor-pointer">
                                Mobiles, Computers
                            </label>
                        </div>
                        <div class="flex items-center gap-2">
                            <input id="category2" class="size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800" type="checkbox" value="">
                            <label for="category2" class="align-middle cursor-pointer">
                                TV, Appliances, Electronics
                            </label>
                        </div>
                        <div class="flex items-center gap-2">
                            <input id="category3" class="size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800" type="checkbox" value="">
                            <label for="category3" class="align-middle cursor-pointer">
                                Men's Fashion
                            </label>
                        </div>
                        <div class="flex items-center gap-2">
                            <input id="category4" class="size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800" type="checkbox" value="">
                            <label for="category4" class="align-middle cursor-pointer">
                                Women's Fashion
                            </label>
                        </div>
                        <div class="flex items-center gap-2">
                            <input id="category5" class="size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800" type="checkbox" value="">
                            <label for="category5" class="align-middle cursor-pointer">
                                Home, Kitchen, Pets
                            </label>
                        </div>
                        <div class="flex items-center gap-2">
                            <input id="category6" class="size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800" type="checkbox" value="">
                            <label for="category6" class="align-middle cursor-pointer">
                                Beauty, Health, Grocery
                            </label>
                        </div>
                        <div class="flex items-center gap-2">
                            <input id="category7" class="size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800" type="checkbox" value="">
                            <label for="category7" class="align-middle cursor-pointer">
                                Books
                            </label>
                        </div>
                    </div>
                </div>
            </div>
            <div class="mt-4 collapsible">
                <button class="flex items-center w-full text-left collapsible-header group">
                    <!-- svelte-ignore a11y-click-events-have-key-events -->
                    <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
                    <h6 class="underline grow"  on:click={e => toggleCollpase(e)}>Rating</h6>
                    <div class="shrink-0 text-slate-500 dark:text-zink-200">
                        <LucideIcon name="ChevronDown" class="hidden size-4 group-[.show]:inline-block"/>
                        <LucideIcon name="ChevronUp" class="inline-block size-4 group-[.show]:hidden"/>
                    </div>
                </button>
                <div class="mt-4 collapsible-content">
                    <div class="flex flex-col gap-2">
                        <div class="flex items-center gap-2">
                            <input id="ratingAll" class="size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800" type="checkbox" value="">
                            <label for="ratingAll" class="align-middle cursor-pointer">
                                All
                            </label>
                        </div>
                        <div class="flex items-center gap-2">
                            <input id="rating5" class="size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800" type="checkbox" value="">
                            <label for="rating5" class="align-middle cursor-pointer">
                                <LucideIcon name="Star" class="inline-block size-4 ml-1 text-yellow-500 align-middle"/> <span class="align-middle">5 Rating</span>
                            </label>
                        </div>
                        <div class="flex items-center gap-2">
                            <input id="rating4" class="size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800" type="checkbox" value="">
                            <label for="rating4" class="align-middle cursor-pointer">
                                <LucideIcon name="Star" class="inline-block size-4 ml-1 text-yellow-500 align-middle"/> <span class="align-middle">4 Rating and Up</span>
                            </label>
                        </div>
                        <div class="flex items-center gap-2">
                            <input id="rating3" class="size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800" type="checkbox" value="">
                            <label for="rating3" class="align-middle cursor-pointer">
                                <LucideIcon name="Star" class="inline-block size-4 ml-1 text-yellow-500 align-middle"/> <span class="align-middle">3 Rating and Up</span>
                            </label>
                        </div>
                        <div class="flex items-center gap-2">
                            <input id="rating2" class="size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800" type="checkbox" value="">
                            <label for="rating2" class="align-middle cursor-pointer">
                                <LucideIcon name="Star" class="inline-block size-4 ml-1 text-yellow-500 align-middle"/> <span class="align-middle">2 Rating and Up</span>
                            </label>
                        </div>
                        <div class="flex items-center gap-2">
                            <input id="rating1" class="size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800" type="checkbox" value="">
                            <label for="rating1" class="align-middle cursor-pointer">
                                <LucideIcon name="Star" class="inline-block size-4 ml-1 text-yellow-500 align-middle"/> <span class="align-middle">1 Rating and Up</span>
                            </label>
                        </div>
                        <div class="flex items-center gap-2">
                            <input id="rating0" class="size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800" type="checkbox" value="">
                            <label for="rating0" class="align-middle cursor-pointer">
                                <LucideIcon name="Star" class="inline-block size-4 ml-1 text-yellow-500 align-middle"/> <span class="align-middle">0 Rating</span>
                            </label>
                        </div>
                    </div>
                </div>
            </div>
            <div class="mt-4 collapsible">
                <button class="flex items-center w-full text-left collapsible-header group show">
                    <!-- svelte-ignore a11y-click-events-have-key-events -->
                    <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
                    <h6 class="underline grow"  on:click={e => toggleCollpase(e)}>Gender</h6>
                    <div class="ml-2 shrink-0">
                        <LucideIcon name="ChevronDown" class="hidden size-4 group-[.show]:inline-block"/>
                        <LucideIcon name="ChevronUp" class="inline-block size-4 group-[.show]:hidden"/>
                    </div>
                </button>
                <div class="hidden mt-4 collapsible-content">
                    <div class="flex flex-col gap-2">
                        <div class="flex items-center gap-2">
                            <input id="genderAll" class="size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800" type="checkbox" value="">
                            <label for="genderAll" class="align-middle cursor-pointer">
                                All
                            </label>
                        </div>
                        <div class="flex items-center gap-2">
                            <input id="gendermal" class="size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800" type="checkbox" value="">
                            <label for="gendermal" class="align-middle cursor-pointer">
                                Male
                            </label>
                        </div>
                        <div class="flex items-center gap-2">
                            <input id="genderFemal" class="size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800" type="checkbox" value="">
                            <label for="genderFemal" class="align-middle cursor-pointer">
                                Female
                            </label>
                        </div>
                        <div class="flex items-center gap-2">
                            <input id="genderOthers" class="size-4 cursor-pointer bg-white border border-slate-200 checked:bg-none dark:bg-zink-700 dark:border-zink-500 rounded-sm appearance-none arrow-none relative after:absolute after:content-['\eb7b'] after:top-0 after:left-0 after:font-remix after:leading-none after:opacity-0 checked:after:opacity-100 after:text-custom-500 checked:border-custom-500 dark:after:text-custom-500 dark:checked:border-custom-800" type="checkbox" value="">
                            <label for="genderOthers" class="align-middle cursor-pointer">
                                Others
                            </label>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</div>