<script>
	import { tooltip } from '@svelte-plugins/tooltips';
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import LucideIcon from '../../../../common/components/LucideIcon.svelte';
</script>

<HeadTitle title="Avatar" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Avatar" pagetitle="UI Elements" />

	<div>
		<div class="grid grid-cols-1 gap-5 xl:grid-cols-2">
			<div class="card">
				<div class="card-body">
					<h6 class="mb-4">Square Avatar with Radius</h6>
					<div class="flex flex-wrap items-center gap-4">
						<img src="/assets/images/users/avatar-2.png" alt="" class="h-8 rounded-md" />
						<img src="/assets/images/users/avatar-2.png" alt="" class="h-10 rounded-md" />
						<img src="/assets/images/users/avatar-2.png" alt="" class="h-12 rounded-md" />
						<img src="/assets/images/users/avatar-2.png" alt="" class="rounded-md h-14" />
						<img src="/assets/images/users/avatar-2.png" alt="" class="h-16 rounded-md" />
						<img src="/assets/images/users/avatar-2.png" alt="" class="h-20 rounded-md" />
						<img src="/assets/images/users/avatar-2.png" alt="" class="h-24 rounded-md" />
						<img src="/assets/images/users/avatar-2.png" alt="" class="rounded-md h-28" />
					</div>
				</div>
			</div>
			<!--end card-->
			<div class="card">
				<div class="card-body">
					<h6 class="mb-4">Circular Avatar</h6>
					<div class="flex flex-wrap items-center gap-4">
						<img src="/assets/images/users/avatar-2.png" alt="" class="h-8 rounded-full" />
						<img src="/assets/images/users/avatar-2.png" alt="" class="h-10 rounded-full" />
						<img src="/assets/images/users/avatar-2.png" alt="" class="h-12 rounded-full" />
						<img src="/assets/images/users/avatar-2.png" alt="" class="rounded-full h-14" />
						<img src="/assets/images/users/avatar-2.png" alt="" class="h-16 rounded-full" />
						<img src="/assets/images/users/avatar-2.png" alt="" class="h-20 rounded-full" />
						<img src="/assets/images/users/avatar-2.png" alt="" class="h-24 rounded-full" />
						<img src="/assets/images/users/avatar-2.png" alt="" class="rounded-full h-28" />
					</div>
				</div>
			</div>
			<!--end card-->
		</div>
		<!--end grid-->
		<div class="grid grid-cols-1 gap-5 xl:grid-cols-2">
			<div class="card">
				<div class="card-body">
					<h6 class="mb-4">Square Avatar with Content</h6>
					<div class="flex flex-wrap items-center gap-4">
						<div
							class="flex items-center justify-center size-8 rounded-md bg-custom-100 text-custom-500 dark:bg-custom-950"
						>
							A
						</div>
						<div
							class="flex items-center justify-center size-10 rounded-md bg-custom-100 text-custom-500 dark:bg-custom-950"
						>
							A
						</div>
						<div
							class="flex items-center justify-center size-12 rounded-md text-15 bg-custom-100 text-custom-500 dark:bg-custom-950"
						>
							A
						</div>
						<div
							class="flex items-center justify-center rounded-md size-14 text-15 bg-custom-100 text-custom-500 dark:bg-custom-950"
						>
							A
						</div>
						<div
							class="flex items-center justify-center size-16 text-xl rounded-md bg-custom-100 text-custom-500 dark:bg-custom-950"
						>
							A
						</div>
						<div
							class="flex items-center justify-center size-20 text-2xl rounded-md bg-custom-100 text-custom-500 dark:bg-custom-950"
						>
							A
						</div>
						<div
							class="flex items-center justify-center size-24 text-2xl rounded-md bg-custom-100 text-custom-500 dark:bg-custom-950"
						>
							A
						</div>
						<div
							class="flex items-center justify-center text-3xl rounded-md size-28 bg-custom-100 text-custom-500 dark:bg-custom-950"
						>
							A
						</div>
					</div>
				</div>
			</div>
			<!--end card-->
			<div class="card">
				<div class="card-body">
					<h6 class="mb-4">Circular Avatar with Content</h6>
					<div class="flex flex-wrap items-center gap-4">
						<div
							class="flex items-center justify-center size-8 rounded-full bg-custom-100 text-custom-500 dark:bg-custom-950"
						>
							A
						</div>
						<div
							class="flex items-center justify-center size-10 rounded-full bg-custom-100 text-custom-500 dark:bg-custom-950"
						>
							A
						</div>
						<div
							class="flex items-center justify-center size-12 rounded-full text-15 bg-custom-100 text-custom-500 dark:bg-custom-950"
						>
							A
						</div>
						<div
							class="flex items-center justify-center rounded-full size-14 text-15 bg-custom-100 text-custom-500 dark:bg-custom-950"
						>
							A
						</div>
						<div
							class="flex items-center justify-center size-16 text-xl rounded-full bg-custom-100 text-custom-500 dark:bg-custom-950"
						>
							A
						</div>
						<div
							class="flex items-center justify-center size-20 text-2xl rounded-full bg-custom-100 text-custom-500 dark:bg-custom-950"
						>
							A
						</div>
						<div
							class="flex items-center justify-center size-24 text-2xl rounded-full bg-custom-100 text-custom-500 dark:bg-custom-950"
						>
							A
						</div>
						<div
							class="flex items-center justify-center text-3xl rounded-full size-28 bg-custom-100 text-custom-500 dark:bg-custom-950"
						>
							A
						</div>
					</div>
				</div>
			</div>
			<!--end card-->
		</div>
		<!--end grid-->
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4">Bordered Avatar</h6>
				<div class="grid grid-cols-1 gap-5 xl:grid-cols-2">
					<div class="flex flex-wrap items-center gap-4">
						<img
							src="/assets/images/users/avatar-2.png"
							alt=""
							class="h-10 rounded-md ring-1 ring-offset-2 ring-slate-200 dark:ring-offset-zink-700 dark:ring-zink-500"
						/>
						<img
							src="/assets/images/users/avatar-2.png"
							alt=""
							class="h-10 rounded-full ring-1 ring-offset-2 ring-slate-200 dark:ring-offset-zink-700 dark:ring-zink-500"
						/>
					</div>
					<div class="flex flex-wrap items-center gap-4">
						<div
							class="flex items-center justify-center size-10 rounded-md bg-custom-100 text-custom-500 ring-1 ring-offset-2 ring-custom-200 dark:ring-offset-zink-700 dark:ring-custom-900 dark:bg-custom-950"
						>
							A
						</div>
						<div
							class="flex items-center justify-center size-10 rounded-full bg-custom-100 text-custom-500 ring-1 ring-offset-2 ring-custom-200 dark:ring-offset-zink-700 dark:ring-custom-900 dark:bg-custom-950"
						>
							A
						</div>
					</div>
				</div>
			</div>
		</div>
		<!--end card-->
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4">Placeholder Icon Avatar</h6>
				<div class="grid grid-cols-1 gap-5 xl:grid-cols-2">
					<div class="flex flex-wrap items-center gap-4">
						<div
							class="relative size-10 overflow-hidden rounded-full bg-slate-100 text-slate-500 dark:bg-zink-600 dark:text-zink-300"
						>
							<LucideIcon
								name="User2"
								class="absolute left-0 right-0 inline-block size-10 mx-auto stroke-1 -bottom-1 fill-slate-200 dark:fill-zink-500"
							/>
						</div>
						<div
							class="relative size-10 overflow-hidden rounded-full bg-custom-100 text-custom-500 dark:bg-custom-950"
						>
							<LucideIcon
								name="User2"
								class="absolute left-0 right-0 inline-block size-10 mx-auto stroke-1 -bottom-1 fill-custom-200 dark:fill-custom-900"
							/>
						</div>
						<div
							class="relative size-10 overflow-hidden text-green-500 bg-green-100 rounded-full dark:bg-green-900"
						>
							<LucideIcon
								name="User2"
								class="absolute left-0 right-0 inline-block size-10 mx-auto stroke-1 -bottom-1 fill-green-200 dark:fill-green-800"
							/>
						</div>
						<div
							class="relative size-10 overflow-hidden text-orange-500 bg-orange-100 rounded-full dark:bg-orange-950"
						>
							<LucideIcon
								name="User2"
								class="absolute left-0 right-0 inline-block size-10 mx-auto stroke-1 -bottom-1 fill-orange-200 dark:fill-orange-900"
							/>
						</div>
						<div
							class="relative size-10 overflow-hidden text-yellow-500 bg-yellow-100 rounded-full dark:bg-yellow-950"
						>
							<LucideIcon
								name="User2"
								class="absolute left-0 right-0 inline-block size-10 mx-auto stroke-1 -bottom-1 fill-yellow-200 dark:fill-yellow-900"
							/>
						</div>
					</div>
					<div class="flex flex-wrap items-center gap-4">
						<div
							class="relative size-10 overflow-hidden rounded-md bg-slate-100 text-slate-500 dark:bg-zink-600 dark:text-zink-300"
						>
							<LucideIcon
								name="User2"
								class="absolute left-0 right-0 inline-block size-10 mx-auto stroke-1 -bottom-1 fill-slate-200 dark:fill-zink-500"
							/>
						</div>
						<div
							class="relative size-10 overflow-hidden rounded-md bg-custom-100 text-custom-500 dark:bg-custom-950"
						>
							<LucideIcon
								name="User2"
								class="absolute left-0 right-0 inline-block size-10 mx-auto stroke-1 -bottom-1 fill-custom-200 dark:fill-custom-900"
							/>
						</div>
						<div
							class="relative size-10 overflow-hidden text-green-500 bg-green-100 rounded-md dark:bg-green-900"
						>
							<LucideIcon
								name="User2"
								class="absolute left-0 right-0 inline-block size-10 mx-auto stroke-1 -bottom-1 fill-green-200 dark:fill-green-800"
							/>
						</div>
						<div
							class="relative size-10 overflow-hidden text-orange-500 bg-orange-100 rounded-md dark:bg-orange-950"
						>
							<LucideIcon
								name="User2"
								class="absolute left-0 right-0 inline-block size-10 mx-auto stroke-1 -bottom-1 fill-orange-200 dark:fill-orange-900"
							/>
						</div>
						<div
							class="relative size-10 overflow-hidden text-yellow-500 bg-yellow-100 rounded-md dark:bg-yellow-950"
						>
							<LucideIcon
								name="User2"
								class="absolute left-0 right-0 inline-block size-10 mx-auto stroke-1 -bottom-1 fill-yellow-200 dark:fill-yellow-900"
							/>
						</div>
					</div>
				</div>
			</div>
		</div>
		<!--end card-->
		<div class="grid grid-cols-1 gap-5 xl:grid-cols-2">
			<div class="card">
				<div class="card-body">
					<h6 class="mb-4">Square Avatar with Status</h6>
					<div class="flex flex-wrap items-center gap-4">
						<div class="relative">
							<img src="/assets/images/users/user-2.jpg" alt="" class="h-8 rounded-md" />
							<span
								class="-top-1 ltr:-right-1 rtl:-left-1 absolute w-2.5 h-2.5 bg-green-400 border-2 border-white rounded-full dark:border-zink-700"
							></span>
						</div>
						<div class="relative">
							<img src="/assets/images/users/user-2.jpg" alt="" class="h-10 rounded-md" />
							<span
								class="-top-1 ltr:-right-1 rtl:-left-1 absolute w-2.5 h-2.5 bg-green-400 border-2 border-white rounded-full dark:border-zink-700"
							></span>
						</div>
						<div class="relative">
							<img src="/assets/images/users/user-2.jpg" alt="" class="h-12 rounded-md" />
							<span
								class="absolute size-3 bg-green-400 border-2 border-white rounded-full -top-1 ltr:-right-1 rtl:-left-1 dark:border-zink-700"
							></span>
						</div>
						<div class="relative">
							<img src="/assets/images/users/user-2.jpg" alt="" class="rounded-md h-14" />
							<span
								class="absolute size-3 bg-green-400 border-2 border-white rounded-full -top-1 ltr:-right-1 rtl:-left-1 dark:border-zink-700"
							></span>
						</div>
						<div class="relative">
							<img src="/assets/images/users/user-2.jpg" alt="" class="h-16 rounded-md" />
							<span
								class="absolute size-3.5 bg-green-400 border-2 border-white rounded-full -top-1.5 ltr:-right-1.5 rtl:-left-1.5 dark:border-zink-700"
							></span>
						</div>
						<div class="relative">
							<img src="/assets/images/users/user-2.jpg" alt="" class="h-20 rounded-md" />
							<span
								class="absolute size-4 bg-green-400 border-2 border-white rounded-full -top-1.5 ltr:-right-1.5 rtl:-left-1.5 dark:border-zink-700"
							></span>
						</div>
						<div class="relative">
							<img src="/assets/images/users/user-2.jpg" alt="" class="h-24 rounded-md" />
							<span
								class="absolute size-5 bg-green-400 border-2 border-white rounded-full -top-2 ltr:-right-2 rtl:-left-2 dark:border-zink-700"
							></span>
						</div>
						<div class="relative">
							<img src="/assets/images/users/user-2.jpg" alt="" class="rounded-md h-28" />
							<span
								class="absolute size-6 bg-green-400 border-2 border-white rounded-full -top-2.5 ltr:-right-2.5 rtl:-left-2.5 dark:border-zink-700"
							></span>
						</div>
					</div>
				</div>
			</div>
			<!--end card-->
			<div class="card">
				<div class="card-body">
					<h6 class="mb-4">Circular Avatar with Status</h6>
					<div class="flex flex-wrap items-center gap-4">
						<div class="relative">
							<img src="/assets/images/users/user-2.jpg" alt="" class="h-8 rounded-full" />
							<span
								class="top-0 ltr:right-0 rtl:left-0 absolute w-2.5 h-2.5 bg-green-400 border-2 border-white rounded-full dark:border-zink-700"
							></span>
						</div>
						<div class="relative">
							<img src="/assets/images/users/user-2.jpg" alt="" class="h-10 rounded-full" />
							<span
								class="top-0 ltr:right-0 rtl:left-0 absolute w-2.5 h-2.5 bg-green-400 border-2 border-white rounded-full dark:border-zink-700"
							></span>
						</div>
						<div class="relative">
							<img src="/assets/images/users/user-2.jpg" alt="" class="h-12 rounded-full" />
							<span
								class="absolute top-0 size-3 bg-green-400 border-2 border-white rounded-full ltr:right-0 rtl:left-0 dark:border-zink-700"
							></span>
						</div>
						<div class="relative">
							<img src="/assets/images/users/user-2.jpg" alt="" class="rounded-full h-14" />
							<span
								class="absolute top-0 size-3 bg-green-400 border-2 border-white rounded-full ltr:right-0 rtl:left-0 dark:border-zink-700"
							></span>
						</div>
						<div class="relative">
							<img src="/assets/images/users/user-2.jpg" alt="" class="h-16 rounded-full" />
							<span
								class="absolute size-3.5 bg-green-400 border-2 border-white rounded-full top-0.5 ltr:right-0.5 rtl:left-0.5 dark:border-zink-700"
							></span>
						</div>
						<div class="relative">
							<img src="/assets/images/users/user-2.jpg" alt="" class="h-20 rounded-full" />
							<span
								class="absolute size-4 bg-green-400 border-2 border-white rounded-full top-0.5 ltr:right-0.5 rtl:left-0.5 dark:border-zink-700"
							></span>
						</div>
						<div class="relative">
							<img src="/assets/images/users/user-2.jpg" alt="" class="h-24 rounded-full" />
							<span
								class="absolute size-5 bg-green-400 border-2 border-white rounded-full top-0.5 ltr:right-0.5 rtl:left-0.5 dark:border-zink-700"
							></span>
						</div>
						<div class="relative">
							<img src="/assets/images/users/user-2.jpg" alt="" class="rounded-full h-28" />
							<span
								class="absolute size-6 bg-green-400 border-2 border-white rounded-full top-1 ltr:right-1 rtl:left-1 dark:border-zink-700"
							></span>
						</div>
					</div>
				</div>
			</div>
			<!--end card-->
		</div>
		<!--end grid-->
		<div class="grid grid-cols-1 gap-4 xl:grid-cols-2">
			<div class="card">
				<div class="card-body">
					<h6 class="mb-4">Avatar Content with Status</h6>
					<div class="flex flex-wrap items-center gap-4">
						<div
							class="relative flex items-center justify-center size-8 rounded-md bg-custom-100 dark:bg-custom-950 text-custom-500"
						>
							A
							<span
								class="-top-1 ltr:-right-1 rtl:-left-1 absolute w-2.5 h-2.5 bg-red-400 border-2 border-white rounded-full dark:border-zink-700"
							></span>
						</div>
						<div
							class="relative flex items-center justify-center size-10 rounded-md bg-custom-100 dark:bg-custom-950 text-custom-500"
						>
							A
							<span
								class="-top-1 ltr:-right-1 rtl:-left-1 absolute w-2.5 h-2.5 bg-red-400 border-2 border-white rounded-full dark:border-zink-700"
							></span>
						</div>
						<div
							class="relative flex items-center justify-center size-12 rounded-md text-15 bg-custom-100 dark:bg-custom-950 text-custom-500"
						>
							A
							<span
								class="absolute size-3 bg-red-400 border-2 border-white rounded-full dark:border-zink-700 -top-1 ltr:-right-1 rtl:-left-1"
							></span>
						</div>
						<div
							class="relative flex items-center justify-center rounded-md size-14 text-15 bg-custom-100 dark:bg-custom-950 text-custom-500"
						>
							A
							<span
								class="absolute size-3 bg-red-400 border-2 border-white rounded-full dark:border-zink-700 -top-1 ltr:-right-1 rtl:-left-1"
							></span>
						</div>
						<div
							class="relative flex items-center justify-center size-16 text-xl rounded-md bg-custom-100 dark:bg-custom-950 text-custom-500"
						>
							A
							<span
								class="absolute size-3.5 bg-red-400 border-2 border-white rounded-full dark:border-zink-700 -top-1.5 ltr:-right-1.5 rtl:-left-1.5"
							></span>
						</div>
						<div
							class="relative flex items-center justify-center size-20 text-2xl rounded-md bg-custom-100 dark:bg-custom-950 text-custom-500"
						>
							A
							<span
								class="absolute size-4 bg-red-400 border-2 border-white rounded-full dark:border-zink-700 -top-1.5 ltr:-right-1.5 rtl:-left-1.5"
							></span>
						</div>
						<div
							class="relative flex items-center justify-center size-24 text-2xl rounded-md bg-custom-100 dark:bg-custom-950 text-custom-500"
						>
							A
							<span
								class="absolute size-5 bg-red-400 border-2 border-white rounded-full dark:border-zink-700 -top-2 ltr:-right-2 rtl:-left-2"
							></span>
						</div>
						<div
							class="relative flex items-center justify-center text-3xl rounded-md size-28 bg-custom-100 dark:bg-custom-950 text-custom-500"
						>
							A
							<span
								class="absolute size-6 bg-red-400 border-2 border-white rounded-full dark:border-zink-700 -top-2.5 ltr:-right-2.5 rtl:-left-2.5"
							></span>
						</div>
					</div>
				</div>
			</div>
			<!--end card-->
			<div class="card">
				<div class="card-body">
					<h6 class="mb-4">Circular Avatar Content with Status</h6>
					<div class="flex flex-wrap items-center gap-4">
						<div
							class="relative flex items-center justify-center size-8 rounded-full bg-custom-100 text-custom-500 dark:bg-custom-950"
						>
							A <span
								class="top-0 ltr:right-0 rtl:left-0 absolute w-2.5 h-2.5 bg-red-400 border-2 border-white rounded-full dark:border-zink-700"
							></span>
						</div>
						<div
							class="relative flex items-center justify-center size-10 rounded-full bg-custom-100 text-custom-500 dark:bg-custom-950"
						>
							A <span
								class="top-0 ltr:right-0 rtl:left-0 absolute w-2.5 h-2.5 bg-red-400 border-2 border-white rounded-full dark:border-zink-700"
							></span>
						</div>
						<div
							class="relative flex items-center justify-center size-12 rounded-full text-15 bg-custom-100 text-custom-500 dark:bg-custom-950"
						>
							A <span
								class="absolute top-0 size-3 bg-red-400 border-2 border-white rounded-full ltr:right-0 rtl:left-0 dark:border-zink-700"
							></span>
						</div>
						<div
							class="relative flex items-center justify-center rounded-full size-14 text-15 bg-custom-100 text-custom-500 dark:bg-custom-950"
						>
							A <span
								class="absolute top-0 size-3 bg-red-400 border-2 border-white rounded-full ltr:right-0 rtl:left-0 dark:border-zink-700"
							></span>
						</div>
						<div
							class="relative flex items-center justify-center size-16 text-xl rounded-full bg-custom-100 text-custom-500 dark:bg-custom-950"
						>
							A <span
								class="absolute size-3.5 bg-red-400 border-2 border-white rounded-full top-0.5 ltr:right-0.5 rtl:left-0.5 dark:border-zink-700"
							></span>
						</div>
						<div
							class="relative flex items-center justify-center size-20 text-2xl rounded-full bg-custom-100 text-custom-500 dark:bg-custom-950"
						>
							A <span
								class="absolute size-4 bg-red-400 border-2 border-white rounded-full top-0.5 ltr:right-0.5 rtl:left-0.5 dark:border-zink-700"
							></span>
						</div>
						<div
							class="relative flex items-center justify-center size-24 text-2xl rounded-full bg-custom-100 text-custom-500 dark:bg-custom-950"
						>
							A <span
								class="absolute size-5 bg-red-400 border-2 border-white rounded-full top-0.5 ltr:right-0.5 rtl:left-0.5 dark:border-zink-700"
							></span>
						</div>
						<div
							class="relative flex items-center justify-center text-3xl rounded-full size-28 bg-custom-100 text-custom-500 dark:bg-custom-950"
						>
							A <span
								class="absolute size-6 bg-red-400 border-2 border-white rounded-full top-1 ltr:right-1 rtl:left-1 dark:border-zink-700"
							></span>
						</div>
					</div>
				</div>
			</div>
			<!--end card-->
		</div>
		<!--end grid-->
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4">Avatar Group</h6>
				<div class="grid grid-cols-1 gap-5 xl:grid-cols-3">
					<div class="flex flex-wrap items-center gap-4">
						<div class="flex -space-x-4">
							<img
								class="size-10 border-2 border-white rounded-full dark:border-zink-700"
								src="/assets/images/users/user-1.jpg"
								alt=""
							/>
							<img
								class="size-10 border-2 border-white rounded-full dark:border-zink-700"
								src="/assets/images/users/user-2.jpg"
								alt=""
							/>
							<img
								class="size-10 border-2 border-white rounded-full dark:border-zink-700"
								src="/assets/images/users/user-3.jpg"
								alt=""
							/>
							<img
								class="size-10 border-2 border-white rounded-full dark:border-zink-700"
								src="/assets/images/users/user-4.jpg"
								alt=""
							/>
						</div>
					</div>
					<div class="flex flex-wrap items-center gap-4">
						<div class="flex -space-x-4">
							<img
								class="size-10 border-2 border-white rounded-full dark:border-zink-700"
								src="/assets/images/users/user-1.jpg"
								alt=""
							/>
							<img
								class="size-10 border-2 border-white rounded-full dark:border-zink-700"
								src="/assets/images/users/user-2.jpg"
								alt=""
							/>
							<img
								class="size-10 border-2 border-white rounded-full dark:border-zink-700"
								src="/assets/images/users/user-3.jpg"
								alt=""
							/>
							<a
								class="flex items-center justify-center size-10 text-xs font-medium text-white border-2 border-white rounded-full bg-slate-700 hover:bg-slate-600 dark:border-zink-700"
								href={'#'}>+99</a
							>
						</div>
					</div>
					<div class="flex flex-wrap items-center gap-4">
						<div class="flex -space-x-4">
							<div
								class="flex items-center justify-center size-10 border-2 border-white rounded-full bg-custom-100 text-custom-500 dark:border-zink-700 dark:bg-custom-950"
							>
								TG
							</div>
							<div
								class="flex items-center justify-center size-10 text-green-500 bg-green-100 border-2 border-white rounded-full dark:border-zink-700 dark:bg-green-900"
							>
								PI
							</div>
							<div
								class="flex items-center justify-center size-10 text-purple-500 bg-purple-100 border-2 border-white rounded-full dark:border-zink-700"
							>
								AU
							</div>
							<img
								class="size-10 border-2 border-white rounded-full dark:border-zink-700"
								src="/assets/images/users/user-3.jpg"
								alt=""
							/>
							<div
								class="flex items-center justify-center size-10 text-yellow-500 bg-yellow-100 border-2 border-white rounded-full dark:bg-yellow-950 dark:border-zink-700"
							>
								SS
							</div>
						</div>
					</div>
				</div>
			</div>
		</div>
		<!--end card-->
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4">Avatar Group with Tooltip</h6>
				<div class="grid grid-cols-1 gap-5 xl:grid-cols-3">
					<div class="flex flex-wrap items-center gap-4">
						<div class="flex -space-x-4">
							<a href="#!" use:tooltip title="Jose White"
								><img
									class="size-10 border-2 border-white rounded-full dark:border-zink-700"
									src="/assets/images/users/user-1.jpg"
									alt=""
								/></a
							>
							<a href="#!" use:tooltip title="Patricia Garcia"
								><img
									class="size-10 border-2 border-white rounded-full dark:border-zink-700"
									src="/assets/images/users/user-2.jpg"
									alt=""
								/></a
							>
							<a href="#!" use:tooltip title="Tonya Johnson"
								><img
									class="size-10 border-2 border-white rounded-full dark:border-zink-700"
									src="/assets/images/users/user-3.jpg"
									alt=""
								/></a
							>
							<a href="#!" use:tooltip title="Paula Keenan"
								><img
									class="size-10 border-2 border-white rounded-full dark:border-zink-700"
									src="/assets/images/users/user-4.jpg"
									alt=""
								/></a
							>
						</div>
					</div>
					<div class="flex flex-wrap items-center gap-4">
						<div class="flex -space-x-4">
							<a href="#!" use:tooltip title="Paula Keenan"
								><img
									class="size-10 border-2 border-white rounded-full dark:border-zink-700"
									src="/assets/images/users/user-3.jpg"
									alt=""
								/></a
							>
							<a href="#!" use:tooltip title="Paula Keenan"
								><img
									class="size-10 border-2 border-white rounded-full dark:border-zink-700"
									src="/assets/images/users/user-3.jpg"
									alt=""
								/></a
							>
							<a href="#!" use:tooltip title="Paula Keenan"
								><img
									class="size-10 border-2 border-white rounded-full dark:border-zink-700"
									src="/assets/images/users/user-3.jpg"
									alt=""
								/></a
							>
							<a
								title="More Users"
								use:tooltip={{
									content:
										'<b>Tooltip Top</b><p>This is an example of using HTML and content wrapping.</p>',
									position: 'bottom',
									animation: 'slide',
									arrow: true
								}}
								class="flex items-center justify-center size-10 text-xs font-medium text-white border-2 border-white rounded-full bg-slate-700 hover:bg-slate-600 dark:border-zink-700"
								href={'#'}>+99</a
							>
						</div>
					</div>
				</div>
			</div>
		</div>
		<!--end card-->
	</div>
</div>
