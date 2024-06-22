<script>
    import { Body, classList, style } from 'svelte-body';
	import Features from './Features.svelte';
	import Home from './Home.svelte';
	import Navbar from './Navbar.svelte';
	import HeadTitle from "../../../../common/components/HeadTitle.svelte"
	import CounterPage from "../../../../common/components/CounterPage.svelte"
	import About from './About.svelte';
	import Pricing from './Pricing.svelte';
	import Contact from './Contact.svelte';
	import LucideIcon from '../../../../common/components/LucideIcon.svelte';
	import { onDestroy, onMount } from 'svelte';
	import {browser} from "$app/environment"
    let year = new Date().getFullYear();

	onMount(() => {
		if (browser) {
			window.addEventListener("scroll", scrollNavigation, true);
		}
	})

    onDestroy(() => {
        if (browser) {
            window.removeEventListener("scroll", scrollNavigation);
        }
    })

	function scrollNavigation(){
		var scrollup = document.documentElement.scrollTop || document.body.scrollTop;
        if (scrollup > 50) {
            if(document.getElementById("back-to-top"))
			document.getElementById("back-to-top").style.opacity = 1;
		}else{
            if(document.getElementById("back-to-top"))
			document.getElementById("back-to-top").style.opacity = 0;
		}
	}

	function backToTop(){
		if(browser){
			document.body.scrollTop = 0;
    		document.documentElement.scrollTop = 0;
		}
	}

</script>
<svelte:body use:classList={'text-base bg-white text-body font-public dark:text-zink-50 dark:bg-zink-800'} />

<HeadTitle title="One Page" />

<CounterPage />

<Navbar />

<Home />

<Features />

<About />

<Pricing />

<Contact />

