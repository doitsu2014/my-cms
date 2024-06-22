<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import LucideIcon from '../../../../common/components/LucideIcon.svelte';

</script>

<HeadTitle title="Breadcrumb" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Breadcrumb" pagetitle="Navigation" />

	<div class="card">
		<div class="card-body">
			<h6 class="mb-4 text-15">Basic Breadcrumb</h6>

			<ul class="flex flex-wrap items-center gap-2 mb-3 text-sm font-normal">
				<li class="relative before:content-['\ea54'] before:font-remix before:ltr:-right-1 before:rtl:-left-1 before:absolute before:text-[18px] before:-top-[3px] ltr:pr-4 rtl:pl-4 before:rtl:rotate-180 before:text-slate-500 dark:before:text-zink-200">
					<a href="#!" class="text-slate-500 dark:text-zink-200">Home</a>
				</li>
				<li class="relative before:content-['\ea54'] before:font-remix before:ltr:-right-1 before:rtl:-left-1 before:absolute before:text-[18px] before:-top-[3px] ltr:pr-4 rtl:pl-4 before:rtl:rotate-180 before:text-slate-500 dark:before:text-zink-200">
					<a href="#!" class="text-slate-500 dark:text-zink-200">Settings</a>
				</li>
				<li class="text-slate-700 dark:text-zink-100">
					Personal Information
				</li>
			</ul>

			<ul class="flex flex-wrap items-center gap-2 mb-3 text-sm font-normal">
				<li class="relative before:content-['\ea54'] before:font-remix before:ltr:-right-1 before:rtl:-left-1 before:absolute before:text-[18px] before:-top-[3px] ltr:pr-4 rtl:pl-4 before:rtl:rotate-180 before:text-slate-500 dark:before:text-zink-200">
					<a href="#!" class="flex items-center gap-1 text-slate-500 dark:text-zink-200"><LucideIcon name="Home" class="size-3"/> Home</a>
				</li>
				<li class="relative before:content-['\ea54'] before:font-remix before:ltr:-right-1 before:rtl:-left-1 before:absolute before:text-[18px] before:-top-[3px] ltr:pr-4 rtl:pl-4 before:rtl:rotate-180 before:text-slate-500 dark:before:text-zink-200">
					<a href="#!" class="flex items-center gap-1 text-slate-500 dark:text-zink-200"><LucideIcon name="Settings" class="size-3"/> Settings</a>
				</li>
				<li class="flex items-center gap-1 text-slate-700 dark:text-zink-100">
					<LucideIcon name="User2" class="size-3"/> Personal Information
				</li>
			</ul>

			<ul class="flex flex-wrap items-center gap-2 mb-3 text-sm font-normal">
				<li class="relative before:content-['\ea54'] before:font-remix before:ltr:-right-1 before:rtl:-left-1 before:absolute before:text-[18px] before:-top-[3px] ltr:pr-4 rtl:pl-4 before:rtl:rotate-180 before:text-slate-500 dark:before:text-zink-200">
					<a href="#!" class="text-slate-500 dark:text-zink-200">Home</a>
				</li>
				<li class="relative before:content-['\ea54'] before:font-remix before:ltr:-right-1 before:rtl:-left-1 before:absolute before:text-[18px] before:-top-[3px] ltr:pr-4 rtl:pl-4 before:rtl:rotate-180 before:text-slate-500 dark:before:text-zink-200">
					<a href="#!" class="text-slate-500 dark:text-zink-200">Cooking</a>
				</li>
				<li class="relative before:content-['\ea54'] before:font-remix before:ltr:-right-1 before:rtl:-left-1 before:absolute before:text-[18px] before:-top-[3px] ltr:pr-4 rtl:pl-4 before:rtl:rotate-180 before:text-slate-500 dark:before:text-zink-200">
					<a href="#!" class="text-slate-500 dark:text-zink-200">Banking</a>
				</li>
				<li class="text-slate-700 dark:text-zink-100">
					Bread Shape
				</li>
			</ul>

			<ul class="flex flex-wrap items-center gap-2 text-sm font-normal">
				<li class="relative before:content-['\ea6d'] before:font-remix before:ltr:-right-1 before:rtl:-left-1 before:absolute before:text-[18px] before:-top-[3px] ltr:pr-4 rtl:pl-4 before:rtl:rotate-180 before:text-slate-500 dark:before:text-zink-200">
					<a href="#!" class="text-slate-500 dark:text-zink-200">Home</a>
				</li>
				<li class="relative before:content-['\ea6d'] before:font-remix before:ltr:-right-1 before:rtl:-left-1 before:absolute before:text-[18px] before:-top-[3px] ltr:pr-4 rtl:pl-4 before:rtl:rotate-180 before:text-slate-500 dark:before:text-zink-200">
					<a href="#!" class="text-slate-500 dark:text-zink-200">Settings</a>
				</li>
				<li class="text-slate-700 dark:text-zink-100">
					Personal Information
				</li>
			</ul>
		</div>
	</div><!--end card-->

	<div class="card">
		<div class="card-body">
			<h6 class="mb-4 text-15">Sizes</h6>

			<ul class="flex flex-wrap items-center gap-2 mb-3 text-sm font-normal">
				<li class="relative before:content-['\ea54'] before:font-remix before:-right-1 before:absolute before:text-[18px] before:top-0 pr-4 before:text-slate-500 dark:before:text-zink-200">
					<a href="#!" class="text-slate-500 dark:text-zink-200">Home</a>
				</li>
				<li class="relative before:content-['\ea54'] before:font-remix before:-right-1 before:absolute before:text-[18px] before:top-0 pr-4 before:text-slate-500 dark:before:text-zink-200">
					<a href="#!" class="text-slate-500 dark:text-zink-200">Cooking</a>
				</li>
				<li class="relative before:content-['\ea54'] before:font-remix before:-right-1 before:absolute before:text-[18px] before:top-0 pr-4 before:text-slate-500 dark:before:text-zink-200">
					<a href="#!" class="text-slate-500 dark:text-zink-200">Banking</a>
				</li>
				<li class="text-slate-700 dark:text-zink-100">
					Bread Shape
				</li>
			</ul>

			<ul class="flex flex-wrap items-center gap-2 mb-3 text-sm font-normal">
				<li class="relative before:content-['\ea54'] before:font-remix before:ltr:-right-1 before:rtl:-left-1 before:absolute before:text-[18px] before:-top-[3px] ltr:pr-4 rtl:pl-4 before:rtl:rotate-180 before:text-slate-500 dark:before:text-zink-200">
					<a href="#!" class="text-slate-500 dark:text-zink-200">Home</a>
				</li>
				<li class="relative before:content-['\ea54'] before:font-remix before:ltr:-right-1 before:rtl:-left-1 before:absolute before:text-[18px] before:-top-[3px] ltr:pr-4 rtl:pl-4 before:rtl:rotate-180 before:text-slate-500 dark:before:text-zink-200">
					<a href="#!" class="text-slate-500 dark:text-zink-200">Cooking</a>
				</li>
				<li class="relative before:content-['\ea54'] before:font-remix before:ltr:-right-1 before:rtl:-left-1 before:absolute before:text-[18px] before:-top-[3px] ltr:pr-4 rtl:pl-4 before:rtl:rotate-180 before:text-slate-500 dark:before:text-zink-200">
					<a href="#!" class="text-slate-500 dark:text-zink-200">Banking</a>
				</li>
				<li class="text-slate-700 dark:text-zink-100">
					Bread Shape
				</li>
			</ul>

			<ul class="flex flex-wrap items-center gap-2 mb-3 font-normal text-15">
				<li class="relative before:content-['\ea54'] before:font-remix before:ltr:-right-1 before:rtl:-left-1 before:absolute before:text-[18px] before:-top-[3px] ltr:pr-4 rtl:pl-4 before:rtl:rotate-180 before:text-slate-500 dark:before:text-zink-200">
					<a href="#!" class="text-slate-500 dark:text-zink-200">Home</a>
				</li>
				<li class="relative before:content-['\ea54'] before:font-remix before:ltr:-right-1 before:rtl:-left-1 before:absolute before:text-[18px] before:-top-[3px] ltr:pr-4 rtl:pl-4 before:rtl:rotate-180 before:text-slate-500 dark:before:text-zink-200">
					<a href="#!" class="text-slate-500 dark:text-zink-200">Cooking</a>
				</li>
				<li class="relative before:content-['\ea54'] before:font-remix before:ltr:-right-1 before:rtl:-left-1 before:absolute before:text-[18px] before:-top-[3px] ltr:pr-4 rtl:pl-4 before:rtl:rotate-180 before:text-slate-500 dark:before:text-zink-200">
					<a href="#!" class="text-slate-500 dark:text-zink-200">Banking</a>
				</li>
				<li class="text-slate-700 dark:text-zink-100">
					Bread Shape
				</li>
			</ul>
			<ul class="flex flex-wrap items-center gap-2 mb-3 font-normal text-16">
				<li class="relative before:content-['\ea54'] before:font-remix before:ltr:-right-1 before:rtl:-left-1 before:absolute before:text-[18px] before:-top-[3px] ltr:pr-4 rtl:pl-4 before:text-lg before:rtl:rotate-180 before:text-slate-500 dark:before:text-zink-200">
					<a href="#!" class="text-slate-500 dark:text-zink-200">Home</a>
				</li>
				<li class="relative before:content-['\ea54'] before:font-remix before:ltr:-right-1 before:rtl:-left-1 before:absolute before:text-[18px] before:-top-[3px] ltr:pr-4 rtl:pl-4 before:text-lg before:rtl:rotate-180 before:text-slate-500 dark:before:text-zink-200">
					<a href="#!" class="text-slate-500 dark:text-zink-200">Cooking</a>
				</li>
				<li class="relative before:content-['\ea54'] before:font-remix before:ltr:-right-1 before:rtl:-left-1 before:absolute before:text-[18px] before:-top-[3px] ltr:pr-4 rtl:pl-4 before:text-lg before:rtl:rotate-180 before:text-slate-500 dark:before:text-zink-200">
					<a href="#!" class="text-slate-500 dark:text-zink-200">Banking</a>
				</li>
				<li class="text-slate-700 dark:text-zink-100">
					Bread Shape
				</li>
			</ul>
			<ul class="flex flex-wrap items-center gap-2 text-lg font-normal">
				<li class="relative before:content-['\ea54'] before:font-remix before:ltr:-right-1 before:rtl:-left-1 before:absolute before:text-[18px] before:-top-[3px] ltr:pr-4 rtl:pl-4 before:text- before:rtl:rotate-180 before:text-slate-500 dark:before:text-zink-200">
					<a href="#!" class="text-slate-500 dark:text-zink-200">Home</a>
				</li>
				<li class="relative before:content-['\ea54'] before:font-remix before:ltr:-right-1 before:rtl:-left-1 before:absolute before:text-[18px] before:-top-[3px] ltr:pr-4 rtl:pl-4 before:text- before:rtl:rotate-180 before:text-slate-500 dark:before:text-zink-200">
					<a href="#!" class="text-slate-500 dark:text-zink-200">Cooking</a>
				</li>
				<li class="relative before:content-['\ea54'] before:font-remix before:ltr:-right-1 before:rtl:-left-1 before:absolute before:text-[18px] before:-top-[3px] ltr:pr-4 rtl:pl-4 before:text- before:rtl:rotate-180 before:text-slate-500 dark:before:text-zink-200">
					<a href="#!" class="text-slate-500 dark:text-zink-200">Banking</a>
				</li>
				<li class="text-slate-700 dark:text-zink-100">
					Bread Shape
				</li>
			</ul>
		</div>
	</div><!--end card-->

	<div class="card">
		<div class="card-body">
			<h6 class="mb-4 text-15">Boxed Breadcrumb</h6>

			<ul class="inline-flex flex-wrap items-center gap-2 p-3 mb-3 text-sm font-normal rounded bg-slate-100 dark:bg-zink-600">
				<li class="relative before:content-['\ea54'] before:font-remix before:ltr:-right-1 before:rtl:-left-1 before:absolute before:text-[18px] before:-top-[3px] ltr:pr-4 rtl:pl-4 before:rtl:rotate-180 before:text-slate-500 dark:before:text-zink-200">
					<a href="#!" class="text-slate-500 dark:text-zink-200">Home</a>
				</li>
				<li class="relative before:content-['\ea54'] before:font-remix before:ltr:-right-1 before:rtl:-left-1 before:absolute before:text-[18px] before:-top-[3px] ltr:pr-4 rtl:pl-4 before:rtl:rotate-180 before:text-slate-500 dark:before:text-zink-200">
					<a href="#!" class="text-slate-500 dark:text-zink-200">Settings</a>
				</li>
				<li class="text-slate-700 dark:text-zink-100">
					Personal Information
				</li>
			</ul>

			<div class="mb-3">
				<ul class="inline-flex flex-wrap items-center gap-2 p-3 text-sm font-normal rounded bg-slate-100 dark:bg-zink-600">
					<li class="relative before:content-['\ea54'] before:font-remix before:ltr:-right-1 before:rtl:-left-1 before:absolute before:text-[18px] before:-top-[3px] ltr:pr-4 rtl:pl-4 before:rtl:rotate-180 before:text-slate-500 dark:before:text-zink-200">
						<a href="#!" class="flex items-center gap-1 text-slate-500 dark:text-zink-200"><LucideIcon name="Home" class="size-3"/> Home</a>
					</li>
					<li class="relative before:content-['\ea54'] before:font-remix before:ltr:-right-1 before:rtl:-left-1 before:absolute before:text-[18px] before:-top-[3px] ltr:pr-4 rtl:pl-4 before:rtl:rotate-180 before:text-slate-500 dark:before:text-zink-200">
						<a href="#!" class="flex items-center gap-1 text-slate-500 dark:text-zink-200"><LucideIcon name="Settings" class="size-3"/> Settings</a>
					</li>
					<li class="flex items-center gap-1 text-slate-700 dark:text-zink-100">
						<LucideIcon name="User2" class="size-3"/> Personal Information
					</li>
				</ul>
			</div>

			<ul class="flex flex-wrap items-center gap-2 text-sm font-normal rounded">
				<li class="relative before:content-['\ea54'] before:font-remix before:ltr:-right-5 before:rtl:-left-5 before:absolute before:text-xl before:top-1.5 ltr:mr-4 rtl:ml-4 before:text-slate-500 dark:before:text-zink-200 px-3 py-2 bg-slate-100 dark:bg-zink-600 rounded  before:rtl:rotate-180">
					<a href="#!" class="text-slate-500 dark:text-zink-200">Home</a>
				</li>
				<li class="relative before:content-['\ea54'] before:font-remix before:ltr:-right-5 before:rtl:-left-5 before:absolute before:text-xl before:top-1.5 ltr:mr-4 rtl:ml-4 before:text-slate-500 dark:before:text-zink-200 px-3 py-2 bg-slate-100 dark:bg-zink-600 rounded  before:rtl:rotate-180">
					<a href="#!" class="text-slate-500 dark:text-zink-200">Settings</a>
				</li>
				<li class="px-3 py-2 rounded text-custom-500 bg-custom-100 dark:bg-zink-600">
					Personal Information
				</li>
			</ul>
		</div>
	</div>
</div>
