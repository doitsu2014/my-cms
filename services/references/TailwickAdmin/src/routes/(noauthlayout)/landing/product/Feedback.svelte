<script>
	import { onMount } from 'svelte';
    import { register } from 'swiper/element/bundle';
    register();

  onMount(() => {
    const swiperEl = document.querySelector('swiper-container');

  // swiper parameters
  const swiperParams = {
    slidesPerView: 3,
    loop: true,
    autoplay: {
        duration: 1000
    },
    breakpoints: {
        768: {
            slidesPerView: 2,
            spaceBetween: 40,
        },
        1024: {
            slidesPerView: 3,
            spaceBetween: 50,
        },
    },
  };

  // now we need to assign all parameters to Swiper element
  Object.assign(swiperEl, swiperParams);

  // and now initialize it
  swiperEl.initialize();
  })
  </script>

<section class="relative py-24 xl:py-32" id="feedback">
    <div class="container 2xl:max-w-[87.5rem] px-4 mx-auto">
        <div class="mx-auto mb-8 text-center xl:max-w-3xl">
            <h1 class="mb-0 leading-normal capitalize">What Our Customer Says</h1>
        </div>
        <!-- Swiper -->
        <swiper-container init="false" pagination="false" slides-per-view="3">
            <swiper-slide>
                <div class="p-5 text-center" data-aos="fade-up" data-aos-easing="linear">
                    <div class="size-20 mx-auto rounded-full bg-custom-500/10">
                        <img src="/assets/images/users/avatar-2.png" alt="" class="size-20 rounded-full">
                    </div>
                    <p class="mt-6 text-16">" The best templates which is supported multiple programming languages with beautiful templates. thank you for the valuable template. "</p>
                    <h6 class="mt-4 mb-1 text-15">Angela Ulligan</h6>
                    <div class="text-yellow-500">
                        <i class="ri-star-fill"></i>
                        <i class="ri-star-fill"></i>
                        <i class="ri-star-fill"></i>
                        <i class="ri-star-fill"></i>
                        <i class="ri-star-fill"></i>
                    </div>
                </div>
            </swiper-slide>
            <swiper-slide>
                <div class="p-5 text-center" data-aos="fade-up" data-aos-easing="linear">
                    <div class="size-20 mx-auto rounded-full bg-yellow-500/10">
                        <img src="/assets/images/users/avatar-4.png" alt="" class="size-20 rounded-full">
                    </div>
                    <p class="mt-6 text-16">" I encountered a few errors in the design of the product detail page in Angular. I contacted the support team and they established. "</p>
                    <h6 class="mt-4 mb-1 text-15"><a href="#!">muratoztrkk01</a></h6>
                    <div class="text-yellow-500">
                        <i class="ri-star-fill"></i>
                        <i class="ri-star-fill"></i>
                        <i class="ri-star-fill"></i>
                        <i class="ri-star-fill"></i>
                        <i class="ri-star-fill"></i>
                    </div>
                </div>
            </swiper-slide>
            <swiper-slide>
                <div class="p-5 text-center" data-aos="fade-up" data-aos-easing="linear">
                    <div class="size-20 mx-auto rounded-full bg-red-500/10">
                        <img src="/assets/images/users/avatar-7.png" alt="" class="size-20 rounded-full">
                    </div>
                    <p class="mt-6 text-16">" This theme is very good. I really recommend it. It's very good optimized, elegant, well documented, etc. "</p>
                    <h6 class="mt-4 mb-1 text-15"><a href="#!">Christine Marbury</a></h6>
                    <div class="text-yellow-500">
                        <i class="ri-star-fill"></i>
                        <i class="ri-star-fill"></i>
                        <i class="ri-star-fill"></i>
                        <i class="ri-star-fill"></i>
                        <i class="ri-star-fill"></i>
                    </div>
                </div>
            </swiper-slide>
            <swiper-slide>
                <div class="swiper-slide">
                    <div class="p-5 text-center" data-aos="fade-up" data-aos-easing="linear">
                        <div class="size-20 mx-auto rounded-full bg-purple-500/10">
                            <img src="/assets/images/users/avatar-9.png" alt="" class="size-20 rounded-full">
                        </div>
                        <p class="mt-6 text-16">" ThemesDesign used Anydesk to fix the bug in Flask and django version. I highly recommend this product! "</p>
                        <h6 class="mt-4 mb-1 text-15"><a href="#!">Anthony Hobbs</a></h6>
                        <div class="text-yellow-500">
                            <i class="ri-star-fill"></i>
                            <i class="ri-star-fill"></i>
                            <i class="ri-star-fill"></i>
                            <i class="ri-star-fill"></i>
                            <i class="ri-star-fill"></i>
                        </div>
                    </div>
                </div>
            </swiper-slide>
          </swiper-container>
    </div><!--end container-->

   
</section>