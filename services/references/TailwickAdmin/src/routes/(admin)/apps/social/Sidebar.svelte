<script>
	import LucideIcon from "../../../../common/components/LucideIcon.svelte";
    export let link;
</script>
<div class="xl:col-span-3">
    <div class="card">
        <div class="card-body">
            <div class="flex items-center gap-3 mb-5">
                <div class="rounded-full bg-slate-100 shrink-0 dark:bg-zink-600">
                    <img src="/assets/images/users/avatar-1.png" alt="" class="rounded-full h-14">
                </div>
                <div class="grow">
                    <h6 class="mb-1 text-15">Paula Keenan <LucideIcon name="BadgeCheck" class="inline-block size-4 text-sky-500 fill-sky-100 dark:fill-sky-500/20"/></h6>
                    <p class="text-slate-500 dark:text-zink-200">@paulak145</p>
                </div>
            </div>
            <div class="grid grid-cols-1 text-center divide-y sm:divide-y-0 sm:divide-x sm:grid-cols-3 divide-slate-200 dark:divide-zink-500 divide-dashed rtl:divide-x-reverse">
                <div class="py-3 sm:py-0 sm:px-3">
                    <h6>489</h6>
                    <p class="text-slate-500 dark:text-zink-200">Posts</p>
                </div>
                <div class="py-3 sm:py-0 sm:px-3">
                    <h6>4.6k</h6>
                    <p class="text-slate-500 dark:text-zink-200">Follower</p>
                </div>
                <div class="py-3 sm:py-0 sm:px-3">
                    <h6>159</h6>
                    <p class="text-slate-500 dark:text-zink-200">Following</p>
                </div>
            </div>
        </div>
    </div><!--end card-->
    <div class="card">
        <div class="card-body">
            <ul class="flex flex-col w-full gap-2 mb-4 text-sm font-medium shrink-0 nav-tabs">
                <li class="group grow {link == "feed" ? "active":""}">
                    <a href="/dashboard/social-media" class="inline-block px-4 w-full py-2 text-base transition-all duration-300 ease-linear rounded-md text-slate-500 dark:text-zink-200 border border-transparent group-[.active]:bg-custom-500 dark:group-[.active]:bg-custom-500 group-[.active]:text-white dark:group-[.active]:text-white hover:text-custom-500 dark:hover:text-custom-500 active:text-custom-500 dark:active:text-custom-500 -mb-[1px]"><LucideIcon name="Home" class="inline-block size-4 align-middle ltr:mr-1 rtl:ml-1"/> <span class="align-middle">Feed</span></a>
                </li>
                <li class="group grow {link == "friends" ? "active":""}">
                    <a href="/apps/social/friends" class="inline-block px-4 w-full py-2 text-base transition-all duration-300 ease-linear rounded-md text-slate-500 dark:text-zink-200 border border-transparent group-[.active]:bg-custom-500 dark:group-[.active]:bg-custom-500 group-[.active]:text-white dark:group-[.active]:text-white hover:text-custom-500 dark:hover:text-custom-500 active:text-custom-500 dark:active:text-custom-500 -mb-[1px]"><LucideIcon name="User2" class="inline-block size-4 align-middle ltr:mr-1 rtl:ml-1"/> <span class="align-middle">Friends</span></a>
                </li>
                <li class="group grow {link == "events" ? "active":""}">
                    <a href="/apps/social/events" class="inline-block px-4 w-full py-2 text-base transition-all duration-300 ease-linear rounded-md text-slate-500 dark:text-zink-200 border border-transparent group-[.active]:bg-custom-500 dark:group-[.active]:bg-custom-500 group-[.active]:text-white dark:group-[.active]:text-white hover:text-custom-500 dark:hover:text-custom-500 active:text-custom-500 dark:active:text-custom-500 -mb-[1px]"><LucideIcon name="CalendarDays" class="inline-block size-4 align-middle ltr:mr-1 rtl:ml-1"/> <span class="align-middle">Event</span></a>
                </li>
                <li class="group grow {link == "video" ? "active":""}">
                    <a href="/apps/social/video" class="inline-block px-4 w-full py-2 text-base transition-all duration-300 ease-linear rounded-md text-slate-500 dark:text-zink-200 border border-transparent group-[.active]:bg-custom-500 dark:group-[.active]:bg-custom-500 group-[.active]:text-white dark:group-[.active]:text-white hover:text-custom-500 dark:hover:text-custom-500 active:text-custom-500 dark:active:text-custom-500 -mb-[1px]"><LucideIcon name="Clapperboard" class="inline-block size-4 align-middle ltr:mr-1 rtl:ml-1"/> <span class="align-middle">Watch Video</span></a>
                </li>
                <li class="group grow {link == "marketplace" ? "active":""}">
                    <a href="/apps/social/marketplace" class="inline-block px-4 w-full py-2 text-base transition-all duration-300 ease-linear rounded-md text-slate-500 dark:text-zink-200 border border-transparent group-[.active]:bg-custom-500 dark:group-[.active]:bg-custom-500 group-[.active]:text-white dark:group-[.active]:text-white hover:text-custom-500 dark:hover:text-custom-500 active:text-custom-500 dark:active:text-custom-500 -mb-[1px]"><LucideIcon name="ShoppingCart" class="inline-block size-4 align-middle ltr:mr-1 rtl:ml-1"/> <span class="align-middle">Marketplace</span></a>
                </li>
                <li class="group grow">
                    <a href="/pages/account-settings" class="inline-block px-4 w-full py-2 text-base transition-all duration-300 ease-linear rounded-md text-slate-500 dark:text-zink-200 border border-transparent group-[.active]:bg-custom-500 dark:group-[.active]:bg-custom-500 group-[.active]:text-white dark:group-[.active]:text-white hover:text-custom-500 dark:hover:text-custom-500 active:text-custom-500 dark:active:text-custom-500 -mb-[1px]"><LucideIcon name="Settings" class="inline-block size-4 align-middle ltr:mr-1 rtl:ml-1"/> <span class="align-middle">Settings</span></a>
                </li>
            </ul>
            <p class="mb-4 text-slate-500 dark:text-zink-200">Suggestion Pages</p>
            <ul class="flex flex-col gap-4">
                <li>
                    <a href="#!" class="flex gap-3">
                        <img src="/assets/images/brand/adwords.png" alt="" class="h-5">
                        <h6>Harmonious Team</h6>
                    </a>
                </li>
                <li>
                    <a href="#!" class="flex gap-3">
                        <img src="/assets/images/brand/twitter.png" alt="" class="h-5">
                        <h6>Twitter</h6>
                    </a>
                </li>
                <li>
                    <a href="#!" class="flex gap-3">
                        <img src="/assets/images/brand/meta.png" alt="" class="h-5">
                        <h6>Design Stack</h6>
                    </a>
                </li>
                <li>
                    <a href="#!" class="flex gap-3">
                        <img src="/assets/images/brand/figma.png" alt="" class="h-5">
                        <h6>UI / UX Community</h6>
                    </a>
                </li>
            </ul>
        </div>
    </div><!--end card-->
    <div class="card">
        <div class="card-body">
            <h6 class="mb-4 text-15">Active Friends</h6>
            <div class="flex flex-wrap items-center gap-3">
                <a href="#!" class="relative">
                    <img src="/assets/images/users/avatar-2.png" alt="" class="h-10 border rounded-full border-slate-200 dark:border-zink-500">
                    <span class="top-0 ltr:right-0 rtl:left-0 absolute  w-2.5 h-2.5 bg-green-400 border-2 border-white dark:border-zink-700 rounded-full"></span>
                </a>
                <a href="#!" class="relative">
                    <img src="/assets/images/users/avatar-3.png" alt="" class="h-10 border rounded-full border-slate-200 dark:border-zink-500">
                    <span class="top-0 ltr:right-0 rtl:left-0 absolute  w-2.5 h-2.5 bg-green-400 border-2 border-white dark:border-zink-700 rounded-full"></span>
                </a>
                <a href="#!" class="relative">
                    <img src="/assets/images/users/avatar-4.png" alt="" class="h-10 border rounded-full border-slate-200 dark:border-zink-500">
                    <span class="top-0 ltr:right-0 rtl:left-0 absolute  w-2.5 h-2.5 bg-green-400 border-2 border-white dark:border-zink-700 rounded-full"></span>
                </a>
                <a href="#!" class="relative">
                    <img src="/assets/images/users/avatar-5.png" alt="" class="h-10 border rounded-full border-slate-200 dark:border-zink-500">
                    <span class="top-0 ltr:right-0 rtl:left-0 absolute  w-2.5 h-2.5 bg-green-400 border-2 border-white dark:border-zink-700 rounded-full"></span>
                </a>
                <a href="#!" class="relative">
                    <img src="/assets/images/users/avatar-6.png" alt="" class="h-10 border rounded-full border-slate-200 dark:border-zink-500">
                    <span class="top-0 ltr:right-0 rtl:left-0 absolute  w-2.5 h-2.5 bg-green-400 border-2 border-white dark:border-zink-700 rounded-full"></span>
                </a>
                <a href="#!" class="relative">
                    <img src="/assets/images/users/avatar-7.png" alt="" class="h-10 border rounded-full border-slate-200 dark:border-zink-500">
                    <span class="top-0 ltr:right-0 rtl:left-0 absolute  w-2.5 h-2.5 bg-green-400 border-2 border-white dark:border-zink-700 rounded-full"></span>
                </a>
                <a href="#!" class="relative">
                    <img src="/assets/images/users/avatar-8.png" alt="" class="h-10 border rounded-full border-slate-200 dark:border-zink-500">
                    <span class="top-0 ltr:right-0 rtl:left-0 absolute  w-2.5 h-2.5 bg-green-400 border-2 border-white dark:border-zink-700 rounded-full"></span>
                </a>
                <a href="#!" class="relative">
                    <img src="/assets/images/users/avatar-9.png" alt="" class="h-10 border rounded-full border-slate-200 dark:border-zink-500">
                    <span class="top-0 ltr:right-0 rtl:left-0 absolute  w-2.5 h-2.5 bg-green-400 border-2 border-white dark:border-zink-700 rounded-full"></span>
                </a>
                <a href="#!" class="relative">
                    <img src="/assets/images/users/avatar-10.png" alt="" class="h-10 border rounded-full border-slate-200 dark:border-zink-500">
                    <span class="top-0 ltr:right-0 rtl:left-0 absolute  w-2.5 h-2.5 bg-green-400 border-2 border-white dark:border-zink-700 rounded-full"></span>
                </a>
                <a href="#!" class="relative flex items-center justify-center size-10 rounded-full bg-custom-100 text-custom-500 dark:bg-custom-500/20">
                    5+
                </a>
            </div>
        </div>
    </div><!--end card-->
</div>