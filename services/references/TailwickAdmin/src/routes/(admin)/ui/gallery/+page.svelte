<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import LucideIcon from '../../../../common/components/LucideIcon.svelte';

</script>

<HeadTitle title="Gallery" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Gallery" pagetitle="UI Elements" />
  
    <div>
        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">Gallery Content</h6>
                <div class="grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-5">
                    <div class="relative rounded-md group/gallery">
                        <img src="/assets/images/small/img-1.jpg" alt="" class="rounded-md">
                        <div class="pt-3">
                            <h5 class="mb-1 text-16"><a href="#!" class="transition-all duration-300 ease-linear hover:text-custom-500">Cinematic Metro</a></h5>
                            <p class="text-slate-500 dark:text-zink-200">Artyom is depicted as a silent protagonist and a blank slate for the player.</p>
                        </div>
                    </div>
                    <div class="relative rounded-md group/gallery">
                        <img src="/assets/images/small/img-2.jpg" alt="" class="rounded-md">
                        <div class="pt-3">
                            <h5 class="mb-1 text-16"><a href="#!" class="transition-all duration-300 ease-linear hover:text-custom-500">Cinematic Metro</a></h5>
                            <p class="text-slate-500 dark:text-zink-200">Artyom is depicted as a silent protagonist and a blank slate for the player.</p>
                        </div>
                    </div>
                    <div class="relative rounded-md group/gallery">
                        <img src="/assets/images/small/img-3.jpg" alt="" class="rounded-md">
                        <div class="pt-3">
                            <h5 class="mb-1 text-16"><a href="#!" class="transition-all duration-300 ease-linear hover:text-custom-500">Cinematic Metro</a></h5>
                            <p class="text-slate-500 dark:text-zink-200">Artyom is depicted as a silent protagonist and a blank slate for the player.</p>
                        </div>
                    </div>
                    <div class="relative rounded-md group/gallery">
                        <img src="/assets/images/small/img-4.jpg" alt="" class="rounded-md">
                        <div class="pt-3">
                            <h5 class="mb-1 text-16"><a href="#!" class="transition-all duration-300 ease-linear hover:text-custom-500">Cinematic Metro</a></h5>
                            <p class="text-slate-500 dark:text-zink-200">Artyom is depicted as a silent protagonist and a blank slate for the player.</p>
                        </div>
                    </div>
                    <div class="relative rounded-md group/gallery">
                        <img src="/assets/images/small/img-5.jpg" alt="" class="rounded-md">
                        <div class="pt-3">
                            <h5 class="mb-1 text-16"><a href="#!" class="transition-all duration-300 ease-linear hover:text-custom-500">Cinematic Metro</a></h5>
                            <p class="text-slate-500 dark:text-zink-200">Artyom is depicted as a silent protagonist and a blank slate for the player.</p>
                        </div>
                    </div>
                </div><!--end grid-->
            </div>
        </div><!--end card-->
        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">Gallery with Overlay Content</h6>
                <div class="grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-5">
                    <div class="relative overflow-hidden rounded-md group/gallery">
                        <img src="/assets/images/small/img-6.jpg" alt="" class="rounded-md">
                        <div class="absolute inset-0 transition-all duration-300 ease-linear opacity-0 group-hover/gallery:opacity-50 bg-gradient-to-t from-gray-900 to-transparent"></div>
                        <div class="absolute bottom-0 transition-all duration-300 ease-linear opacity-0 left-3 right-3 group-hover/gallery:opacity-100 group-hover/gallery:bottom-3">
                            <h5 class="font-normal text-white"><a href="#!">Image title</a></h5>
                        </div>
                    </div>
                    <div class="relative overflow-hidden rounded-md group/gallery">
                        <img src="/assets/images/small/img-7.jpg" alt="" class="rounded-md">
                        <div class="absolute inset-0 transition-all duration-300 ease-linear opacity-0 group-hover/gallery:opacity-50 bg-gradient-to-t from-gray-900 to-transparent"></div>
                        <div class="absolute bottom-0 transition-all duration-300 ease-linear opacity-0 left-3 right-3 group-hover/gallery:opacity-100 group-hover/gallery:bottom-3">
                            <h5 class="font-normal text-white"><a href="#!">Image title</a></h5>
                        </div>
                    </div>
                    <div class="relative overflow-hidden rounded-md group/gallery">
                        <img src="/assets/images/small/img-8.jpg" alt="" class="rounded-md">
                        <div class="absolute inset-0 transition-all duration-300 ease-linear opacity-0 group-hover/gallery:opacity-50 bg-gradient-to-t from-gray-900 to-transparent"></div>
                        <div class="absolute bottom-0 transition-all duration-300 ease-linear opacity-0 left-3 right-3 group-hover/gallery:opacity-100 group-hover/gallery:bottom-3">
                            <h5 class="font-normal text-white"><a href="#!">Image title</a></h5>
                        </div>
                    </div>
                    <div class="relative overflow-hidden rounded-md group/gallery">
                        <img src="/assets/images/small/img-9.jpg" alt="" class="rounded-md">
                        <div class="absolute inset-0 transition-all duration-300 ease-linear opacity-0 group-hover/gallery:opacity-50 bg-gradient-to-t from-gray-900 to-transparent"></div>
                        <div class="absolute bottom-0 transition-all duration-300 ease-linear opacity-0 left-3 right-3 group-hover/gallery:opacity-100 group-hover/gallery:bottom-3">
                            <h5 class="font-normal text-white"><a href="#!">Image title</a></h5>
                        </div>
                    </div>
                    <div class="relative overflow-hidden rounded-md group/gallery">
                        <img src="/assets/images/small/img-10.jpg" alt="" class="rounded-md">
                        <div class="absolute inset-0 transition-all duration-300 ease-linear opacity-0 group-hover/gallery:opacity-50 bg-gradient-to-t from-gray-900 to-transparent"></div>
                        <div class="absolute bottom-0 transition-all duration-300 ease-linear opacity-0 left-3 right-3 group-hover/gallery:opacity-100 group-hover/gallery:bottom-3">
                            <h5 class="font-normal text-white"><a href="#!">Image title</a></h5>
                        </div>
                    </div>
                </div><!--end grid-->
            </div>
        </div><!--end card-->
        
        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">With Animation</h6>
                <div class="grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-5">
                    <div class="relative overflow-hidden rounded-md group/gallery">
                        <div class="overflow-hidden">
                            <img src="/assets/images/small/img-1.jpg" alt="" class="transition-all duration-300 ease-linear group-hover/gallery:scale-110">
                        </div>
                        <div class="pt-3">
                            <h5 class="mb-1 text-16"><a href="#!" class="transition-all duration-300 ease-linear hover:text-custom-500">Cinematic Metro</a></h5>
                            <p class="text-slate-500 dark:zink-200">Artyom is depicted as a silent protagonist and a blank slate for the player.</p>
                        </div>
                    </div>
                    <div class="relative overflow-hidden rounded-md group/gallery">
                        <div class="overflow-hidden">
                            <img src="/assets/images/small/img-2.jpg" alt="" class="transition-all duration-300 ease-linear group-hover/gallery:scale-110">
                        </div>
                        <div class="pt-3">
                            <h5 class="mb-1 text-16"><a href="#!" class="transition-all duration-300 ease-linear hover:text-custom-500">Cinematic Metro</a></h5>
                            <p class="text-slate-500 dark:zink-200">Artyom is depicted as a silent protagonist and a blank slate for the player.</p>
                        </div>
                    </div>
                    <div class="relative overflow-hidden rounded-md group/gallery">
                        <div class="overflow-hidden">
                            <img src="/assets/images/small/img-3.jpg" alt="" class="transition-all duration-300 ease-linear group-hover/gallery:scale-110">
                        </div>
                        <div class="pt-3">
                            <h5 class="mb-1 text-16"><a href="#!" class="transition-all duration-300 ease-linear hover:text-custom-500">Cinematic Metro</a></h5>
                            <p class="text-slate-500 dark:zink-200">Artyom is depicted as a silent protagonist and a blank slate for the player.</p>
                        </div>
                    </div>
                    <div class="relative overflow-hidden rounded-md group/gallery">
                        <div class="overflow-hidden">
                            <img src="/assets/images/small/img-4.jpg" alt="" class="transition-all duration-300 ease-linear group-hover/gallery:scale-110">
                        </div>
                        <div class="pt-3">
                            <h5 class="mb-1 text-16"><a href="#!" class="transition-all duration-300 ease-linear hover:text-custom-500">Cinematic Metro</a></h5>
                            <p class="text-slate-500 dark:zink-200">Artyom is depicted as a silent protagonist and a blank slate for the player.</p>
                        </div>
                    </div>
                    <div class="relative overflow-hidden rounded-md group/gallery">
                        <div class="overflow-hidden">
                            <img src="/assets/images/small/img-5.jpg" alt="" class="transition-all duration-300 ease-linear group-hover/gallery:scale-110">
                        </div>
                        <div class="pt-3">
                            <h5 class="mb-1 text-16"><a href="#!" class="transition-all duration-300 ease-linear hover:text-custom-500">Cinematic Metro</a></h5>
                            <p class="text-slate-500 dark:zink-200">Artyom is depicted as a silent protagonist and a blank slate for the player.</p>
                        </div>
                    </div>
                </div><!--end grid-->
            </div>
        </div><!--end card-->

        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">Gallery with Card</h6>
                <div class="grid grid-cols-1 gap-4 md:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-5">
                    <div class="relative mb-0 overflow-hidden transition-all duration-300 ease-linear rounded-md group/gallery card hover:-translate-y-2">
                        <div class="overflow-hidden">
                            <img src="/assets/images/small/img-6.jpg" alt="" class="transition-all duration-300 ease-linear group-hover/gallery:scale-110">
                        </div>
                        <div class="card-body">
                            <h5 class="mb-0 text-16"><a href="#!" class="transition-all duration-300 ease-linear hover:text-custom-500">Cinematic Metro</a></h5>
                        </div>
                    </div>
                    <div class="relative mb-0 overflow-hidden transition-all duration-300 ease-linear rounded-md group/gallery card hover:-translate-y-2">
                        <div class="overflow-hidden">
                            <img src="/assets/images/small/img-7.jpg" alt="" class="transition-all duration-300 ease-linear group-hover/gallery:scale-110">
                        </div>
                        <div class="card-body">
                            <h5 class="mb-0 text-16"><a href="#!" class="transition-all duration-300 ease-linear hover:text-custom-500">Cinematic Metro</a></h5>
                        </div>
                    </div>
                    <div class="relative mb-0 overflow-hidden transition-all duration-300 ease-linear rounded-md group/gallery card hover:-translate-y-2">
                        <div class="overflow-hidden">
                            <img src="/assets/images/small/img-8.jpg" alt="" class="transition-all duration-300 ease-linear group-hover/gallery:scale-110">
                        </div>
                        <div class="card-body">
                            <h5 class="mb-0 text-16"><a href="#!" class="transition-all duration-300 ease-linear hover:text-custom-500">Cinematic Metro</a></h5>
                        </div>
                    </div>
                    <div class="relative mb-0 overflow-hidden transition-all duration-300 ease-linear rounded-md group/gallery card hover:-translate-y-2">
                        <div class="overflow-hidden">
                            <img src="/assets/images/small/img-9.jpg" alt="" class="transition-all duration-300 ease-linear group-hover/gallery:scale-110">
                        </div>
                        <div class="card-body">
                            <h5 class="mb-0 text-16"><a href="#!" class="transition-all duration-300 ease-linear hover:text-custom-500">Cinematic Metro</a></h5>
                        </div>
                    </div>
                    <div class="relative mb-0 overflow-hidden transition-all duration-300 ease-linear rounded-md group/gallery card hover:-translate-y-2">
                        <div class="overflow-hidden">
                            <img src="/assets/images/small/img-10.jpg" alt="" class="transition-all duration-300 ease-linear group-hover/gallery:scale-110">
                        </div>
                        <div class="card-body">
                            <h5 class="mb-0 text-16"><a href="#!" class="transition-all duration-300 ease-linear hover:text-custom-500">Cinematic Metro</a></h5>
                        </div>
                    </div>
                </div><!--end grid-->
            </div>
        </div><!--end card-->

        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">Grid Gallery</h6>
                <div class="grid grid-cols-1 gap-5 md:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-5">
                    <div class="relative overflow-hidden rounded-md group/gallery">
                        <img src="/assets/images/small/img-6.jpg" alt="" class="rounded-md">
                        <div class="absolute inset-0 transition-all duration-300 ease-linear opacity-0 group-hover/gallery:opacity-50 bg-gradient-to-t from-gray-900 to-transparent"></div>
                        <div class="absolute bottom-0 transition-all duration-300 ease-linear opacity-0 left-3 right-3 group-hover/gallery:opacity-100 group-hover/gallery:bottom-3">
                            <h5 class="font-normal text-white"><a href="#!">Image title</a></h5>
                        </div>
                    </div>
                    <div class="relative overflow-hidden rounded-md group/gallery">
                        <img src="/assets/images/small/img-7.jpg" alt="" class="rounded-md">
                        <div class="absolute inset-0 transition-all duration-300 ease-linear opacity-0 group-hover/gallery:opacity-50 bg-gradient-to-t from-gray-900 to-transparent"></div>
                        <div class="absolute bottom-0 transition-all duration-300 ease-linear opacity-0 left-3 right-3 group-hover/gallery:opacity-100 group-hover/gallery:bottom-3">
                            <h5 class="font-normal text-white"><a href="#!">Image title</a></h5>
                        </div>
                    </div>
                    <div class="relative overflow-hidden rounded-md group/gallery">
                        <img src="/assets/images/small/img-8.jpg" alt="" class="rounded-md">
                        <div class="absolute inset-0 transition-all duration-300 ease-linear opacity-0 group-hover/gallery:opacity-50 bg-gradient-to-t from-gray-900 to-transparent"></div>
                        <div class="absolute bottom-0 transition-all duration-300 ease-linear opacity-0 left-3 right-3 group-hover/gallery:opacity-100 group-hover/gallery:bottom-3">
                            <h5 class="font-normal text-white"><a href="#!">Image title</a></h5>
                        </div>
                    </div>
                    <div class="relative overflow-hidden rounded-md group/gallery">
                        <img src="/assets/images/small/img-9.jpg" alt="" class="rounded-md">
                        <div class="absolute inset-0 transition-all duration-300 ease-linear opacity-0 group-hover/gallery:opacity-50 bg-gradient-to-t from-gray-900 to-transparent"></div>
                        <div class="absolute bottom-0 transition-all duration-300 ease-linear opacity-0 left-3 right-3 group-hover/gallery:opacity-100 group-hover/gallery:bottom-3">
                            <h5 class="font-normal text-white"><a href="#!">Image title</a></h5>
                        </div>
                    </div>
                    <div class="relative overflow-hidden rounded-md group/gallery">
                        <img src="/assets/images/small/img-10.jpg" alt="" class="rounded-md">
                        <div class="absolute inset-0 transition-all duration-300 ease-linear opacity-0 group-hover/gallery:opacity-50 bg-gradient-to-t from-gray-900 to-transparent"></div>
                        <div class="absolute bottom-0 transition-all duration-300 ease-linear opacity-0 left-3 right-3 group-hover/gallery:opacity-100 group-hover/gallery:bottom-3">
                            <h5 class="font-normal text-white"><a href="#!">Image title</a></h5>
                        </div>
                    </div>
                    <div class="relative overflow-hidden rounded-md group/gallery">
                        <img src="/assets/images/small/img-1.jpg" alt="" class="rounded-md">
                        <div class="absolute inset-0 transition-all duration-300 ease-linear opacity-0 group-hover/gallery:opacity-50 bg-gradient-to-t from-gray-900 to-transparent"></div>
                        <div class="absolute bottom-0 transition-all duration-300 ease-linear opacity-0 left-3 right-3 group-hover/gallery:opacity-100 group-hover/gallery:bottom-3">
                            <h5 class="font-normal text-white"><a href="#!">Image title</a></h5>
                        </div>
                    </div>
                    <div class="relative overflow-hidden rounded-md group/gallery">
                        <img src="/assets/images/small/img-2.jpg" alt="" class="rounded-md">
                        <div class="absolute inset-0 transition-all duration-300 ease-linear opacity-0 group-hover/gallery:opacity-50 bg-gradient-to-t from-gray-900 to-transparent"></div>
                        <div class="absolute bottom-0 transition-all duration-300 ease-linear opacity-0 left-3 right-3 group-hover/gallery:opacity-100 group-hover/gallery:bottom-3">
                            <h5 class="font-normal text-white"><a href="#!">Image title</a></h5>
                        </div>
                    </div>
                    <div class="relative overflow-hidden rounded-md group/gallery">
                        <img src="/assets/images/small/img-3.jpg" alt="" class="rounded-md">
                        <div class="absolute inset-0 transition-all duration-300 ease-linear opacity-0 group-hover/gallery:opacity-50 bg-gradient-to-t from-gray-900 to-transparent"></div>
                        <div class="absolute bottom-0 transition-all duration-300 ease-linear opacity-0 left-3 right-3 group-hover/gallery:opacity-100 group-hover/gallery:bottom-3">
                            <h5 class="font-normal text-white"><a href="#!">Image title</a></h5>
                        </div>
                    </div>
                    <div class="relative overflow-hidden rounded-md group/gallery">
                        <img src="/assets/images/small/img-4.jpg" alt="" class="rounded-md">
                        <div class="absolute inset-0 transition-all duration-300 ease-linear opacity-0 group-hover/gallery:opacity-50 bg-gradient-to-t from-gray-900 to-transparent"></div>
                        <div class="absolute bottom-0 transition-all duration-300 ease-linear opacity-0 left-3 right-3 group-hover/gallery:opacity-100 group-hover/gallery:bottom-3">
                            <h5 class="font-normal text-white"><a href="#!">Image title</a></h5>
                        </div>
                    </div>
                    <div class="relative overflow-hidden rounded-md group/gallery">
                        <img src="/assets/images/small/img-5.jpg" alt="" class="rounded-md">
                        <div class="absolute inset-0 transition-all duration-300 ease-linear opacity-0 group-hover/gallery:opacity-50 bg-gradient-to-t from-gray-900 to-transparent"></div>
                        <div class="absolute bottom-0 transition-all duration-300 ease-linear opacity-0 left-3 right-3 group-hover/gallery:opacity-100 group-hover/gallery:bottom-3">
                            <h5 class="font-normal text-white"><a href="#!">Image title</a></h5>
                        </div>
                    </div>
                </div><!--end grid-->
            </div>
        </div><!--end card-->
    </div>
</div>
