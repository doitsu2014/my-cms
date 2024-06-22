<script>
    import HeadTitle from "../../../../../common/components/HeadTitle.svelte";
	import Breadcrumb from "../../../../../common/components/Breadcrumb.svelte";
	import LucideIcon from "../../../../../common/components/LucideIcon.svelte";

	import Sidebar from "../Sidebar.svelte";
</script>
<HeadTitle title="Watch Video" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto relative">
    <Breadcrumb title="Watch Video" pagetitle="Social Media" />

    <div class="grid grid-cols-1 xl:grid-cols-12 gap-x-5">
        <Sidebar link="video"/>

        <div class="xl:col-span-9">
            <div class="grid items-center grid-cols-1 gap-4 mb-4 xl:grid-cols-12">
                <div class="xl:col-span-3">
                    <h6 class="mb-0 text-15">Watch Video</h6>
                </div>
                <div class="flex gap-2 xl:col-span-4 xl:col-start-9">
                    <div class="relative grow">
                        <input type="text" class="ltr:pl-8 rtl:pr-8 search form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="Search for ..." autocomplete="off">
                        <LucideIcon name="Search" class="inline-block size-4 absolute ltr:left-2.5 rtl:right-2.5 top-2.5 text-slate-500 dark:text-zink-200 fill-slate-100 dark:fill-zink-600"/>
                    </div>
                    <button type="button" class="text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"><LucideIcon name="Plus" class="inline-block size-4"/> <span class="align-middle">Add Video</span></button>
                </div>
            </div>
            <div class="grid grid-cols-1 gap-5 mb-5 md:grid-cols-12">
                <div class="md:col-span-12 xl:row-span-6 xl:col-span-8">
                    <div class="relative h-full bg-white rounded-md dark:bg-zink-600">
                        <h3 class="absolute inset-0 flex items-center justify-center">Welcome to <span class="text-sky-500">Tailwick</span> 🤩</h3>
                        <iframe name="iframe_a" class="relative w-full h-full rounded-md aspect-video" title="Iframe Example"></iframe>
                    </div>
                </div>
                <div class="md:col-span-6 xl:row-span-3 xl:col-span-4">
                    <a href="https://www.youtube.com/embed/qYgogv4R8zg?si=_YxUDmc2fDgHyPae" target="iframe_a" class="block !mb-0 card relative">
                        <div class="card-body">
                            <div class="absolute flex items-center justify-center size-12 -translate-x-1/2 rounded-full bg-white/70 top-1/2 left-1/2">
                                <LucideIcon name="PlaySquare" class="text-red-500"/>
                            </div>
                            <h6 class="mb-3 truncate text-15">Setting Up Tailwind CSS</h6>
                            <img src="https://i.ytimg.com/vi/qYgogv4R8zg/hqdefault.jpg?sqp=-oaymwEcCPYBEIoBSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLBcI9Cw0yBtGv_ashbS-ogqh1OGpQ" alt="" class="object-cover w-full rounded-md h-44">
                        </div>
                    </a>
                </div>
                <div class="md:col-span-6 xl:row-span-3 xl:col-span-4">
                    <a href="https://www.youtube.com/embed/waoOK5s9ypc?si=yzK5f339AF_hoXdn" target="iframe_a" class="block !mb-0 card relative">
                        <div class="card-body">
                            <div class="absolute flex items-center justify-center size-12 -translate-x-1/2 rounded-full bg-white/70 top-1/2 left-1/2">
                                <LucideIcon name="PlaySquare" class="text-red-500"/>
                            </div>
                            <h6 class="mb-3 truncate text-15">Upgrading to Tailwind CSS v3.0</h6>
                            <img src="https://i.ytimg.com/vi/waoOK5s9ypc/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLCkSGeAQ17_LFepTKdEByyVTZkeVw" alt="" class="object-cover w-full rounded-md h-44">
                        </div>
                    </a>
                </div>
                <div class="md:col-span-6 xl:col-span-4">
                    <a href="https://www.youtube.com/embed/TrftauE2Vyk?si=HRCFmq5956C01yB3" target="iframe_a" class="block !mb-0 card relative">
                        <div class="card-body">
                            <div class="absolute flex items-center justify-center size-12 -translate-x-1/2 rounded-full bg-white/70 top-1/2 left-1/2">
                                <LucideIcon name="PlaySquare" class="text-red-500"/>
                            </div>
                            <h6 class="mb-3 truncate text-15">Composing Utilities with @apply – Tailwind CSS</h6>
                            <img src="https://i.ytimg.com/vi/TrftauE2Vyk/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLCm6UjEfDC3R5dJik1gEW__HEnaAA" alt="" class="object-cover w-full rounded-md h-44">
                        </div>
                    </a>
                </div>
                <div class="md:col-span-6 xl:col-span-4">
                    <a href="https://www.youtube.com/embed/_CntOc4hBcg?si=-gNbpwS69E8EmjvI" target="iframe_a" class="block !mb-0 card relative">
                        <div class="card-body">
                            <div class="absolute flex items-center justify-center size-12 -translate-x-1/2 rounded-full bg-white/70 top-1/2 left-1/2">
                                <LucideIcon name="PlaySquare" class="text-red-500"/>
                            </div>
                            <h6 class="mb-3 truncate text-15">Sorting Tailwind CSS Classes Automatically with Prettier</h6>
                            <img src="https://i.ytimg.com/vi/_CntOc4hBcg/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLCh9x1KP6NaenewDWambvyEqCZ-Wg" alt="" class="object-cover w-full rounded-md h-44">
                        </div>
                    </a>
                </div>
                <div class="md:col-span-6 xl:col-span-4">
                    <a href="https://www.youtube.com/embed/GEYkwfYytAM?si=UL3B-HARGE7df_G2" target="iframe_a" class="block !mb-0 card relative">
                        <div class="card-body">
                            <div class="absolute flex items-center justify-center size-12 -translate-x-1/2 rounded-full bg-white/70 top-1/2 left-1/2">
                                <LucideIcon name="PlaySquare" class="text-red-500"/>
                            </div>
                            <h6 class="mb-3 truncate text-15">Effortless typography, even in dark mode — Tailwind CSS</h6>
                            <img src="https://i.ytimg.com/vi/GEYkwfYytAM/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLCQ3H5Fu81Ffh22LmJThZWWDumwkw" alt="" class="object-cover w-full rounded-md h-44">
                        </div>
                    </a>
                </div>
            </div>
        </div>
    </div>
</div>
