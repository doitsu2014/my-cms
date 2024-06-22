<script>
    import { onDestroy, onMount } from 'svelte';
    import { browser } from "$app/environment"
    import Features from './Features.svelte';
    import Home from './Home.svelte';
    import Navbar from './Navbar.svelte';
    import HeadTitle from "../../../../common/components/HeadTitle.svelte"
    import CounterPage from "../../../../common/components/CounterPage.svelte"
    import Product from './Product.svelte';
    import About from './About.svelte';
	import Feedback from './Feedback.svelte';
	import Newsletter from './Newsletter.svelte';
	import Footer from './Footer.svelte';
	import LucideIcon from '../../../../common/components/LucideIcon.svelte';

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

    function scrollNavigation() {
        var scrollup = document.documentElement.scrollTop || document.body.scrollTop;
        if (scrollup > 50) {
            if(document.getElementById("back-to-top"))
            document.getElementById("back-to-top").style.opacity = 1;
        } else {
            if(document.getElementById("back-to-top"))
            document.getElementById("back-to-top").style.opacity = 0;
        }
    }

    function backToTop() {
        if (browser) {
            document.body.scrollTop = 0;
            document.documentElement.scrollTop = 0;
        }
    }

</script>

<svelte:head>
    <link href="https://fonts.googleapis.com/css2?family=Tourney:wght@100&display=swap" rel="stylesheet">
</svelte:head>

<HeadTitle title="Product Landing Page" />

<CounterPage />

<Navbar />

<Home />

<Product />

<Features />

<About />

<Feedback />

<Newsletter />

<Footer />

<button id="back-to-top" class="fixed flex items-center justify-center size-10 text-white bg-purple-500 rounded-md bottom-10 right-10"  on:click={backToTop}>
    <LucideIcon name="ChevronUp" class="animate animate-icons"/>
</button>