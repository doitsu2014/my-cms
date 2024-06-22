<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import LucideIcon from '../../../../common/components/LucideIcon.svelte';
    import {browser} from "$app/environment";

    function getDivFromTheElement(element) {
        if(browser){
            let temp = element.parentNode.querySelector('input.product-quantity');
        
            if (!temp) {
                const upperParent = element.parentNode;
                return getDivFromTheElement(upperParent);
            }
            return temp;

        }
}


function isData() {
    if(browser){
        const plusElements = document.querySelectorAll('button.plusBtn');
        const minusElements = document.querySelectorAll('button.minusBtn');

        if (plusElements.length) {
            plusElements.forEach(element => {
                element.addEventListener('click', (event) => {
                    const inputElement = getDivFromTheElement(event.target);
                    let inputVal = Number(inputElement.value);
                    const maxVal = Number(inputElement.getAttribute('max'));
                    if (inputVal < maxVal) {
                        inputElement.value = inputVal + 1;
                    }
                });
            });
        }

        if (minusElements.length) {
            minusElements.forEach(element => {
                element.addEventListener('click', (event) => {
                    const inputElement = getDivFromTheElement(event.target);
                    let inputVal = Number(inputElement.value);
                    const minVal = Number(inputElement.getAttribute('min'));
                    if (inputVal > minVal) {
                        inputElement.value = inputVal - 1;
                    }
                });
            });
        }
    }
}

isData()
</script>

