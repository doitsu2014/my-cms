<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import { onMount } from 'svelte';
	import { register } from 'swiper/element/bundle';
	register();

	onMount(() => {
		const swiperEl = document.querySelector('#default');

		// swiper parameters
		const swiperParams = {
			slidesPerView: 1,
			loop: true,
			pagination: false,
			navigation: false,
			autoplay: {
				duration: 1000
			},
			breakpoints: {}
		};

		// now we need to assign all parameters to Swiper element
		Object.assign(swiperEl, swiperParams);

		// and now initialize it
		swiperEl.initialize();

		/* ------------------------------------------------------------ */
		const swiperEl2 = document.querySelector('#navigation');

		// swiper parameters
		const swiperParams2 = {
			slidesPerView: 1,
			loop: true,
			navigation: true,
			pagination: false,
			autoplay: {
				duration: 1000
			},
			breakpoints: {}
		};

		// now we need to assign all parameters to Swiper element
		Object.assign(swiperEl2, swiperParams2);

		// and now initialize it
		swiperEl2.initialize();

		/* ------------------------------------------------------------ */
		const swiperEl3 = document.querySelector('#pagination');

		// swiper parameters
		const swiperParams3 = {
			slidesPerView: 1,
			loop: true,
			navigation: false,
			pagination: true,
			autoplay: {
				duration: 1000
			},
			breakpoints: {},
			injectStyles: [
				`
            .swiper-pagination-bullets {
                .swiper-pagination-bullet{
                    '@apply bg-custom-500 size-4 border-4 dark:border-zinc-900 outline outline-1 outline-custom-500': {},
                }
            },
            `
			]
		};

		// now we need to assign all parameters to Swiper element
		Object.assign(swiperEl3, swiperParams3);

		// and now initialize it
		swiperEl3.initialize();

		/* ------------------------------------------------------------ */
		const swiperEl4 = document.querySelector('#paginationDynamic');

		// swiper parameters
		const swiperParams4 = {
			slidesPerView: 1,
			loop: false,
			navigation: false,
			pagination: { dynamicBullets: true },
			dynamicBullets: true,
			autoplay: {
				duration: 1000
			},
			breakpoints: {}
		};

		// now we need to assign all parameters to Swiper element
		Object.assign(swiperEl4, swiperParams4);

		// and now initialize it
		swiperEl4.initialize();

		/* ------------------------------------------------------------ */
		const swiperEl5 = document.querySelector('#paginationVertical');

		// swiper parameters
		const swiperParams5 = {
			direction: 'vertical',
			slidesPerView: 1,
			loop: false,
			navigation: false,
			pagination: true,
			autoplay: {
				duration: 1000
			},
			breakpoints: {}
		};

		Object.assign(swiperEl5, swiperParams5);

		swiperEl5.initialize();

		/* ------------------------------------------------------------ */
		const swiperEl6 = document.querySelector('#paginationProgress');

		// swiper parameters
		const swiperParams6 = {
			slidesPerView: 1,
			loop: false,
			autoplay: {
				duration: 1000
			},
			pagination: {
				type: 'progressbar'
			},
			navigation: true
		};

		Object.assign(swiperEl6, swiperParams6);

		swiperEl6.initialize();
	});
</script>

<HeadTitle title="Swiper Slider" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Swiper Slider" pagetitle="UI Elements" />

	<div
		class="px-4 py-3 mb-5 text-sm text-red-500 border border-transparent rounded-md bg-red-50 dark:bg-red-400/20"
	>
		The <a href="https://swiperjs.com/" class="font-semibold underline" target="_blank"
			>Swiper.js documentation</a
		> provides even more options and customization possibilities for creating engaging carousels. Remember
		to adjust image URLs and content according to your project.
	</div>

	<div class="grid grid-cols-1 gap-x-5 lg:grid-cols-2 2xl:grid-cols-3">
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Default</h6>
				<swiper-container init="false" pagination="false" navigation="false" id="default">
					<swiper-slide>
						<img src="/assets/images/small/img-2.jpg" alt="" />
					</swiper-slide>
					<swiper-slide>
						<img src="/assets/images/small/img-3.jpg" alt="" />
					</swiper-slide>
					<swiper-slide>
						<img src="/assets/images/small/img-4.jpg" alt="" />
					</swiper-slide>
					<swiper-slide>
						<img src="/assets/images/small/img-5.jpg" alt="" />
					</swiper-slide>
				</swiper-container>
			</div>
		</div>
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Navigation</h6>
				<swiper-container init="false" pagination="false" navigation="true" id="navigation">
					<swiper-slide>
						<img src="/assets/images/small/img-6.jpg" alt="" />
					</swiper-slide>
					<swiper-slide>
						<img src="/assets/images/small/img-7.jpg" alt="" />
					</swiper-slide>
					<swiper-slide>
						<img src="/assets/images/small/img-8.jpg" alt="" />
					</swiper-slide>
					<swiper-slide>
						<img src="/assets/images/small/img-9.jpg" alt="" />
					</swiper-slide>
				</swiper-container>
			</div>
		</div>
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Pagination</h6>
				<swiper-container init="false" pagination="true" navigation="false" id="pagination">
					<swiper-slide>
						<img src="/assets/images/small/img-6.jpg" alt="" />
					</swiper-slide>
					<swiper-slide>
						<img src="/assets/images/small/img-7.jpg" alt="" />
					</swiper-slide>
					<swiper-slide>
						<img src="/assets/images/small/img-8.jpg" alt="" />
					</swiper-slide>
					<swiper-slide>
						<img src="/assets/images/small/img-9.jpg" alt="" />
					</swiper-slide>
				</swiper-container>
			</div>
		</div>
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Pagination Dynamic</h6>
				<swiper-container init="false" pagination="true" navigation="false" id="paginationDynamic">
					<swiper-slide>
						<img src="/assets/images/small/img-10.jpg" alt="" />
					</swiper-slide>
					<swiper-slide>
						<img src="/assets/images/small/img-11.jpg" alt="" />
					</swiper-slide>
					<swiper-slide>
						<img src="/assets/images/small/img-12.jpg" alt="" />
					</swiper-slide>
				</swiper-container>
			</div>
		</div>
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Vertical</h6>
				<swiper-container
					init="false"
					pagination="true"
					navigation="false"
					id="paginationVertical"
					style="height: 327px;"
				>
					<swiper-slide>
						<img src="/assets/images/small/img-1.jpg" alt="" />
					</swiper-slide>
					<swiper-slide>
						<img src="/assets/images/small/img-2.jpg" alt="" />
					</swiper-slide>
					<swiper-slide>
						<img src="/assets/images/small/img-3.jpg" alt="" />
					</swiper-slide>
				</swiper-container>
			</div>
		</div>
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Pagination progress</h6>
				<swiper-container init="false" pagination="true" navigation="false" id="paginationProgress">
					<swiper-slide>
						<img src="/assets/images/small/img-4.jpg" alt="" />
					</swiper-slide>
					<swiper-slide>
						<img src="/assets/images/small/img-5.jpg" alt="" />
					</swiper-slide>
					<swiper-slide>
						<img src="/assets/images/small/img-6.jpg" alt="" />
					</swiper-slide>
				</swiper-container>
			</div>
		</div>
	</div>
</div>
