<script>
      import { tooltip } from "@svelte-plugins/tooltips";
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import LucideIcon from '../../../../common/components/LucideIcon.svelte';
</script>

<HeadTitle title="Cards" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Cards" pagetitle="UI Elements" />

    <div class="grid grid-cols-1 gap-x-5 md:grid-cols-2 xl:grid-cols-4">
        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">
                    Card title
                </h6>
                <p class="text-slate-500 dark:text-zink-200">
                    With supporting text below as a natural lead-in to additional content.
                </p>
                <a class="inline-flex items-center gap-2 mt-3 text-sm font-medium transition-all duration-200 ease-linear text-custom-500 hover:text-custom-600" href="#">
                    Card link <LucideIcon name="ChevronRight" class="inline-block size-4"/>
                </a>
            </div>
        </div>
        <div class="card ">
            <div class="card-body">
                <h6 class="mb-2 text-15">
                    Card title
                </h6>
                <p class="mb-4 text-xs font-medium uppercase">
                    Card subtitle
                </p>
                <p class="mt-2 text-slate-500 dark:text-zink-200">
                    Some quick example text to build on the card title and make up the bulk of the card's content.
                </p>
                <a class="inline-flex items-center gap-2 mt-3 text-sm font-medium transition-all duration-200 ease-linear text-custom-500 hover:text-custom-600" href="#">
                    Card link <LucideIcon name="ChevronRight" class="inline-block size-4"/>
                </a>
            </div>
        </div>
        <div class="card">
            <div class="bg-slate-200 card-body rounded-t-md dark:bg-zink-600">
                <h6 class="text-15">
                    Card title
                </h6>
            </div>
            <div class="card-body">
                <h6 class="mb-4 text-15">
                    Card title
                </h6>
                <p class="text-slate-500 dark:text-zink-200">
                    With supporting text below as a natural lead-in to additional content.
                </p>
                <a class="inline-flex items-center gap-2 mt-3 text-sm font-medium transition-all duration-200 ease-linear text-custom-500 hover:text-custom-600" href="#">
                    Card link <LucideIcon name="ChevronRight" class="inline-block size-4"/>
                </a>
            </div>
        </div>
        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">
                    Card title
                </h6>
                <p class="text-slate-500 dark:text-zink-200">
                    With supporting text below as a natural lead-in to additional content.
                </p>
                <a class="inline-flex items-center gap-2 mt-3 text-sm font-medium transition-all duration-200 ease-linear text-custom-500 hover:text-custom-600" href="#">
                    Card link <LucideIcon name="ChevronRight" class="inline-block size-4"/>
                </a>
            </div>
            <div class="bg-slate-200 card-body dark:bg-zink-600 rounded-b-md">
                <p class="text-slate-500 ext-sm dark:text-zink-200">
                    Last updated 5 mins ago
                </p>
            </div>
        </div>
    </div><!--end grid-->

    <h5 class="mb-5 underline">Horizontal Card</h5>
    <div class="grid grid-cols-1 gap-5 xl:grid-cols-2">
        <div class="card sm:flex">
            <div>
                <img class="transition-transform duration-500 ease-in-out group-hover/card:scale-105 rounded-l-md" src="/assets/images/small/img-4.jpg" alt="Image">
            </div>
            <div class="flex flex-wrap">
                <div class="flex flex-col h-full card-body">
                    <h6 class="mb-4 text-15">
                        Card title
                    </h6>
                    <p class="text-slate-500 dark:text-zink-200">
                        Some quick example text to build on the card title and make up the bulk of the card's content.
                    </p>
                    <div class="mt-5 sm:mt-auto">
                        <p class="text-sm text-slate-500 dark:text-zink-200">
                            Last updated 5 mins ago
                        </p>
                    </div>
                </div>
            </div>
        </div><!--end card-->
        <div class="card sm:flex">
            <div class="flex flex-wrap">
                <div class="flex flex-col h-full card-body">
                    <h6 class="text-gray-800 text-15 dark:text-zink-50">
                        Card title
                    </h6>
                    <p class="mt-2 text-slate-500 dark:text-zink-200">
                        Some quick example text to build on the card title and make up the bulk of the card's content.
                    </p>
                    <div class="mt-5 sm:mt-auto">
                        <p class="text-slate-500 ext-sm dark:text-zink-200">
                            Last updated 5 mins ago
                        </p>
                    </div>
                </div>
            </div>
            <div>
                <img class="transition-transform duration-500 ease-in-out group-hover/card:scale-105 rounded-r-md" src="/assets/images/small/img-1.jpg" alt="Image">
            </div>
        </div><!--end card-->
    </div><!--end grid-->

    <h5 class="mb-5 underline">Card Animation</h5>
    <div class="grid grid-cols-1 gap-x-5 md:grid-cols-2 xl:grid-cols-4">
        <a class="block overflow-hidden transition card group/card hover:shadow-lg" href="#">
            <div class="relative">
                <img class="transition-transform duration-500 ease-in-out group-hover/card:scale-105 rounded-t-md" src="/assets/images/small/img-4.jpg" alt="Image">
            </div>
            <div class="card-body">
                <h6 class="mb-4 text-15">
                    Card title
                </h6>
                <p class="mt-2 text-slate-500 dark:text-zink-200">
                    With supporting text below as a natural lead-in to additional content.
                </p>
            </div>
        </a>
        <a class="block overflow-hidden transition card group/card hover:shadow-lg" href="#">
            <div class="card-body">
                <h6 class="mb-4 text-15">
                    Card title
                </h6>
                <p class="text-slate-500 dark:text-zink-200">
                    With supporting text below as a natural lead-in to additional content.
                </p>
            </div>
            <div class="relative">
                <img class="transition-transform duration-500 ease-in-out group-hover/card:scale-105 rounded-b-md" src="/assets/images/small/img-9.jpg" alt="Image">
            </div>
        </a>
        <a class="block overflow-hidden transition card hover:shadow-lg" href="#">
            <img class="rounded-t-md" src="/assets/images/small/img-10.jpg" alt="Image">
            <div class="card-body">
                <h6 class="mb-4 text-15">
                    Card title
                </h6>
                <p class="text-slate-500 dark:text-zink-200">
                    With supporting text below as a natural lead-in to additional content.
                </p>
            </div>
        </a>
        <a class="block overflow-hidden transition card hover:shadow-lg hover:-translate-y-1" href="#">
            <img class="rounded-t-md" src="/assets/images/small/img-10.jpg" alt="Image">
            <div class="card-body">
                <h6 class="mb-4 text-15">
                    Card title
                </h6>
                <p class="text-slate-500 dark:text-zink-200">
                    With supporting text below as a natural lead-in to additional content.
                </p>
            </div>
        </a>
    </div><!--end grid-->

    <h5 class="mb-5 underline">Border Card</h5>
    <div class="grid grid-cols-1 gap-x-5 md:grid-cols-2 xl:grid-cols-4">
        <div class="border card border-custom-200 dark:border-custom-800">
            <div class="card-body">
                <h6 class="mb-4 text-15">
                    Custom Card
                </h6>
                <p class="mb-4 text-slate-500 dark:text-zink-200">
                    With supporting text below as a natural lead-in to additional content.
                </p>
                <a class="inline-flex items-center gap-2 text-sm font-medium transition-all duration-200 ease-linear text-custom-500 hover:text-custom-600" href="#">
                    Card link <LucideIcon name="ChevronRight" class="inline-block size-4"/>
                </a>
            </div>
        </div><!--end card-->
        <div class="border border-green-200 card dark:border-green-800">
            <div class="card-body">
                <h6 class="mb-4 text-15">
                    Green Card
                </h6>
                <p class="mb-4 text-slate-500 dark:text-zink-200">
                    Some quick example text to build on the card title and make up the bulk of the card's content.
                </p>
                <a class="inline-flex items-center gap-2 text-sm font-medium transition-all duration-200 ease-linear text-custom-500 hover:text-custom-600" href="#">
                    Card link <LucideIcon name="ChevronRight" class="inline-block size-4"/>
                </a>
            </div>
        </div><!--end card-->
        <div class="border border-orange-200 card dark:border-orange-800">
            <div class="card-body">
                <h6 class="mb-4 text-15">
                    Orange Card
                </h6>
                <p class="mb-4 text-slate-500 dark:text-zink-200">
                    Some quick example text to build on the card title and make up the bulk of the card's content.
                </p>
                <a class="inline-flex items-center gap-2 text-sm font-medium transition-all duration-200 ease-linear text-custom-500 hover:text-custom-600" href="#">
                    Card link <LucideIcon name="ChevronRight" class="inline-block size-4"/>
                </a>
            </div>
        </div><!--end card-->
        <div class="border card border-sky-200 dark:border-sky-800">
            <div class="card-body">
                <h6 class="mb-4 text-15">
                    Sky Card
                </h6>
                <p class="text-slate-500 dark:text-zink-200">
                    Some quick example text to build on the card title and make up the bulk of the card's content.
                </p>
                <a class="inline-flex items-center gap-2 mt-3 text-sm font-medium transition-all duration-200 ease-linear text-custom-500 hover:text-custom-600" href="#">
                    Card link <LucideIcon name="ChevronRight" class="inline-block size-4"/>
                </a>
            </div>
        </div><!--end card-->
        <div class="border border-yellow-200 card dark:border-yellow-800">
            <div class="card-body">
                <h6 class="mb-4 text-15">
                    Yellow Card
                </h6>
                <p class="mb-4 text-slate-500 dark:text-zink-200">
                    Some quick example text to build on the card title and make up the bulk of the card's content.
                </p>
                <a class="inline-flex items-center gap-2 text-sm font-medium transition-all duration-200 ease-linear text-custom-500 hover:text-custom-600" href="#">
                    Card link <LucideIcon name="ChevronRight" class="inline-block size-4"/>
                </a>
            </div>
        </div><!--end card-->
        <div class="border border-red-200 card dark:border-red-800">
            <div class="card-body">
                <h6 class="mb-4 text-15">
                    Red Card
                </h6>
                <p class="mb-4 text-slate-500 dark:text-zink-200">
                    Some quick example text to build on the card title and make up the bulk of the card's content.
                </p>
                <a class="inline-flex items-center gap-2 text-sm font-medium transition-all duration-200 ease-linear text-custom-500 hover:text-custom-600" href="#">
                    Card link <LucideIcon name="ChevronRight" class="inline-block size-4"/>
                </a>
            </div>
        </div><!--end card-->
        <div class="border border-purple-200 card dark:border-purple-800">
            <div class="card-body">
                <h6 class="mb-4 text-15">
                    Purple Card
                </h6>
                <p class="mb-4 text-slate-500 dark:text-zink-200">
                    Some quick example text to build on the card title and make up the bulk of the card's content.
                </p>
                <a class="inline-flex items-center gap-2 text-sm font-medium transition-all duration-200 ease-linear text-custom-500 hover:text-custom-600" href="#">
                    Card link <LucideIcon name="ChevronRight" class="inline-block size-4"/>
                </a>
            </div>
        </div><!--end card-->
        <div class="border card border-slate-200 dark:border-zink-500">
            <div class="card-body">
                <h6 class="mb-4 text-15">
                    Slate Card
                </h6>
                <p class="mb-4 text-slate-500 dark:text-zink-200">
                    Some quick example text to build on the card title and make up the bulk of the card's content.
                </p>
                <a class="inline-flex items-center gap-2 text-sm font-medium transition-all duration-200 ease-linear text-custom-500 hover:text-custom-600" href="#">
                    Card link <LucideIcon name="ChevronRight" class="inline-block size-4"/>
                </a>
            </div>
        </div><!--end card-->
    </div><!--end grid-->

    <h5 class="mb-5 underline">Dashed Border Card</h5>
    <div class="grid grid-cols-1 gap-x-5 md:grid-cols-2 xl:grid-cols-4">
        <div class="border border-dashed card border-custom-200 dark:border-custom-800">
            <div class="card-body">
                <h6 class="mb-4 text-15">
                    Custom Card
                </h6>
                <p class="mb-4 text-slate-500 dark:text-zink-200">
                    With supporting text below as a natural lead-in to additional content.
                </p>
                <a class="inline-flex items-center gap-2 text-sm font-medium transition-all duration-200 ease-linear text-custom-500 hover:text-custom-600" href="#">
                    Card link <LucideIcon name="ChevronRight" class="inline-block size-4"/>
                </a>
            </div>
        </div><!--end card-->
        <div class="border border-green-200 border-dashed card dark:border-green-800">
            <div class="card-body">
                <h6 class="mb-4 text-15">
                    Green Card
                </h6>
                <p class="mb-4 text-slate-500 dark:text-zink-200">
                    Some quick example text to build on the card title and make up the bulk of the card's content.
                </p>
                <a class="inline-flex items-center gap-2 text-sm font-medium transition-all duration-200 ease-linear text-custom-500 hover:text-custom-600" href="#">
                    Card link <LucideIcon name="ChevronRight" class="inline-block size-4"/>
                </a>
            </div>
        </div><!--end card-->
        <div class="border border-orange-200 border-dashed card dark:border-orange-800">
            <div class="card-body">
                <h6 class="mb-4 text-15">
                    Orange Card
                </h6>
                <p class="mb-4 text-slate-500 dark:text-zink-200">
                    Some quick example text to build on the card title and make up the bulk of the card's content.
                </p>
                <a class="inline-flex items-center gap-2 text-sm font-medium transition-all duration-200 ease-linear text-custom-500 hover:text-custom-600" href="#">
                    Card link <LucideIcon name="ChevronRight" class="inline-block size-4"/>
                </a>
            </div>
        </div><!--end card-->
        <div class="border border-dashed card border-sky-200 dark:border-sky-800">
            <div class="card-body">
                <h6 class="mb-4 text-15">
                    Sky Card
                </h6>
                <p class="text-slate-500 dark:text-zink-200">
                    Some quick example text to build on the card title and make up the bulk of the card's content.
                </p>
                <a class="inline-flex items-center gap-2 mt-3 text-sm font-medium transition-all duration-200 ease-linear text-custom-500 hover:text-custom-600" href="#">
                    Card link <LucideIcon name="ChevronRight" class="inline-block size-4"/>
                </a>
            </div>
        </div><!--end card-->
        <div class="border border-yellow-200 border-dashed card dark:border-yellow-800">
            <div class="card-body">
                <h6 class="mb-4 text-15">
                    Yellow Card
                </h6>
                <p class="mb-4 text-slate-500 dark:text-zink-200">
                    Some quick example text to build on the card title and make up the bulk of the card's content.
                </p>
                <a class="inline-flex items-center gap-2 text-sm font-medium transition-all duration-200 ease-linear text-custom-500 hover:text-custom-600" href="#">
                    Card link <LucideIcon name="ChevronRight" class="inline-block size-4"/>
                </a>
            </div>
        </div><!--end card-->
        <div class="border border-red-200 border-dashed card dark:border-red-800">
            <div class="card-body">
                <h6 class="mb-4 text-15">
                    Red Card
                </h6>
                <p class="mb-4 text-slate-500 dark:text-zink-200">
                    Some quick example text to build on the card title and make up the bulk of the card's content.
                </p>
                <a class="inline-flex items-center gap-2 text-sm font-medium transition-all duration-200 ease-linear text-custom-500 hover:text-custom-600" href="#">
                    Card link <LucideIcon name="ChevronRight" class="inline-block size-4"/>
                </a>
            </div>
        </div><!--end card-->
        <div class="border border-purple-200 border-dashed card dark:border-purple-800">
            <div class="card-body">
                <h6 class="mb-4 text-15">
                    Purple Card
                </h6>
                <p class="mb-4 text-slate-500 dark:text-zink-200">
                    Some quick example text to build on the card title and make up the bulk of the card's content.
                </p>
                <a class="inline-flex items-center gap-2 text-sm font-medium transition-all duration-200 ease-linear text-custom-500 hover:text-custom-600" href="#">
                    Card link <LucideIcon name="ChevronRight" class="inline-block size-4"/>
                </a>
            </div>
        </div><!--end card-->
        <div class="border border-dashed card border-slate-200 dark:border-zink-500">
            <div class="card-body">
                <h6 class="mb-4 text-15">
                    Slate Card
                </h6>
                <p class="mb-4 text-slate-500 dark:text-zink-200">
                    Some quick example text to build on the card title and make up the bulk of the card's content.
                </p>
                <a class="inline-flex items-center gap-2 text-sm font-medium transition-all duration-200 ease-linear text-custom-500 hover:text-custom-600" href="#">
                    Card link <LucideIcon name="ChevronRight" class="inline-block size-4"/>
                </a>
            </div>
        </div><!--end card-->
    </div><!--end grid-->

    <h5 class="mb-5 underline">Fill Card</h5>
    <div class="grid grid-cols-1 gap-x-5 md:grid-cols-2 xl:grid-cols-4">
        <div class="card bg-custom-500 border-custom-500 dark:bg-custom-800 dark:border-custom-800">
            <div class="card-body">
                <h6 class="mb-4 text-white text-15">
                    Custom Card
                </h6>
                <p class="mb-4 dark:text-custom-300 text-custom-200">
                    With supporting text below as a natural lead-in to additional content.
                </p>
                <a class="inline-flex items-center gap-2 text-sm font-medium transition-all duration-200 ease-linear text-custom-300 hover:text-custom-100" href="#">
                    Card link <LucideIcon name="ChevronRight" class="inline-block size-4"/>
                </a>
            </div>
        </div><!--end card-->
        <div class="bg-green-500 border-green-500 card dark:bg-green-800 dark:border-green-800">
            <div class="card-body">
                <h6 class="mb-4 text-white text-15">
                    Card title
                </h6>
                <p class="mb-4 text-green-100 dark:text-green-200">
                    With supporting text below as a natural lead-in to additional content.
                </p>
                <a class="inline-flex items-center gap-2 text-sm font-medium text-green-300 transition-all duration-200 ease-linear hover:text-green-100" href="#">
                    Card link <LucideIcon name="ChevronRight" class="inline-block size-4"/>
                </a>
            </div>
        </div>
        <div class="bg-yellow-500 border-yellow-500 card dark:bg-yellow-800 dark:border-yellow-800">
            <div class="card-body">
                <h6 class="mb-4 text-white text-15">
                    Yellow Card
                </h6>
                <p class="mb-4 text-yellow-100 dark:text-yellow-200">
                    With supporting text below as a natural lead-in to additional content.
                </p>
                <a class="inline-flex items-center gap-2 text-sm font-medium text-yellow-300 transition-all duration-200 ease-linear hover:text-yellow-100" href="#">
                    Card link <LucideIcon name="ChevronRight" class="inline-block size-4"/>
                </a>
            </div>
        </div><!--end card-->
        <div class="card bg-sky-500 border-sky-500 dark:bg-sky-800 dark:border-sky-800">
            <div class="card-body">
                <h6 class="mb-4 text-white text-15">
                    Sky Card
                </h6>
                <p class="mb-4 text-sky-200 dark:text-sky-200">
                    With supporting text below as a natural lead-in to additional content.
                </p>
                <a class="inline-flex items-center gap-2 text-sm font-medium transition-all duration-200 ease-linear text-sky-300 hover:text-sky-100" href="#">
                    Card link <LucideIcon name="ChevronRight" class="inline-block size-4"/>
                </a>
            </div>
        </div><!--end card-->
        <div class="bg-red-500 border-red-500 card dark:bg-red-500 dark:border-red-500">
            <div class="card-body">
                <h6 class="mb-4 text-white text-15">
                    Red Card
                </h6>
                <p class="mb-4 text-red-200 dark:text-red-200">
                    With supporting text below as a natural lead-in to additional content.
                </p>
                <a class="inline-flex items-center gap-2 text-sm font-medium text-red-300 transition-all duration-200 ease-linear hover:text-red-100" href="#">
                    Card link <LucideIcon name="ChevronRight" class="inline-block size-4"/>
                </a>
            </div>
        </div><!--end card-->
        <div class="bg-purple-500 border-purple-500 card dark:bg-purple-500 dark:border-purple-500">
            <div class="card-body">
                <h6 class="mb-4 text-white text-15">
                    Purple Card
                </h6>
                <p class="mb-4 text-purple-200 dark:text-purple-200">
                    With supporting text below as a natural lead-in to additional content.
                </p>
                <a class="inline-flex items-center gap-2 text-sm font-medium text-purple-300 transition-all duration-200 ease-linear hover:text-purple-100" href="#">
                    Card link <LucideIcon name="ChevronRight" class="inline-block size-4"/>
                </a>
            </div>
        </div><!--end card-->
        <div class="card bg-slate-500 border-slate-500 dark:bg-slate-500 dark:border-slate-500">
            <div class="card-body">
                <h6 class="mb-4 text-white text-15">
                    Slate Card
                </h6>
                <p class="mb-4 text-slate-200 dark:text-slate-200">
                    With supporting text below as a natural lead-in to additional content.
                </p>
                <a class="inline-flex items-center gap-2 text-sm font-medium transition-all duration-200 ease-linear text-slate-300 hover:text-slate-100" href="#">
                    Card link <LucideIcon name="ChevronRight" class="inline-block size-4"/>
                </a>
            </div>
        </div><!--end card-->
    </div><!--end grid-->

    <h5 class="mb-5 underline">Modern Card</h5>
    <div class="grid grid-cols-1 gap-x-5 md:grid-cols-2 xl:grid-cols-4">
        <div class="card bg-custom-100 border-custom-200 dark:bg-custom-400/20 dark:border-custom-600">
            <div class="card-body">
                <h6 class="mb-4 text-custom-500 dark:text-custom-200 text-15">
                    Custom Card
                </h6>
                <p class="mb-4 text-custom-400">
                    With supporting text below as a natural lead-in to additional content.
                </p>
                <a class="inline-flex items-center gap-2 text-sm font-medium transition-all duration-200 ease-linear text-custom-400 hover:text-custom-600" href="#">
                    Card link <LucideIcon name="ChevronRight" class="inline-block size-4"/>
                </a>
            </div>
        </div><!--end card-->
        <div class="bg-green-100 border-green-200 card dark:bg-green-400/20 dark:border-green-600">
            <div class="card-body">
                <h6 class="mb-4 text-green-500 text-15 dark:text-green-200">
                    Green Card
                </h6>
                <p class="mb-4 text-green-400">
                    With supporting text below as a natural lead-in to additional content.
                </p>
                <a class="inline-flex items-center gap-2 text-sm font-medium text-green-400 transition-all duration-200 ease-linear hover:text-green-600" href="#">
                    Card link <LucideIcon name="ChevronRight" class="inline-block size-4"/>
                </a>
            </div>
        </div><!--end card-->
        <div class="bg-orange-100 border-orange-200 card dark:bg-orange-400/20 dark:border-orange-600">
            <div class="card-body">
                <h6 class="mb-4 text-orange-500 text-15 dark:text-orange-200">
                    Orange Card
                </h6>
                <p class="mb-4 text-orange-400">
                    With supporting text below as a natural lead-in to additional content.
                </p>
                <a class="inline-flex items-center gap-2 text-sm font-medium text-orange-400 transition-all duration-200 ease-linear hover:text-orange-600" href="#">
                    Card link <LucideIcon name="ChevronRight" class="inline-block size-4"/>
                </a>
            </div>
        </div><!--end card-->
        <div class="card bg-sky-100 border-sky-200 dark:bg-sky-400/20 dark:border-sky-600">
            <div class="card-body">
                <h6 class="mb-4 text-sky-500 text-15 dark:text-sky-200">
                    Sky Card
                </h6>
                <p class="mb-4 text-sky-400">
                    With supporting text below as a natural lead-in to additional content.
                </p>
                <a class="inline-flex items-center gap-2 text-sm font-medium transition-all duration-200 ease-linear text-sky-400 hover:text-sky-600" href="#">
                    Card link <LucideIcon name="ChevronRight" class="inline-block size-4"/>
                </a>
            </div>
        </div><!--end card-->
        <div class="bg-yellow-100 border-yellow-200 card dark:bg-yellow-400/20 dark:border-yellow-600">
            <div class="card-body">
                <h6 class="mb-4 text-yellow-500 text-15 dark:text-yellow-200">
                    Yellow Card
                </h6>
                <p class="mb-4 text-yellow-400">
                    With supporting text below as a natural lead-in to additional content.
                </p>
                <a class="inline-flex items-center gap-2 text-sm font-medium text-yellow-400 transition-all duration-200 ease-linear hover:text-yellow-600" href="#">
                    Card link <LucideIcon name="ChevronRight" class="inline-block size-4"/>
                </a>
            </div>
        </div><!--end card-->
        <div class="bg-red-100 border-red-200 card dark:bg-red-400/20 dark:border-red-600">
            <div class="card-body">
                <h6 class="mb-4 text-red-500 text-15 dark:text-red-200">
                    Red
                </h6>
                <p class="mb-4 text-red-400">
                    With supporting text below as a natural lead-in to additional content.
                </p>
                <a class="inline-flex items-center gap-2 text-sm font-medium text-red-400 transition-all duration-200 ease-linear hover:text-red-600" href="#">
                    Card link <LucideIcon name="ChevronRight" class="inline-block size-4"/>
                </a>
            </div>
        </div><!--end card-->
        <div class="bg-purple-100 border-purple-200 card dark:bg-purple-400/20 dark:border-purple-600">
            <div class="card-body">
                <h6 class="mb-4 text-purple-500 text-15 dark:text-purple-200">
                    Purple Card
                </h6>
                <p class="mb-4 text-purple-400">
                    With supporting text below as a natural lead-in to additional content.
                </p>
                <a class="inline-flex items-center gap-2 text-sm font-medium text-purple-400 transition-all duration-200 ease-linear hover:text-purple-600" href="#">
                    Card link <LucideIcon name="ChevronRight" class="inline-block size-4"/>
                </a>
            </div>
        </div><!--end card-->
        <div class="card bg-slate-200 border-slate-200 dark:bg-slate-400/20 dark:border-slate-600">
            <div class="card-body">
                <h6 class="mb-4 text-slate-400 text-15 dark:text-slate-200">
                    Slate Card
                </h6>
                <p class="mb-4 text-slate-400">
                    With supporting text below as a natural lead-in to additional content.
                </p>
                <a class="inline-flex items-center gap-2 text-sm font-medium transition-all duration-200 ease-linear text-slate-400 hover:text-slate-600" href="#">
                    Card link <LucideIcon name="ChevronRight" class="inline-block size-4"/>
                </a>
            </div>
        </div><!--end card-->
    </div><!--end grid-->

    <h5 class="mb-5 underline">Group Card</h5>
    <div class="grid overflow-hidden border sm:flex card">
        <div class="flex flex-col flex-[1_0_0%] rtl:border-b rtl:sm:border-b-0 ltr:border-r rtl:sm:border-l border-slate-200 dark:border-zink-500">
            <img class="w-full h-auto rounded-t-md sm:rounded-tr-none" src="/assets/images/small/img-2.jpg" alt="Image">
            <div class="flex-1 card-body">
                <h6 class="mb-4 text-15">
                    Card title
                </h6>
                <p class="text-slate-500 dark:text-zink-200">
                    This is a wider card with supporting text below as a natural lead-in to additional content. This content is a little bit longer.
                </p>
            </div>
            <div class="border-t card-body border-slate-200 dark:border-zink-500">
                <p class="text-xs">
                    Last updated 5 mins ago
                </p>
            </div>
        </div>
        <div class="flex flex-col flex-[1_0_0%] rtl:border-b rtl:sm:border-b-0 ltr:border-r rtl:sm:border-l border-slate-200 dark:border-zink-500">
            <img class="w-full h-auto" src="/assets/images/small/img-7.jpg" alt="Image">
            <div class="flex-1 card-body">
                <h6 class="mb-4 text-15">
                    Card title
                </h6>
                <p class="text-slate-500 dark:text-zink-200">
                    This card has supporting text below as a natural lead-in to additional content.
                </p>
            </div>
            <div class="border-t card-body border-slate-200 dark:border-zink-500">
                <p class="text-xs">
                    Last updated 5 mins ago
                </p>
            </div>
        </div>

        <div class="flex flex-col flex-[1_0_0%]">
            <!-- svelte-ignore a11y-img-redundant-alt -->
            <img class="w-full h-auto sm:rounded-tr-md" src="/assets/images/small/img-5.jpg" alt="Image">
            <div class="flex-1 card-body">
                <h6 class="mb-4 text-15">
                    Card title
                </h6>
                <p class="text-slate-500 dark:text-zink-200">
                    This is a wider card with supporting text below as a natural lead-in to additional content. This card has even longer content than the first to show that equal height action.
                </p>
            </div>
            <div class="border-t card-body border-slate-200 dark:border-zink-500">
                <p class="text-xs">
                    Last updated 5 mins ago
                </p>
            </div>
        </div>
    </div>
</div>
