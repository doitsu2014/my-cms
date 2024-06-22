<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import LucideIcon from '../../../../common/components/LucideIcon.svelte';
	import Collapse from '../../../../common/components/Collapse.svelte';

    let isDefaultOpen = false;
    const toggle = () => isDefaultOpen = !isDefaultOpen;

    let isDefaultOpen2 = false;
    const toggle2 = () => isDefaultOpen2 = !isDefaultOpen2;

    let isDefaultOpen3 = false;
    const toggle3 = () => isDefaultOpen3 = !isDefaultOpen3;
</script>

<HeadTitle title="Collapse" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Collapse" pagetitle="UI Elements" />

    <div>
        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">Default</h6>

                <div class="grid grid-cols-1 gap-5 md:grid-cols-2">
                    <div class="collapsible">
                        <button on:click={toggle} class="flex text-white collapsible-header group/item btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20 {isDefaultOpen ? "show":""}">
                            Collapsible Button
                            <div class="ltr:ml-2 rtl:mr-2 shrink-0">
                                <LucideIcon name="ChevronDown" class="hidden size-4 group-[.show]/item:inline-block"/>
                                <LucideIcon name="ChevronUp" class="inline-block size-4 group-[.show]/item:hidden"/>
                            </div>
                        </button>
                        <Collapse isOpen={isDefaultOpen}>
                            <p>For that very reason, I went on a quest and spoke to many different professional graphic designers and asked them what graphic design tips they live. You've probably heard that opposites attract. The same is true for fonts. Don't be afraid to combine font styles that are different but complementary, like sans serif with serif, short with tall, or decorative with simple. Qui photo booth letterpress, commodo enim craft beer mlkshk aliquip jean shorts ullamco.</p>
                        </Collapse>
                    </div>

                    <div class="collapsible">
                        <button on:click={toggle2} class="flex text-white collapsible-header group/item btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20 {isDefaultOpen2 ? "show":""}">
                            Collapsible Button
                            <div class="ltr:ml-2 rtl:mr-2 shrink-0">
                                <LucideIcon name="ChevronDown" class="hidden size-4 group-[.show]/item:inline-block"/>
                                <LucideIcon name="ChevronUp" class="inline-block size-4 group-[.show]/item:hidden"/>
                            </div>
                        </button>
                        <Collapse isOpen={isDefaultOpen2}>
                            <p>For that very reason, I went on a quest and spoke to many different professional graphic designers and asked them what graphic design tips they live. You've probably heard that opposites attract. The same is true for fonts. Don't be afraid to combine font styles that are different but complementary, like sans serif with serif, short with tall, or decorative with simple. Qui photo booth letterpress, commodo enim craft beer mlkshk aliquip jean shorts ullamco.</p>
                        </Collapse>
                    </div>


                  
                </div>
            </div>
        </div><!--end card-->

        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">Link Collapse</h6>
                <div class="collapsible">
                    <a href="#!" on:click={toggle3} class="flex bg-white border-white collapsible-header group/item text-custom-500 btn hover:text-custom-700 focus:text-custom-700 active:text-custom-700 dark:bg-zink-700 dark:border-zink-700">
                        Collapsible Link
                    </a>
                    <Collapse isOpen={isDefaultOpen3}>
                        <p>For that very reason, I went on a quest and spoke to many different professional graphic designers and asked them what graphic design tips they live. You've probably heard that opposites attract. The same is true for fonts. Don't be afraid to combine font styles that are different but complementary, like sans serif with serif, short with tall, or decorative with simple. Qui photo booth letterpress, commodo enim craft beer mlkshk aliquip jean shorts ullamco.</p>
                    </Collapse>
                </div>
            </div>
        </div>
    </div>
</div>
