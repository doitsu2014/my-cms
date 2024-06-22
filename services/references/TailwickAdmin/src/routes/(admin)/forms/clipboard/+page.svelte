<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import LucideIcon from '../../../../common/components/LucideIcon.svelte';
    let copy = "Welcome to Tailwick😊";
    let cut = "Welcome to Tailwick😊";

    const copyText = () => {
        navigator.clipboard.writeText(copy);

        // Alert the copied text
        alert("Copied the text: " + copy);
    }

    const cutText = () => {
        navigator.clipboard.writeText(cut);

        // Alert the copied text
        alert("Cut the text: " + cut);
        cut ='';
    }
</script>

<HeadTitle title="Clipboard" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Clipboard" pagetitle="Forms" />

    <div class="card">
        <div class="card-body">
            <h6 class="mb-4 text-15">Basic Clipboard Copy</h6>
            <div class="flex">
                <input type="text" id="clipboard" bind:value={copy} aria-describedby="button-addon2" class="ltr:rounded-r-none rtl:rounded-l-none form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="">
                <button type="button" id="copyButton" class="flex items-center justify-center w-[39px] h-[39px] ltr:rounded-l-none rtl:rounded-r-none p-0 text-slate-500 btn bg-slate-200 border-slate-200 hover:text-slate-600 hover:bg-slate-300 hover:border-slate-300 focus:text-slate-600 focus:bg-slate-300 focus:border-slate-300 focus:ring focus:ring-slate-100 active:text-slate-600 active:bg-slate-300 active:border-slate-300 active:ring active:ring-slate-100 dark:bg-zink-600 dark:hover:bg-zink-500 dark:border-zink-600 dark:hover:border-zink-500 dark:text-zink-200 dark:ring-zink-400/50" on:click={copyText}>
                    <LucideIcon name="ClipboardList" class="inline-block size-4"/>
                </button>
            </div>
        </div>
    </div><!--end card-->
    <div class="card">
        <div class="card-body">
            <h6 class="mb-4 text-15">Basic Clipboard Cut</h6>
            <div class="flex">
                <input type="text" id="clipboard1" bind:value={cut}  aria-describedby="button-addon" class="ltr:rounded-r-none rtl:rounded-l-none form-input border-slate-200 dark:border-zink-500 focus:outline-none focus:border-custom-500 disabled:bg-slate-100 dark:disabled:bg-zink-600 disabled:border-slate-300 dark:disabled:border-zink-500 dark:disabled:text-zink-200 disabled:text-slate-500 dark:text-zink-100 dark:bg-zink-700 dark:focus:border-custom-800 placeholder:text-slate-400 dark:placeholder:text-zink-200" placeholder="">
                <button type="button" id="cutButton" class="flex items-center justify-center w-[39px] h-[39px] ltr:rounded-l-none rtl:rounded-r-none p-0 text-slate-500 btn bg-slate-200 border-slate-200 hover:text-slate-600 hover:bg-slate-300 hover:border-slate-300 focus:text-slate-600 focus:bg-slate-300 focus:border-slate-300 focus:ring focus:ring-slate-100 active:text-slate-600 active:bg-slate-300 active:border-slate-300 active:ring active:ring-slate-100 dark:bg-zink-600 dark:hover:bg-zink-500 dark:border-zink-600 dark:hover:border-zink-500 dark:text-zink-200 dark:ring-zink-400/50" on:click={cutText}>
                    <LucideIcon name="Scissors" class="inline-block size-4"/>
                </button>
            </div>
        </div>
    </div><!--end card-->
</div>