<HeadTitle title="Input Spin" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Input Spin" pagetitle="Forms" />

	<div class="grid grid-cols-1 gap-x-5 xl:grid-cols-1">
        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">Basic Example</h6>
                <div class="flex flex-wrap gap-4">
                    <div class="inline-flex p-2 text-center border rounded input-step border-slate-200 dark:border-zink-500">
                        <button type="button" class="border w-7 leading-[15px] minusBtn bg-slate-200 dark:bg-zink-600 dark:border-zink-600 rounded transition-all duration-200 ease-linear border-slate-200 text-slate-500 dark:text-zink-200 hover:bg-custom-500 dark:hover:bg-custom-500 hover:text-custom-50 dark:hover:text-custom-50 hover:border-custom-500 dark:hover:border-custom-500 focus:bg-custom-500 dark:focus:bg-custom-500 focus:border-custom-500 dark:focus:border-custom-500 focus:text-custom-50 dark:focus:text-custom-50"><LucideIcon name="Minus" class="inline-block size-4"/></button>
                        <input type="number" class="text-center ltr:pl-2 rtl:pr-2 w-15 h-7 product-quantity dark:bg-zink-700 focus:shadow-none" value="50" min="0" max="100" readonly>
                        <button type="button" class="transition-all duration-200 ease-linear border rounded border-slate-200 bg-slate-200 dark:bg-zink-600 dark:border-zink-600 w-7 plusBtn text-slate-500 dark:text-zink-200 hover:bg-custom-500 dark:hover:bg-custom-500 hover:text-custom-50 dark:hover:text-custom-50 hover:border-custom-500 dark:hover:border-custom-500 focus:bg-custom-500 dark:focus:bg-custom-500 focus:border-custom-500 dark:focus:border-custom-500 focus:text-custom-50 dark:focus:text-custom-50"><LucideIcon name="Plus" class="inline-block size-4"/></button>
                    </div>
                    <div class="inline-flex p-2 text-center border rounded input-step border-slate-200 dark:border-zink-500">
                        <button type="button" class="border w-7 leading-[15px] minusBtn bg-custom-200 dark:bg-custom-900 dark:border-custom-900 rounded transition-all duration-200 ease-linear border-custom-200 text-custom-500  hover:bg-custom-500 dark:hover:bg-custom-500 hover:text-custom-50 hover:border-custom-500 dark:hover:border-custom-500 focus:bg-custom-500 dark:focus:bg-custom-500 focus:border-custom-500 dark:focus:border-custom-500 focus:text-custom-50"><LucideIcon name="Minus" class="inline-block size-4"/></button>
                        <input type="number" class="text-center ltr:pl-2 rtl:pr-2 w-15 h-7 product-quantity dark:bg-zink-700 focus:shadow-none" value="19" min="0" max="100" readonly>
                        <button type="button" class="transition-all duration-200 ease-linear border rounded border-custom-200 bg-custom-200 dark:bg-custom-900 dark:border-custom-900 w-7 plusBtn text-custom-500 hover:bg-custom-500 dark:hover:bg-custom-500 hover:text-custom-50 hover:border-custom-500 dark:hover:border-custom-500 focus:bg-custom-500 dark:focus:bg-custom-500 focus:border-custom-500 dark:focus:border-custom-500 focus:text-custom-50"><LucideIcon name="Plus" class="inline-block size-4"/></button>
                    </div>
                    <div class="inline-flex p-2 text-center border rounded input-step border-slate-200 dark:border-zink-500">
                        <button type="button" class="border w-7 leading-[15px] minusBtn bg-green-200 dark:bg-green-900 dark:border-green-900 rounded transition-all duration-200 ease-linear border-green-200 text-green-500 hover:bg-green-500 dark:hover:bg-green-500 hover:text-green-50 hover:border-green-500 dark:hover:border-green-500 focus:bg-green-500 dark:focus:bg-green-500 focus:border-green-500 dark:focus:border-green-500 focus:text-green-50"><LucideIcon name="Minus" class="inline-block size-4"/></button>
                        <input type="number" class="text-center ltr:pl-2 rtl:pr-2 w-15 h-7 product-quantity dark:bg-zink-700 focus:shadow-none" value="33" min="0" max="100" readonly>
                        <button type="button" class="text-green-500 transition-all duration-200 ease-linear bg-green-200 border border-green-200 rounded dark:bg-green-900 dark:border-green-900 w-7 plusBtn hover:bg-green-500 dark:hover:bg-green-500 hover:text-green-50 hover:border-green-500 dark:hover:border-green-500 focus:bg-green-500 dark:focus:bg-green-500 focus:border-green-500 dark:focus:border-green-500 focus:text-green-50"><LucideIcon name="Plus" class="inline-block size-4"/></button>
                    </div>
                    <div class="inline-flex p-2 text-center border rounded input-step border-slate-200 dark:border-zink-500">
                        <button type="button" class="border w-7 leading-[15px] minusBtn bg-orange-200 dark:bg-orange-900 dark:border-orange-900 rounded transition-all duration-200 ease-linear border-orange-200 text-orange-500 hover:bg-orange-500 dark:hover:bg-orange-500 hover:text-orange-50 hover:border-orange-500 dark:hover:border-orange-500 focus:bg-orange-500 dark:focus:bg-orange-500 focus:border-orange-500 dark:focus:border-orange-500 focus:text-orange-50"><LucideIcon name="Minus" class="inline-block size-4"/></button>
                        <input type="number" class="text-center ltr:pl-2 rtl:pr-2 w-15 h-7 product-quantity dark:bg-zink-700 focus:shadow-none" value="10" min="0" max="100" readonly>
                        <button type="button" class="text-orange-500 transition-all duration-200 ease-linear bg-orange-200 border-orange-200 rounded dark:bg-orange-900 dark:border-orange-900border w-7 plusBtn hover:bg-orange-500 dark:hover:bg-orange-500 hover:text-orange-50 hover:border-orange-500 dark:hover:border-orange-500 focus:bg-orange-500 dark:focus:bg-orange-500 focus:border-orange-500 dark:focus:border-orange-500 focus:text-orange-50"><LucideIcon name="Plus" class="inline-block size-4"/></button>
                    </div>
                    <div class="inline-flex p-2 text-center border rounded input-step border-slate-200 dark:border-zink-500">
                        <button type="button" class="border w-7 leading-[15px] minusBtn bg-sky-200 dark:bg-sky-900 dark:border-sky-900 rounded transition-all duration-200 ease-linear border-sky-200 text-sky-500 hover:bg-sky-500 dark:hover:bg-sky-500 hover:text-sky-50 hover:border-sky-500 dark:hover:border-sky-500 focus:bg-sky-500 dark:focus:bg-sky-500 focus:border-sky-500 dark:focus:border-sky-500 focus:text-sky-50"><LucideIcon name="Minus" class="inline-block size-4"/></button>
                        <input type="number" class="text-center ltr:pl-2 rtl:pr-2 w-15 h-7 product-quantity dark:bg-zink-700 focus:shadow-none" value="25" min="0" max="100" readonly>
                        <button type="button" class="transition-all duration-200 ease-linear border rounded text-sky-500 bg-sky-200 dark:bg-sky-900 dark:border-sky-900 border-sky-200 w-7 plusBtn hover:bg-sky-500 dark:hover:bg-sky-500 hover:text-sky-50 hover:border-sky-500 dark:hover:border-sky-500 focus:bg-sky-500 dark:focus:bg-sky-500 focus:border-sky-500 dark:focus:border-sky-500 focus:text-sky-50"><LucideIcon name="Plus" class="inline-block size-4"/></button>
                    </div>
                    <div class="inline-flex p-2 text-center border rounded input-step border-slate-200 dark:border-zink-500">
                        <button type="button" class="border w-7 leading-[15px] minusBtn bg-yellow-200 dark:bg-yellow-900 dark:border-yellow-900 rounded transition-all duration-200 ease-linear border-yellow-200 text-yellow-500 hover:bg-yellow-500 dark:hover:bg-yellow-500 hover:text-yellow-50 hover:border-yellow-500 dark:hover:border-yellow-500 focus:bg-yellow-500 dark:focus:bg-yellow-500 focus:border-yellow-500 dark:focus:border-yellow-500 focus:text-yellow-50"><LucideIcon name="Minus" class="inline-block size-4"/></button>
                        <input type="number" class="text-center ltr:pl-2 rtl:pr-2 w-15 h-7 product-quantity dark:bg-zink-700 focus:shadow-none" value="10" min="0" max="100" readonly>
                        <button type="button" class="text-yellow-500 transition-all duration-200 ease-linear bg-yellow-200 border border-yellow-200 rounded dark:bg-yellow-900 dark:border-yellow-900 w-7 plusBtn hover:bg-yellow-500 dark:hover:bg-yellow-500 hover:text-yellow-50 hover:border-yellow-500 dark:hover:border-yellow-500 focus:bg-yellow-500 dark:focus:bg-yellow-500 focus:border-yellow-500 dark:focus:border-yellow-500 focus:text-yellow-50"><LucideIcon name="Plus" class="inline-block size-4"/></button>
                    </div>
                    <div class="inline-flex p-2 text-center border rounded input-step border-slate-200 dark:border-zink-500">
                        <button type="button" class="border w-7 leading-[15px] minusBtn bg-red-200 dark:bg-red-900 dark:border-red-900 rounded transition-all duration-200 ease-linear border-red-200 text-red-500 hover:bg-red-500 dark:hover:bg-red-500 hover:text-red-50 hover:border-red-500 dark:hover:border-red-500 focus:bg-red-500 dark:focus:bg-red-500 focus:border-red-500 dark:focus:border-red-500 focus:text-red-50"><LucideIcon name="Minus" class="inline-block size-4"/></button>
                        <input type="number" class="text-center ltr:pl-2 rtl:pr-2 w-15 h-7 product-quantity dark:bg-zink-700 focus:shadow-none" value="18" min="0" max="100" readonly>
                        <button type="button" class="text-red-500 transition-all duration-200 ease-linear bg-red-200 border border-red-200 rounded dark:bg-red-900 dark:border-red-900 w-7 plusBtn hover:bg-red-500 dark:hover:bg-red-500 hover:text-red-50 hover:border-red-500 dark:hover:border-red-500 focus:bg-red-500 dark:focus:bg-red-500 focus:border-red-500 dark:focus:border-red-500 focus:text-red-50"><LucideIcon name="Plus" class="inline-block size-4"/></button>
                    </div>
                    <div class="inline-flex p-2 text-center border rounded input-step border-slate-200 dark:border-zink-500">
                        <button type="button" class="border w-7 leading-[15px] minusBtn bg-purple-200 dark:bg-purple-900 dark:border-purple-900 rounded transition-all duration-200 ease-linear border-purple-200 text-purple-500 hover:bg-purple-500 dark:hover:bg-purple-500 hover:text-purple-50 hover:border-purple-500 dark:hover:border-purple-500 focus:bg-purple-500 dar:focus:bg-purple-500 focus:border-purple-500 dark:focus:border-purple-500 focus:text-purple-50"><LucideIcon name="Minus" class="inline-block size-4"/></button>
                        <input type="number" class="text-center ltr:pl-2 rtl:pr-2 w-15 h-7 product-quantity dark:bg-zink-700 focus:shadow-none" value="18" min="0" max="100" readonly>
                        <button type="button" class="text-purple-500 transition-all duration-200 ease-linear bg-purple-200 border border-purple-200 rounded dark:bg-purple-900 dark:border-purple-900 w-7 plusBtn hover:bg-purple-500 dark:hover:bg-purple-500 hover:text-purple-50 hover:border-purple-500 dark:hover:border-purple-500 focus:bg-purple-500 dar:focus:bg-purple-500 focus:border-purple-500 dark:focus:border-purple-500 focus:text-purple-50"><LucideIcon name="Plus" class="inline-block size-4"/></button>
                    </div>
                </div>
            </div>
        </div><!--end card-->
        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">Group Input Spin Example</h6>
                <div class="flex flex-wrap gap-4">
                    <div class="inline-flex text-center input-step">
                        <button type="button" class="border size-9 leading-[15px] minusBtn bg-white dark:bg-zink-700 dark:border-zink-500 ltr:rounded-l rtl:rounded-r transition-all duration-200 ease-linear border-slate-200 text-slate-500 dark:text-zink-200 hover:bg-custom-500 dark:hover:bg-custom-500 hover:text-custom-50 dark:hover:text-custom-50 hover:border-custom-500 dark:hover:border-custom-500 focus:bg-custom-500 dark:focus:bg-custom-500 focus:border-custom-500 dark:focus:border-custom-500 focus:text-custom-50 dark:focus:text-custom-50"><LucideIcon name="Minus" class="inline-block size-4"/></button>
                        <input type="number" class="w-12 text-center ltr:pl-2 rtl:pr-2 h-9 border-y product-quantity dark:bg-zink-700 focus:shadow-none dark:border-zink-500" value="10" min="0" max="100" readonly>
                        <button type="button" class="transition-all duration-200 ease-linear bg-white border dark:bg-zink-700 dark:border-zink-500 ltr:rounded-r rtl:rounded-l size-9 border-slate-200 plusBtn text-slate-500 dark:text-zink-200 hover:bg-custom-500 dark:hover:bg-custom-500 hover:text-custom-50 dark:hover:text-custom-50 hover:border-custom-500 dark:hover:border-custom-500 focus:bg-custom-500 dark:focus:bg-custom-500 focus:border-custom-500 dark:focus:border-custom-500 focus:text-custom-50 dark:focus:text-custom-50"><LucideIcon name="Plus" class="inline-block size-4"/></button>
                    </div>
                    <div class="inline-flex text-center input-step">
                        <button type="button" class="border size-9 leading-[15px] minusBtn bg-white dark:bg-zink-700 dark:border-zink-500 ltr:rounded-l rtl:rounded-r transition-all duration-200 ease-linear border-slate-200 text-slate-500 dark:text-zink-200 hover:bg-green-500 dark:hover:bg-green-500 hover:text-green-50 dark:hover:text-green-50 hover:border-green-500 dark:hover:border-green-500 focus:bg-green-500 dark:focus:bg-green-500 focus:border-green-500 dark:focus:border-green-500 focus:text-green-50 dark:focus:text-green-50"><LucideIcon name="Minus" class="inline-block size-4"/></button>
                        <input type="number" class="w-12 text-center ltr:pl-2 rtl:pr-2 h-9 border-y product-quantity dark:bg-zink-700 focus:shadow-none dark:border-zink-500" value="20" min="0" max="100" readonly>
                        <button type="button" class="transition-all duration-200 ease-linear bg-white border dark:bg-zink-700 dark:border-zink-500 ltr:rounded-r rtl:rounded-l size-9 border-slate-200 plusBtn text-slate-500 dark:text-zink-200 hover:bg-green-500 dark:hover:bg-green-500 hover:text-green-50 dark:hover:text-green-50 hover:border-green-500 dark:hover:border-green-500 focus:bg-green-500 dark:focus:bg-green-500 focus:border-green-500 dark:focus:border-green-500 focus:text-green-50 dark:focus:text-green-50"><LucideIcon name="Plus" class="inline-block size-4"/></button>
                    </div>
                    <div class="inline-flex text-center input-step">
                        <button type="button" class="border size-9 leading-[15px] minusBtn bg-white dark:bg-zink-700 dark:border-zink-500 ltr:rounded-l rtl:rounded-r transition-all duration-200 ease-linear border-slate-200 text-slate-500 dark:text-zink-200 hover:bg-orange-500 dark:hover:bg-orange-500 hover:text-orange-50 dark:hover:text-orange-50 hover:border-orange-500 dark:hover:border-orange-500 focus:bg-orange-500 dark:focus:bg-orange-500 focus:border-orange-500 dark:focus:border-orange-500 focus:text-orange-50 dark:focus:text-orange-50"><LucideIcon name="Minus" class="inline-block size-4"/></button>
                        <input type="number" class="w-12 text-center ltr:pl-2 rtl:pr-2 h-9 border-y product-quantity dark:bg-zink-700 focus:shadow-none dark:border-zink-500" value="30" min="0" max="100" readonly>
                        <button type="button" class="transition-all duration-200 ease-linear bg-white border dark:bg-zink-700 dark:border-zink-500 ltr:rounded-r rtl:rounded-l size-9 border-slate-200 plusBtn text-slate-500 dark:text-zink-200 hover:bg-orange-500 dark:hover:bg-orange-500 hover:text-orange-50 dark:hover:text-orange-50 hover:border-orange-500 dark:hover:border-orange-500 focus:bg-orange-500 dark:focus:bg-orange-500 focus:border-orange-500 dark:focus:border-orange-500 focus:text-orange-50 dark:focus:text-orange-50"><LucideIcon name="Plus" class="inline-block size-4"/></button>
                    </div>
                    <div class="inline-flex text-center input-step">
                        <button type="button" class="border size-9 leading-[15px] minusBtn bg-white dark:bg-zink-700 dark:border-zink-500 ltr:rounded-l rtl:rounded-r transition-all duration-200 ease-linear border-slate-200 text-slate-500 dark:text-zink-200 hover:bg-sky-500 dark:hover:bg-sky-500 hover:text-sky-50 dark:hover:text-sky-50 hover:border-sky-500 dark:hover:border-sky-500 focus:bg-sky-500 dar:focus:bg-sky-500 focus:border-sky-500 dark:focus:border-sky-500 focus:text-sky-50 dark:focus:text-sky-50"><LucideIcon name="Minus" class="inline-block size-4"/></button>
                        <input type="number" class="w-12 text-center ltr:pl-2 rtl:pr-2 h-9 border-y product-quantity dark:bg-zink-700 focus:shadow-none dark:border-zink-500" value="30" min="0" max="100" readonly>
                        <button type="button" class="transition-all duration-200 ease-linear bg-white border dark:bg-zink-700 dark:border-zink-500 ltr:rounded-r rtl:rounded-l size-9 border-slate-200 plusBtn text-slate-500 dark:text-zink-200 hover:bg-sky-500 dark:hover:bg-sky-500 hover:text-sky-50 dark:hover:text-sky-50 hover:border-sky-500 dark:hover:border-sky-500 focus:bg-sky-500 dar:focus:bg-sky-500 focus:border-sky-500 dark:focus:border-sky-500 focus:text-sky-50 dark:focus:text-sky-50"><LucideIcon name="Plus" class="inline-block size-4"/></button>
                    </div>
                    <div class="inline-flex text-center input-step">
                        <button type="button" class="border size-9 leading-[15px] minusBtn bg-white dark:bg-zink-700 dark:border-zink-500 ltr:rounded-l rtl:rounded-r transition-all duration-200 ease-linear border-slate-200 text-slate-500 dark:text-zink-200 hover:bg-yellow-500 dark:hover:bg-yellow-500 hover:text-yellow-50 dark:hover:text-yellow-50 hover:border-yellow-500 dark:hover:border-yellow-500 focus:bg-yellow-500 dark:focus:bg-yellow-500 focus:border-yellow-500 dark:focus:border-yellow-500 focus:text-yellow-50 dark:focus:text-yellow-50"><LucideIcon name="Minus" class="inline-block size-4"/></button>
                        <input type="number" class="w-12 text-center ltr:pl-2 rtl:pr-2 h-9 border-y product-quantity dark:bg-zink-700 focus:shadow-none dark:border-zink-500" value="40" min="0" max="100" readonly>
                        <button type="button" class="transition-all duration-200 ease-linear bg-white border dark:bg-zink-700 dark:border-zink-500 ltr:rounded-r rtl:rounded-l size-9 border-slate-200 plusBtn text-slate-500 dark:text-zink-200 hover:bg-yellow-500 dark:hover:bg-yellow-500 hover:text-yellow-50 dark:hover:text-yellow-50 hover:border-yellow-500 dark:hover:border-yellow-500 focus:bg-yellow-500 dark:focus:bg-yellow-500 focus:border-yellow-500 dark:focus:border-yellow-500 focus:text-yellow-50 dark:focus:text-yellow-50"><LucideIcon name="Plus" class="inline-block size-4"/></button>
                    </div>
                    <div class="inline-flex text-center input-step">
                        <button type="button" class="border size-9 leading-[15px] minusBtn bg-white dark:bg-zink-700 dark:border-zink-500 ltr:rounded-l rtl:rounded-r transition-all duration-200 ease-linear border-slate-200 text-slate-500 dark:text-zink-200 hover:bg-red-500 dark:hover:bg-red-500 hover:text-red-50 dark:hover:text-red-50 hover:border-red-500 dark:hover:border-red-500 focus:bg-red-500 dark:focus:bg-red-500 focus:border-red-500 dark:focus:border-red-500 focus:text-red-50 dark:focus:text-red-50"><LucideIcon name="Minus" class="inline-block size-4"/></button>
                        <input type="number" class="w-12 text-center ltr:pl-2 rtl:pr-2 h-9 border-y product-quantity dark:bg-zink-700 focus:shadow-none dark:border-zink-500" value="45" min="0" max="100" readonly>
                        <button type="button" class="transition-all duration-200 ease-linear bg-white border dark:bg-zink-700 dark:border-zink-500 ltr:rounded-r rtl:rounded-l size-9 border-slate-200 plusBtn text-slate-500 dark:text-zink-200 hover:bg-red-500 dark:hover:bg-red-500 hover:text-red-50 dark:hover:text-red-50 hover:border-red-500 dark:hover:border-red-500 focus:bg-red-500 dark:focus:bg-red-500 focus:border-red-500 dark:focus:border-red-500 focus:text-red-50 dark:focus:text-red-50"><LucideIcon name="Plus" class="inline-block size-4"/></button>
                    </div>
                    <div class="inline-flex text-center input-step">
                        <button type="button" class="border size-9 leading-[15px] minusBtn bg-white dark:bg-zink-700 dark:border-zink-500 ltr:rounded-l rtl:rounded-r transition-all duration-200 ease-linear border-slate-200 text-slate-500 dark:text-zink-200 hover:bg-purple-500 dark:hover:bg-purple-500 hover:text-purple-50 dark:hover:text-purple-50 hover:border-purple-500 dark:hover:border-purple-500 focus:bg-purple-500 dark:focus:bg-purple-500 focus:border-purple-500 dark:focus:border-purple-500 focus:text-purple-50 dark:focus:text-purple-50"><LucideIcon name="Minus" class="inline-block size-4"/></button>
                        <input type="number" class="w-12 text-center ltr:pl-2 rtl:pr-2 h-9 border-y product-quantity dark:bg-zink-700 focus:shadow-none dark:border-zink-500" value="34" min="0" max="100" readonly>
                        <button type="button" class="transition-all duration-200 ease-linear bg-white border dark:bg-zink-700 dark:border-zink-500 ltr:rounded-r rtl:rounded-l size-9 border-slate-200 plusBtn text-slate-500 dark:text-zink-200 hover:bg-purple-500 dark:hover:bg-purple-500 hover:text-purple-50 dark:hover:text-purple-50 hover:border-purple-500 dark:hover:border-purple-500 focus:bg-purple-500 dark:focus:bg-purple-500 focus:border-purple-500 dark:focus:border-purple-500 focus:text-purple-50 dark:focus:text-purple-50"><LucideIcon name="Plus" class="inline-block size-4"/></button>
                    </div>
                    <div class="inline-flex text-center input-step">
                        <button type="button" class="border size-9 leading-[15px] minusBtn bg-white dark:bg-zink-700 dark:border-zink-500 ltr:rounded-l rtl:rounded-r transition-all duration-200 ease-linear border-slate-200 text-slate-500 dark:text-zink-200 hover:bg-slate-500 dark:hover:bg-zink-500 hover:text-slate-50 dark:hover:text-zink-50 hover:border-slate-500 dark:hover:border-zink-500 focus:bg-slate-500 dark:focus:bg-zink-500 focus:border-slate-500 dark:focus:border-zink-500 focus:text-slate-50 dark:focus:text-zink-50"><LucideIcon name="Minus" class="inline-block size-4"/></button>
                        <input type="number" class="w-12 text-center ltr:pl-2 rtl:pr-2 h-9 border-y product-quantity dark:bg-zink-700 focus:shadow-none dark:border-zink-500" value="55" min="0" max="100" readonly>
                        <button type="button" class="transition-all duration-200 ease-linear bg-white border dark:bg-zink-700 dark:border-zink-500 ltr:rounded-r rtl:rounded-l size-9 border-slate-200 plusBtn text-slate-500 dark:text-zink-200 hover:bg-slate-500 dark:hover:bg-zink-500 hover:text-slate-50 dark:hover:text-zink-50 hover:border-slate-500 dark:hover:border-zink-500 focus:bg-slate-500 dark:focus:bg-zink-500 focus:border-slate-500 dark:focus:border-zink-500 focus:text-slate-50 dark:focus:text-zink-50"><LucideIcon name="Plus" class="inline-block size-4"/></button>
                    </div>
                </div>
            </div>
        </div><!--end card-->
        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">Rounded Example</h6>
                <div class="flex flex-wrap items-center gap-4">
                    <div class="inline-flex text-center input-step">
                        <button type="button" class="border w-7 leading-[15px] minusBtn bg-slate-200 dark:bg-zink-600 dark:border-zink-600 rounded-full transition-all duration-200 ease-linear border-slate-200 text-slate-500 dark:text-zink-200 hover:bg-custom-500 dark:hover:bg-custom-500 hover:text-custom-50 dark:hover:text-custom-50 hover:border-custom-500 focus:bg-custom-500 focus:border-custom-500 focus:text-custom-50"><LucideIcon name="Minus" class="inline-block size-4"/></button>
                        <input type="number" class="text-center ltr:pl-2 rtl:pr-2 w-15 h-7 product-quantity dark:bg-zink-700 focus:shadow-none" value="50" min="0" max="100" readonly>
                        <button type="button" class="transition-all duration-200 ease-linear border rounded-full border-slate-200 dark:bg-zink-600 dark:border-zink-600 bg-slate-200 w-7 plusBtn text-slate-500 dark:text-zink-200 hover:bg-custom-500 dark:hover:bg-custom-500 hover:text-custom-50 dark:hover:text-custom-50 hover:border-custom-500 focus:bg-custom-500 focus:border-custom-500 focus:text-custom-50"><LucideIcon name="Plus" class="inline-block size-4"/></button>
                    </div>
                    <div class="inline-flex text-center input-step">
                        <button type="button" class="border w-7 leading-[15px] minusBtn bg-custom-200 dark:bg-custom-900 dark:border-custom-900 rounded-full transition-all duration-200 ease-linear border-custom-200 text-custom-500  hover:bg-custom-500 dark:hover:bg-custom-500 hover:text-custom-50 hover:border-custom-500 dark:hover:border-custom-500 focus:bg-custom-500 dark:focus:bg-custom-500 focus:border-custom-500 dark:focus:border-custom-500 focus:text-custom-50"><LucideIcon name="Minus" class="inline-block size-4"/></button>
                        <input type="number" class="text-center ltr:pl-2 rtl:pr-2 w-15 h-7 product-quantity dark:bg-zink-700 focus:shadow-none" value="19" min="0" max="100" readonly>
                        <button type="button" class="border w-7 leading-[15px] minusBtn bg-custom-200 dark:bg-custom-900 dark:border-custom-900 rounded-full transition-all duration-200 ease-linear border-custom-200 text-custom-500  hover:bg-custom-500 dark:hover:bg-custom-500 hover:text-custom-50 hover:border-custom-500 dark:hover:border-custom-500 focus:bg-custom-500 dark:focus:bg-custom-500 focus:border-custom-500 dark:focus:border-custom-500 focus:text-custom-50"><LucideIcon name="Plus" class="inline-block size-4"/></button>
                    </div>
                    <div class="inline-flex text-center input-step">
                        <button type="button" class="border w-7 leading-[15px] minusBtn bg-green-200 dark:bg-green-900 dark:border-green-900 rounded-full transition-all duration-200 ease-linear border-green-200 text-green-500 hover:bg-green-500 dark:hover:bg-green-500 hover:text-green-50 hover:border-green-500 dark:hover:border-green-500 focus:bg-green-500 dark:focus:bg-green-500 focus:border-green-500 dark:focus:border-green-500 focus:text-green-50"><LucideIcon name="Minus" class="inline-block size-4"/></button>
                        <input type="number" class="text-center ltr:pl-2 rtl:pr-2 w-15 h-7 product-quantity dark:bg-zink-700 focus:shadow-none" value="33" min="0" max="100" readonly>
                        <button type="button" class="text-green-500 transition-all duration-200 ease-linear bg-green-200 border border-green-200 rounded-full dark:bg-green-900 dark:border-green-900 w-7 plusBtn hover:bg-green-500 dark:hover:bg-green-500 hover:text-green-50 hover:border-green-500 dark:hover:border-green-500 focus:bg-green-500 dark:focus:bg-green-500 focus:border-green-500 dark:focus:border-green-500 focus:text-green-50"><LucideIcon name="Plus" class="inline-block size-4"/></button>
                    </div>
                    <div class="inline-flex text-center input-step">
                        <button type="button" class="border w-7 leading-[15px] minusBtn bg-orange-200 dark:bg-orange-900 dark:border-orange-900 rounded-full transition-all duration-200 ease-linear border-orange-200 text-orange-500 hover:bg-orange-500 dark:hover:bg-orange-500 hover:text-orange-50 hover:border-orange-500 dark:hover:border-orange-500 focus:bg-orange-500 dark:focus:bg-orange-500 focus:border-orange-500 dark:focus:border-orange-500 focus:text-orange-50"><LucideIcon name="Minus" class="inline-block size-4"/></button>
                        <input type="number" class="text-center ltr:pl-2 rtl:pr-2 w-15 h-7 product-quantity dark:bg-zink-700 focus:shadow-none" value="10" min="0" max="100" readonly>
                        <button type="button" class="text-orange-500 transition-all duration-200 ease-linear bg-orange-200 border-orange-200 rounded-full dark:bg-orange-900 dark:border-orange-900border w-7 plusBtn hover:bg-orange-500 dark:hover:bg-orange-500 hover:text-orange-50 hover:border-orange-500 dark:hover:border-orange-500 focus:bg-orange-500 dark:focus:bg-orange-500 focus:border-orange-500 dark:focus:border-orange-500 focus:text-orange-50"><LucideIcon name="Plus" class="inline-block size-4"/></button>
                    </div>
                    <div class="inline-flex text-center input-step">
                        <button type="button" class="border size-9 leading-[15px] minusBtn bg-white dark:bg-zink-700 dark:border-zink-500 ltr:rounded-l-full rtl:rounded-r-full transition-all duration-200 ease-linear border-slate-200 text-slate-500 dark:text-zink-200 hover:bg-yellow-500 dark:hover:bg-yellow-500 hover:text-yellow-50 dark:hover:text-yellow-50 hover:border-yellow-500 dark:hover:border-yellow-500 focus:bg-yellow-500 dark:focus:bg-yellow-500 focus:border-yellow-500 dark:focus:border-yellow-500 focus:text-yellow-50 dark:focus:text-yellow-50"><LucideIcon name="Minus" class="inline-block size-4"/></button>
                        <input type="number" class="w-12 text-center ltr:pl-2 rtl:pr-2 h-9 border-y product-quantity dark:bg-zink-700 focus:shadow-none dark:border-zink-500" value="40" min="0" max="100" readonly>
                        <button type="button" class="transition-all duration-200 ease-linear bg-white border dark:bg-zink-700 dark:border-zink-500 ltr:rounded-r-full rtl:rounded-l-full size-9 border-slate-200 plusBtn text-slate-500 dark:text-zink-200 hover:bg-yellow-500 dark:hover:bg-yellow-500 hover:text-yellow-50 dark:hover:text-yellow-50 hover:border-yellow-500 dark:hover:border-yellow-500 focus:bg-yellow-500 dark:focus:bg-yellow-500 focus:border-yellow-500 dark:focus:border-yellow-500 focus:text-yellow-50 dark:focus:text-yellow-50"><LucideIcon name="Plus" class="inline-block size-4"/></button>
                    </div>
                    <div class="inline-flex text-center input-step">
                        <button type="button" class="border size-9 leading-[15px] minusBtn bg-white dark:bg-zink-700 dark:border-zink-500 ltr:rounded-l-full rtl:rounded-r-full transition-all duration-200 ease-linear border-slate-200 text-slate-500 dark:text-zink-200 hover:bg-red-500 dark:hover:bg-red-500 hover:text-red-50 dark:hover:text-red-50 hover:border-red-500 dark:hover:border-red-500 focus:bg-red-500 dark:focus:bg-red-500 focus:border-red-500 dark:focus:border-red-500 focus:text-red-50 dark:focus:text-red-50"><LucideIcon name="Minus" class="inline-block size-4"/></button>
                        <input type="number" class="w-12 text-center ltr:pl-2 rtl:pr-2 h-9 border-y product-quantity dark:bg-zink-700 focus:shadow-none dark:border-zink-500" value="45" min="0" max="100" readonly>
                        <button type="button" class="transition-all duration-200 ease-linear bg-white border dark:bg-zink-700 dark:border-zink-500 ltr:rounded-r-full rtl:rounded-l-full size-9 border-slate-200 plusBtn text-slate-500 dark:text-zink-200 hover:bg-red-500 dark:hover:bg-red-500 hover:text-red-50 dark:hover:text-red-50 hover:border-red-500 dark:hover:border-red-500 focus:bg-red-500 dark:focus:bg-red-500 focus:border-red-500 dark:focus:border-red-500 focus:text-red-50 dark:focus:text-red-50"><LucideIcon name="Plus" class="inline-block size-4"/></button>
                    </div>
                    <div class="inline-flex text-center input-step">
                        <button type="button" class="border size-9 leading-[15px] minusBtn bg-white dark:bg-zink-700 dark:border-zink-500 ltr:rounded-l-full rtl:rounded-r-full transition-all duration-200 ease-linear border-slate-200 text-slate-500 dark:text-zink-200 hover:bg-purple-500 dark:hover:bg-purple-500 hover:text-purple-50 dark:hover:text-purple-50 hover:border-purple-500 dark:hover:border-purple-500 focus:bg-purple-500 dark:focus:bg-purple-500 focus:border-purple-500 dark:focus:border-purple-500 focus:text-purple-50 dark:focus:text-purple-50"><LucideIcon name="Minus" class="inline-block size-4"/></button>
                        <input type="number" class="w-12 text-center ltr:pl-2 rtl:pr-2 h-9 border-y product-quantity dark:bg-zink-700 focus:shadow-none dark:border-zink-500" value="34" min="0" max="100" readonly>
                        <button type="button" class="transition-all duration-200 ease-linear bg-white border dark:bg-zink-700 dark:border-zink-500 ltr:rounded-r-full rtl:rounded-l-full size-9 border-slate-200 plusBtn text-slate-500 dark:text-zink-200 hover:bg-purple-500 dark:hover:bg-purple-500 hover:text-purple-50 dark:hover:text-purple-50 hover:border-purple-500 dark:hover:border-purple-500 focus:bg-purple-500 dark:focus:bg-purple-500 focus:border-purple-500 dark:focus:border-purple-500 focus:text-purple-50 dark:focus:text-purple-50"><LucideIcon name="Plus" class="inline-block size-4"/></button>
                    </div>
                    <div class="inline-flex text-center input-step">
                        <button type="button" class="border size-9 leading-[15px] minusBtn bg-white dark:bg-zink-700 dark:border-zink-500 ltr:rounded-l-full rtl:rounded-r-full transition-all duration-200 ease-linear border-slate-200 text-slate-500 dark:text-zink-200 hover:bg-slate-500 dark:hover:bg-zink-500 hover:text-slate-50 dark:hover:text-zink-50 hover:border-slate-500 dark:hover:border-zink-500 focus:bg-slate-500 dark:focus:bg-zink-500 focus:border-slate-500 dark:focus:border-zink-500 focus:text-slate-50 dark:focus:text-zink-50"><LucideIcon name="Minus" class="inline-block size-4"/></button>
                        <input type="number" class="w-12 text-center ltr:pl-2 rtl:pr-2 h-9 border-y product-quantity dark:bg-zink-700 focus:shadow-none dark:border-zink-500" value="55" min="0" max="100" readonly>
                        <button type="button" class="transition-all duration-200 ease-linear bg-white border dark:bg-zink-700 dark:border-zink-500 ltr:rounded-r-full rtl:rounded-l-full size-9 border-slate-200 plusBtn text-slate-500 dark:text-zink-200 hover:bg-slate-500 dark:hover:bg-zink-500 hover:text-slate-50 dark:hover:text-zink-50 hover:border-slate-500 dark:hover:border-zink-500 focus:bg-slate-500 dark:focus:bg-zink-500 focus:border-slate-500 dark:focus:border-zink-500 focus:text-slate-50 dark:focus:text-zink-50"><LucideIcon name="Plus" class="inline-block size-4"/></button>
                    </div>
                </div>
            </div>
        </div><!--end card-->
    </div>
</div>