<footer class="relative pt-20 pb-12 bg-slate-800 dark:bg-zink-700">
    <div class="container 2xl:max-w-[87.5rem] px-4 mx-auto">
        <div class="relative z-10 grid grid-cols-12 gap-5 xl:grid-cols-12">
            <div class="col-span-12 md:col-span-6 lg:col-span-4 xl:col-span-3">
                <h5 class="mb-4 font-medium tracking-wider text-slate-50 dark:text-zink-50">Dashboards</h5>
                <ul class="flex flex-col gap-3 text-15">
                    <li>
                        <a href="#!" class="relative inline-block transition-all duration-200 ease-linear text-slate-400 dark:text-zink-200 hover:text-slate-300 dark:hover:text-zink-50 before:absolute before:border-b before:border-slate-500 dark:before:border-zink-500 before:inset-x-0 before:bottom-0 before:w-0 hover:before:w-full before:transition-all before:duration-300 before:ease-linear">Analytics</a>
                    </li>
                    <li>
                        <a href="#!" class="relative inline-block transition-all duration-200 ease-linear text-slate-400 dark:text-zink-200 hover:text-slate-300 dark:hover:text-zink-50 before:absolute before:border-b before:border-slate-500 dark:before:border-zink-500 before:inset-x-0 before:bottom-0 before:w-0 hover:before:w-full before:transition-all before:duration-300 before:ease-linear">CRM</a>
                    </li>
                    <li>
                        <a href="#!" class="relative inline-block transition-all duration-200 ease-linear text-slate-400 dark:text-zink-200 hover:text-slate-300 dark:hover:text-zink-50 before:absolute before:border-b before:border-slate-500 dark:before:border-zink-500 before:inset-x-0 before:bottom-0 before:w-0 hover:before:w-full before:transition-all before:duration-300 before:ease-linear">Ecommerce</a>
                    </li>
                    <li>
                        <a href="#!" class="relative inline-block transition-all duration-200 ease-linear text-slate-400 dark:text-zink-200 hover:text-slate-300 dark:hover:text-zink-50 before:absolute before:border-b before:border-slate-500 dark:before:border-zink-500 before:inset-x-0 before:bottom-0 before:w-0 hover:before:w-full before:transition-all before:duration-300 before:ease-linear">Email</a>
                    </li>
                    <li>
                        <a href="#!" class="relative inline-block transition-all duration-200 ease-linear text-slate-400 dark:text-zink-200 hover:text-slate-300 dark:hover:text-zink-50 before:absolute before:border-b before:border-slate-500 dark:before:border-zink-500 before:inset-x-0 before:bottom-0 before:w-0 hover:before:w-full before:transition-all before:duration-300 before:ease-linear">HR</a>
                    </li>
                    <li>
                        <a href="#!" class="relative inline-block transition-all duration-200 ease-linear text-slate-400 dark:text-zink-200 hover:text-slate-300 dark:hover:text-zink-50 before:absolute before:border-b before:border-slate-500 dark:before:border-zink-500 before:inset-x-0 before:bottom-0 before:w-0 hover:before:w-full before:transition-all before:duration-300 before:ease-linear">Social Media</a>
                    </li>
                </ul>
            </div>
            <div class="col-span-12 md:col-span-6 lg:col-span-4 xl:col-span-3">
                <h5 class="mb-4 font-medium tracking-wider text-slate-50 dark:text-zink-50">Apps Pages</h5>
                <ul class="flex flex-col gap-3 text-15">
                    <li>
                        <a href="#!" class="relative inline-block transition-all duration-200 ease-linear text-slate-400 dark:text-zink-200 hover:text-slate-300 dark:hover:text-zink-50 before:absolute before:border-b before:border-slate-500 dark:before:border-zink-500 before:inset-x-0 before:bottom-0 before:w-0 hover:before:w-full before:transition-all before:duration-300 before:ease-linear">Ecommerce Apps</a>
                    </li>
                    <li>
                        <a href="#!" class="relative inline-block transition-all duration-200 ease-linear text-slate-400 dark:text-zink-200 hover:text-slate-300 dark:hover:text-zink-50 before:absolute before:border-b before:border-slate-500 dark:before:border-zink-500 before:inset-x-0 before:bottom-0 before:w-0 hover:before:w-full before:transition-all before:duration-300 before:ease-linear">Invoices</a>
                    </li>
                    <li>
                        <a href="#!" class="relative inline-block transition-all duration-200 ease-linear text-slate-400 dark:text-zink-200 hover:text-slate-300 dark:hover:text-zink-50 before:absolute before:border-b before:border-slate-500 dark:before:border-zink-500 before:inset-x-0 before:bottom-0 before:w-0 hover:before:w-full before:transition-all before:duration-300 before:ease-linear">Email App</a>
                    </li>
                    <li>
                        <a href="#!" class="relative inline-block transition-all duration-200 ease-linear text-slate-400 dark:text-zink-200 hover:text-slate-300 dark:hover:text-zink-50 before:absolute before:border-b before:border-slate-500 dark:before:border-zink-500 before:inset-x-0 before:bottom-0 before:w-0 hover:before:w-full before:transition-all before:duration-300 before:ease-linear">Chat App</a>
                    </li>
                    <li>
                        <a href="#!" class="relative inline-block transition-all duration-200 ease-linear text-slate-400 dark:text-zink-200 hover:text-slate-300 dark:hover:text-zink-50 before:absolute before:border-b before:border-slate-500 dark:before:border-zink-500 before:inset-x-0 before:bottom-0 before:w-0 hover:before:w-full before:transition-all before:duration-300 before:ease-linear">Users Apps</a>
                    </li>
                    <li>
                        <a href="#!" class="relative inline-block transition-all duration-200 ease-linear text-slate-400 dark:text-zink-200 hover:text-slate-300 dark:hover:text-zink-50 before:absolute before:border-b before:border-slate-500 dark:before:border-zink-500 before:inset-x-0 before:bottom-0 before:w-0 hover:before:w-full before:transition-all before:duration-300 before:ease-linear">HR Management</a>
                    </li>
                    <li>
                        <a href="#!" class="relative inline-block transition-all duration-200 ease-linear text-slate-400 dark:text-zink-200 hover:text-slate-300 dark:hover:text-zink-50 before:absolute before:border-b before:border-slate-500 dark:before:border-zink-500 before:inset-x-0 before:bottom-0 before:w-0 hover:before:w-full before:transition-all before:duration-300 before:ease-linear">Social Media</a>
                    </li>
                </ul>
            </div>
            <div class="col-span-12 md:col-span-6 lg:col-span-4 xl:col-span-2">
                <h5 class="mb-4 font-medium tracking-wider text-slate-50 dark:text-zink-50">Resources</h5>
                <ul class="flex flex-col gap-3 text-15">
                    <li>
                        <a href="#!" class="relative inline-block transition-all duration-200 ease-linear text-slate-400 dark:text-zink-200 hover:text-slate-300 dark:hover:text-zink-50 before:absolute before:border-b before:border-slate-500 dark:before:border-zink-500 before:inset-x-0 before:bottom-0 before:w-0 hover:before:w-full before:transition-all before:duration-300 before:ease-linear">All Resources</a>
                    </li>
                    <li>
                        <a href="#!" class="relative inline-block transition-all duration-200 ease-linear text-slate-400 dark:text-zink-200 hover:text-slate-300 dark:hover:text-zink-50 before:absolute before:border-b before:border-slate-500 dark:before:border-zink-500 before:inset-x-0 before:bottom-0 before:w-0 hover:before:w-full before:transition-all before:duration-300 before:ease-linear">Blog</a>
                    </li>
                    <li>
                        <a href="#!" class="relative inline-block transition-all duration-200 ease-linear text-slate-400 dark:text-zink-200 hover:text-slate-300 dark:hover:text-zink-50 before:absolute before:border-b before:border-slate-500 dark:before:border-zink-500 before:inset-x-0 before:bottom-0 before:w-0 hover:before:w-full before:transition-all before:duration-300 before:ease-linear">FAQ's</a>
                    </li>
                    <li>
                        <a href="#!" class="relative inline-block transition-all duration-200 ease-linear text-slate-400 dark:text-zink-200 hover:text-slate-300 dark:hover:text-zink-50 before:absolute before:border-b before:border-slate-500 dark:before:border-zink-500 before:inset-x-0 before:bottom-0 before:w-0 hover:before:w-full before:transition-all before:duration-300 before:ease-linear">Help Center</a>
                    </li>
                </ul>
            </div>
            <div class="col-span-12 md:col-span-6 lg:col-span-12 xl:col-span-4">
                <form action="#!" class="relative mb-6">
                    <input type="email" id="subscribeInput" class="py-3 ltr:pr-40 rtl:pl-40 bg-slate-700/60 dark:bg-zink-600/40 form-input text-slate-200 border-slate-700 dark:border-zink-500 focus:outline-none focus:border-custom-500 dark:focus:border-custom-500 placeholder:text-slate-500 dark:placeholder:text-zink-200 backdrop-blur-md" autocomplete="off" placeholder="tailwick@themesdesign.com" required>
                    <button type="submit" class="absolute px-6 py-2 text-base transition-all duration-200 ease-linear border-custom-500 bg-custom-500 ltr:right-1 rtl:left-1 text-custom-50 btn hover:text-custom-50 hover:bg-custom-600 hover:border-custom-600 top-1 bottom-1">Subscribe Now</button>
                </form>

                <p class="mb-1 text-slate-500 dark:text-zink-200 text-15">Support Email</p>
                <h5 class="text-lg !font-normal text-slate-200 dark:text-zink-50">support@themesdesign.com</h5>

                <p class="mt-6 mb-1 text-slate-500 dark:text-zink-200 text-15">Contact Us</p>
                <h5 class="text-lg !font-normal text-slate-200 dark:text-zink-50">+(012) 1202 012 4567</h5>
            </div>
        </div>

        <div class="py-5 mt-20 border-y border-slate-700">
            <div class="relative z-10 grid grid-cols-1 gap-5 md:grid-cols-12">
                <div class="md:col-span-2">
                    <div class="relative dropdown">
                        <button type="button" class="inline-flex items-center gap-3 transition-all duration-200 ease-linear dropdown-toggle btn border-slate-700 group/items focus:border-custom-500" id="dropdownMenuButton" data-bs-toggle="dropdown">
                            <img src="/assets/images/flags/us.svg" alt="" class="object-cover h-5 rounded-full">
                            <h6 class="text-base font-medium transition-all duration-200 ease-linear text-slate-300 group-hover/items:text-custom-500">English</h6>
                        </button>
                    </div>
                </div>
                <div class="md:col-span-3 md:col-start-10">
                    <ul class="flex items-center gap-3 md:justify-end">
                        <li>
                            <a href="#!" class="flex items-center justify-center size-10 transition-all duration-200 ease-linear border rounded-full text-slate-400 border-slate-700 hover:text-custom-500"><LucideIcon name="Facebook" class="size-4"/></a>
                        </li>
                        <li>
                            <a href="#!" class="flex items-center justify-center size-10 transition-all duration-200 ease-linear border rounded-full text-slate-400 border-slate-700 hover:text-custom-500"><LucideIcon name="Linkedin" class="size-4"/></a>
                        </li>
                        <li>
                            <a href="#!" class="flex items-center justify-center size-10 transition-all duration-200 ease-linear border rounded-full text-slate-400 border-slate-700 hover:text-custom-500"><LucideIcon name="Instagram" class="size-4"/></a>
                        </li>
                        <li>
                            <a href="#!" class="flex items-center justify-center size-10 transition-all duration-200 ease-linear border rounded-full text-slate-400 border-slate-700 hover:text-custom-500"><LucideIcon name="Twitter" class="size-4"/></a>
                        </li>
                        <li>
                            <a href="#!" class="flex items-center justify-center size-10 transition-all duration-200 ease-linear border rounded-full text-slate-400 border-slate-700 hover:text-custom-500"><LucideIcon name="Youtube" class="size-4"/></a>
                        </li>
                    </ul>
                </div>
            </div>
        </div>

        <div class="mt-12 text-center text-slate-400 dark:text-zink-200 text-16">
            <p>
                {year} © Tailwick. Design & Develop by <a href="#!" class="underline text-slate-300 dark:text-zink-100">Themesdesign</a>
            </p>
        </div>
    </div>
</footer>

<button id="back-to-top" class="fixed flex items-center justify-center size-10 text-white bg-purple-500 rounded-md bottom-10 right-10" on:click={backToTop}>
	<LucideIcon name="ChevronUp" data-lucide="chevron-up" class="animate animate-icons"/>
</button>
