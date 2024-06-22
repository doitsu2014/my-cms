<script>
	import HeadTitle from '../../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../../common/components/Breadcrumb.svelte';
	import LucideIcon from '../../../../../common/components/LucideIcon.svelte';
	import Modal from '../../../../../common/components/Modal.svelte';
	import data from '../../../../../common/data/cart';

	let isDeleteModal = false;
	const toggleDelete = () => (isDeleteModal = !isDeleteModal);

	let taxRate = 0.125;
	let shippingRate = 65.0;
	let discountRate = 0.15;
	let subtotal = 239.97;
	let tax = 30;
	let discount = 36;
	let shipping = 65;
	let total = 298.97;

	const plusItem = (product) => {
		console.log('product', product);
		for (let item of data.cartlist) {
			if (item.id === product.id) {
				product.quantity += 1;
				product.total = (product.quantity * parseFloat(product.current)).toFixed(2);
				data.cartlist = data.cartlist;
				calculateCart();
				return;
			}
		}
	};

	const minusItem = (product) => {
		for (let item of data.cartlist) {
			if (item.id === product.id) {
				if (product.quantity > 0) {
					product.quantity -= 1;
					product.total = (product.quantity * parseFloat(product.current)).toFixed(2);
					data.cartlist = data.cartlist;
				}
				calculateCart();
				return;
			}
		}
	};

	const calculateCart = () => {
		subtotal = 0;
		for (let item1 of data.cartlist) {
			if (!Number.isNaN(parseFloat(item1.total))) {
				subtotal += parseFloat(item1.total);
			}
		}

		subtotal = subtotal.toFixed(2);
		/* Calculate totals */
		tax = (subtotal * taxRate).toFixed(2);
		discount = (subtotal * discountRate).toFixed(2);

		shipping = (subtotal > 0 ? shippingRate : 0).toFixed(2);
		total = parseFloat(subtotal) + parseFloat(tax) + parseFloat(shipping) - parseFloat(discount);
		total = total.toFixed(2);
	};
</script>

