<script>
    import HeadTitle from '../../../../common/components/HeadTitle.svelte';
    import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
    import { Lightbox, LightboxGallery, GalleryThumbnail, GalleryImage } from 'svelte-lightbox';
</script>

<HeadTitle title="Lightbox" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
    <Breadcrumb title="Lightbox" pagetitle="UI Elements" />

    <div>
        <div class="grid grid-cols-1 gap-x-5 xl:grid-cols-1">
            <div class="card">
                <div class="card-body">
                    <h6 class="mb-4 text-15">Simple Image Gallery</h6>
                    <!-- svelte-ignore a11y-img-redundant-alt -->
                    <LightboxGallery >
                        <svelte:fragment slot="thumbnail">
                         <div class="grid grid-cols-1 gap-5 md:grid-cols-2 xl:grid-cols-4">
                            <GalleryThumbnail>
                                <img src="/assets/images/small/img-2.jpg" alt="image" class="">
                            </GalleryThumbnail>
                            <GalleryThumbnail>
                            <div>
                                <a href="#!" class="glightbox">
                                    <img src="/assets/images/small/img-3.jpg" alt="image" class="">
                                </a>
                            </div>
                            </GalleryThumbnail>
                            <GalleryThumbnail>
                            <div>
                                <a href="#!" class="glightbox">
                                    <img src="/assets/images/small/img-4.jpg" alt="image" class="">
                                </a>
                            </div>
                            </GalleryThumbnail>
                            <GalleryThumbnail>
                            <div>
                                <a href="#!" class="glightbox">
                                    <img src="/assets/images/small/img-5.jpg" alt="image" class="">
                                </a>
                            </div>
                            </GalleryThumbnail>
                        </div>
                        </svelte:fragment>
                        <GalleryImage>
                            <img src="/assets/images/small/img-2.jpg" alt="Simple lightbox">
                        </GalleryImage>
                        <GalleryImage>
                            <img src="/assets/images/small/img-3.jpg" alt="Simple lightbox">
                        </GalleryImage>
                        <GalleryImage>
                            <img src="/assets/images/small/img-4.jpg" alt="Simple lightbox">
                        </GalleryImage>
                        <GalleryImage>
                            <img src="/assets/images/small/img-5.jpg" alt="Simple lightbox">
                        </GalleryImage>
                    </LightboxGallery>
                </div>
            </div>
            <div class="card">
                <div class="card-body">
                    <h6 class="mb-4 text-15">Videos Gallery</h6>
                    <!-- svelte-ignore a11y-img-redundant-alt -->
                    <LightboxGallery >
                        <svelte:fragment slot="thumbnail">
                         <div class="grid grid-cols-1 gap-5 md:grid-cols-2 xl:grid-cols-4">
                            <GalleryThumbnail>
                                <img src="https://i.ytimg.com/vi/qYgogv4R8zg/hqdefault.jpg?sqp=-oaymwEcCPYBEIoBSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLBcI9Cw0yBtGv_ashbS-ogqh1OGpQ" alt="image" class="object-cover w-full rounded-md h-30">

                            </GalleryThumbnail>
                            <GalleryThumbnail>
                            <div>
                                <a href="#!" class="">
                                    <img src="https://i.ytimg.com/vi/waoOK5s9ypc/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLCkSGeAQ17_LFepTKdEByyVTZkeVw" alt="image" class="object-cover w-full rounded-md h-30">

                                </a>
                            </div>
                            </GalleryThumbnail>
                            <GalleryThumbnail>
                            <div>
                                <a href="#!" class="">
                                    <img src="https://i.ytimg.com/vi/waoOK5s9ypc/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLCkSGeAQ17_LFepTKdEByyVTZkeVw" alt="image" class="object-cover w-full rounded-md h-30">

                                </a>
                            </div>
                            </GalleryThumbnail>
                            <GalleryThumbnail>
                            <div>
                                <a href="#!" class="">
                                    <img src="https://i.ytimg.com/vi/TrftauE2Vyk/hqdefault.jpg?sqp=-oaymwEcCNACELwBSFXyq4qpAw4IARUAAIhCGAFwAcABBg==&rs=AOn4CLCm6UjEfDC3R5dJik1gEW__HEnaAA" alt="image" class="object-cover w-full rounded-md h-30">
                                </a>
                            </div>
                            </GalleryThumbnail>
                        </div>
                        </svelte:fragment>
                        <GalleryImage>
                            <!-- svelte-ignore a11y-missing-attribute -->
                            <iframe src="https://www.youtube.com/embed/qYgogv4R8zg?si=_YxUDmc2fDgHyPae"  class="object-cover w-full rounded-md h-30" frameborder="0"></iframe>
                        </GalleryImage>
                        <GalleryImage>
                            <!-- svelte-ignore a11y-missing-attribute -->
                            <iframe src="https://www.youtube.com/embed/waoOK5s9ypc?si=yzK5f339AF_hoXdn"  class="object-cover w-full rounded-md h-30" frameborder="0"></iframe>
                        </GalleryImage>
                        <!-- svelte-ignore a11y-missing-attribute -->
                        <GalleryImage>
                            <iframe src="https://www.youtube.com/embed/waoOK5s9ypc?si=yzK5f339AF_hoXdn"  class="object-cover w-full rounded-md h-30" frameborder="0"></iframe>
                        </GalleryImage>
                        <GalleryImage>
                            <!-- svelte-ignore a11y-missing-attribute -->
                            <iframe src="https://www.youtube.com/embed/TrftauE2Vyk?si=HRCFmq5956C01yB3"  class="object-cover w-full rounded-md h-30" frameborder="0"></iframe>
                        </GalleryImage>
                    </LightboxGallery>
                </div>
            </div>
        </div>
    </div>
</div>