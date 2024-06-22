<script>
	import HeadTitle from '../../../../common/components/HeadTitle.svelte';
	import Breadcrumb from '../../../../common/components/Breadcrumb.svelte';
	import Alert from '../../../../common/components/Alert.svelte';
	import LucideIcon from '../../../../common/components/LucideIcon.svelte';
</script>

<HeadTitle title="Alerts" />

<div class="container-fluid group-data-[content=boxed]:max-w-boxed mx-auto">
	<Breadcrumb title="Alerts" pagetitle="UI Elements" />

	<div>
		<div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Basic Alerts</h6>
				<div class="grid grid-cols-1 gap-5 xl:grid-cols-2">
					<div class="flex flex-col gap-3">
                        <Alert class="px-4 py-3 text-sm border border-transparent rounded-md text-custom-500 bg-custom-50 dark:bg-custom-400/20">
                            <span class="font-bold">Custom</span> alert! You should check in on some of those fields
							below.</Alert>
                        <Alert class="px-4 py-3 text-sm text-green-500 border border-transparent rounded-md bg-green-50 dark:bg-green-400/20">
                            <span class="font-bold">Green</span> alert! You should check in on some of those fields
							below.</Alert>
                        <Alert class="px-4 py-3 text-sm text-orange-500 border border-transparent rounded-md bg-orange-50 dark:bg-orange-400/20">
                            <span class="font-bold">Orange</span> alert! You should check in on some of those fields
							below.</Alert>
                        <Alert class="px-4 py-3 text-sm border border-transparent rounded-md text-sky-500 bg-sky-50 dark:bg-sky-400/20">
                            <span class="font-bold">Sky</span> alert! You should check in on some of those fields
							below.</Alert>
					</div>
					<div class="flex flex-col gap-3">
						
                        <Alert class="px-4 py-3 text-sm text-yellow-500 border border-transparent rounded-md bg-yellow-50 dark:bg-yellow-400/20">
                            <span class="font-bold">Yellow</span> alert! You should check in on some of those fields
							below.
                        </Alert>
						
                        <Alert class="px-4 py-3 text-sm text-red-500 border border-transparent rounded-md bg-red-50 dark:bg-red-400/20">
                            <span class="font-bold">Red</span> alert! You should check in on some of those fields
							below.
                        </Alert>
						
                        <Alert class="px-4 py-3 text-sm text-purple-500 border border-transparent rounded-md bg-purple-50 dark:bg-purple-400/20">
                            <span class="font-bold">Warning</span> alert! You should check in on some of those fields
							below.
                        </Alert>
						
                        <Alert class="px-4 py-3 text-sm rounded-md text-slate-500 bg-slate-50 dark:bg-zink-400/20 dark:text-zink-200">
                            <span class="font-bold">Light</span> alert! You should check in on some of those fields
							below.
                        </Alert>
					</div>
				</div>
			</div>
		</div>

        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">Outline Alerts</h6>
                <div class="grid grid-cols-1 gap-5 xl:grid-cols-2">
                    <div class="flex flex-col gap-3">
                        <Alert class="px-4 py-3 text-sm bg-white border rounded-md border-custom-300 text-custom-500 dark:bg-zink-700 dark:border-custom-500">
                            <span class="font-bold">Custom</span> alert! You should check in on some of those fields below.
                        </Alert>

                        <Alert class="px-4 py-3 text-sm text-green-500 bg-white border border-green-300 rounded-md dark:bg-zink-700 dark:border-green-500">
                            <span class="font-bold">Green</span> alert! You should check in on some of those fields below.
                        </Alert>

                        <Alert class="px-4 py-3 text-sm text-orange-500 bg-white border border-orange-300 rounded-md dark:bg-zink-700 dark:border-orange-500">
                            <span class="font-bold">Orange</span> alert! You should check in on some of those fields below.
                        </Alert>

                        <Alert class="px-4 py-3 text-sm bg-white border rounded-md border-sky-300 text-sky-500 dark:bg-zink-700">
                            <span class="font-bold">Sky</span> alert! You should check in on some of those fields below.
                        </Alert>
                        
                    </div>
                    <div class="flex flex-col gap-3">

                        <Alert class="px-4 py-3 text-sm text-yellow-500 bg-white border border-yellow-300 rounded-md dark:bg-zink-700 dark:border-yellow-500">
                            <span class="font-bold">Yellow</span> alert! You should check in on some of those fields below.
                        </Alert>

                        <Alert class="px-4 py-3 text-sm text-red-500 bg-white border border-red-300 rounded-md dark:bg-zink-700 dark:border-red-500">
                            <span class="font-bold">Red</span> alert! You should check in on some of those fields below.
                        </Alert>

                        <Alert class="px-4 py-3 text-sm text-purple-500 bg-white border border-purple-300 rounded-md dark:bg-zink-700 dark:border-purple-500">
                            <span class="font-bold">Warning</span> alert! You should check in on some of those fields below.
                        </Alert>

                        <Alert class="px-4 py-3 text-sm bg-white border rounded-md text-slate-500 border-slate-300 dark:bg-zink-700 dark:border-zink-500 dark:text-zink-200">
                            <span class="font-bold">Light</span> alert! You should check in on some of those fields below.
                        </Alert>
                    </div>
                </div>
            </div>
        </div>
        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">Bordered Alerts</h6>
                <div class="grid grid-cols-1 gap-5 xl:grid-cols-2">
                    <div class="flex flex-col gap-3">

                        <Alert class="px-4 py-3 text-sm border rounded-md border-custom-200 text-custom-500 bg-custom-50 dark:bg-custom-400/20 dark:border-custom-500/50">
                            <span class="font-bold">Custom</span> alert! You should check in on some of those fields below.
                        </Alert>

                        <Alert class="px-4 py-3 text-sm text-green-500 border border-green-200 rounded-md bg-green-50 dark:bg-green-400/20 dark:border-green-500/50">
                            <span class="font-bold">Green</span> alert! You should check in on some of those fields below.
                        </Alert>

                        <Alert class="px-4 py-3 text-sm text-orange-500 border border-orange-200 rounded-md bg-orange-50 dark:bg-orange-400/20 dark:border-orange-500/50">
                            <span class="font-bold">Orange</span> alert! You should check in on some of those fields below.
                        </Alert>

                        <Alert class="px-4 py-3 text-sm border rounded-md border-sky-200 text-sky-500 bg-sky-50 dark:bg-sky-400/20 dark:border-sky-500/50">
                            <span class="font-bold">Sky</span> alert! You should check in on some of those fields below.
                        </Alert>
                    </div>
                    <div class="flex flex-col gap-3">

                        <Alert class="px-4 py-3 text-sm text-yellow-500 border border-yellow-200 rounded-md bg-yellow-50 dark:bg-yellow-400/20 dark:border-yellow-500/50">
                            <span class="font-bold">Yellow</span> alert! You should check in on some of those fields below.
                        </Alert>

                        <Alert class="px-4 py-3 text-sm text-red-500 border border-red-200 rounded-md bg-red-50 dark:bg-red-400/20 dark:border-red-500/50">
                            <span class="font-bold">Red</span> alert! You should check in on some of those fields below.
                        </Alert>

                        <Alert class="px-4 py-3 text-sm text-purple-500 border border-purple-200 rounded-md bg-purple-50 dark:bg-purple-400/20 dark:border-purple-500/50">
                            <span class="font-bold">Warning</span> alert! You should check in on some of those fields below.
                        </Alert>

                        <Alert class="px-4 py-3 text-sm border rounded-md text-slate-500 border-slate-200 bg-slate-50 dark:bg-zink-500/30 dark:border-zink-500 dark:text-zink-200">
                            <span class="font-bold">Light</span> alert! You should check in on some of those fields below.
                        </Alert>
                    </div>
                </div>
            </div>
        </div>
        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">Dismiss Alerts</h6>
                <div class="grid grid-cols-1 gap-5 xl:grid-cols-2">
                    <div class="flex flex-col gap-3">
                        <Alert class="relative px-4 py-3 text-sm border border-transparent rounded-md text-custom-500 bg-custom-50 dark:bg-custom-400/20" dismissible dismissibleClass="absolute top-0 bottom-0 right-0 p-3 transition text-custom-200 hover:text-custom-500 dark:text-custom-400/50 dark:hover:text-custom-500">
                            <span class="font-bold">Custom</span> alert! You should check in on some of those fields
							below.</Alert>
                        <Alert class="relative px-4 py-3 text-sm text-green-500 border border-transparent rounded-md bg-green-50 dark:bg-green-400/20" dismissible dismissibleClass="absolute top-0 bottom-0 right-0 p-3 text-green-200 transition hover:text-green-500 dark:text-green-400/50 dark:hover:text-green-500">
                            <span class="font-bold">Green</span> alert! You should check in on some of those fields
							below.</Alert>
                        <Alert class="relative px-4 py-3 text-sm text-orange-500 border border-transparent rounded-md bg-orange-50 dark:bg-orange-400/20" dismissible dismissibleClass="absolute top-0 bottom-0 right-0 p-3 text-orange-200 transition hover:text-orange-500 dark:text-orange-400/50 dark:hover:text-orange-500">
                            <span class="font-bold">Orange</span> alert! You should check in on some of those fields
							below.</Alert>
                        <Alert class="relative px-4 py-3 text-sm border border-transparent rounded-md text-sky-500 bg-sky-50 dark:bg-sky-400/20" dismissible dismissibleClass="absolute top-0 bottom-0 right-0 p-3 transition text-sky-200 hover:text-sky-500 dark:text-sky-400/50 dark:hover:text-sky-500">
                            <span class="font-bold">Sky</span> alert! You should check in on some of those fields
							below.</Alert>
					</div>
					<div class="flex flex-col gap-3">
						
                        <Alert class="relative px-4 py-3 text-sm text-yellow-500 border border-transparent rounded-md bg-yellow-50 dark:bg-yellow-400/20" dismissible dismissibleClass="absolute top-0 bottom-0 right-0 p-3 text-yellow-200 transition hover:text-yellow-500 dark:text-yellow-400/50 dark:hover:text-yellow-500">
                            <span class="font-bold">Yellow</span> alert! You should check in on some of those fields
							below.
                        </Alert>
						
                        <Alert class="relative px-4 py-3 text-sm text-red-500 border border-transparent rounded-md bg-red-50 dark:bg-red-400/20" dismissible dismissibleClass="absolute top-0 bottom-0 right-0 p-3 text-red-200 transition hover:text-red-500 dark:text-red-400/50 dark:hover:text-red-500">
                            <span class="font-bold">Red</span> alert! You should check in on some of those fields
							below.
                        </Alert>
						
                        <Alert class="relative px-4 py-3 text-sm text-purple-500 border border-transparent rounded-md bg-purple-50 dark:bg-purple-400/20" dismissible dismissibleClass="absolute top-0 bottom-0 right-0 p-3 text-purple-200 transition hover:text-purple-500 dark:text-purple-400/50 dark:hover:text-purple-500">
                            <span class="font-bold">Warning</span> alert! You should check in on some of those fields
							below.
                        </Alert>
						
                        <Alert class="relative px-4 py-3 text-sm rounded-md text-slate-500 bg-slate-50 dark:bg-zink-400/20 dark:text-zink-200" dismissible dismissibleClass="absolute top-0 bottom-0 right-0 p-3 transition text-slate-200 hover:text-slate-500 dark:text-zink-300 dark:hover:text-zink-200">
                            <span class="font-bold">Light</span> alert! You should check in on some of those fields
							below.
                        </Alert>
					</div>
                </div>
            </div>
        </div>
        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">Alerts with Icon</h6>
                <div class="grid grid-cols-1 gap-5 xl:grid-cols-2">
                    <div class="flex flex-col gap-3">
                        <Alert class="flex gap-1 px-4 py-3 text-sm border rounded-md md:items-center border-custom-200 text-custom-500 bg-custom-50 dark:bg-custom-400/20 dark:border-custom-500/50">
                            <LucideIcon name="AlertCircle" class="h-4"/>
                            <span class="font-bold">Custom</span> alert! You should check in on some of those fields
							below.</Alert>
                        <Alert class="flex gap-1 px-4 py-3 text-sm text-green-500 border border-green-200 rounded-md md:items-center bg-green-50 dark:bg-green-400/20 dark:border-green-500/50">
                            <LucideIcon name="AlertCircle" class="h-4"/>
                            <span class="font-bold">Green</span> alert! You should check in on some of those fields
							below.</Alert>
                        <Alert class="flex gap-1 px-4 py-3 text-sm text-orange-500 border border-orange-200 rounded-md md:items-center bg-orange-50 dark:bg-orange-400/20 dark:border-orange-500/50">
                            <LucideIcon name="AlertCircle" class="h-4"/>
                            <span class="font-bold">Orange</span> alert! You should check in on some of those fields
							below.</Alert>
                        <Alert class="flex gap-1 px-4 py-3 text-sm border rounded-md md:items-center border-sky-200 text-sky-500 bg-sky-50 dark:bg-sky-400/20 dark:border-sky-500/50">
                            <LucideIcon name="AlertCircle" class="h-4"/>
                            <span class="font-bold">Sky</span> alert! You should check in on some of those fields
							below.</Alert>
					</div>
					<div class="flex flex-col gap-3">
						
                        <Alert class="flex gap-1 px-4 py-3 text-sm text-yellow-500 border border-yellow-200 rounded-md md:items-center bg-yellow-50 dark:bg-yellow-400/20 dark:border-yellow-500/50">
                            <LucideIcon name="AlertCircle" class="h-4"/>
                            <span class="font-bold">Yellow</span> alert! You should check in on some of those fields
							below.
                        </Alert>
						
                        <Alert class="flex gap-1 px-4 py-3 text-sm text-red-500 border border-red-200 rounded-md md:items-center bg-red-50 dark:bg-red-400/20 dark:border-red-500/50">
                            <LucideIcon name="AlertCircle" class="h-4"/>
                            <span class="font-bold">Red</span> alert! You should check in on some of those fields
							below.
                        </Alert>
						
                        <Alert class="flex gap-1 px-4 py-3 text-sm text-purple-500 border border-purple-200 rounded-md md:items-center bg-purple-50 dark:bg-purple-400/20 dark:border-purple-500/50">
                            <LucideIcon name="AlertCircle" class="h-4"/>
                            <span class="font-bold">Warning</span> alert! You should check in on some of those fields
							below.
                        </Alert>
						
                        <Alert class="flex gap-1 px-4 py-3 text-sm border rounded-md md:items-center text-slate-500 border-slate-200 bg-slate-50 dark:bg-zink-500/30 dark:border-zink-500 dark:text-zink-200">
                            <LucideIcon name="AlertCircle" class="h-4"/>
                            <span class="font-bold">Light</span> alert! You should check in on some of those fields
							below.
                        </Alert>
					</div>
                </div>
            </div>
        </div>
        <div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Alerts with Images</h6>
				<div class="grid grid-cols-1 gap-5 xl:grid-cols-2">
                    <div class="flex flex-col gap-3">
                        <Alert class="flex gap-2 px-4 py-3 text-sm border rounded-md md:items-center border-custom-200 text-custom-500 bg-custom-50 dark:bg-custom-400/20 dark:border-custom-500/50">
                            <img src="/assets/images/users/avatar-1.png" alt="" class="size-8 rounded-full">
                            <span class="font-bold">Custom</span> alert! You should check in on some of those fields
							below.</Alert>
                        <Alert class="flex gap-2 px-4 py-3 text-sm text-green-500 border border-green-200 rounded-md md:items-center bg-green-50 dark:bg-green-400/20 dark:border-green-500/50">
                            <img src="/assets/images/users/avatar-2.png" alt="" class="size-8 rounded-full">
                            <span class="font-bold">Green</span> alert! You should check in on some of those fields
							below.</Alert>
                        <Alert class="flex gap-2 px-4 py-3 text-sm text-orange-500 border border-orange-200 rounded-md md:items-center bg-orange-50 dark:bg-orange-400/20 dark:border-orange-500/50">
                            <img src="/assets/images/users/avatar-3.png" alt="" class="size-8 rounded-full">
                            <span class="font-bold">Orange</span> alert! You should check in on some of those fields
							below.</Alert>
                        <Alert class="flex gap-2 px-4 py-3 text-sm border rounded-md md:items-center border-sky-200 text-sky-500 bg-sky-50 dark:bg-sky-400/20 dark:border-sky-500/50">
                            <img src="/assets/images/users/avatar-4.png" alt="" class="size-8 rounded-full">
                            <span class="font-bold">Sky</span> alert! You should check in on some of those fields
							below.</Alert>
					</div>
					<div class="flex flex-col gap-3">
						
                        <Alert class="flex gap-2 px-4 py-3 text-sm text-yellow-500 border border-yellow-200 rounded-md md:items-center bg-yellow-50 dark:bg-yellow-400/20 dark:border-yellow-500/50">
                            <img src="/assets/images/users/avatar-5.png" alt="" class="size-8 rounded-full">
                            <span class="font-bold">Yellow</span> alert! You should check in on some of those fields
							below.
                        </Alert>
						
                        <Alert class="flex gap-2 px-4 py-3 text-sm text-red-500 border border-red-200 rounded-md md:items-center bg-red-50 dark:bg-red-400/20 dark:border-red-500/50">
                            <img src="/assets/images/users/avatar-6.png" alt="" class="size-8 rounded-full">
                            <span class="font-bold">Red</span> alert! You should check in on some of those fields
							below.
                        </Alert>
						
                        <Alert class="flex gap-2 px-4 py-3 text-sm text-purple-500 border border-purple-200 rounded-md md:items-center bg-purple-50 dark:bg-purple-400/20 dark:border-purple-500/50">
                            <img src="/assets/images/users/avatar-7.png" alt="" class="size-8 rounded-full">
                            <span class="font-bold">Warning</span> alert! You should check in on some of those fields
							below.
                        </Alert>
						
                        <Alert class="flex gap-2 px-4 py-3 text-sm border rounded-md md:items-center text-slate-500 border-slate-200 bg-slate-50 dark:bg-zink-500/30 dark:border-zink-500 dark:text-zink-200">
                            <img src="/assets/images/users/avatar-8.png" alt="" class="size-8 rounded-full">
                            <span class="font-bold">Light</span> alert! You should check in on some of those fields
							below.
                        </Alert>
					</div>
                </div>
			</div>
		</div>
        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">Top Border Alerts</h6>
                <div class="grid grid-cols-1 gap-5 xl:grid-cols-2">
                    <div class="flex flex-col gap-3">
                        <Alert class="relative p-3 pr-12 text-sm border border-t-2 border-transparent rounded-md border-t-custom-500 text-custom-500 bg-custom-50 dark:bg-custom-400/20 dark:border-t-custom-500/50" dismissible dismissibleClass="absolute top-0 bottom-0 right-0 p-3 transition text-custom-200 hover:text-custom-500 dark:text-custom-400/50 dark:hover:text-custom-500">
                            <span class="font-bold">Custom</span> alert! You should check in on some of those fields below.
                        </Alert>
                    </div>
                    <div class="flex flex-col gap-3">
                        <Alert class="relative p-3 pr-12 text-sm text-green-500 border border-t-2 border-transparent rounded-md bg-green-50 border-t-green-300 dark:bg-green-400/20 dark:border-t-green-500/50" dismissible dismissibleClass="absolute top-0 bottom-0 right-0 p-3 text-green-200 transition hover:text-green-500 dark:text-green-400/50 dark:hover:text-green-500">
                            <span class="font-bold">Green</span> alert! You should check in on some of those fields below.
                        </Alert>
                    </div>
                </div>
            </div>
        </div>
        <div class="card">
            <div class="card-body">
                <h6 class="mb-4 text-15">Content Alerts</h6>
                <div class="grid grid-cols-1 gap-5 xl:grid-cols-2">
                    <div class="flex flex-col gap-3">
                        <Alert class="flex gap-3 p-4 text-sm rounded-md text-custom-500 bg-custom-50 dark:bg-custom-400/20">
                            <LucideIcon name="AlertCircle" class="inline-block size-4 mt-0.5 shrink-0" />
                            <div>
                                <h6 class="mb-1">Hi! A simple Primary alert —check it out!</h6>
                                <ul class="ml-2 list-disc list-inside">
                                    <li>Send the billing agreement</li>
                                    <li>Send over all the documentation.</li>
                                    <li>Meeting with daron to review the intake form</li>
                                </ul>
                            </div>
                        </Alert>
                        <Alert class="flex gap-3 p-4 text-sm text-green-500 rounded-md bg-green-50 dark:bg-green-400/20">
                            <LucideIcon name="CheckCircle" class="inline-block size-4 mt-0.5 shrink-0" />
                            <div>
                                <h6 class="mb-1">Yey! Everything worked! A simple success alert —check it out!</h6>
                                <ul class="ml-2 list-disc list-inside">
                                    <li>Send the billing agreement</li>
                                    <li>Send over all the documentation.</li>
                                    <li>Meeting with daron to review the intake form</li>
                                </ul>
                            </div>
                        </Alert>
                    </div>
                    <div class="flex flex-col gap-3">
                        <Alert class="flex gap-3 p-4 text-sm text-yellow-500 rounded-md bg-yellow-50 dark:bg-yellow-400/20">
                            <LucideIcon name="AlertCircle" class="inline-block size-4 mt-0.5 shrink-0" />
                            <div>
                                <h6 class="mb-1">Uh oh, something went wrong!</h6>
                                <p>Better check yourself, you're not looking too good. Whenever you need to, be sure to use margin utilities to keep things nice and tidy. If your device is running an outdated version of Android, this could be causing the issue. To check for updates, open your device's Settings app, go to System, and then tap System update.</p>
                            </div>
                        </Alert>
                        <Alert class="flex gap-3 p-4 text-sm text-red-500 rounded-md bg-red-50 dark:bg-red-400/20">
                            <LucideIcon name="AlertTriangle" class="inline-block size-4 mt-0.5 shrink-0" />
                            <div>
                                <h6 class="mb-1">Something is very wrong!</h6>
                                <p class="mb-2">Change a few things up and try submitting again. A few informal nouns for a mistake are a blooper, boo-boo, and blunder. We also have hiccup, which means a minor problem/mistake that was corrected.</p>
                                <a href="#!" class="font-semibold text-red-600 transition hover:text-red-700">Read More <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" data-lucide="move-right" class="lucide lucide-move-right inline-block h-4 align-middle"><path d="M18 8L22 12L18 16"></path><path d="M2 12H22"></path></svg></a>
                            </div>
                        </Alert>
                    </div>
                </div>
            </div>
        </div>

        <div class="card">
			<div class="card-body">
				<h6 class="mb-4 text-15">Fill Solid Alerts</h6>
				<div class="grid grid-cols-1 gap-5 xl:grid-cols-2">
                    <div class="flex flex-col gap-3">
                        <Alert class="relative p-3 pr-12 text-sm border border-transparent rounded-md text-custom-50 bg-custom-500" dismissible dismissibleClass="absolute top-0 bottom-0 right-0 p-3 transition text-custom-200 hover:text-custom-100">
                            <span class="font-bold">Custom</span> alert! You should check in on some of those fields
							below.</Alert>
                        <Alert class="relative p-3 pr-12 text-sm bg-green-500 border border-transparent rounded-md text-green-50" dismissible dismissibleClass="absolute top-0 bottom-0 right-0 p-3 text-green-200 transition hover:text-green-100">
                            <span class="font-bold">Green</span> alert! You should check in on some of those fields
							below.</Alert>
                        <Alert class="relative p-3 pr-12 text-sm bg-orange-500 border border-transparent rounded-md text-orange-50" dismissible dismissibleClass="absolute top-0 bottom-0 right-0 p-3 text-orange-200 transition hover:text-orange-100">
                            <span class="font-bold">Orange</span> alert! You should check in on some of those fields
							below.</Alert>
                        <Alert class="relative p-3 pr-12 text-sm border border-transparent rounded-md text-sky-50 bg-sky-500" dismissible dismissibleClass="absolute top-0 bottom-0 right-0 p-3 transition text-sky-200 hover:text-sky-100">
                            <span class="font-bold">Sky</span> alert! You should check in on some of those fields
							below.</Alert>
					</div>
					<div class="flex flex-col gap-3">
						
                        <Alert class="relative p-3 pr-12 text-sm bg-yellow-500 border border-transparent rounded-md text-yellow-50" dismissible dismissibleClass="absolute top-0 bottom-0 right-0 p-3 text-yellow-200 transition hover:text-yellow-100">
                            <span class="font-bold">Yellow</span> alert! You should check in on some of those fields
							below.
                        </Alert>
						
                        <Alert class="relative p-3 pr-12 text-sm bg-red-500 border border-transparent rounded-md text-red-50" dismissible dismissibleClass="absolute top-0 bottom-0 right-0 p-3 text-red-200 transition hover:text-red-100">
                            <span class="font-bold">Red</span> alert! You should check in on some of those fields
							below.
                        </Alert>
						
                        <Alert class="relative p-3 pr-12 text-sm bg-purple-500 border border-transparent rounded-md text-purple-50" dismissible dismissibleClass="absolute top-0 bottom-0 right-0 p-3 text-purple-200 transition hover:text-purple-100">
                            <span class="font-bold">Warning</span> alert! You should check in on some of those fields
							below.
                        </Alert>
						
                        <Alert class="relative p-3 pr-12 text-sm border border-transparent rounded-md text-slate-50 bg-slate-500 dark:bg-zink-500 dark:border-zink-500 dark:text-zink-100" dismissible dismissibleClass="absolute top-0 bottom-0 right-0 p-3 transition text-slate-200 hover:text-slate-100">
                            <span class="font-bold">Light</span> alert! You should check in on some of those fields
							below.
                        </Alert>
					</div>
				</div>
			</div>
		</div>
	</div>
</div>