<HeadTitle title="Shopping Cart" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto relative">
	<Breadcrumb title="Shopping Cart" pagetitle="Ecommerce" />

	<div class="grid grid-cols-1 xl:grid-cols-12 gap-x-5">
		<div class="xl:col-span-9 products-list">
			<div class="flex items-center gap-3 mb-5">
				<h5 class="underline text-16 grow">Shopping Cart (3)</h5>
				<div>
					<a
						href="#!"
						class="text-red-500 transition-all duration-300 ease-linear hover:text-red-600"
					>
						<LucideIcon name="Trash2" class="inline-block size-4 mr-1 align-middle" />
						<span class="align-middle">Delete All</span></a
					>
				</div>
			</div>
			{#each data.cartlist as item}
				<div class="card products">
					<div class="card-body">
						<div class="grid grid-cols-1 gap-4 lg:grid-cols-12">
							<div class="p-4 rounded-md lg:col-span-2 bg-slate-100 dark:bg-zink-600">
								<img src={item.img} alt="" />
							</div>
							<!--end col-->
							<div class="flex flex-col lg:col-span-4">
								<div>
									<h5 class="mb-1 text-16">
										<a href="/apps/ecommerce/products-overview">{item.name}</a>
									</h5>
									<p class="mb-2 text-slate-500 dark:text-zink-200">
										<a href="#!">{item.category}</a>
									</p>
									<p class="mb-1 text-slate-500 dark:text-zink-200">
										Color: <span class="text-slate-800 dark:text-zink-50">{item.color}</span>
									</p>
									<p class="mb-3 text-slate-500 dark:text-zink-200">
										Size: <span class="text-slate-800 dark:text-zink-50">{item.size}</span>
									</p>
								</div>
								<div class="flex items-center gap-2 mt-auto">
									<div
										class="inline-flex p-2 text-center border rounded input-step border-slate-200 dark:border-zink-500"
									>
										<button
											type="button"
											class="border w-7 leading-[15px] minus-value bg-slate-200 dark:bg-zink-600 dark:border-zink-600 rounded transition-all duration-200 ease-linear border-slate-200 text-slate-500 dark:text-zink-200 hover:bg-custom-500 dark:hover:bg-custom-500 hover:text-custom-50 dark:hover:text-custom-50 hover:border-custom-500 dark:hover:border-custom-500 focus:bg-custom-500 dark:focus:bg-custom-500 focus:border-custom-500 dark:focus:border-custom-500 focus:text-custom-50 dark:focus:text-custom-50"
											on:click={() => minusItem(item)}
											><LucideIcon name="Minus" class="inline-block size-4" /></button
										>
										<input
											type="number"
											class="text-center ltr:pl-2 rtl:pr-2 w-15 h-7 products-quantity dark:bg-zink-700 focus:shadow-none"
											value={item.quantity}
											min="0"
											max="100"
											readonly
										/>
										<button
											type="button"
											class="transition-all duration-200 ease-linear border rounded border-slate-200 bg-slate-200 dark:bg-zink-600 dark:border-zink-600 w-7 plus-value text-slate-500 dark:text-zink-200 hover:bg-custom-500 dark:hover:bg-custom-500 hover:text-custom-50 dark:hover:text-custom-50 hover:border-custom-500 dark:hover:border-custom-500 focus:bg-custom-500 dark:focus:bg-custom-500 focus:border-custom-500 dark:focus:border-custom-500 focus:text-custom-50 dark:focus:text-custom-50"
											on:click={() => plusItem(item)}
											><LucideIcon name="Plus" class="inline-block size-4" /></button
										>
									</div>
									<button
										type="button"
										class="flex items-center justify-center size-[37.5px] p-0 text-red-500 bg-red-100 btn hover:text-white hover:bg-red-600 focus:text-white focus:bg-red-600 focus:ring focus:ring-red-100 active:text-white active:bg-red-600 active:ring active:ring-red-100 dark:bg-red-500/20 dark:text-red-500 dark:hover:bg-red-500 dark:hover:text-white dark:focus:bg-red-500 dark:focus:text-white dark:active:bg-red-500 dark:active:text-white dark:ring-red-400/20 remove-button"
										on:click={toggleDelete}><LucideIcon name="Trash2" class="size-4" /></button
									>
								</div>
							</div>
							<!--end col-->
							<div class="flex justify-between w-full lg:flex-col lg:col-end-13 lg:col-span-2">
								<div class="mb-auto ltr:lg:text-right rtl:lg:text-left">
									<h6 class="text-16 products-price">
										$<span>{item.current}</span>
										<small class="font-normal line-through text-slate-500 dark:text-zink-200"
											>${item.original}</small
										>
									</h6>
								</div>
								<h6 class="mt-auto text-16 ltr:lg:text-right rtl:lg:text-left">
									$<span class="products-line-price">{item.total}</span>
								</h6>
							</div>
							<!--end col-->
						</div>
						<!--end grid-->
					</div>
				</div>
				<!--end card-->
			{/each}
		</div>
		<!--end col-->
		<div class="xl:col-span-3">
			<div class="sticky top-[calc(theme('spacing.header')_*_1.3)] mb-5">
				<div class="card">
					<div class="card-body">
						<h6 class="mb-4 text-15">Order Summary</h6>
						<div class="overflow-x-auto">
							<table class="w-full">
								<tbody class="table-total">
									<tr>
										<td class="py-2 text-slate-500 dark:text-zink-200"> Sub Total </td>
										<td class="py-2 text-slate-500 dark:text-zink-200 cart-subtotal">
											${subtotal}
										</td>
									</tr>
									<tr>
										<td class="py-2 text-slate-500 dark:text-zink-200"> Estimated Tax (18%) </td>
										<td class="py-2 text-slate-500 dark:text-zink-200 cart-tax">
											${tax}
										</td>
									</tr>
									<tr>
										<td class="py-2 text-slate-500 dark:text-zink-200"> Item Discounts (12%) </td>
										<td class="py-2 text-slate-500 dark:text-zink-200 cart-discount">
											-${discount}
										</td>
									</tr>
									<tr>
										<td class="py-2 text-slate-500 dark:text-zink-200"> Shipping Charge </td>
										<td class="py-2 text-slate-500 dark:text-zink-200 cart-shipping">
											${shipping}
										</td>
									</tr>
									<tr class="font-semibold">
										<td class="pt-2"> Total Amount (USD) </td>
										<td class="pt-2 cart-total">
											${total}
										</td>
									</tr>
								</tbody>
							</table>
						</div>
					</div>
				</div>
				<div class="flex gap-2 mt-5 shrink-0">
					<a
						href="/apps/ecommerce/products-list"
						class="w-full text-white bg-red-500 border-red-500 btn hover:text-white hover:bg-red-600 hover:border-red-600 focus:text-white focus:bg-red-600 focus:border-red-600 focus:ring focus:ring-red-100 active:text-white active:bg-red-600 active:border-red-600 active:ring active:ring-red-100 dark:ring-red-400/20"
						>Continue to Shopping</a
					>
					<a
						href="/apps/ecommerce/checkout"
						class="w-full text-white btn bg-custom-500 border-custom-500 hover:text-white hover:bg-custom-600 hover:border-custom-600 focus:text-white focus:bg-custom-600 focus:border-custom-600 focus:ring focus:ring-custom-100 active:text-white active:bg-custom-600 active:border-custom-600 active:ring active:ring-custom-100 dark:ring-custom-400/20"
						>Checkout</a
					>
				</div>

				<div class="flex items-center gap-5 p-4 mt-5 card">
					<div
						class="flex items-center justify-center size-12 rounded-md bg-slate-100 dark:bg-zink-600"
					>
						<LucideIcon
							name="Truck"
							class="size-6 text-slate-500 fill-slate-300 dark:text-zink-200 dark:fill-zink-500"
						/>
					</div>
					<div>
						<h6 class="mb-1">Estimated Delivery</h6>
						<p class="text-slate-500">01 - 07 Dec, 2023</p>
					</div>
				</div>
				<div class="flex items-center gap-5 p-4 card">
					<div
						class="flex items-center justify-center size-12 rounded-md bg-slate-100 dark:bg-zink-600"
					>
						<LucideIcon
							name="Container"
							class="size-6 text-slate-500 fill-slate-300 dark:text-zink-200 dark:fill-zink-500"
						/>
					</div>
					<div>
						<h6 class="mb-1">Free Shipping & Returns</h6>
						<p class="text-slate-500 dark:text-zink-200">On all orders over $200.00</p>
					</div>
				</div>
			</div>
		</div>
		<!--end col-->
	</div>
</div>

<Modal modal-center className="-translate-y-2/4" isOpen={isDeleteModal} toggle={toggleDelete}>
	<div class="w-screen md:w-[25rem] bg-white shadow rounded-md dark:bg-zink-600">
		<div class="max-h-[calc(theme('height.screen')_-_180px)] overflow-y-auto px-6 py-8">
			<div class="float-right">
				<button
					data-modal-close="deleteModal"
					class="transition-all duration-200 ease-linear text-slate-500 hover:text-red-500"
					on:click={toggleDelete}><LucideIcon name="X" class="size-5" /></button
				>
			</div>
			<img src="/assets/images/delete.png" alt="" class="block h-12 mx-auto" />
			<div class="mt-5 text-center">
				<h5 class="mb-1">Are you sure?</h5>
				<p class="text-slate-500 dark:text-zink-200">
					Are you certain you want to delete this record?
				</p>
				<div class="flex justify-center gap-2 mt-6">
					<button
						type="reset"
						data-modal-close="deleteModal"
						class="bg-white text-slate-500 btn hover:text-slate-500 hover:bg-slate-100 focus:text-slate-500 focus:bg-slate-100 active:text-slate-500 active:bg-slate-100 dark:bg-zink-600 dark:hover:bg-slate-500/10 dark:focus:bg-slate-500/10 dark:active:bg-slate-500/10"
						on:click={toggleDelete}>Cancel</button
					>
					<button
						type="submit"
						class="text-white bg-red-500 border-red-500 btn hover:text-white hover:bg-red-600 hover:border-red-600 focus:text-white focus:bg-red-600 focus:border-red-600 focus:ring focus:ring-red-100 active:text-white active:bg-red-600 active:border-red-600 active:ring active:ring-red-100 dark:ring-custom-400/20"
						>Yes, Delete It!</button
					>
				</div>
			</div>
		</div>
	</div>
</Modal>
