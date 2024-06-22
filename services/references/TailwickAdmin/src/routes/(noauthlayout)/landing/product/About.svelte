<script>
	import LucideIcon from "../../../../common/components/LucideIcon.svelte";
</script>

<section class="relative py-24 xl:py-32" id="about">
    <div class="absolute bottom-0 right-0 size-64 bg-custom-500/10 blur-3xl"></div>
    <div class="container 2xl:max-w-[87.5rem] px-4 mx-auto">
        <div class="grid items-center grid-cols-1 gap-6 mt-20 lg:grid-cols-12">
            <div class="relative lg:col-span-5">
                <div class="relative before:absolute before:h-full before:w-full before:border-[15px] before:border-double before:border-green-500/10 before:-top-16 lg:before:-right-16" data-aos="zoom-out-up">
                    <img src="/assets/images/landing/about.jpg" alt="" class="relative inline-block rounded-md" data-aos="zoom-out-up" data-aos-delay="500">
                </div>
            </div><!--end col-->
            <div class="ml-auto lg:col-span-5 lg:col-start-8">
                <p class="mb-2 text-purple-500 text-15" data-aos="fade-left" data-aos-delay="300">About Us</p>
                <h1 class="mb-3 leading-normal capitalize" data-aos="fade-left" data-aos-delay="400">We Provide high Quality shoes</h1>
                <p class="mb-5 text-lg text-slate-500 dark:text-zinc-400" data-aos="fade-left" data-aos-delay="500">Look for a shoe with solid construction that will give your feet the support they need. Next, look for quality materials that will make your feet comfortable and keep them healthy.</p>
                <p class="mb-5 text-lg text-slate-500 dark:text-zinc-400" data-aos="fade-left" data-aos-delay="500">Low-quality shoes may skimp on stitching, or use low quality glue that's prone to coming apart. A higher-quality shoe will use advanced construction to ensure that the shoe holds up over time, and also eliminate any spots.</p>
                <button type="button" class="px-8 py-3 text-white border-0 text-15 btn bg-gradient-to-r from-custom-500 to-purple-500 hover:text-white hover:from-purple-500 hover:to-custom-500" data-aos="fade-left" data-aos-delay="600">Explore Now 
                    <LucideIcon name="MoveRight" class="inline-block size-4 align-middle rtl:mr-1 ltr:ml-1"/></button>
            </div><!--end col-->
        </div><!--end grid-->
    </div><!--end container-->
</section>